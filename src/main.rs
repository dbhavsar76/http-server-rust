use std::io::Write;
use std::net::{Shutdown, TcpListener};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for _stream in listener.incoming() {
        match _stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes());
                stream.shutdown(Shutdown::Both);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
