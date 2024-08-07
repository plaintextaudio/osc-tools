use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use clap::Parser;
use rosc::{OscMessage, OscType};

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

    // Initialize reply
    let mut reply = OscMessage {
        addr: "/".to_string(),
        args: Vec::new(),
    };

    let mut buffer = [0u8; rosc::decoder::MTU];
    println!("Waiting for messages on {}", server_addr);
    println!("Press Ctrl+C to exit");

    loop {
        // Receive message
        let (client_addr, _) = osc_tools::recv_packet(&socket, &mut buffer, true)?;

        // Send reply
        reply.addr = "/server/reply".to_string();
        reply.args = vec![OscType::String("Message received".to_string())];
        osc_tools::send_packet(&socket, &reply, &client_addr)?;
    }
}
