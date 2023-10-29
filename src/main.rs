use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use itertools::Itertools;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for _stream in listener.incoming() {
        match _stream {
            Ok(stream) => {
                println!("accepted new connection");
                handle_request(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(mut stream: TcpStream) {
    let mut request = String::new();
    let _ = stream.read_to_string(&mut request);
    // let _ = stream.shutdown(Shutdown::Read);

    let path = get_request_path(request);
    let response = match path.as_str() {
        "/" => "HTTP/1.1 200 OK\r\n\r\n",
        _ => "HTTP/1.1 404 Not Found\r\n\r\n",
    };

    let _ = stream.write_all(response.as_bytes());
    let _ = stream.shutdown(Shutdown::Both);
}

fn get_request_path(request: String) -> String {
    let sections = request.split("\r\n\r\n").collect_vec();

    let head = sections[0];
    let parts = head.split("\r\n").collect_vec();

    let start_line = parts[0];
    let path = start_line.split_whitespace().nth(1).unwrap();

    path.to_string()
}