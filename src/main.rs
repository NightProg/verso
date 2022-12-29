use std::collections::HashMap;
use std::convert::TryInto;
use verso::server::Server;
use verso::router::Router;
use verso::request::RequestBody;

use verso::response::Response;

fn main() {
    let server = Server::new("localhost", 2000);
    let mut router = Router::new();
    router.get("/", |req| {
        Response::new(200, format!("{}", match req.header.get("Accept-Language") {
            Some(value) => value,
            None => ""
        }))
    });
    router.get("/hello", |req| {
        println!("{:?}", req.url_query);
        Response::new(200, format!("<h1>bonjour {}</h1>", req.url_query
            .as_ref()
            .unwrap()
            .get("name")
            .unwrap()
        ))
    });
    router.post("/coucou", |req| {
        Response::new(200, format!("hello {}", <RequestBody<'_> as TryInto<&str>>::try_into(req.clone().post_info.unwrap()).unwrap()))
    });

    server.start(router.clone());
}


