use std::collections::HashMap;

use crate::{Request, Response};

pub type Handler = fn(Request) -> Response;

#[derive(Clone)]
pub struct Router {
    pub get_routes: HashMap<String, fn(Request) -> Response>,
    pub post_routes: HashMap<String, fn(Request) -> Response>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            get_routes: HashMap::new(),
            post_routes: HashMap::new(),
        }
    }
    pub fn add_get(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.get_routes.insert(path.to_string(), handler);
    }

    pub fn add_post(&mut self, path: &str, handler: fn(Request) -> Response) {
        self.post_routes.insert(path.to_string(), handler);
    }

    pub fn resolve(&self, method: &str, path: &str) -> Option<fn(Request) -> Response> {
        match method {
            "GET" => self.get_routes.get(path).copied(),
            "POST" => self.post_routes.get(path).copied(),
            _ => None,
        }
    }
}
