use std::collections::HashMap;
use std::convert::TryInto;


#[derive(Copy, Clone, Debug)]
pub enum Method<'a> {
    Get,
    Post,
    Put,
    Delete,
    Auth(&'a str)
}

#[derive(Clone, Debug)]
pub enum RequestBody<'a> {
    Json(HashMap<String,String>),
    Text(&'a str),
    Query(HashMap<String,String>)
}

#[derive(Clone, Debug)]
pub struct Request<'a> {
    pub url: String,
    pub method: Method<'a>,
    pub header: HashMap<String,String>,
    pub get_info: Option<RequestBody<'a>>,
    pub post_info: Option<RequestBody<'a>>,
    pub put_info: Option<RequestBody<'a>>,
    pub delete_info: Option<RequestBody<'a>>,
    pub url_query: Option<HashMap<String, String>>
}


impl<'a> TryInto<HashMap<String, String>> for RequestBody<'a> {
    type Error = &'a str;

    fn try_into(self) -> Result<HashMap<String, String>, Self::Error> {
        match self {
            RequestBody::Json(h) => Ok(h),
            RequestBody::Query(q) => Ok(q),
            _ => Err("Not json or query value")
        }
    }
}

impl<'a> TryInto<&'a str> for RequestBody<'a> {
    type Error = &'a str;

    fn try_into(self) -> Result<&'a str, Self::Error> {
        if let Self::Text(e) = self {
            Ok(e)
        } else {
            Err("Self isnt of type Text")
        }
    }
}