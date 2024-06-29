// Test with:
// - osc-server (this package)
// - oscdump 3131 (shipped with liblo)

use std::error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use clap::Parser;
use rosc::{OscMessage, OscPacket, OscType};

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

fn main() -> Result<(), Box<dyn error::Error>> {
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

    let message = OscPacket::Message(OscMessage {
        addr: "/client/message".to_string(),
        args: vec![OscType::String(cli.msg)],
    });

    // Send message to server
    println!("Sending message to {}", server_addr);
    osc_utils::send_msg(&socket, &server_addr, &message)?;

    // Receive reply from server
    let mut buffer = [0u8; rosc::decoder::MTU];
    let (reply_addr, reply) = osc_utils::recv_msg(&socket, &mut buffer)?;

    if reply_addr != server_addr {
        Err("send and reply address mismatch")?
    }

    match reply {
        OscPacket::Message(msg) => {
            println!("{:?}", msg);
        }
        OscPacket::Bundle(bun) => {
            println!("{:?}", bun);
        }
    }

    Ok(())
}
