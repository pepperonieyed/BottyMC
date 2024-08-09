use std::net::TcpStream;
use std::io::BufReader;

use super::unsigned_byte;

pub fn from_buffer(reader: &mut BufReader<TcpStream>) -> i32 {
    // Get the size of the incoming packet from the first up to 5 bytes
    let mut position: u8 = 0;
    let mut value: i32 = 0;
    loop {
        let next_byte = unsigned_byte::from_buffer(reader);
        value |= (next_byte as i32 & 0x7F) << position;

        if next_byte & 0x80 == 0 {
            break;
        }

        position += 7;

        if position >= 32 {
            panic!("VarInt is too big");
        }
    }

    value
}
