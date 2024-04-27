use std::{
    io::{self, Write},
    net::TcpStream,
};

use crate::{
    models::{Response, ResponseHeaders},
    parse_request::parse_request,
    routes::router::router,
};

pub fn handle_stream_connection(mut stream: TcpStream) -> io::Result<()> {
    let request = parse_request(&stream)?;
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
    } else {
        response.push_str("\r\n");
    }

    if body.is_some() {
        response.push_str(&body.unwrap());
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
