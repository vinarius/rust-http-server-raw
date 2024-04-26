use crate::{HttpHeaders, HttpResponse, HttpStatus};

pub fn handle_echo(path: &str) -> HttpResponse {
    HttpResponse {
        status: HttpStatus::Ok,
        headers: HttpHeaders {
            content_type: String::from("text/plain"),
            content_length: path.len(),
        },
        body: String::from(path),
    }
}
