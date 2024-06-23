// Test with:
// - osc-server (this package)
// - oscdump 3131 (shipped with liblo)

use std::net::{SocketAddr, SocketAddrV4, Ipv4Addr, UdpSocket};

use anyhow::{Context, Result};
use clap::Parser;
use rosc::{OscPacket, OscMessage, OscType};

/// Send a message to an OSC server
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// User message to send
    msg: String,

    /// Server IP address (default: 127.0.0.1)
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number (default: 3131)
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let addr = match cli.addr.as_deref() {
        Some(ip) => ip,
        None => "127.0.0.1",
    };

    let addr = addr.parse::<Ipv4Addr>()?;

    let port = match cli.port {
        Some(num) => num,
        None => 3131,
    };

    let server_addr = SocketAddrV4::new(addr, port);

    // Allow client to send message to any IP address (0.0.0.0)
    // with a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);

    let socket = UdpSocket::bind(client_addr)
        .with_context(|| "Cannot bind socket")?;

    let packet = OscPacket::Message(OscMessage {
        addr: "/client/message".to_string(),
        args: vec![OscType::String(cli.msg)],
    });

    let buffer = rosc::encoder::encode(&packet)
        .with_context(|| "Cannot encode message")?;

    println!("Sending message to {server_addr}");

    socket.send_to(&buffer, server_addr)
        .with_context(|| "Cannot send message")?;

    let mut buffer = [0u8; rosc::decoder::MTU];

    let (size, reply_addr) = socket.recv_from(&mut buffer)
        .with_context(|| "Cannot receive reply")?;

    if reply_addr != SocketAddr::V4(server_addr) {
        panic!("Alert: send and reply address mismatch");
    }

    println!("Received packet of {size} bytes from {reply_addr}");

    let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])
        .with_context(|| "Cannot decode reply")?;

    match packet {
        OscPacket::Message(msg) => {
            println!("{:?}", msg);
        }
        OscPacket::Bundle(bun) => {
            println!("{:?}", bun);
        }
    }

    Ok(())
}
