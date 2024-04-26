use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    models::{HttpHeaders, HttpMethod, HttpResponse, HttpStatus, HttpVersion},
    routes::{echo::handle_echo, root::handle_root},
};

mod models;
mod routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                if let Err(stream_error) = handle_stream_connection(stream) {
                    println!("error handling stream connection: {stream_error}");
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut my_buf_reader = BufReader::new(&stream);
    let mut status_line = String::new();
    my_buf_reader.read_line(&mut status_line)?;

    let mut status_line_split = status_line.split_whitespace();
    let method = status_line_split.next().unwrap();
    let path = status_line_split.next().unwrap();
    let version = status_line_split.next().unwrap();

    let _method = match method {
        "GET" => HttpMethod::Get,
        _ => HttpMethod::Unhandled(),
    };

    let _version = match version {
        "HTTP1/1" => HttpVersion::V1_1,
        _ => HttpVersion::Unhandled(),
    };

    let mut resources_split = path.split('/');
    let root_resource = resources_split.nth(1).unwrap();
    let nested_resources = resources_split.collect::<Vec<&str>>().join("/");

    println!("root_resource: {root_resource}");

    let response = match root_resource {
        "" => handle_root(),
        "echo" => handle_echo(&nested_resources),
        _ => HttpResponse {
            status: HttpStatus::NotFound,
            headers: HttpHeaders {
                content_type: String::from("text/plain"),
                content_length: 0,
            },
            body: String::new(),
        },
    };

    let HttpResponse {
        status: response_status,
        headers,
        body,
    } = response;

    let mut response = String::from("HTTP/1.1 ");

    response.push_str(&response_status.into_string());
    response.push_str("\r\n");

    if headers.content_length > 0 {
        response.push_str(format!("Content-Type: {}", headers.content_type).as_str());
        response.push_str("\r\n");
        response.push_str(format!("Content-Length: {}", headers.content_length).as_str());
        response.push_str("\r\n");
        response.push_str("\r\n");
        response.push_str(&body);
    } else {
        response.push_str("\r\n");
    }

    println!();
    println!("response:");
    println!("---------------");
    println!("{response}");
    println!("---------------");

    stream.write_all(response.as_bytes())?;

    Ok(())
}
