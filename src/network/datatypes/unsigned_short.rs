use std::{io::BufReader, net::TcpStream};

use crate::network::io;

pub fn from_buffer(reader: &mut BufReader<&TcpStream>) -> u16 {
    let bytes = io::read_bytes(reader, 2);

    // Big endian from network
    let value: u16 = *bytes.get(1).unwrap() as u16 | (*bytes.get(0).unwrap() as u16) << 8;

    value
}
