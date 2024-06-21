// Test with:
// - osc-client (this package)
// - oscsend 127.0.0.1 3131 /test/address s "hello, world!" (cli tool shipped with liblo)

use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};

use clap::Parser;
use rosc::OscPacket;

/// Receive OSC messages from client
#[derive(Parser)]
#[command(version, long_about = None)]
struct Cli {
    /// Server IP address
    #[arg(short, long)]
    addr: Option<String>,

    /// Server port number
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() {
    let cli = Cli::parse();

    let addr = match cli.addr.as_deref() {
        Some(ip) => match ip.parse::<Ipv4Addr>() {
            Ok(ip) => ip,
            Err(error) => {
                println!("{error}, default to 0.0.0.0");
                Ipv4Addr::UNSPECIFIED
            }
        },
        None => Ipv4Addr::UNSPECIFIED
    };

    let port = match cli.port {
        Some(num) => if num < 1024 {
            println!("server cannot bind to system port, default to 3131");
            3131
        } else {
            num
        },
        None => 3131
    };

    println!("value for addr: {addr}");
    println!("value for port: {port}");

    // Allow server to receive and send from/to any IP address ("0.0.0.0")
    let addr = SocketAddrV4::new(addr, port);

    let socket = UdpSocket::bind(addr)
        .expect("cannot bind socket");

    let mut buffer = [0u8; rosc::decoder::MTU];

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((size, addr)) => {
                println!("received packet with size {size} from: {addr}");

                let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])
                    .expect("error decoding message");

                match packet {
                    OscPacket::Message(msg) => {
                        println!("osc address: {}", msg.addr);
                        println!("osc arguments: {:?}", msg.args);
                    }
                    OscPacket::Bundle(bundle) => {
                        println!("osc bundle: {:?}", bundle);
                    }
                }
            }
            Err(error) => {
                println!("error receiving message: {}", error);
                break;
            }
        }
    }
}
