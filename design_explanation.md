# Communication Method
The server and client communicate using a tcp connection. The server is listents on a socket with a tcp listener, which is recieving info from the client with a tcp stream. [tcplistener](https://doc.rust-lang.org/std/net/struct.TcpListener.html) and [tcpstream](https://doc.rust-lang.org/std/net/struct.TcpStream.html).

# Concurrancy Method:
For concurrancy of handling multiple clients, rust's basic implementation is used. Rust's [fearless concurrancy](https://doc.rust-lang.org/book/ch16-00-concurrency.html) means no direct asyncronous or multithreading is needed. A major point of rust is to safetly handle concurrent programming safetly and effeciently. Rust's [ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) policy means that it is nearly impossible to break mutual exclusion. Throughout this whole project "borrowing" is used, meaning each value, like our tcpstream, has a owner at every moment.

# Encryption Method:
As this is not a cybersecurity class, and I am not a cybersecurity major, I opted to use a library to encrpyt the data sent from the client to the server. [simple_crypt](https://docs.rs/simple_crypt/latest/simple_crypt/) is what is used to encrypt and decrypt data. It uses [AES-GCM-SIV](https://en.wikipedia.org/wiki/AES-GCM-SIV) and [Argon2](https://en.wikipedia.org/wiki/Argon2). I just pass it the data to be encrypted or decrypted along with the key, and it handles the rest from there.
