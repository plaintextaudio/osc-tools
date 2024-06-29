use std::error;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use rosc::OscPacket;

pub fn send_msg(
    socket: &UdpSocket,
    peer_addr: &SocketAddrV4,
    packet: &OscPacket,
) -> Result<(), Box<dyn error::Error>> {
    let buffer = rosc::encoder::encode(packet)?;

    socket.send_to(&buffer, peer_addr)?;

    Ok(())
}

pub fn recv_msg(
    socket: &UdpSocket,
    buffer: &mut [u8],
) -> Result<(SocketAddrV4, OscPacket), Box<dyn error::Error>> {
    let (size, packet_addr) = socket.recv_from(buffer)?;
    println!("Received packet of {} bytes from {}", size, packet_addr);

    let packet_addr = match packet_addr {
        SocketAddr::V4(ipv4) => ipv4,
        SocketAddr::V6(_) => Err("IPv6 address not handled")?,
    };

    let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])?;

    Ok((packet_addr, packet))
}
