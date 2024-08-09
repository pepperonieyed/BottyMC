use std::{io::BufReader, net::TcpStream};

use crate::network::io;

pub fn from_buffer(reader: &mut BufReader<TcpStream>) -> u8 {
    let bytes = io::read_bytes(reader, 1);

    *bytes.get(0).unwrap() as u8
}
