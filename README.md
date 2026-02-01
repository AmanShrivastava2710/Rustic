# Rustic --- A Minimal Rust HTTP Framework

A lightweight, multi-threaded HTTP framework built from scratch in Rust
for learning and experimentation.

This project demonstrates how to build:

-   HTTP request parsing
-   Response building
-   Routing (GET + POST)
-   Query parameter parsing
-   POST body handling
-   Thread pool (multi-threaded server)
-   Static file serving (if implemented)

------------------------------------------------------------------------

## Features

-   ✅ GET routing
-   ✅ POST routing
-   ✅ Query parameters (`/search?q=rust`)
-   ✅ Request body parsing (Content-Length based)
-   ✅ Custom Response builder
-   ✅ Status enum (200, 404, 400, etc.)
-   ✅ Multi-threaded server
-   ✅ Minimal dependencies (std only)

------------------------------------------------------------------------

## Project Structure

    src/
     ├── app.rs
     ├── router.rs
     ├── request.rs
     ├── response.rs
     ├── thread_pool.rs
     ├── status.rs (or inside response.rs)
     ├── lib.rs
     └── main.rs

------------------------------------------------------------------------

## How It Works

1.  `TcpListener` accepts connections\
2.  Each request is parsed inside `Request::from_stream`\
3.  Router matches `(method, path)`\
4.  Corresponding handler is executed\
5.  `Response` builds proper HTTP format\
6.  Response is written to `TcpStream`

------------------------------------------------------------------------

## Installation

Clone the repository:

``` bash
git clone https://github.com/YOUR_USERNAME/Rustic.git
cd Rustic
```

Build the project:

``` bash
cargo build
```

Run the server:

``` bash
cargo run
```

Server runs at:

    http://127.0.0.1:7878

------------------------------------------------------------------------

## Example Usage

### main.rs

``` rust
use rustic::{App, Request, Response, Status};

fn hello(_req: Request) -> Response {
    Response::text("Hello World")
}

fn login(req: Request) -> Response {
    let body = String::from_utf8_lossy(&req.body);
    Response::text(format!("Received: {}", body))
}

fn main() {
    let mut app = App::new();

    app.get("/", hello);
    app.post("/login", login);

    app.run("127.0.0.1:7878");
}
```

------------------------------------------------------------------------

## Testing with curl

### GET Request

``` bash
curl http://127.0.0.1:7878/
```

### POST Request

``` bash
curl -X POST http://127.0.0.1:7878/login \
     -d "username=aman&password=123"
```

------------------------------------------------------------------------

## Supported HTTP Methods

  Method   Supported
  -------- -------------------
  GET      Yes
  POST     Yes
  PUT      No (can be added)
  DELETE   No (can be added)

------------------------------------------------------------------------

## Future Improvements

-   JSON parsing with `serde`
-   Middleware system
-   Route parameters (`/user/:id`)
-   Async support (Tokio)
-   Static file middleware
-   Logging system
-   Error handling abstraction
-   HTTP/1.1 keep-alive support

------------------------------------------------------------------------

## Purpose

This framework is built for:

-   Learning HTTP internals
-   Understanding TCP-based web servers
-   Exploring Rust ownership in networking
-   Building a web framework from scratch

------------------------------------------------------------------------

## License

MIT License 
