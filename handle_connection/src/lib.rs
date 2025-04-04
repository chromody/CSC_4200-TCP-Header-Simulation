use std::net::{TcpStream};
use std::io::{Read, Write, stdin, BufRead};
use simple_crypt::{encrypt, decrypt};

mod tcp_header;
mod tcp_packet;
use tcp_header::{TCPHeader, to_tcp_header};
use tcp_packet::{TCPPacket, to_tcp_packet};

const KEY: &str = "THISISINSECURE";

fn display_recieved(mut stream: &TcpStream) -> Option<()> {
    // Buffer to store the received message
    let mut buffer = Vec::new();//dynamic array
    println!("Waiting for message...");
    stream.read_to_end(&mut buffer).ok()?;//beginning to end allows for buffer to be dynamic
    println!("Read message...");

    let decrypted_message = match decrypt(&buffer, KEY.as_bytes()) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error with decrypting message: {}", e);
            return None;
        },
    };
    let decrypted_message_string = String::from_utf8(decrypted_message).unwrap();

    println!("Received: {}", decrypted_message_string);
    return Some(());
}

pub fn handle_recieving(mut stream: &TcpStream) -> Option<()> {
    display_recieved(&stream);
    return Some(());
}

// Handle client connection
pub fn handle_sending(mut stream: &TcpStream) -> Option<()> {
    // Send "HELLO" to the client
    let mut message: String = String::new();//our input string
    println!("Please enter your message: ");

    let stdin = stdin(); //our stdin
    let mut handle = stdin.lock(); //locking our only stdin to prevent race conditions
    handle.read_line(&mut message).ok()?; //reading in and checking if ok
    let encrypted_message = encrypt(message.as_bytes(), KEY.as_bytes()).unwrap();//encrypting message

    let source_port = stream.local_addr().unwrap().port();
    let dest_port = stream.peer_addr().unwrap().port();
    let sequence_number = 0;
    let ack_flag = 0;
    let syn_flag = 0;
    let fin_flag = 0;
    let payload_size = 0;
    let header = TCPHeader::new(source_port, dest_port, sequence_number, ack_flag, syn_flag, fin_flag, payload_size);
    let mut packet = TCPPacket::new(header, encrypted_message.clone()).ok();
    if packet.is_none() {
        return None;
    }
    let packet_buffer = packet?.to_bytes();

    stream.write_all(&packet_buffer).expect("Failed to write to stream");//write to screen if failed

    print!("\nSent: {}", message); // write to terminal

    return Some(()); //return nothing
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_crypt::{encrypt, decrypt};

    #[test]
    fn test_encode_decode() {
        let message = "Hello World".to_string();
        let encrypted_message: Vec<u8> = encrypt(message.as_bytes(), KEY.as_bytes()).unwrap();
        let encrypted_message_string = encrypted_message.clone().into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");

        let decoded_string: Vec<u8> = encrypted_message_string.clone()
        .split(',')
        .filter_map(|s| s.parse::<u8>().ok()) // Convert back to u8
        .collect();
        println!("{:?}", encrypted_message);
        
        let decrypted_message = decrypt(&decoded_string, KEY.as_bytes()).unwrap();
        let decrypted_message_string = String::from_utf8(decrypted_message).unwrap();
        assert_eq!(message, decrypted_message_string);
    }
}
