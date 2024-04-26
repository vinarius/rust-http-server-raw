use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use crate::{
    models::{Method, Request, RequestHeaders, Response, ResponseHeaders, Status, Version},
    routes::router::router,
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

    println!("request received");
    println!("---------------");
    println!("{status_line}");

    let mut status_line_split = status_line.split_whitespace();
    let method = status_line_split.next().unwrap();
    let path = status_line_split.next().unwrap();
    let version = status_line_split.next().unwrap();

    // i think this could be better somehow
    let method = match method {
        "GET" => Method::Get,
        "OPTIONS" => Method::Options,
        "HEAD" => Method::Head,
        "POST" => Method::Post,
        "PUT" => Method::Put,
        "DELETE" => Method::Delete,
        "TRACE" => Method::Trace,
        "CONNECT" => Method::Connect,
        _ => Method::Unhandled,
    };

    let version = match version {
        "HTTP/1.1" => Version::V1_1,
        _ => Version::Unhandled,
    };

    let request_headers = RequestHeaders {
        host: Some(String::new()),       // TODO:
        user_agent: Some(String::new()), // TODO:
    };

    let request = Request {
        method,
        path: String::from(path),
        version,
        headers: request_headers,
    };

    println!("{request:#?}");

    let handler_response = router(request);

    let Response {
        status: response_status,
        headers,
        body,
    } = handler_response;

    let mut response = String::from("HTTP/1.1 ");

    response.push_str(&response_status.into_string());
    response.push_str("\r\n");

    if headers
        .as_ref()
        .is_some_and(|headers| headers.content_length > 0)
    {
        let ResponseHeaders {
            content_type,
            content_length,
        } = headers.unwrap();

        response.push_str(format!("Content-Type: {}", content_type).as_str());
        response.push_str("\r\n");
        response.push_str(format!("Content-Length: {}", content_length).as_str());
        response.push_str("\r\n");
        response.push_str("\r\n");

        if body.is_some() {
            response.push_str(&body.unwrap());
        }
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
