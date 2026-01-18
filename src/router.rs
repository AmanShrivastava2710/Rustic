use std::collections::HashMap;

use crate::{Request, Response};

pub type Handler = fn(Request) -> Response;

#[derive(Clone)]
pub struct Router {
    pub routes: HashMap<(String, String), Handler>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    pub fn add(&mut self, method: &str, path: &str, handler: Handler) {
        self.routes
            .insert((method.to_string(), path.to_string()), handler);
    }
    pub fn find(&self, method: &str, path: &str) -> Option<Handler> {
        self.routes
            .get(&(method.to_string(), path.to_string()))
            .copied()
    }
}
