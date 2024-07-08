use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, BufWriter};

mod network;
use network::datatypes::{varint, string, unsigned_short};

fn handle_client(stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut writer = BufWriter::new(&stream);

    let packet_len = varint::from_buffer(&mut reader);

    println!("Size of incoming packet: {packet_len}");

    process_packet(&mut reader, packet_len as usize);
}

// Reads a packet from a buffer
fn process_packet(reader: &mut BufReader<&TcpStream>, packet_len: usize) {
    // Get the ID of this packet so we know how to proceed
    let packet_id = varint::from_buffer(reader);
    println!("ID of incoming packet: {packet_id:#X}");

    if packet_id == 0x0 {
        let protocol_version = varint::from_buffer(reader);
        println!("Incoming protocol version: {protocol_version}");

        let server_address = string::from_buffer(reader);
        println!("Server address: {server_address}");

        let server_port = unsigned_short::from_buffer(reader);
        println!("Server port: {server_port}");

        let next_state = varint::from_buffer(reader);
        println!("Next state: {next_state}");
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:25565")
        .unwrap();

    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    Ok(())
}
