use crate::models::{Request, Response, Status};
use std::env;

pub fn handle_get_file(_request: Request) -> Response {
    let args = env::args();

    println!("args: {args:#?}");

    return Response {
        status: Status::BadRequest,
        headers: None,
        body: Some(String::from(
            "Expected request to have a user_agent header, but found none",
        )),
    };
}
