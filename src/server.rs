
use crate::router::Router;
use crate::request::{Request, Method, RequestBody};
use crate::queryweb::QueryWeb;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{BufReader, Read};
use std::io::{Write,BufRead};
use std::collections::HashMap;


pub struct Server {
    host: String,
    port: u32,
    tcp: TcpListener,
    keep_alive: bool
}

impl Server {
    pub fn new(host: &str, port: u32) -> Server {
        let port_and_host: String = format!("{}:{}", host, port);

        println!("server running at {}", port_and_host);
        Server {
            host: host.to_string(),
            port,
            tcp: TcpListener::bind(port_and_host).unwrap(),
            keep_alive: false
        }
    }
    fn clone(&self) -> Server {
        let port_and_host: String = format!("{}:{}", self.host, self.port);

        Server {
            host: self.host.clone(),
            port: self.port,
            tcp: TcpListener::bind(port_and_host).unwrap(),
            keep_alive: self.keep_alive
        }
    }
    pub fn start(self, router: Router) {
        for mut stream in self.tcp.incoming() {
            match stream {
                Ok(ref mut s) => {

                        stream = match self.clone().handle(s, router.clone()) {
                            Some(v) => Ok(v),
                            None => Err(std::io::Error::new(std::io::ErrorKind::ConnectionReset, ""))
                        };


                }
                Err(_) => { /* connection failed */ }
            }

        }
    }

    pub fn handle(&mut self, s: &mut TcpStream, router: Router) -> Option<TcpStream> {
        let mut stream = s;
        let mut buffer = [0;99999];
        stream.read(&mut buffer).unwrap();


        let x = std::str::from_utf8(&buffer).unwrap();
        let r = x.replace(" ", "");
        if r.is_empty() || r == "\r" || r == "\n" {
            return None
        }

        let mut http_request: Vec<_> = x
            .replace(" ", "")
            .replace("\r", "")
            .split("\n")
            .map(|e| e.to_string())
            .filter(|e| !e.is_empty() || e.as_str() != "\r")
            .collect();

        println!("{}", http_request.as_slice().join(" \n"));

        if http_request.is_empty() {
            return None;
        }
        let binding = http_request.clone();
        let split_first_line = binding[0].split(" ").collect::<Vec<&str>>();
        let mut get_info = None;
        let mut post_info = None;
        let mut put_info = None;
        let mut delete_info = None;
        let [method_str, mut url] = [split_first_line[0], split_first_line[1]];
        let method = match method_str {
            "GET" => Method::Get,
            "POST" => Method::Post,
            "PUT" => Method::Put,
            "DELETE" => Method::Delete,
            m => Method::Auth(m)
        };
        let h = http_request[1..http_request.len()].to_vec();
        let header = Server::parse_header(h.as_slice())
            .unwrap()
            .into_iter()
            .map(|a| (a.0.to_string(), a.1.to_string()))
            .collect::<HashMap<String, String>>();

        let query = Server::parse_url(url);
        if url.contains("?") {
            url = &url[..url.find("?").unwrap()]
        }
        let body = Server::parse_body(http_request.last().unwrap().as_str(), header.clone());

        match method {
            Method::Get => get_info = body,
            Method::Post => post_info = body,
            Method::Put => put_info = body,
            Method::Delete => delete_info = body,
            _ => {}
        }


        let request = Request {
            url: url.to_string(),
            method,
            header,
            get_info,
            post_info,
            put_info,
            delete_info,
            url_query : query
        };
        let response = router.handle_request(&request);
        let res = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            response.code,
            response.len,
            response.text
        );


        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();




        None
    }

    fn parse_header<'a>(list: &'a [String]) -> Option<HashMap<&'a str, &'a str>> {
        let mut hm: HashMap<&str, &str> = HashMap::new();
        for elt in list.iter().map(|e| e.as_str()) {
            let mut s = elt.split(": ").collect::<Vec<&str>>();
            if s.is_empty() {
                return None;
            }
            if s[0].is_empty() {
                break;
            }
            /*if s[1].contains(";") {
                s[1] = &s[1][..s[1].find(";").unwrap()]
            }*/
            hm.insert(s[0], s[1]);
        }
        Some(hm)
    }

    fn parse_url(url: &str) -> Option<HashMap<String, String>> {

        if ! url.contains('?') {
            return None
        }
        let url_split = url.split("?")
            .filter(|e| e != &"/")
            .collect::<Vec<&str>>();
        if url_split.len() < 2 {
            return None;
        }
        return Some(url_split[1].from_query_map())
    }

    fn parse_body(body: &str, header: HashMap<String, String>) -> Option<RequestBody> {
        if header.contains_key("Content-Type") {
            match header.get("Content-Type").unwrap().as_str() {
                "application/x-www-form-urlencoded" => {
                    let res: Vec<_> = body.split("=").collect();
                    let mut map = HashMap::new();
                    map.insert(res[0].to_string(), res[1].to_string());
                    Some(RequestBody::Query(map))
                },
                "text/plain" => {
                    Some(RequestBody::Text(body))
                },
                "application/json" => {
                    Some(RequestBody::Json(body.from_json().expect("bad json format")))
                }
                e => panic!("unknown type: {}", e)
            }
        } else {
            Some(RequestBody::Text(body))
        }
    }

}