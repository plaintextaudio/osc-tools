use std::error;
use std::io::ErrorKind;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use clap::builder::{styling, Styles};
use rosc::{OscMessage, OscPacket, OscType};

pub fn colors() -> Styles {
    styling::Styles::styled()
        .usage(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .header(styling::AnsiColor::Blue.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::White.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::White.on_default())
}

pub fn fill_packet(osc_addr: &str, osc_args: &str) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: osc_addr.to_string(),
        args: vec![OscType::String(osc_args.to_string())],
    })
}

pub fn send_packet(
    socket: &UdpSocket,
    peer_addr: SocketAddrV4,
    packet: &OscPacket,
) -> Result<(), Box<dyn error::Error>> {
    let buffer = rosc::encoder::encode(packet)?;

    socket.send_to(&buffer, peer_addr)?;

    Ok(())
}

pub fn recv_packet(
    socket: &UdpSocket,
    buffer: &mut [u8],
) -> Result<(SocketAddrV4, OscPacket), Box<dyn error::Error>> {
    let (size, packet_addr) = match socket.recv_from(buffer) {
        Ok(packet) => packet,
        Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
            Err("timeout reached while receiving packet")?
        }
        Err(e) => Err(e)?,
    };
    println!("\nReceived packet of {} bytes from {}", size, packet_addr);

    let packet_addr = match packet_addr {
        SocketAddr::V4(ipv4) => ipv4,
        SocketAddr::V6(_) => Err("IPv6 address not handled")?,
    };

    let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])?;

    Ok((packet_addr, packet))
}
