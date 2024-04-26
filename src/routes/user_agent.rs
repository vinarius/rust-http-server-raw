use crate::{
    models::{Request, RequestHeaders},
    Response, ResponseHeaders, Status,
};

pub fn handle_user_agent(request: Request) -> Response {
    let Request { headers, .. } = request;
    let RequestHeaders { user_agent, .. } = headers;

    if user_agent.is_none() {
        return Response {
            status: Status::BadRequest,
            headers: None,
            body: Some(String::from(
                "Expected request to have a user_agent header, but found none",
            )),
        };
    }

    let user_agent = user_agent.unwrap();

    Response {
        status: Status::Ok,
        headers: Some(ResponseHeaders {
            content_type: String::from("text/plain"),
            content_length: user_agent.len(),
        }),
        body: Some(String::from(user_agent)),
    }
}
