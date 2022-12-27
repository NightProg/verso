use verso::server::Server;
use verso::error_http::ErrorHttp;
use verso::router::Router;
use verso::queryweb::*;
use std::collections::HashMap;
use verso::response::Response;

fn main() {
    let mut server = Server::new("localhost", 2000);
    let mut router = Router::new();
    router.get("/", |req| {
        Response::new(200, format!("{}", match req.header.get("User-Agent") {
        Some(value) => value,
        None => ""
    }))
    });
    router.get("/hello", |req| {
        Response::new(200, format!("<h1>bonjour {}</h1>", req.get_args("name", "")))
    });
    router.post("/coucou", |req| {
        Response::new(200, format!("hello {}", req.post_info))
    });

    server.start(router.clone());
}


