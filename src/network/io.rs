use std::net::TcpStream;
use std::io::{Read, BufReader};

pub fn read_bytes(reader: &mut BufReader<&TcpStream>, num_bytes: u64) -> Vec<u8> {
    let mut buffer = vec![];
    reader
        .take(num_bytes)
        .read_to_end(&mut buffer)
        .expect("Failed to read from TCP stream");

    buffer
}
