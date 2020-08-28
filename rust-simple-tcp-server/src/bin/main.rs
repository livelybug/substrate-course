use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(_) => {
                println!("Error getting stream!");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let empty_msg = "Empty message!\n";

    loop {
        let mut buffer = [0; 512];
        match stream.read(&mut buffer) {
            Ok(n) => {
                // 获取信息有效长度
                let real_len = std::str::from_utf8(&buffer).unwrap().trim().trim_matches(std::char::from_u32(0).unwrap()).len();
                if real_len == 0 {
                    println!("empty!!");
                    stream.write(empty_msg.as_bytes()).unwrap();
                    break;
                }
                println!("Received: {}", std::str::from_utf8(&buffer).unwrap());
                stream.write(&buffer[0..n]).unwrap();
                stream.flush().unwrap();
            }
            Err(err) => {
                println!("{:?}", err);
            }
        }
    }
}

