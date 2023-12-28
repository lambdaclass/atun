use std::{
    io::{self, Read, Write},
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    sync::{Mutex, RwLock},
};
use tun::platform::posix::{Reader, Writer};

pub struct Peer {
    endpoint: Mutex<Option<SocketAddrV4>>,
}

impl Peer {
    pub fn new(socket: Option<SocketAddrV4>) -> Self {
        Self {
            endpoint: Mutex::new(socket),
        }
    }

    pub fn endpoint(&self) -> Option<SocketAddrV4> {
        self.endpoint.lock().unwrap().clone()
    }

    pub fn set_endpoint(&self, addr: SocketAddrV4) {
        let mut endpoint = self.endpoint.lock().unwrap();

        if endpoint.is_none() {
            *endpoint = Some(addr);
        }
    }
}

/// A representation of a VPN interface
pub struct VpnDevice {
    socket: UdpSocket,
    /// tun device
    interface_reader: RwLock<Reader>,
    interface_writer: RwLock<Writer>,
    peer: Peer,
}

impl VpnDevice {
    pub fn new(peer: Peer) -> Self {
        let mut config = tun::Configuration::default();

        let dev_addr = if peer.endpoint().is_some() {
            (10, 8, 0, 3)
        } else {
            (10, 8, 0, 1)
        };

        config
            .address(dev_addr)
            .netmask((255, 255, 255, 0))
            .name("utun5")
            .mtu(1400)
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });

        let interface = tun::create(&config).unwrap();
        let (interface_reader, interface_writer) = interface.split();

        Self {
            socket: UdpSocket::bind("0.0.0.0:19988").expect("port is already in use"),
            interface_reader: RwLock::new(interface_reader),
            interface_writer: RwLock::new(interface_writer),
            peer,
        }
    }

    pub fn loop_listen_iface(&self) -> io::Result<()> {
        // a large enough buffer, recall the MTU on iface was to be set to 1472
        let mut buf = [0u8; 1504];

        loop {
            let peer = &self.peer.endpoint();
            let nbytes = self.interface_reader.write().unwrap().read(&mut buf[..])?;

            if let Some(peer_addr) = peer.as_ref() {
                println!("PEER ADDR: {}", peer_addr);
                println!("BYTES: {:?}", &buf[..nbytes]);
                self.socket.send_to(&buf[..nbytes], peer_addr)?;
            } else {
                println!("..no peer");
            }
        }
    }

    pub fn loop_listen_udp(&self) -> io::Result<()> {
        let mut buf = [0u8; 1504];

        loop {
            let (nbytes, peer_addr) = self.socket.recv_from(&mut buf[..])?;

            if let SocketAddr::V4(peer_addr_v4) = peer_addr {
                println!("Peer connected with address {}", peer_addr_v4);
                if contains_sequence(&buf[..nbytes], b"hello?") {
                    println!("\"handshake\" received");
                    self.peer.set_endpoint(peer_addr_v4);
                    continue;
                }
                self.interface_writer
                    .write()
                    .unwrap()
                    .write(&buf[..nbytes])?;
            }
        }
    }
}

pub fn contains_sequence<T: PartialEq>(sequence: &[T], subsequence: &[T]) -> bool {
    sequence
        .windows(subsequence.len())
        .any(|window| window == subsequence)
}
