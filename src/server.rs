
use crate::router::Router;
use crate::request::{Request, Method};
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
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
            println!("begin");
            match stream {
                Ok(s) => {
                    println!("ok");
                    if ! self.keep_alive {
                        println!("not keep alive: {}", self.keep_alive);
                        stream = match self.clone().handle(s, router.clone()) {
                            Some(v) => Ok(v),
                            None => Err(std::io::Error::new(std::io::ErrorKind::ConnectionReset, ""))
                        };
                    }
                    println!("end");
                }
                Err(e) => { /* connection failed */ }
            }
            /*let mut stream = stream.unwrap();
            println!("unwrap");
            self.clone().handle(&mut stream, router.clone());
            println!("end");*/
        }
    }

    pub fn handle(self: &mut Self, s: TcpStream, router: Router) -> Option<TcpStream> {
        if self.keep_alive {
            return None;
        }
        let mut stream = s;

        let mut buffer = [0; 5000];
        stream.read(&mut buffer).unwrap();
        let sbuffer = String::from_utf8_lossy(&buffer[..]);
        let split_buffer = sbuffer.split("\n").by_ref().collect::<Vec<_>>();
        let len_split_buffer = split_buffer.len();
        let split_space_at_first_line = split_buffer[0].split(" ").by_ref().collect::<Vec<_>>();
        println!("{:?}", split_space_at_first_line);
        let method = split_space_at_first_line[0];
        let mut url = split_space_at_first_line[1];
        let mut get_info = String::new();
        let mut post_info = String::new();
        let mut put_info = String::new();
        let mut delete_info = String::new();
        if url.contains("?") {
            let split_url = url.split("?").by_ref().collect::<Vec<_>>();
            url = split_url[0];
            get_info = split_url[1].to_string();
        }
        let mut header = HashMap::new();


        let mut not_a_header = 0;
        let mut index = 0;
        for (i,elt) in split_buffer[1..].to_vec().iter().enumerate() {

            let mut e = elt.split(":").by_ref().collect::<Vec<_>>();
            if e.len() > 1 {
                let mut value = e[1].to_string();
                value = e[1].replace("\r", "");
                let mut _char = value.chars();
                _char.next();
                value = _char.as_str().to_string();
                header.insert(e[0].to_string(), value);
            } else {
                not_a_header = 1;
                index = i;
            }
        }

        /*if not_a_header != 0 {
            let s = split_buffer[index+2].to_string();
            let e = s.chars().filter(|c| c.is_ascii()).collect::<String>();
            match method {
                "GET" => get_info = e,
                "POST" => post_info = e,
                "PUT" => put_info = e,
                "DELETE" => delete_info = e,
                _ => {}
            }
        }*/
        let request = Request {
            url: url.to_string(),
            method: match method {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                _ => Method::Auth
            },
            header: header.clone(),
            get_info: get_info,
            post_info: post_info,
            put_info: put_info,
            delete_info: delete_info
        };
        println!("{:?}", header.clone().get("Connection"));
        if match header.get("Connection") {
            Some(v) => v == "keep-alive",
            None => false
        } {

            self.keep_alive = true;
            println!("keep alive: {}", self.keep_alive);
        }

        let mut responce = router.handle_request(&request);
        let res = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            responce.code,
            responce.len,
            responce.text
        );


        stream.write(res.as_bytes()).unwrap();
        stream.flush().unwrap();


        println!("Request: {}", sbuffer);
        Some(stream)
    }
}