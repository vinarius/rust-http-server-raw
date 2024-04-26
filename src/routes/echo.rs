use crate::{Response, ResponseHeaders, Status};

pub fn handle_echo(path: &str) -> Response {
    Response {
        status: Status::Ok,
        headers: Some(ResponseHeaders {
            content_type: String::from("text/plain"),
            content_length: path.len(),
        }),
        body: Some(String::from(path)),
    }
}
