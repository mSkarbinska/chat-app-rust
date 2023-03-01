use std::thread;

use std::{
    io::{prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let client_info = stream.peer_addr().unwrap();

        println!("New client: {}", client_info);

       //make new thread to handle connection
        thread::spawn(move || {
            handle_connection(&stream);
        });

    }
}

fn handle_connection(mut stream: &TcpStream) {
    loop {
        let mut buffer = [0; 512];
        stream.read(&mut buffer).unwrap();

        let request = String::from_utf8_lossy(&buffer[..]);

        println!("Request: {}", request);
    }
}
