pub enum HttpMethod {
    Get,
    Unhandled(),
}

pub enum HttpVersion {
    V1_1,
    Unhandled(),
}

pub enum HttpStatus {
    Ok,
    NotFound,
}

impl HttpStatus {
    pub fn into_string(&self) -> String {
        match self {
            Self::Ok => String::from("200 OK"),
            Self::NotFound => String::from("404 Not Found"),
        }
    }
}

pub struct HttpHeaders {
    pub content_type: String,
    pub content_length: usize,
}

pub struct HttpResponse {
    pub status: HttpStatus,
    pub headers: HttpHeaders,
    pub body: String,
}
