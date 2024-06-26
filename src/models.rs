#[derive(Debug, PartialEq)]
pub enum Method {
    Get,
    Options,
    Head,
    Post,
    Put,
    Delete,
    Trace,
    Connect,
    Unhandled,
}

#[derive(Debug)]
pub enum Version {
    V1_1,
    Unhandled,
}

pub enum Status {
    Ok,
    Created,
    BadRequest,
    NotFound,
    InternalServerError,
}

impl Status {
    pub fn into_string(&self) -> String {
        match self {
            Self::Ok => String::from("200 OK"),
            Self::Created => String::from("201 Created"),
            Self::BadRequest => String::from("400 Bad Request"),
            Self::NotFound => String::from("404 Not Found"),
            Self::InternalServerError => String::from("500 Internal Server Error"),
        }
    }
}

#[derive(Debug)]
pub struct RequestHeaders {
    pub host: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub content_length: Option<usize>,
}

pub struct ResponseHeaders {
    pub content_type: String,
    pub content_length: usize,
}

#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub version: Version,
    pub headers: RequestHeaders,
    pub body: String,
}

pub struct Response {
    pub status: Status,
    pub headers: Option<ResponseHeaders>,
    pub body: Option<String>,
}
