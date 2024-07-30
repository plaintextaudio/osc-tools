use std::error::Error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};
use std::time::Duration;

use clap::Parser;
use rosc::OscMessage;

const DESCRIPTION: &str = "\
Types:
  i  32-bit signed integer
  f  32-bit floating point number
  s  string of ASCII characters

Examples:
  osc-client /server/status
  osc-client /synth ifs 6 -12.00 string
";

/// Send a message to an OSC server
#[derive(Parser)]
#[command(styles(osc_tools::color_help()), version)]
#[command(after_help = DESCRIPTION)]
struct Args {
    /// OSC address
    address: String,

    /// OSC types
    types: Option<String>,

    /// OSC values
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

    /// Use verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // Allow client to send a message to any IP address (0.0.0.0)
    // from a port number assigned by the operating system (0)
    let client_addr = SocketAddrV4::new(Ipv4Addr::UNSPECIFIED, 0);
    let socket = UdpSocket::bind(client_addr)?;

    let server_addr = SocketAddrV4::new(args.addr, args.port);

    // Initialize message
    let mut msg = OscMessage {
        addr: "/".to_string(),
        args: Vec::new(),
    };

    if args.verbose {
        println!("Sending message:");
        println!("addr:\t{}", args.address);
        println!("types:\t{:?}", args.types);
        println!("values:\t{:?}", args.values);
    }

    // Send message
    msg.addr = args.address;
    msg.args = osc_tools::parse_osc_args(&args.types, &args.values)?;
    osc_tools::send_packet(&socket, &msg, &server_addr)?;

    if args.wait > 0 {
        socket.set_read_timeout(Some(Duration::from_millis(args.wait)))?;
    } else {
        println!("\nRead timeout disabled, press Ctrl+C to exit");
    }

    // Receive reply
    let mut buffer = [0u8; rosc::decoder::MTU];
    let (reply_addr, _) = osc_tools::recv_packet(&socket, &mut buffer, args.verbose)?;

    if reply_addr != server_addr {
        Err("send and reply address mismatch")?
    }

    Ok(())
}
