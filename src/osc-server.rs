// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /oscsend/message s "hello!" (shipped with liblo)

use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use anyhow::{Context, Result};
use clap::Parser;
use rosc::{OscPacket, OscType};

/// Receive messages from OSC clients
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Server IP address (default: 0.0.0.0)
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
        None => "0.0.0.0",
    };

    let addr = addr.parse::<Ipv4Addr>()?;

    let port = match cli.port {
        Some(num) => if num < 1024 {
            panic!("Error: cannot bind socket to system port");
        } else {
            num
        },
        None => 3131,
    };

    let server_addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(server_addr)
        .with_context(|| "Cannot bind socket")?;

    let mut buffer = [0u8; rosc::decoder::MTU];

    println!("Waiting for messages on {server_addr}");

    loop {

        let (size, client_addr) = socket.recv_from(&mut buffer)
            .with_context(|| "Cannot receive message")?;

        println!("Received packet of {size} bytes from {client_addr}");

        let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])
            .with_context(|| "Cannot decode message")?;

        match packet {
            OscPacket::Message(msg) => {
                println!("{:?}", msg);

                match &msg.args[0] {
                    OscType::String(s) => if s == "stop" { break; },
                    _ => (),
                }
            }
            OscPacket::Bundle(bun) => {
                println!("{:?}", bun);
                break;
            }
        }
    }

    Ok(())
}
