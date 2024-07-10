use std::error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use clap::{Parser, Subcommand};
use rosc::OscPacket;

/// Send a message to an OSC server
#[derive(Parser)]
#[command(styles(osc_tools::colors()), version)]
struct Args {
    /// Server IP address
    #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST)]
    addr: Ipv4Addr,

    /// Server port number
    #[arg(short, long, default_value_t = 3131)]
    port: u16,

    /// Time to wait for reply (in ms)
    #[arg(short, long, default_value_t = 500, value_name = "TIME")]
    wait: u64,

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
    let args = Args::parse();

    let server_addr = SocketAddrV4::new(args.addr, args.port);

    // Allow client to send a message to any IP address (0.0.0.0)
    // from a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(client_addr)?;

    match &args.command {
        Commands::Message { msg } => {
            let message = osc_tools::fill_packet("/client/message", msg);
            osc_tools::send_packet(&socket, server_addr, &message)?;
        }
        Commands::Stop {} => {
            let message = osc_tools::fill_packet("/client/message", "stop");
            osc_tools::send_packet(&socket, server_addr, &message)?;
        }
    }

    if args.wait > 0 {
        socket.set_read_timeout(Some(Duration::from_millis(args.wait)))?;
    } else {
        println!("Timeout disabled, press Ctrl+C to exit...");
    }

    // Receive reply from server
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
