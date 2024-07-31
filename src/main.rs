use std::{collections::HashMap, fs::File, io::Write, path::Path};
use tokio::{io::{AsyncReadExt, AsyncWriteExt}, net::{TcpListener, TcpStream}};

use http_server_starter_rust::response;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").await.unwrap();

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                tokio::spawn(async move {
                    handle_connection(stream).await;
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 4096];
    let len = stream.read(&mut buf).await.unwrap();
    println!("{}", std::str::from_utf8(&buf[..len]).unwrap());
    let mut args = std::str::from_utf8(&buf[..len]).unwrap().split("\r\n");
    let mut start_line_args = args.next().unwrap().split(" ");
    let method = start_line_args.next().unwrap();
    let mut path = start_line_args.next().unwrap().split("/").skip(1);

    let mut headers = HashMap::new();
    while let Some(arg) = args.next() {
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
            stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).await.unwrap();
        },
        Some("echo") => {
            stream.write(&response("text/plain", &headers, &path.collect::<Vec<&str>>().join("/"))).await.unwrap();
        }
        Some("user-agent") => {
            let user_agent = headers.get("User-Agent").unwrap();
            stream.write(&response("text/plain", &headers, user_agent)).await.unwrap();
        }
        Some("files") => {
            let args_: Vec<String> = std::env::args().collect();
            let dir = args_[2].clone();
            let path_to_file = format!(
                "{}{}",
                dir,
                path.collect::<Vec<&str>>().join("/")
            );
            println!("{}", path_to_file);
            match method {
                "GET" => {
                    match Path::new(&path_to_file).exists() {
                        true => {
                            stream.write(
                                &response(
                                    "application/octet-stream",
                                    &headers,
                                    &std::fs::read_to_string(path_to_file).unwrap()
                                )
                            )
                            .await
                            .unwrap();
                        },
                        false => {
                            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).await.unwrap();
                        }
                    }
                }
                "POST" => {
                    let text = args.next().unwrap().as_bytes();
                    let mut file = File::create(path_to_file).unwrap();
                    file.write_all(text).unwrap();
                    stream.write("HTTP/1.1 201 Created\r\n\r\n".as_bytes()).await.unwrap();
                }
                _ => unimplemented!()
            }

        }
        Some(_) => {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes()).await.unwrap();
        },
        None => eprintln!("Something went wrong!")
    }
}
