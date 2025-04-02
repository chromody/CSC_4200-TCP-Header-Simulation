use std::net::{TcpListener};
use std::thread;
use handle_connection::{handle_recieving};

// Server function
fn start_server() -> Option<()> {
    // Bind the server to localhost on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to bind server");

    println!("Server listening on 127.0.0.1:7878");

    // Accept incoming connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                // Spawn a new thread to handle the connection
                thread::spawn(move || {
                    handle_recieving(&stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to accept connection: {}", e);
            }
        }
    }
    return Some(());
}

fn main() {
    // Start the server in a separate thread
    let server_thread = thread::spawn(|| { // spawning a thread for our server
        start_server(); //interesting
    });

    // Give the server some time to start
    std::thread::sleep(std::time::Duration::from_secs(1));//race condition??

    // Wait for the server thread to finish (it won't in this example)
    server_thread.join().unwrap();
}