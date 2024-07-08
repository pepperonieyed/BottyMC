use std::{io::BufReader, net::TcpStream};

use crate::network::datatypes::varint;
use crate::network::io;

pub fn from_buffer(reader: &mut BufReader<&TcpStream>) -> String {
    // Get length of string
    let length = varint::from_buffer(reader);
    println!("Length of string: {length}");

    // Create buffer for rest of string
    let bytes = io::read_bytes(reader, length as u64);

    String::from_utf8(bytes).unwrap()
}
