use std::{
    io::{self, Read, Write},
    net::{SocketAddr, SocketAddrV4, UdpSocket},
    sync::{Mutex, MutexGuard},
};
use tun::platform::Device;

pub struct Peer {
    endpoint: Mutex<Option<SocketAddrV4>>,
}

impl Peer {
    pub fn new(socket: Option<SocketAddrV4>) -> Self {
        Self {
            endpoint: Mutex::new(socket),
        }
    }

    pub fn endpoint(&self) -> MutexGuard<Option<SocketAddrV4>> {
        self.endpoint.lock().unwrap()
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
    interface: Device,
    peer: Peer,
}

impl VpnDevice {
    pub fn new(peer: Peer) -> Self {
        let mut config = tun::Configuration::default();
        config
            .address((10, 0, 0, 1))
            .netmask((255, 255, 255, 0))
            .up();

        #[cfg(target_os = "linux")]
        config.platform(|config| {
            config.packet_information(true);
        });

        let interface = tun::create(&config).unwrap();

        Self {
            socket: UdpSocket::bind("127.0.0.1:19288").expect("port is already in use"),
            interface,
            peer,
        }
    }

    pub fn loop_listen_iface(&mut self) -> io::Result<()> {
        // a large enough buffer, recall the MTU on iface was to be set to 1472
        let mut buf = [0u8; 1504];

        loop {
            let nbytes = self.interface.read(&mut buf[..])?;
            let peer = &self.peer.endpoint();

            if let Some(peer_addr) = peer.as_ref() {
                self.socket.send_to(&buf[..nbytes], peer_addr)?;
            } else {
                eprintln!("..no peer");
            }
        }
    }

    pub fn loop_listen_udp(&mut self) -> io::Result<()> {
        let mut buf = [0u8; 1504];

        loop {
            let (nbytes, peer_addr) = self.socket.recv_from(&mut buf[..])?;

            if let SocketAddr::V4(peer_addr_v4) = peer_addr {
                if &buf[..nbytes] == b"hello?" {
                    eprintln!("\"handshake\" received");
                    self.peer.set_endpoint(peer_addr_v4);
                    continue;
                }
                self.interface.write(&buf[..nbytes])?;
            }
        }
    }
}
