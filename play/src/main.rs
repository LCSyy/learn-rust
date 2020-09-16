use std::{
    net::{ TcpStream },
    io::{ Write },
};

const HOST_ADDR: &'static str = "47.107.106.105:8081";

fn main() {
    let mut stream = TcpStream::connect(HOST_ADDR).unwrap();
    stream.write("self:1233;srv:1234".as_bytes()).unwrap();
}

