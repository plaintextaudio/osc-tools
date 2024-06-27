// Test with:
// - osc-server (this package)
// - oscdump 3131 (shipped with liblo)

use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use anyhow::Result;
use clap::Parser;
use rosc::{OscMessage, OscPacket, OscType};

mod net;

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

    // Allow client to send a message to any IP address (0.0.0.0)
    // from a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);

    let socket = UdpSocket::bind(client_addr)?;

    let packet = OscPacket::Message(OscMessage {
        addr: "/client/message".to_string(),
        args: vec![OscType::String(cli.msg)],
    });

    net::send_msg(&socket, server_addr, &packet)?;
    net::recv_msg(&socket, server_addr)?;

    Ok(())
}
