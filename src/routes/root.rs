use crate::models::{HttpHeaders, HttpResponse, HttpStatus};

pub fn handle_root() -> HttpResponse {
    HttpResponse {
        status: HttpStatus::Ok,
        headers: HttpHeaders {
            content_type: String::from("text/plain"),
            content_length: 0,
        },
        body: String::new(),
    }
}
