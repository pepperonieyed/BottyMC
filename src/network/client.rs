use std::{io::{BufReader, BufWriter}, net::TcpStream};
use crate::network::datatypes::{varint, string, unsigned_short};

enum ClientState {
    Status,
    Login,
    Transfer
}

/*
 * This is used for initial connections before the client logs in as a
 * player
 */
struct Client {
    stream: TcpStream,
    reader: BufReader<TcpStream>,
    writer: BufWriter<TcpStream>,
    state: ClientState
}

pub fn spawn(stream: TcpStream) {
    // All state
    let mut state = Client {
        stream: stream.try_clone().unwrap(),
        reader: BufReader::new(stream.try_clone().unwrap()),
        writer: BufWriter::new(stream),
        state: ClientState::Status
    };

    // Length of packet
    let packet_length = varint::from_buffer(&mut state.reader);
    println!("Length of incoming packet: {packet_length}");

    // Get the ID of this packet so we know how to proceed
    let packet_id = varint::from_buffer(&mut state.reader);
    println!("ID of incoming packet: {packet_id:#X}");

    match packet_id {
        0x0 => {
            let protocol_version = varint::from_buffer(&mut state.reader);
            println!("Incoming protocol version: {protocol_version}");

            let server_address = string::from_buffer(&mut state.reader);
            println!("Server address: {server_address}");

            let server_port = unsigned_short::from_buffer(&mut state.reader);
            println!("Server port: {server_port}");

            let next_state = varint::from_buffer(&mut state.reader);
            println!("Next state: {next_state}");
        }

        _ => {
            // Drop this request as state has become invalid
            println!("Invalid packet ID {packet_id:#X}");
        }
    }
}
