use std::io::Read;

use pktparse::{ethernet, ipv4};

extern crate tun;

fn main() {
	let mut config = tun::Configuration::default();
	config.address((10, 0, 0, 1))
	       .netmask((255, 255, 255, 0))
	       .up();

	#[cfg(target_os = "linux")]
	config.platform(|config| {
		config.packet_information(true);
	});

	let mut dev = tun::create(&config).unwrap();
	let mut buf = [0; 4096];

	loop {
		let amount = dev.read(&mut buf).unwrap();
        if let Ok((remaining, header)) = ipv4::parse_ipv4_header(&buf) {
        println!("{}", header.source_addr);
        }
		//println!("{:?}", &buf[0 .. amount]);
	}
}
