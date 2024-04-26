use crate::models::{Response, Status};

pub fn handle_root() -> Response {
    Response {
        status: Status::Ok,
        headers: None,
        body: None,
    }
}
