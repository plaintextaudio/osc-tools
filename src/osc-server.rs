// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /oscsend/message s "hello!" (shipped with liblo)

use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use anyhow::{Context, Result};
use clap::Parser;
use rosc::OscPacket;

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
            panic!("error: cannot bind socket to system port");
        } else {
            num
        },
        None => 3131,
    };

    let server_addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(server_addr)
        .with_context(|| "Cannot bind socket")?;

    let mut buffer = [0u8; rosc::decoder::MTU];

    println!("waiting for messages on {server_addr}");

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, client_addr)) => {
                println!("received packet of {size} bytes from {client_addr}");

                let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])
                    .expect("error: cannot decode message");

                match packet {
                    OscPacket::Message(msg) => {
                        println!("{:?}", msg);
                    }
                    OscPacket::Bundle(bun) => {
                        println!("{:?}", bun);
                    }
                }
            }
            Err(error) => {
                println!("error: cannot receive message: {}", error);
                break;
            }
        }
    }

    Ok(())
}
