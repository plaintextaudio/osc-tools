use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use anyhow::{Context, Result};
use rosc::OscPacket;

pub fn send_msg(socket: &UdpSocket, server_addr: SocketAddrV4, packet: &OscPacket) -> Result<()> {
    let buffer = rosc::encoder::encode(packet).with_context(|| "Cannot encode message")?;

    println!("Sending message to {}", server_addr);

    socket
        .send_to(&buffer, server_addr)
        .with_context(|| "Cannot send message")?;

    Ok(())
}

pub fn recv_msg(socket: &UdpSocket, server_addr: SocketAddrV4) -> Result<()> {
    let mut buffer = [0u8; rosc::decoder::MTU];

    let (size, reply_addr) = socket
        .recv_from(&mut buffer)
        .with_context(|| "Cannot receive reply")?;

    if reply_addr != SocketAddr::V4(server_addr) {
        panic!("Alert: send and reply address mismatch");
    }

    println!("Received packet of {size} bytes from {reply_addr}");

    let (_, packet) =
        rosc::decoder::decode_udp(&buffer[..size]).with_context(|| "Cannot decode reply")?;

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
