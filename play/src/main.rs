use std::{
    net::{ TcpStream },
    io::{ Write },
};

const HOST_ADDR: &'static str = "127.0.0.1:9543";

fn main() {
    let mut stream = TcpStream::connect(HOST_ADDR).unwrap();
    stream.write("1;111;2562;This is a data from client, Hello server!".as_bytes()).unwrap();
}

