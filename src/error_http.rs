use crate::request::Request;
use crate::response::Response;

pub trait ErrorHttp {
    fn http_404(_req: &Request) -> Response {
        return Response {
            code: 404,
            text: "<h1>http 404</h1>".to_string(),
            len: "<h1>http 404</h1>".to_string().len()
        }
    }
    fn http_500(_req: &Request) -> Response {
        return Response {
            code: 500,
            text: "<h1>http 500</h1>".to_string(),
            len: "<h1>http 500</h1>".to_string().len()
        }
    }
}