use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use clap::Parser;

/// Receive messages from OSC clients
#[derive(Parser)]
#[command(styles(osc_tools::color_help()), version)]
struct Args {
    /// Server IP address
    #[arg(short, long, default_value_t = Ipv4Addr::UNSPECIFIED)]
    addr: Ipv4Addr,

    /// Server port number
    #[arg(short, long, default_value_t = 3131)]
    port: u16,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if args.port < 1024 {
        Err("cannot bind socket to system port")?;
    }

    let server_addr = SocketAddrV4::new(args.addr, args.port);
    let socket = UdpSocket::bind(server_addr)?;

    let mut buffer = [0u8; rosc::decoder::MTU];
    println!("Waiting for messages on {}", server_addr);

    loop {
        // Receive message
        let (client_addr, _) = osc_tools::recv_packet(&socket, &mut buffer)?;

        // Send reply
        let reply = osc_tools::fill_packet("/server/reply", "message received");
        osc_tools::send_packet(&socket, client_addr, &reply)?;
    }
}
