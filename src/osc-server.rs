use std::net::{SocketAddrV4, Ipv4Addr, UdpSocket};
use rosc::OscPacket;

fn main() {
    let addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 3131);

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
