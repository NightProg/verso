use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::io::Write;
use std::collections::HashMap;

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

struct Request {
    url: String,
    method: Method,
    header: HashMap<String,String>,
    get_info: HashMap<String,String>,
    post_info: HashMap<String,String>,
    put_info: HashMap<String,String>,
    delete_info: HashMap<String,String>
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
    tcp: TcpListener
}

impl Server {
    fn new(host: &str, port: u32) -> Server {
        let port_and_host: String = format!("{}:{}", host, port);

        Server {
            host: host.to_string(),
            port: port,
            tcp: TcpListener::bind(port_and_host).unwrap()
        } 
    }
    fn clone(&self) -> Server {
        let port_and_host: String = format!("{}:{}", self.host, self.port);

        Server {
            host: self.host.clone(),
            port: self.port,
            tcp: TcpListener::bind(port_and_host).unwrap()
        } 
    }
    fn start(self, router: Router) {
        for stream in self.tcp.incoming() {
            let mut stream = stream.unwrap();
            self.clone().handle(&mut stream, router.clone());
        }
    }

    fn handle(self, stream: &mut TcpStream, router: Router) {
        let mut buffer = [0; 500];
        stream.read(&mut buffer).unwrap();
        let sbuffer = String::from_utf8_lossy(&buffer[..]);
        let split_buffer = sbuffer.split("\n").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>();

        let method = split_buffer[0].split(" ").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>()[0];

        let mut url = split_buffer[0].split(" ").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>()[1];
        let mut get_info = HashMap::new();
        let mut post_info = HashMap::new();
        let mut put_info = HashMap::new();
        let mut delete_info = HashMap::new();
        if url.contains("?") {
            let split_url = url.split("?").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>();
            url = split_url[0];
            if split_url.len() > 1{
                let dict_split = split_url[1].split("&").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>();
                for i in dict_split.iter() {
                    let dict_split_element = i.clone().split("=").by_ref().map(|ref x| x.clone()).collect::<Vec<_>>();
                    get_info.insert(dict_split_element[0].to_string(), dict_split_element[1].to_string());
                }

            }
        } else {

        }
        let mut header = HashMap::new();
        for elt in split_buffer[1..].to_vec().iter() {
            let e = elt.split(":").by_ref().collect::<Vec<_>>();
            if e.len() > 1 {
                header.insert(e[0].to_string(), e[1].to_string());

            }
        }
        let request = Request {
            url: url.to_string(),
            method: match method {
                "GET" => Method::Get,
                "POST" => Method::Post,
                "PUT" => Method::Put,
                "DELETE" => Method::Delete,
                _ => Method::Auth
            },
            header: header,
            get_info: get_info,
            post_info: post_info,
            put_info: put_info,
            delete_info: delete_info
        };

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
}


impl Router {

    fn new() -> Router{
        Router {
            routes: HashMap::new()
        }
    }

    fn get (&mut self, url: &str, handle: fn(&Request) -> Responce) 
    {
        self.routes.insert(url.to_string(), (Method::Get,Box::new(handle)));
    }
    fn post (&mut self, url: &str, handle: fn(&Request) -> Responce)  {
        self.routes.insert(url.to_string(), (Method::Post, Box::new(handle)));
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
    router.get("/", |req| {
        match req.get_info.get("name") {
            Some(name) => responce!(code=200, text=format!("<p>hello {}</p>", name)),
            None => responce!(code=200, text="<p>hello world</p>")
        }
        
    });
    router.get("/hello", |req| responce!(code=200, text="<h1>bonjour</h1>"));
    router.post("/coucou", |req| responce!(code=200,text="<h1>COUCOU</h1>"));

    server.start(router.clone());
}


