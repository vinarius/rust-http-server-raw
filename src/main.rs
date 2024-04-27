use std::{net::TcpListener, thread};

use crate::handle_stream_connection::handle_stream_connection;
use crate::models::{Response, ResponseHeaders, Status};

mod handle_stream_connection;
mod models;
mod parse_request;
mod routes;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    if let Err(stream_error) = handle_stream_connection(stream) {
                        println!("error handling stream connection: {stream_error}");
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
