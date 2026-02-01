use crate::request::Request;
use crate::response::Response;
use crate::response::Status;
use crate::router::{Handler, Router};
use crate::thread_pool::ThreadPool;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

pub struct App {
    router: Arc<Router>,
    thread_pool_size: usize,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Arc::new(Router::new()),
            thread_pool_size: 4,
        }
    }
    pub fn get(&mut self, path: &str, handler: fn(Request) -> Response) {
        Arc::get_mut(&mut self.router)
            .expect("Router already shared")
            .add_get(path, handler);
    }

    pub fn post(&mut self, path: &str, handler: fn(Request) -> Response) {
        Arc::get_mut(&mut self.router)
            .expect("Router already shared")
            .add_post(path, handler);
    }
    pub fn run(&self, addr: &str) {
        let listener = TcpListener::bind(addr).expect("Failed to bind address");

        println!(" Framework server running at http://{}/", addr);

        let pool = ThreadPool::new(self.thread_pool_size);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let router = Arc::clone(&self.router);

                    pool.execute(move || {
                        handle_connection(stream, router);
                    });
                }
                Err(e) => {
                    eprintln!("Connection failed: {}", e);
                }
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream, router: Arc<Router>) {
    let request = match Request::from_stream(&mut stream) {
        Ok(req) => req,
        Err(_) => {
            let response = Response::text("400 Bad Request").status(Status::BadRequest);
            let _ = response.send(&mut stream);
            return;
        }
    };

    match router.resolve(&request.method, &request.path) {
        Some(handler) => {
            let response = handler(request);
            let _ = response.send(&mut stream);
        }
        None => {
            let response = Response::text("404 Not Found").status(Status::NotFound);
            let _ = response.send(&mut stream);
        }
    }
}
