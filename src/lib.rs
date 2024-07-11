use std::error::Error;
use std::io::ErrorKind;
use std::net::{SocketAddr, SocketAddrV4, UdpSocket};

use clap::builder::{styling, Styles};
use rosc::{OscMessage, OscPacket, OscType};

pub fn color_help() -> Styles {
    styling::Styles::styled()
        .usage(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .header(styling::AnsiColor::Green.on_default() | styling::Effects::BOLD)
        .literal(styling::AnsiColor::Cyan.on_default() | styling::Effects::BOLD)
        .placeholder(styling::AnsiColor::Cyan.on_default())
}

pub fn fill_packet(osc_addr: &str, osc_args: &str) -> OscPacket {
    OscPacket::Message(OscMessage {
        addr: osc_addr.to_string(),
        args: vec![OscType::String(osc_args.to_string())],
    })
}

pub fn parse_osc_args(
    types: &Option<String>,
    values: &Vec<String>,
) -> Result<Vec<OscType>, Box<dyn Error>> {
    let mut arguments: Vec<OscType> = Vec::new();

    match types {
        Some(t) => {
            if values.len() > t.chars().count() {
                Err("too many values provided for the given types")?;
            }

            for (i, c) in t.chars().enumerate() {
                if values.len() < (i + 1) {
                    Err("missing values for the given types")?;
                }
                match c {
                    'i' => arguments.push(OscType::Int(values[i].parse::<i32>()?)),
                    'f' => arguments.push(OscType::Float(values[i].parse::<f32>()?)),
                    's' => arguments.push(OscType::String(values[i].parse::<String>()?)),
                    _ => (), // TODO: Error on unknown type
                }
            }
        }
        None => (),
    }

    Ok(arguments)
}

pub fn send_packet(
    osc_addr: &str,
    osc_args: &[OscType],
    socket: &UdpSocket,
    peer_addr: SocketAddrV4,
) -> Result<(), Box<dyn Error>> {
    let packet = OscPacket::Message(OscMessage {
        addr: osc_addr.to_string(),
        args: osc_args.to_vec(),
    });

    let buffer = rosc::encoder::encode(&packet)?;

    socket.send_to(&buffer, peer_addr)?;

    Ok(())
}

pub fn recv_packet(
    socket: &UdpSocket,
    buffer: &mut [u8],
) -> Result<(SocketAddrV4, OscPacket), Box<dyn Error>> {
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

    match &packet {
        OscPacket::Message(msg) => {
            println!("{:?}", msg);
        }
        OscPacket::Bundle(_) => {
            Err("OSC bundles not supported")?;
        }
    }

    Ok((packet_addr, packet))
}
