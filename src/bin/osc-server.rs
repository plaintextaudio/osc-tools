// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /oscsend/message s "hello!" (shipped with liblo)

use std::error;
use std::net::UdpSocket;

use clap::Parser;
use rosc::{OscMessage, OscPacket, OscType};

/// Receive messages from OSC clients
#[derive(Parser)]
#[command(version, long_about = None)]
struct Arguments {
    /// Server IP address (default: 0.0.0.0)
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number (default: 3131)
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Arguments::parse();

    let server_addr = osc_utils::parse_addr(args.addr, args.port, "0.0.0.0")?;

    if server_addr.port() < 1024 {
        Err("cannot bind socket to system port")?;
    }

    let socket = UdpSocket::bind(server_addr)?;

    let mut buffer = [0u8; rosc::decoder::MTU];
    println!("Waiting for messages on {}", server_addr);

    loop {
        let (client_addr, message) = osc_utils::recv_msg(&socket, &mut buffer)?;

        match message {
            OscPacket::Message(msg) => {
                println!("{:?}", msg);

                match &msg.args[0] {
                    OscType::String(s) => {
                        if s == "stop" {
                            println!("Stopping server");
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

        let reply = OscPacket::Message(OscMessage {
            addr: "/server/reply".to_string(),
            args: vec![OscType::String("message received!".to_string())],
        });

        println!("Sending reply to {}", client_addr);
        osc_utils::send_msg(&socket, &client_addr, &reply)?;
    }

    Ok(())
}
