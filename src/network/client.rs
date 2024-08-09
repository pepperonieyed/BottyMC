use std::{io::{BufReader, BufWriter, Read}, net::TcpStream};
use crate::network::{datatypes::{string, unsigned_byte, unsigned_short, varint}, io};

enum ClientState {
    Status = 1,
    Login = 2,
    Transfer = 3
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

    // Special case for legacy ping packets
    if packet_length & 0xFF == 0xFE {
        println!("LEGACY PING");
        return;
    }
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

            match next_state {
                next_state if next_state == ClientState::Status as i32 => {
                    // We're just responding with status, no need to instantiate Client
                },
                next_state if next_state == ClientState::Login as i32 => {

                },
                next_state if next_state == ClientState::Transfer as i32 => {

                },
                _ => println!("Invalid next state: {next_state}. Dropping connection.")
            }
        }

        _ => {
            // Drop this request as state has become invalid
            println!("Invalid packet ID {packet_id:#X}");
        }
    }
}
