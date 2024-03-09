

pub fn response(content_type: &str, text: &str) -> Vec<u8> {
    format!(
        "HTTP/1.1 200 OK\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
        content_type,
        text.len(),
        text
    ).as_bytes().to_vec()
}