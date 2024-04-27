use std::{
    io::{self, BufRead, BufReader},
    net::TcpStream,
};

use crate::models::{Method, Request, RequestHeaders, Version};

pub fn parse_request(stream: &TcpStream) -> io::Result<Request> {
    let mut my_buf_reader = BufReader::new(stream);
    let mut raw_request = String::new();

    loop {
        let mut buf = String::new();
        my_buf_reader.read_line(&mut buf)?;
        raw_request.push_str(&buf);

        if buf == "\r\n" {
            break;
        }
    }

    println!("request received");
    println!("---------------");
    println!("{raw_request}");

    let mut raw_request_split = raw_request.split_whitespace();
    let method = raw_request_split.next().unwrap();
    let path = raw_request_split.next().unwrap();
    let version = raw_request_split.next().unwrap();

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

    let mut request_headers = RequestHeaders {
        host: None,
        user_agent: None,
        accept: None,
    };

    let mut raw_request_lines = raw_request.lines();
    raw_request_lines.next();

    for line in raw_request_lines {
        if line.is_empty() {
            break;
        }

        let header_name_split = line.split(": ").collect::<Vec<&str>>();

        let header_name = header_name_split
            .get(0)
            .expect("expected header to have a name prior to a : char");

        let header_value = header_name_split
            .get(1)
            .expect("expected header to have a value after a : char");

        if *header_name == "Host" {
            request_headers.host = Some(header_value.to_string());
        }

        if *header_name == "User-Agent" {
            request_headers.user_agent = Some(header_value.to_string());
        }

        if *header_name == "Accept" {
            request_headers.accept = Some(header_value.to_string());
        }
    }

    let request = Request {
        method,
        path: String::from(path),
        version,
        headers: request_headers,
    };

    // println!("{request:#?}");

    Ok(request)
}
