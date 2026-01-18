use std::io::Write;
use std::net::{TcpListener, TcpStream};

use crate::request::Request;
use crate::response::Response;
use crate::router::{Handler, Router};
use crate::thread_pool::ThreadPool;

pub struct App {
    router: Router,
    thread_pool_size: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            thread_pool_size: 4,
        }
    }
    // Register a GET route
    pub fn get(&mut self, path: &str, handler: Handler) {
        self.router.add("GET", path, handler);
    }

    // Register a POST route
    pub fn post(&mut self, path: &str, handler: Handler) {
        self.router.add("POST", path, handler);
    }
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind address");

        println!(" Framework server running at http://{}/", addr);

        let pool = ThreadPool::new(self.thread_pool_size);

        for stream in listener.incoming() {
            let stream = match stream {
                Ok(s) => s,
                Err(_) => continue,
            };
            let router = self.router.clone();
            pool.execute(|| {
                handle_connection(stream, router);
            })
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Router) {
    // Parse request (your Request struct handles this)
    let req = match Request::from_stream(&mut stream) {
        Ok(r) => r,
        Err(_) => return,
    };

    // Find matching route handler
    let handler_opt = router.find(&req.method, &req.path);

    let response = match handler_opt {
        Some(handler) => handler(req), // User's handler returns a Response
        None => Response::text("404 Not Found"), // Default 404
    };

    // Send response back to client
    response.send(&mut stream).unwrap_or_else(|err| {
        eprintln!("Error sending response: {}", err);
    });
}
