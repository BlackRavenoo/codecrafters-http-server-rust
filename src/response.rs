use std::collections::HashMap;

pub fn response(content_type: &str, headers: &HashMap<&str, &str>, text: &str) -> Vec<u8> {
    let headers = headers.iter().fold(HashMap::new(), |mut acc, (key, val)| {
        match *key {
            "Accept-Encoding" if val.split(',').any(|encoding| encoding.trim() == "gzip") => {
                acc.insert("Content-Encoding", "gzip");
            }
            _ => ()
        }
        acc
    });
    format!(
        "HTTP/1.1 200 OK\r\n{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        headers.iter().fold(String::new(), |acc, (key, val)| {
            acc + key + ": " + val + "\r\n"
          }
        ),
        content_type,
        text.len(),
        text
    ).as_bytes().to_vec()
}