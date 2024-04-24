use std::{
    io::{self, BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

enum HttpMethod {
    Get,
    Unhandled(String),
}

enum HttpVersion {
    V1_1,
    Unhandled(String),
}

struct Request {
    method: HttpMethod,
    path: String,
    version: HttpVersion,
}

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

    println!("status_line: {status_line}");

    let mut status_line_split = status_line.split_whitespace();
    let method = status_line_split.next().unwrap();
    let path = status_line_split.next().unwrap();
    let version = status_line_split.next().unwrap();

    println!("method: {method}");
    println!("path: {path}");
    println!("version: {version}");

    let _method = match method {
        "GET" => HttpMethod::Get,
        _ => HttpMethod::Unhandled(String::from(method)),
    };

    let _version = match version {
        "HTTP1/1" => HttpVersion::V1_1,
        _ => HttpVersion::Unhandled(String::from(version)),
    };

    let response_status = match path {
        "/" => "200 OK",
        _ => "404 Not Found",
    };

    let response = format!("HTTP/1.1 {response_status}\r\n\r\n");

    stream.write_all(response.as_bytes())?;

    Ok(())
}
