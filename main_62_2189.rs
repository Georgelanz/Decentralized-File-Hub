use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;

fn handle_node(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    // Simulate P2P handshake
    let response = b"NODE_SYNC_ACK";
    stream.write(response).unwrap();
}

fn main() {
    println!("[INFO] Starting Decentralized File Hub Node...");
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_node(stream);
                });
            }
            Err(e) => { println!("Connection failed: {}", e); }
        }
    }
}
