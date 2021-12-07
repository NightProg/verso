use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::{Write,BufRead};
use std::collections::HashMap;
//use std::string::String;
type BoxCallback = Box<fn(&Request) -> Responce >;

macro_rules! responce {
    (code=$code:expr,text=$text:expr) => {
        
        Responce {
            code: $code,
            text: $text.to_string(),
            len: $text.to_string().len()
        }
        
    }
}

#[derive(Copy, Clone)]
enum Method {
    Get,
    Post,
    Put,
    Delete,
    Auth
}
/*
fn split_string_element(e: String, v: &str) -> Vec<&str> {
    e.split(v).by_ref().map(|ref x| x.clone()).collect::<Vec<_>>()
}*/
trait QueryWeb: std::fmt::Display {
    
    fn from_query_map(&self) -> HashMap<String, String> {
        let string_self = self.to_string();
        let split_element = string_self.split("&").by_ref().collect::<Vec<_>>();
        let mut obj = HashMap::new();

        if split_element.len() == 0 {
            let split_e = string_self.split("=").by_ref().collect::<Vec<_>>();
            if split_e.len() == 2 {
                obj.insert(split_e[0].to_string(), split_e[1].to_string());
            }
            return obj;

        } else {
            for i in split_element.clone() {
                let e = i.split("=").by_ref().collect::<Vec<_>>();
                if e.len() == 2 {
                    obj.insert(e[0].to_string(),e[1].to_string());
                }
            }
        }
        
        obj
    }

    fn from_json(&self) -> Result<HashMap<String,String>, String>{
        let split_self = self.to_string();
        if ! (split_self.starts_with("{") && split_self.ends_with("}")) {
            return Err("unclosed { or }".to_string());
        }
        let mut object = HashMap::new();
        let dict_body = &split_self[1..split_self.len()-1];
        let split_dict_body = dict_body.split(",");
        for i in split_dict_body {
            let element = i.split(":").by_ref().collect::<Vec<_>>();
            if element.len() <= 1 {
                return Err("Json parse error".to_string());
            }
            object.insert(element[0].to_string(),element[1].to_string());

        }

        return Ok(object);

    }
}
impl QueryWeb for str {}



struct Request {
    url: String,
    method: Method,
    header: HashMap<String,String>,
    get_info: String,
    post_info: String,
    put_info: String,
    delete_info: String
}

struct Responce {
    code: u32,
    text: String,
    len: usize
}

trait ErrorHttp {
    fn http_404(req: &Request) -> Responce {
        return Responce {
            code: 404,
            text: "<h1>http 404</h1>".to_string(),
            len: "<h1>http 404</h1>".to_string().len()
        }
    }
    fn http_500(req: &Request) -> Responce {
        return Responce {
            code: 500,
            text: "<h1>http 500</h1>".to_string(),
            len: "<h1>http 500</h1>".to_string().len()
        }
    }
}

#[derive(Clone)]
struct Router  {
    routes: HashMap<String,(Method, BoxCallback)>
}

struct Server {
    host: String,
    port: u32,
    tcp: TcpListener,
    keep_alive: bool
}

impl Server {
    fn new(host: &str, port: u32) -> Server {
        let port_and_host: String = format!("{}:{}", host, port);

        Server {
            host: host.to_string(),
            port: port,
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
    fn start(self, router: Router) {
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

    fn handle(self: &mut Self, s: TcpStream, router: Router) -> Option<TcpStream> {
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

impl Request {
    fn clone(&self) -> Self {
        Request {
            url: self.url.clone(),
            method: self.method,
            header: self.header.clone(),
            get_info: self.get_info.clone(),
            post_info: self.post_info.clone(),
            put_info: self.put_info.clone(),
            delete_info: self.delete_info.clone()
        }
    }

    fn get_json(&self) -> Result<HashMap<String, String>, String> {
        self.get_info.clone().from_json()
    }

    fn get_querymap(&self) -> HashMap<String, String> {
        self.get_info.clone().from_query_map()
    }

    fn get_args(&self, index: &str, default: &str) -> String {
        return match self.get_querymap().get(index) {
            Some(v) => v.to_string(),
            None => default.to_string()
        }
    }
}


impl Router {

    fn new() -> Router{
        Router {
            routes: HashMap::new()
        }
    }

    fn get (&mut self, url: &str, handle: fn(&Request) -> Responce) 
    {
        if url.ends_with("/") {
            self.routes.insert(url.to_string(), (Method::Post, Box::new(handle)));
        } else {
            self.routes.insert(url.to_string() + "/", (Method::Post, Box::new(handle)));
        }   
        self.routes.insert(url.to_string(), (Method::Get,Box::new(handle)));
    }
    fn post (&mut self, url: &str, handle: fn(&Request) -> Responce)  {
        if url.ends_with("/") {
           
            self.routes.insert(url.to_string(), (Method::Post, Box::new(handle)));
        } else {
            self.routes.insert(url.to_string() + "/", (Method::Post, Box::new(handle)));
        }
        self.routes.insert(url.to_string(), (Method::Get,Box::new(handle)));     
        
    }
    fn put (&mut self, url: &str, handle: fn(&Request) -> Responce) {
        self.routes.insert(url.to_string(), (Method::Put,Box::new(handle)));
    }
    fn delete (&mut self, url: &str, handle: fn(&Request) -> Responce)  {
        self.routes.insert(url.to_string(), (Method::Delete, Box::new(handle)));
    }

    fn handle_request(&self, request: &Request) -> Responce{
        let res = match self.routes.get(&request.url) {
            Some((method, callback)) => {
                if let method = &request.method {
                    callback(request)
                } else {
                    Router::http_404(request)
                }
            },
            None => Router::http_404(request)
        };
        return res;
    }
}
impl ErrorHttp for Router {}



fn main() {
    let mut server = Server::new("localhost", 2000);
    let mut router = Router::new();
    router.get("/", |req| responce!(code=200, text=format!("{}", match req.header.get("User-Agent") {
        Some(value) => value,
        None => ""
    })));
    router.get("/hello", |req| {
        responce!(code=200, text=format!("<h1>bonjour {}</h1>", req.get_args("name", "")))
    });
    router.post("/coucou", |req| responce!(code=200, text=format!("{}", req.post_info)));

    server.start(router.clone());
}


