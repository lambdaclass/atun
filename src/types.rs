use std::{
    io::{self, Read},
    net::{SocketAddrV4, UdpSocket},
    sync::Mutex,
};
use tun::platform::Device;

pub struct Peer {
    endpoint: Mutex<Option<SocketAddrV4>>,
}

/// A representation of a VPN interface
pub struct VpnDevice {
    udp: UdpSocket,
    /// tun device
    iface: Device,
    peer: Peer,
}

impl VpnDevice {
    fn loop_listen_iface(&mut self) -> io::Result<()> {
        // a large enough buffer, recall the MTU on iface was to be set to 1472

        let mut buf = [0u8; 1504];

        loop {
            let nbytes = self.iface.read(&mut buf[..])?;

            let peer = &self.peer.endpoint.lock().unwrap();

            if let Some(peer_addr) = peer.as_ref() {
                self.udp.send_to(&buf[..nbytes], peer_addr)?;
            } else {
                eprintln!("..no peer");
            }
        }
    }
}
