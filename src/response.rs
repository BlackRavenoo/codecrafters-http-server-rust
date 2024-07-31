use core::str;
use std::{collections::HashMap, io::Write};

use flate2::{write::GzEncoder, Compression};

enum Encoding {
    Gzip,
    None
}

pub fn response(content_type: &str, headers: &HashMap<&str, &str>, text: &str) -> Vec<u8> {
    let mut encoding = Encoding::None;
    let headers = headers.iter().fold(HashMap::new(), |mut acc, (key, val)| {
        match *key {
            "Accept-Encoding" if val.split(',').any(|encoding| encoding.trim() == "gzip") => {
                acc.insert("Content-Encoding", "gzip");
                encoding = Encoding::Gzip;
            }
            _ => ()
        }
        acc
    });

    let mut result = format!(
        "HTTP/1.1 200 OK\r\n{}Content-Type: {}\r\n",
        headers.iter().fold(String::new(), |acc, (key, val)| {
            acc + key + ": " + val + "\r\n"
          }
        ),
        content_type,
    ).as_bytes().to_vec();

    match encoding {
        Encoding::Gzip => {
            let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
            let _ = encoder.write_all(text.as_bytes());
            let text = encoder.finish().unwrap();
            
            result.extend_from_slice(format!("Content-Length: {}\r\n\r\n", text.len()).as_bytes());
            result.extend_from_slice(&text)
        },
        Encoding::None => {
            result.extend_from_slice(format!("Content-Length: {}\r\n\r\n", text.len()).as_bytes());
            result.extend_from_slice(text.as_bytes())
        },
    }

    result
}