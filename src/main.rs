use std::{collections::HashMap, io::{Read, Write}, net::{TcpListener, TcpStream}};

use http_server_starter_rust::text_plain;

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
    let mut args = std::str::from_utf8(&buf[..len]).unwrap().split("\r\n");
    //println!("{:#?}", args); //debug
    let start_line_args = args.next().unwrap().split(" ");
    let mut path = start_line_args.skip(1).next().unwrap().split("/").skip(1);

    let mut headers = HashMap::new();
    for arg in args {
        let mut key_value = arg.split(": ");
        let key = match key_value.next() {
            Some(key) => {
                if key == "" {break}
                key
            },
            None => {
                eprintln!("Key was not found");
                continue;
            }
        };
        let value = match key_value.next() {
            Some(value) => value,
            None => {
                eprintln!("Value was not found");
                continue;
            }
        };
        headers.insert(key, value);
    }

    match path.next() {
        Some("") => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        },
        Some("echo") => {
            stream.write(&text_plain(&path.collect::<Vec<&str>>().join("/"))).unwrap();
        }
        Some("user-agent") => {

            stream.write(&text_plain(headers.get("User-Agent").unwrap())).unwrap();
        }
        Some(_) => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        },
        None => eprintln!("Something went wrong!")
    }
}
