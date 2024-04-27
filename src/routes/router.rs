use crate::{
    models::{Method, Request, Response, Status},
    routes::{echo::handle_echo, root::handle_root, user_agent::handle_user_agent},
};

pub fn router(request: Request) -> Response {
    let mut resources_split = request.path.split('/');
    let root_resource = resources_split.nth(1).unwrap();
    let nested_resources = resources_split.collect::<Vec<&str>>().join("/");

    // not great, but works for now
    match root_resource {
        "" if request.method == Method::Get => handle_root(),
        "echo" if request.method == Method::Get => handle_echo(&nested_resources),
        "user-agent" if request.method == Method::Get => handle_user_agent(request),
        _ => Response {
            status: Status::NotFound,
            headers: None,
            body: None,
        },
    }
}
