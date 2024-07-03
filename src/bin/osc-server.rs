// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /oscsend/message s "hello!" (shipped with liblo)

use std::error;
use std::net::UdpSocket;

use clap::Parser;
use rosc::{OscPacket, OscType};

/// Receive messages from OSC clients
#[derive(Parser)]
#[command(styles(osc_tools::colors()))]
#[command(version, long_about = None)]
struct Arguments {
    /// Server IP address  (default: 0.0.0.0)
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number (default: 3131)
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Arguments::parse();

    let server_addr = osc_tools::parse_addr(args.addr, args.port, "0.0.0.0")?;

    if server_addr.port() < 1024 {
        Err("cannot bind socket to system port")?;
    }

    let socket = UdpSocket::bind(server_addr)?;

    let mut buffer = [0u8; rosc::decoder::MTU];
    println!("Waiting for messages on {}", server_addr);

    loop {
        let (client_addr, message) = osc_tools::recv_packet(&socket, &mut buffer)?;

        match message {
            OscPacket::Message(msg) => {
                println!("{:?}", msg);

                match &msg.args[0] {
                    OscType::String(s) => {
                        if s == "stop" {
                            println!("\nStopping server...");
                            println!("Sending reply to {}", client_addr);
                            let reply = osc_tools::fill_packet("/server/reply", "stopping server");
                            osc_tools::send_packet(&socket, client_addr, &reply)?;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            OscPacket::Bundle(bun) => {
                println!("{:?}", bun);
                break;
            }
        }

        println!("Sending reply to {}", client_addr);
        let reply = osc_tools::fill_packet("/server/reply", "message received");
        osc_tools::send_packet(&socket, client_addr, &reply)?;
    }

    Ok(())
}
