# Simple Client-Server Communication

In this assignment, you will implement a basic client-server model using TCP sockets to facilitate
communication between a server and multiple clients. The server will listen for incoming connections, process
messages sent by clients, log these messages, and send an acknowledgment back to the respective client. The
client will establish a connection with the server, send a user-provided message, and receive/display the
server's response.
This project aims to introduce you to socket programming, multi-threaded or asynchronous network
handling, and the basics of encryption in network communication. By working on this assignment, you will
gain practical experience with TCP/IP protocols, secure messaging, and software development best
practices in a collaborative environment using GitHub.

Professor Brown said we could use any language, so I decided to use rust so I could better learn it. Note, I split up the client and server to make it easier to test multiple clients with a single server. Also another note, the server runs on a socket, so do not try to run the server project multiple times. This was not stated to be a requirement.

## Table of Contents

- [Installation](#installation)
- [Required Libraries](#required-libraries)
- [Building and Running](#building-and-running)

## Installation

1. Clone the repository:
   ```sh
   git clone https://github.com/chromody/CSC_4200-TCP-Client.git

2. Install Language Dependency(ies)\
   [rust](https://www.rust-lang.org/tools/install)
   [more info](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Required Libraries
   We use cargo to manage our rust installation and packages, sort of how you can use npm/node to manage js. Running cargo install in any of the rust projects will install all dependancys, the same with cargo run.
   1. [simple_crypt](https://crates.io/crates/simple_crypt)
      We use simple_crypt to encrpyt the info when we send it, and when we recieve it. I did not want to completely implement my own cryptographic scheme, as this is not a cybersecurity class. I am not a cybersecurtity student.
   2. [std::net::{TcpStream, TcpListener}](https://doc.rust-lang.org/std/net/struct.TcpStream.html?search=tcp)
      Rust's standard library has a tcp stream automatically built into it. The instructions did not specify to use a udp or tcp stream, so I opted to use a tcp stream. This does
      somewhat make the point of the tcp handshake we implement pointless, as this is handled by rust implicitly. Tcp listener is used to accept the tcp stream at that port.
   4. [std::io::{Read, Write, stdin, BufRead}](https://doc.rust-lang.org/std/io/index.html)
      To read from terminal and write to terminal, we use Read and Write.
      To do this, rust requires the use of stdin, as propery concurancy is important to rust. So it is required to lock input and output due to mutual exclusivity.\
      BufRead is to read the sent info through our tcp stream.
   5. [std::thread](https://doc.rust-lang.org/std/thread/)
      From the standard library thread is used to create a new thread for concurrancy
   7. handle_connection
      This is a library i created just for this assignment, properly split up all the handling. Otherwise I would just have one main.rs with everything.
## Building and Running
   We have a makefile to handle the building, running, and cleaning of this project. Though this is unrequired due to using cargo to manage everything. The makefile
   is effectively just a wrapper for cargo. Warning, I developed the makefile to be used on windows. This is due to it navigating to the server and client cargo projects to run.

   ```sh
   make build
   ```
   This builds each folder with cargo build.
   ```sh
   make run
   ```
   This runs the server once and the client 4 times, to test concurrancy. Though, make run will automatically run cargo build if it isnt built, so the cargo build is effectively
   useless.
   ```sh
   make clean
   ```
   This cleans each project folder
   ```
   make test
   ```
   This runs a server and a single client.
