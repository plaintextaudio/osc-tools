use std::error;
use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

use clap::Parser;
use rosc::{OscPacket, OscType};

/// Receive messages from OSC clients
#[derive(Parser)]
#[command(styles(osc_tools::colors()), version)]
struct Args {
    /// Server IP address
    #[arg(short, long, default_value_t = Ipv4Addr::UNSPECIFIED)]
    addr: Ipv4Addr,

    /// Server port number
    #[arg(short, long, default_value_t = 3131)]
    port: u16,
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let args = Args::parse();

    if args.port < 1024 {
        Err("cannot bind socket to system port")?;
    }

    let server_addr = SocketAddrV4::new(args.addr, args.port);
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

        let reply = osc_tools::fill_packet("/server/reply", "message received");
        osc_tools::send_packet(&socket, client_addr, &reply)?;
    }

    Ok(())
}
