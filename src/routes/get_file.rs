use std::{env, path::Path};

use itertools::Itertools;

use crate::{
    models::{Request, Response, ResponseHeaders, Status},
    routes::router::get_nested_resources,
};

pub fn handle_get_file(request: Request) -> Response {
    let mut args = env::args();
    let args_find_attempt = args.find_position(|arg| arg == "--directory");

    if args_find_attempt.is_none() {
        return Response {
            status: Status::BadRequest,
            headers: None,
            body: Some(String::from(
                "Expected a --directory flag to be included as an argument to the program",
            )),
        };
    }

    let get_directory_attempt = args.next();

    if get_directory_attempt.is_none() {
        return Response {
            status: Status::BadRequest,
            headers: None,
            body: Some(String::from(
                "Expected a directory argument to be included after the --directory flag",
            )),
        };
    }

    let directory_string = get_directory_attempt.unwrap();
    let directory = Path::new(&directory_string);

    if !directory.exists() {
        return Response {
            status: Status::NotFound,
            headers: None,
            body: Some(String::from(format!(
                "Directory passed in not found: {:?}",
                directory
            ))),
        };
    }

    let file_name = get_nested_resources(request);
    println!("file_name: {file_name}");

    let file_path_string = format!("{}{}", directory_string, file_name);
    let file_path = Path::new(&file_path_string);
    println!("file_path: {file_path:?}");

    if !file_path.exists() {
        return Response {
            status: Status::NotFound,
            headers: None,
            body: Some(String::from(format!("file not found: {:?}", file_name))),
        };
    }

    let file_contents = std::fs::read_to_string(file_path).unwrap();

    return Response {
        status: Status::Ok,
        headers: Some(ResponseHeaders {
            content_type: String::from("application/octet-stream"),
            content_length: file_contents.len(),
        }),
        body: Some(file_contents),
    };
}
