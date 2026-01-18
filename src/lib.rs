// lib.rs

// Make each module visible to this file
pub mod app;
pub mod request;
pub mod response;
pub mod router;
pub mod thread_pool;
// pub mod utils;
pub use response::Status;
// Re-export important types so users don't need to write long paths.

pub use app::App;
pub use request::Request;
pub use response::Response;
pub use router::Router;
