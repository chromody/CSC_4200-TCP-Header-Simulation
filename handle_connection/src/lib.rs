use std::net::{TcpStream};
use std::io::{Read, Write, stdin, BufRead};
use simple_crypt::{encrypt, decrypt};
const KEY: &str = "THISISINSECURE";

fn display_recieved(mut stream: &TcpStream) -> Option<()> {
    // Buffer to store the received message
    let mut buffer = Vec::new();//dynamic array
    println!("Waiting for message...");
    stream.read_to_end(&mut buffer).ok()?;//beginning to end allows for buffer to be dynamic
    println!("Read message...");

    // Convert the buffer to a string and print it
    //  encrypted_message.into_iter().map(|i| i.to_string()).collect::<String>(); undoing this
    let message = String::from_utf8_lossy(&buffer);
    let encrypted_message: Vec<u8> = message
    .split(',')
    .filter_map(|s| s.parse::<u8>().ok()) // Convert back to u8
    .collect();
    //println!("Received: {:?}", &encrypted_message);
    let decrypted_message = match decrypt(&encrypted_message, KEY.as_bytes()) {
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
    //--handshake
    stream.write_all(b"HELLO").expect("Failed to write stream"); // send start of handshake
    let mut buffer = [0; 3];//
    stream.read_exact(&mut buffer).ok()?;//beginning to end allows for buffer to be dynamic
    let handshake = String::from_utf8_lossy(&buffer);
    println!("{}", handshake);
    if handshake == "ACK" {
        println!("Handshake successful! Communication established.");
    } else {
        eprintln!("Error with handshake.");
        return None;
    }
    //---

    display_recieved(&stream);
    return Some(());
}

// Handle client connection
pub fn handle_sending(mut stream: &TcpStream) -> Option<()> {
    //--handshake
    let mut buffer = [0; 5];//cant use a dynamic array for some reason idk idk idk
    stream.read_exact(&mut buffer).ok()?;//check for handshake
    let handshake = String::from_utf8_lossy(&buffer);
    if handshake == "HELLO" {
        stream.write_all(b"ACK").expect("Failed to write stream");
        println!("Handshake successful! Communication established.");
    } else {
        eprintln!("Error with handshake.");
        return None;
    }
    //--handshake

    // Send "HELLO" to the client
    let mut message: String = String::new();//our input string
    println!("Please enter your message: ");


    let stdin = stdin(); //our stdin
    let mut handle = stdin.lock(); //locking our only stdin to prevent race conditions
    handle.read_line(&mut message).ok()?; //reading in and checking if ok
    let encrypted_message = encrypt(message.as_bytes(), KEY.as_bytes()).unwrap();//encrypting message
    //println!("{:?}", encrypted_message); // i need to a delimiter duh
    let encrypted_message_string = encrypted_message.into_iter().map(|i| i.to_string()).collect::<Vec<_>>().join(",");
    stream.write_all(encrypted_message_string.trim().as_bytes()).expect("Failed to write to stream");//write to screen if failed

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
        let encrypted_message = encrypt(message.as_bytes(), KEY.as_bytes()).unwrap();
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
