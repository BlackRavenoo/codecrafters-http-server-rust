use std::{io::{Read, Write}, net::{TcpListener, TcpStream}};

use http_server_starter_rust::echo_response;

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
    let start_line_args = args[0].split(" ");
    let mut path = start_line_args.skip(1).next().unwrap().split("/").skip(1);
    match path.next() {
        Some("") => {
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
        },
        Some("echo") => {
            stream.write(&echo_response(&path.collect::<Vec<&str>>().join("/"))).unwrap();
            /*match path.next() {
                Some(text) => {
                    stream.write(&echo_response(text)).unwrap();
                },
                None => {
                    stream.write("HTTP/1.1 400 Bad Request\r\n\r\n".as_bytes()).unwrap();
                }
            }*/
        }
        Some(_) => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).unwrap();
        },
        None => eprintln!("Something went wrong!")
    }
}
