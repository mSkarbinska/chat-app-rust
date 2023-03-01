
use std::{
    io::{prelude::*},
    net::{TcpStream},
};
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").unwrap();
    loop {
        stream.write(b"hello").unwrap();
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}