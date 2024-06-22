// Test with:
// - osc-server (this package)
// - oscdump 3131 (shipped with liblo)

use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use clap::Parser;
use rosc::{OscPacket,OscMessage,OscType};

/// Send a message to an OSC server
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Server IP address (default: 127.0.0.1)
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number (default: 3131)
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let addr = match cli.addr.as_deref() {
        Some(ip_str) => match ip_str.parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(error) => {
                panic!("error: {error}");
            }
        },
        // By default, send messages to localhost only (127.0.0.1)
        None => Ipv4Addr::LOCALHOST,
    };

    let port = match cli.port {
        Some(num) => num,
        // By default, send messages to this port
        None => 3131,
    };

    let server_addr = SocketAddrV4::new(addr, port);

    // Allow client to send message to any IP address (0.0.0.0)
    // with a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);

    let socket = UdpSocket::bind(client_addr)
        .expect("error: cannot bind socket");

    let packet = OscPacket::Message(OscMessage {
        addr: "/client/message".to_string(),
        args: vec![OscType::String("hi!".to_string())]
    });

    let buffer = rosc::encoder::encode(&packet)
        .expect("error: cannot encode message");

    println!("sending message to {server_addr}");

    socket.send_to(&buffer, server_addr)
        .expect("error: cannot send message");
}

