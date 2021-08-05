#![allow(non_snake_case)]

use std::io::prelude::*;
use std::fs;
use askama::Template; 

#[derive(Template)]
#[template(path = "404.html")] 
struct ErrorTemplate<'a> { 
    requested_file: &'a str,
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:80").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        HandleConnection(stream);
    }
}

fn HandleConnection(mut stream: std::net::TcpStream){
    let mut buffer = [0u8;1024];
    stream.read(&mut buffer).unwrap();

    let requested_file = GetRequestedFile(&buffer);

    let (status_line, content) = match requested_file{
        b"/" =>("HTTP/1.1 200 OK", fs::read_to_string("templates/index.html").unwrap()),
        _    =>("HTTP/1.1 404 NOT FOUND", ErrorTemplate{requested_file : &String::from_utf8_lossy(requested_file)}.render().unwrap() ),
    };

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}",
                                 status_line, content.len(), content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}

fn GetRequestedFile(line : &[u8]) -> &[u8]{
 
    let start_bytes = FindInBytes(line, b"GET ").unwrap_or(0) + "GET ".len();

    let end_bytes = FindInBytes(line, b" HTTP").unwrap_or(b" HTTP".len()); 

    return &line[start_bytes..end_bytes]; 
    
}

fn FindInBytes(bytes : &[u8], needle : &[u8]) -> Option<usize>{
    bytes.windows(needle.len()).position(|window| window == needle)
}