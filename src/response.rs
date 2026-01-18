use std::collections::HashMap;
use std::io::{Result, Write};
use std::net::{TcpListener, TcpStream};

pub enum Status {
    Ok,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl Status {
    pub fn as_tuple(&self) -> (u16, &'static str) {
        match self {
            Status::Ok => (200, "OK"),
            Status::NotFound => (404, "NotFound"),
            Status::BadRequest => (400, "Bad Request"),
            Status::InternalServerError => (500, "Interal Server Error"),
        }
    }
}

pub struct Response {
    status: Status,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl Response {
    pub fn new(status: Status, body: impl Into<Vec<u8>>) -> Self {
        let body_bytes = body.into();

        let mut headers = HashMap::new();
        headers.insert("Content-Length:".to_string(), body_bytes.len().to_string());

        Self {
            status,
            headers,
            body: body_bytes,
        }
    }

    pub fn text(body: impl Into<Vec<u8>>) -> Response {
        let mut response = Self::new(Status::Ok, body);

        response
            .headers
            .insert("Content-type:".to_string(), "text/plain".to_string());
        response
    }

    pub fn html(body: impl Into<Vec<u8>>) -> Response {
        let mut response = Self::new(Status::Ok, body);

        response
            .headers
            .insert("Content-type:".to_string(), "text/html".to_string());
        response
    }

    pub fn json(body: impl Into<Vec<u8>>) -> Response {
        let mut response = Self::new(Status::Ok, body);

        response
            .headers
            .insert("Content-type:".to_string(), "application/json".to_string());
        response
    }

    pub fn status(mut self, new_status: Status) -> Self {
        self.status = new_status;
        self
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn to_http_string(&self) -> String {
        let (code, phrase) = self.status.as_tuple();

        let mut response = format!("HTTP/1.1 {} {}\r\n", code, phrase);

        for (key, value) in &self.headers {
            response.push_str(&format!("{} {} \r\n", key, value));
        }
        response.push_str("\r\n");

        response.push_str(&String::from_utf8_lossy(&self.body));
        println!("RESPONSE:\n---\n{}\n---", response);

        response
    }

    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let header_string = self.to_http_string();

        let header_bytes = header_string.as_bytes();

        stream.write_all(header_bytes);
        stream.write_all(&self.body);
        stream.flush();
        Ok(())
    }
}
