use crate::{
    models::{Method, Request, Response, Status},
    routes::{echo::handle_echo, root::handle_root, user_agent::handle_user_agent},
};

use super::get_file::handle_get_file;

pub fn router(request: Request) -> Response {
    let mut resources_split = request.path.split('/');
    let root_resource = resources_split.nth(1).unwrap();

    // not great, but works for now
    match root_resource {
        "" if request.method == Method::Get => handle_root(),
        "echo" if request.method == Method::Get => handle_echo(request),
        "user-agent" if request.method == Method::Get => handle_user_agent(request),
        "files" if request.method == Method::Get => handle_get_file(request),
        _ => Response {
            status: Status::NotFound,
            headers: None,
            body: None,
        },
    }
}

pub fn get_nested_resources(request: Request) -> String {
    let mut resources_split = request.path.split('/');
    resources_split.next().unwrap();
    resources_split.next().unwrap();

    let collected_resources = resources_split.collect::<Vec<&str>>();
    let joined_resources = collected_resources.join("/");

    joined_resources
}
