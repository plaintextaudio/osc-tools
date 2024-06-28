use std::error;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use rosc::OscPacket;

pub fn send_msg(
    socket: &UdpSocket,
    server_addr: SocketAddrV4,
    packet: &OscPacket,
) -> Result<(), Box<dyn error::Error>> {
    let buffer = rosc::encoder::encode(packet)?;

    println!("Sending message to {}", server_addr);
    socket.send_to(&buffer, server_addr)?;

    Ok(())
}

pub fn recv_msg(
    socket: &UdpSocket,
    server_addr: SocketAddrV4,
) -> Result<(), Box<dyn error::Error>> {
    let mut buffer = [0u8; rosc::decoder::MTU];

    let (size, reply_addr) = socket.recv_from(&mut buffer)?;
    println!("Received packet of {size} bytes from {reply_addr}");

    if reply_addr != SocketAddr::V4(server_addr) {
        panic!("Alert: send and reply address mismatch");
    }

    let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])?;

    match packet {
        OscPacket::Message(msg) => {
            println!("{:?}", msg);
        }
        OscPacket::Bundle(bun) => {
            println!("{:?}", bun);
        }
    }

    Ok(())
}
