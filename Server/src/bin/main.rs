use std::fmt::format;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::prelude::*;
use std::{fs, thread};
use std::time::Duration;
use Server::Threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = Threadpool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_stream(stream);
        });
    }
}

fn handle_stream(mut stream:TcpStream){
    let mut buffer = [0;1024];      // Buffer 1024 bytes long
    stream.read(&mut buffer).unwrap();
    let get = b"GET / HTTP/1.1\r\n";    // GET request
    let sleep = b"GET /sleep HTTP/1.1\r\n";    // GET request

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if {buffer.starts_with(sleep)} {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!("{}{}", status_line, contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
