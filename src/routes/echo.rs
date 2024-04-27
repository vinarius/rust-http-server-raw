use crate::{models::Request, Response, ResponseHeaders, Status};

use super::router::get_nested_resources;

pub fn handle_echo(request: Request) -> Response {
    let nested_resources = get_nested_resources(request);

    Response {
        status: Status::Ok,
        headers: Some(ResponseHeaders {
            content_type: String::from("text/plain"),
            content_length: nested_resources.len(),
        }),
        body: Some(String::from(nested_resources)),
    }
}
