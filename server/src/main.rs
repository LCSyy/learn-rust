// local-server ---> 
//                   transfer
// client --------->

use std::{
    net::{ TcpListener, IpAddr, Ipv4Addr },
    io::Read
};

const SERVER_ADDR: &'static str = "127.0.0.1:9543";

fn main() {
    let listener = TcpListener::bind(SERVER_ADDR).unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0;1024];
                let _size = stream.read(&mut buf[..]).unwrap();
                let val = std::str::from_utf8(&buf).unwrap();
                let peer = stream.peer_addr().unwrap();
                println!("From: {}, Recieve: {}",peer,val);
            },
            Err(err) => { println!("Accept error:{}",err); }
        }            
    }
}
