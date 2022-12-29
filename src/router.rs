use std::collections::HashMap;
use crate::response::Response;
use crate::request::{Request, Method};
use crate::error_http::ErrorHttp;

type BoxCallback = Box<fn(&Request) -> Response >;

#[derive(Clone)]
pub struct Router<'a>  {
    routes: HashMap<String,(Method<'a>, BoxCallback)>
}

impl<'a> Router<'a> {

    pub fn new() -> Router<'a> {
        Router {
            routes: HashMap::new()
        }
    }

    pub fn get (&mut self, url: &str, handle: fn(&Request) -> Response)
    {
        if url.ends_with("/") {
            self.routes.insert(url.to_string(), (Method::Post, Box::new(handle)));
        } else {
            self.routes.insert(url.to_string() + "/", (Method::Post, Box::new(handle)));
        }
        self.routes.insert(url.to_string(), (Method::Get,Box::new(handle)));
    }
    pub fn post (&mut self, url: &str, handle: fn(&Request) -> Response)  {
        if url.ends_with("/") {

            self.routes.insert(url.to_string(), (Method::Post, Box::new(handle)));
        } else {
            self.routes.insert(url.to_string() + "/", (Method::Post, Box::new(handle)));
        }
        self.routes.insert(url.to_string(), (Method::Get,Box::new(handle)));

    }
    pub fn put(&mut self, url: &str, handle: fn(&Request) -> Response) {
        self.routes.insert(url.to_string(), (Method::Put,Box::new(handle)));
    }
    pub fn delete(&mut self, url: &str, handle: fn(&Request) -> Response)  {
        self.routes.insert(url.to_string(), (Method::Delete, Box::new(handle)));
    }

    pub(crate) fn handle_request(&self, request: &Request) -> Response {
        let res = match self.routes.get(&request.url) {
            Some((_, callback)) => {
                    callback(request)
            },
            None => Router::http_404(request)
        };
        return res;
    }
}

impl<'a> ErrorHttp for Router<'a> {}