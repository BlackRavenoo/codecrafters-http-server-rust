use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 1024];
    let len = stream.read(&mut buf).unwrap();
    let args: Vec<_> = std::str::from_utf8(&buf[..len]).unwrap().split("\r\n").collect();
    println!("{:#?}", args); //debug
    let iter = args[0].split(" ");
    match iter.skip(1).next() {
        Some("/") => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        },
        Some(_) => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        },
        None => eprintln!("Something went wrong!")
    }
}
