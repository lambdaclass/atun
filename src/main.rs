use clap::{arg, Parser};
use std::{io, net::SocketAddr, sync::Arc};
use types::{Peer, VpnDevice};
mod types;
extern crate tun;

/// Simple program to greet a person
#[derive(Parser, Debug, Clone)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP of the peer, followed by ":" and the port
    /// For example: 127.0.0.1:8080
    #[arg(long)]
    peer: Option<String>,
}

fn print_banner() {
    println!("Atun ready to swim ");
    println!(
        r" 
         /`-._
        /_,.._`:-    
    ,.-'  ,   ``-:_,-')
   : o ):';       _  |  
    `-._ `'__,.-''\`-.)
        `\\  \,.-'``
        "
    );
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.peer == Option::None {
        println!("No peer provided, starting in server mode\n");
    } else {
        let peer_address_result = args.peer.clone().unwrap().parse::<SocketAddr>();

        if let Ok(peer_address) = peer_address_result {
            println!("Starting in client mode - Connected to server {peer_address}");
        } else {
            println!("Couldn't decode peer address");
            println!("Remember to add a port to the ip, for example: 127.0.0.1:8080");
        }
        println!("")
    }

    print_banner();
    run(args.peer.as_deref())?;

    Ok(())
}

fn run(peer_addr: Option<&str>) -> io::Result<()> {
    let peer = peer_addr
        .and_then(|addr| addr.parse::<SocketAddr>().ok())
        .and_then(|addr| {
            if let SocketAddr::V4(addr) = addr {
                Some(addr)
            } else {
                None
            }
        });

    let peer = Peer::new(peer);
    let dev = VpnDevice::new(peer);

    let dev1 = Arc::new(dev);
    let dev2 = Arc::clone(&dev1);

    let join_handle_1 = std::thread::spawn(move || {
        if let Err(err) = (*dev1).loop_listen_iface() {
            eprintln!("err loop 1: {:?}", err);
        }
    });

    let join_handle_2 = std::thread::spawn(move || {
        if let Err(err) = (*dev2).loop_listen_udp() {
            eprintln!("err loop 2: {:?}", err);
        }
    });

    join_handle_1.join().unwrap();
    join_handle_2.join().unwrap();

    Ok(())
}
