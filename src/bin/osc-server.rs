// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /oscsend/message s "hello!" (shipped with liblo)

use std::error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

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

    let addr = match args.addr.as_deref() {
        Some(ip) => ip,
        None => "0.0.0.0",
    };

    let addr = addr.parse::<Ipv4Addr>()?;

    let port = match args.port {
        Some(num) => num,
        None => 3131,
    };

    if port < 1024 {
        Err("cannot bind socket to system port")?;
    }

    let server_addr = SocketAddrV4::new(addr, port);
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
