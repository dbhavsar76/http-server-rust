use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
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

struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

fn handle_request(mut stream: TcpStream) {
    stream.set_read_timeout(Option::from(Duration::from_secs(1)));
    stream.set_write_timeout(Option::from(Duration::from_secs(1)));

    let mut request_str = String::new();
    stream.read_to_string(&mut request_str);
    let request = parse_request(request_str);

    let path = request.path;
    let response = match path.as_str() {
        "/" => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
        "/user-agent" => {
            let default = String::new();
            let user_agent = request.headers.get("User-Agent").unwrap_or(&default);
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                user_agent.len(),
                user_agent,
            )
        },
        p if p.starts_with("/echo/") => {
            let body = path.strip_prefix("/echo/").unwrap();
            format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body,
            )
        },
        _ => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
    };

    stream.write_all(response.as_bytes());
}

fn parse_request(request_str: String) -> Request {
    let sections = request_str.split("\r\n\r\n").collect_vec();

    let head = sections[0];
    let body = sections[1];

    let head_parts = head.split("\r\n").collect_vec();
    let start_line = head_parts[0];
    let headers = parse_headers(&head_parts[1..]);

    let start_line_parts = start_line.split_whitespace().collect_vec();
    let method = start_line_parts[0];
    let path = start_line_parts[1];
    let version = start_line_parts[2];

    Request {
        method: method.to_string(),
        path: path.to_string(),
        version: version.to_string(),
        headers,
        body: body.to_string(),
    }
}

fn parse_headers(headers_str: &[&str]) -> HashMap<String, String> {
    let mut headers: HashMap<String, String> = HashMap::new();

    for header in headers_str {
        let header_parts = header.split(":").map(|p| p.trim()).collect_vec();
        let key = header_parts[0];
        let value = header_parts[1];
        headers.insert(key.to_string(), value.to_string());
    }

    headers
}