
use std::net::SocketAddr;
use clap::{Parser, arg};
mod types;
extern crate tun;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// IP of the peer, followed by ":" and the port
    /// For example: 127.0.0.1:8080
    #[arg(long)]
    peer: Option<String>
}

fn print_banner(){
println!("Atun ready to swim ");
println!(r" 
         /`-._
        /_,.._`:-    
    ,.-'  ,   ``-:_,-')
   : o ):';       _  |  
    `-._ `'__,.-''\`-.)
        `\\  \,.-'``
        ");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {


    let args = Args::parse();

    if args.peer == Option::None {
        println!("No peer provided, starting in server mode\n");
    } else {
        let peer_address_result = args.peer.unwrap().parse::<SocketAddr>();
    
        if let Ok(_peer_address) = peer_address_result {

        } else {
            println!("Couldn't decode peer address");
            println!("Remember to add a port to the ip, for example: 127.0.0.1:8080");
        }
        println!("")
    }

    print_banner();
    // run(args.peer.as_deref())?;

    Ok(())

}


/*


Max   /`-._
     /_,.._`:-    
 ,.-'  ,   ``-:_,-')
: o ):';       _  {  
 `-._ `'__,.-''\`-.)
     `\\  \,.-'``
     
     */
