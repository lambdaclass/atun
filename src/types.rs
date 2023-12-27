use std::net::{SocketAddrV4, UdpSocket};
use tun::Device;
use parking_lot::Mutex;

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