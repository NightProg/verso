use std::collections::HashMap;
use crate::queryweb::QueryWeb;
#[derive(Copy, Clone)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Auth
}

pub struct Request {
    pub url: String,
    pub method: Method,
    pub header: HashMap<String,String>,
    pub get_info: String,
    pub post_info: String,
    pub put_info: String,
    pub delete_info: String
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

    pub fn get_json(&self) -> Result<HashMap<String, String>, String> {
        self.get_info.from_json()
    }

    pub fn get_querymap(&self) -> HashMap<String, String> {
        self.get_info.clone().from_query_map()
    }

    pub fn get_args(&self, index: &str, default: &str) -> String {
        return match self.get_querymap().get(index) {
            Some(v) => v.to_string(),
            None => default.to_string()
        }
    }
}
