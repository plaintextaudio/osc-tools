// Test with:
// - osc-server (this package)
// - oscdump 3131 (shipped with liblo)

use std::error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use clap::{Parser, Subcommand};
use rosc::OscPacket;

/// Send a message to an OSC server
#[derive(Parser)]
#[command(version, long_about = None)]
struct Arguments {
    /// Server IP address  (default: 127.0.0.1)
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number (default: 3131)
    #[arg(short, long)]
    port: Option<u16>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a user message to server
    Message {
        /// Message to send
        msg: String,
    },
    /// Ask the server to stop
    Stop {},
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Arguments::parse();

    let server_addr = osc_tools::parse_addr(args.addr, args.port, "127.0.0.1")?;

    // Allow client to send a message to any IP address (0.0.0.0)
    // from a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(client_addr)?;

    match &args.command {
        Commands::Message { msg } => {
            // Send message to server
            println!("Sending message to {}", server_addr);
            let message = osc_tools::fill_packet("/client/message", msg);
            osc_tools::send_packet(&socket, server_addr, &message)?;
        }
        Commands::Stop {} => {
            // Send message to server
            println!("Sending message to {}", server_addr);
            let message = osc_tools::fill_packet("/client/message", "stop");
            osc_tools::send_packet(&socket, server_addr, &message)?;
        }
    }

    // Receive reply from server
    socket.set_read_timeout(Some(Duration::from_secs(3)))?;
    let mut buffer = [0u8; rosc::decoder::MTU];
    let (reply_addr, reply) = osc_tools::recv_packet(&socket, &mut buffer)?;

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
