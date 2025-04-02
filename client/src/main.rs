use std::net::{TcpStream};
use handle_connection::{handle_sending};

// Client function
fn start_client() -> Option<()> {
    // Connect to the server
    let stream = match TcpStream::connect("127.0.0.1:7878") {
        Ok(stream) => {
            println!("Connected to server at 127.0.0.1:7878");
            stream
        },
        Err(e) => {
            eprintln!("Failed to connect to server: {}", e);
            return None; // Return early if connection fails
        }
    };
    println!("Connected to server at 127.0.0.1:7878");

    handle_sending(&stream);

    stream.shutdown(std::net::Shutdown::Write).expect("Failed to shut down connection");
    println!("Shutdown connection");

    return Some(()); //return nothing
}

fn main() {
    // Start the client
    start_client();//client is not running in a separate thread
}