// [ client ] --> [ public ip: transfer ] --> [ lan: proxy --> local server ]
// server.register(self_code,auth_code)
//   format: {msg_type;self_code;auth_code}
// client.connect(self_code,peer_code)
//   format: {msg_type;self_code;server_code;data}

use std::{
    net::{ TcpListener },
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
                let data = std::str::from_utf8(&buf).unwrap();

                let _peer = stream.peer_addr().unwrap();

                parse_data(data);
            },
            Err(err) => { println!("Accept error:{}",err); }
        }            
    }
}

fn parse_data(data: &str) {
    let mut msgs: Vec<String> = Vec::new();
    for msg in data.split(";") {
        msgs.push(msg.to_string());
    }

    println!("msg size: {}", msgs.len());

    if msgs.len() >= 1 {
        match msgs[0].as_str() {
            "1" => {
                println!("msg from client");
            },
            _ => {
                println!("msg from server");
            }
        }
    }

    println!("msg type is: {}", msgs[0].as_str());
}


