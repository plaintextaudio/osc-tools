use std::error;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, UdpSocket};

use rosc::{OscMessage, OscPacket, OscType};

pub fn parse_addr(
    args_addr: Option<String>,
    args_port: Option<u16>,
    default: &str,
) -> Result<SocketAddrV4, Box<dyn error::Error>> {
    let addr = match args_addr.as_deref() {
        Some(ip) => ip,
        None => default,
    };

    let addr = addr.parse::<Ipv4Addr>()?;

    let port = match args_port {
        Some(num) => num,
        None => 3131,
    };

    Ok(SocketAddrV4::new(addr, port))
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
    let (size, packet_addr) = socket.recv_from(buffer)?;
    println!("\nReceived packet of {} bytes from {}", size, packet_addr);

    let packet_addr = match packet_addr {
        SocketAddr::V4(ipv4) => ipv4,
        SocketAddr::V6(_) => Err("IPv6 address not handled")?,
    };

    let (_, packet) = rosc::decoder::decode_udp(&buffer[..size])?;

    Ok((packet_addr, packet))
}
