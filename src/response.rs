
macro_rules! response {
    (code=$code:expr,text=$text:expr) => {

        Response {
            code: $code,
            text: $text.to_string(),
            len: $text.to_string().len()
        }

    }
}

pub struct Response {
    pub code: u32,
    pub text: String,
    pub len: usize
}

impl Response {
    pub fn new(code: u32, text: String) -> Self {
        Response {
            code,
            text: text.clone(),
            len: text.len()
        }
    }
}

