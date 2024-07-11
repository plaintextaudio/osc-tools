use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use clap::Parser;

/// Send a message to an OSC server
#[derive(Parser)]
#[command(styles(osc_tools::color_help()), version)]
struct Args {
    /// OSC address (e.g. /synth)
    address: String,

    /// OSC types (e.g. ifs)
    types: Option<String>,

    /// OSC values (e.g. 6 -12.00 message)
    #[arg(allow_negative_numbers = true)]
    values: Vec<String>,

    /// Server IP address
    #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST)]
    addr: Ipv4Addr,

    /// Server port number
    #[arg(short, long, default_value_t = 3131)]
    port: u16,

    /// Time to wait for reply (in ms)
    #[arg(short, long, default_value_t = 500, value_name = "TIME")]
    wait: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    let server_addr = SocketAddrV4::new(args.addr, args.port);

    // Allow client to send a message to any IP address (0.0.0.0)
    // from a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(client_addr)?;

    // Send message
    println!("Sending message:");
    let message = osc_tools::parse_cli_osc(&args.address, &args.types, &args.values)?;
    osc_tools::send_packet(&socket, server_addr, &message)?;

    if args.wait > 0 {
        socket.set_read_timeout(Some(Duration::from_millis(args.wait)))?;
    } else {
        println!("Timeout disabled, press Ctrl+C to exit...");
    }

    // Receive reply
    let mut buffer = [0u8; rosc::decoder::MTU];
    let (reply_addr, _) = osc_tools::recv_packet(&socket, &mut buffer)?;

    if reply_addr != server_addr {
        Err("send and reply address mismatch")?
    }

    Ok(())
}
