use std::collections::HashMap;
use std::io::Read;
use std::net::TcpStream;
use std::result::Result as StdResult;
#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub raw: String,
    pub body: Vec<u8>,
}

impl Request {
    pub fn from_stream(stream: &mut TcpStream) -> StdResult<Self, String> {
        let mut buffer = [0; 1024];
        let _bytes_read = stream.read(&mut buffer).map_err(|e| e.to_string())?;

        let raw_request = String::from_utf8_lossy(&buffer).to_string();

        let line = raw_request.lines().next().unwrap_or("");
        let parts: Vec<&str> = raw_request.split_whitespace().collect();

        if parts.len() < 2 {
            return Err("Malformed requset line".into());
        }

        let method = parts.get(0).unwrap_or(&"").to_string();
        let path = parts.get(1).unwrap_or(&"").to_string();

        let (path, query) = if let Some(pos) = path.find('?') {
            let p = path[..pos].to_string();
            let q = path[pos + 1..].to_string();

            let mut map = HashMap::new();
            for pair in q.split('&') {
                if let Some(eq) = pair.find('=') {
                    let key = pair[..eq].to_string();
                    let val = pair[eq + 1..].to_string();
                    map.insert(key, val);
                }
            }

            (p, map)
        } else {
            (path, HashMap::new())
        };

        let split = raw_request
            .find("\r\n\r\n")
            .ok_or("Malformed request: missing header/body seperator")?;

        let mut header_block = &raw_request[..split];
        let body_start = split + 4;

        let mut headers = HashMap::new();

        for line in header_block.lines().skip(1) {
            // skip request line
            if let Some(colon) = line.find(':') {
                let key = line[..colon].trim().to_string();
                let val = line[colon + 1..].trim().to_string();
                headers.insert(key, val);
            }
        }

        let content_length = headers
            .get("Content-Length")
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(0);

        let mut body = Vec::new();
        let body_str = &raw_request[body_start..];
        body.extend_from_slice(body_str.as_bytes());
        let remaining = content_length.saturating_sub(body_str.len());

        if remaining > 0 {
            let mut temp_buffer = vec![0u8; remaining];
            stream
                .read_exact(&mut temp_buffer)
                .map_err(|e| e.to_string())?;
            body.extend_from_slice(&temp_buffer);
        }

        Ok(Self {
            method,
            path,
            headers,
            query,
            raw: raw_request,
            body,
        })
    }
}
