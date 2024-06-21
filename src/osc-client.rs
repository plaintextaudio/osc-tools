use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use clap::Parser;
use rosc::{OscPacket,OscMessage,OscType};

// Test with:
// - osc-server (this package)
// - oscdump 3131 (cli tool shipped with liblo)

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Server IP address
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let addr = match cli.addr.as_deref() {
        None => "127.0.0.1",
        Some(a) => a,
    };

    let port = match cli.port {
        None => 3131,
        Some(p) => p,
    };

    println!("value for addr: {addr}");
    println!("value for port: {port}");

    // Allow client to send and receive to/from any IP address ("0.0.0.0")
    // with a port number assigned by the operating system (":0")
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);

    // The server address and port number to exchange messages with
    let server_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), port);

    let socket = UdpSocket::bind(client_addr)
        .expect("cannot bind socket");

    let packet = OscPacket::Message(OscMessage{
        addr: "/greet/me".to_string(),
        args: vec![OscType::String("hi!".to_string())]
    });

    let buffer = rosc::encoder::encode(&packet)
        .expect("cannot encode message");

    socket.send_to(&buffer, server_addr)
        .expect("cannot send message");
}
