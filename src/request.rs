use std::io::{Read, Result};
use std::net::TcpStream;

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub raw: String,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> Result<Self> {
        let mut buffer = [0; 1024];
        let _bytes_read = stream.read(&mut buffer)?;

        let raw_request = String::from_utf8_lossy(&buffer).to_string();

        let line = raw_request.lines().next().unwrap_or("");
        let parts: Vec<&str> = raw_request.split_whitespace().collect();

        let method = parts.get(0).unwrap_or(&"").to_string();
        let path = parts.get(1).unwrap_or(&"").to_string();

        Ok(Self {
            method,
            path,
            raw: raw_request,
        })
    }
}
