use web_framework::{App, Request, Response, Status};

fn hello_handler(_req: Request) -> Response {
    Response::text("Hello from your Rust framework!")
}

fn html_handler(_req: Request) -> Response {
    Response::html("<h1>This is HTML</h1>")
}

fn json_handler(_req: Request) -> Response {
    Response::json(r#"{"message":"Hello JSON!"}"#)
}

fn notfound_handler(_req: Request) -> Response {
    Response::text("This route does not exist").status(Status::NotFound)
}

fn main() {
    let mut app = App::new();

    // Register routes
    app.get("/", hello_handler);
    app.get("/html", html_handler);
    app.get("/json", json_handler);

    // Custom 404 route (optional)
    app.get("/404", notfound_handler);

    // Run your server
    app.run("127.0.0.1:7878");
}
