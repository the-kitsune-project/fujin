use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream, ToSocketAddrs},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let buf_reader = BufReader::new(&mut stream);
    // let http_request: Vec<_> = buf_reader
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();

    // println!("Request: {:#?}", http_request);
    match fetch() {
        Ok(_) => {},
        Err(_e) => println!("failed")
    }
}

fn fetch() -> Result<(), Box<dyn std::error::Error>> {
    // Create a TCP stream to the target server.
    let addr = "www.google.com:80".to_socket_addrs()?.next().unwrap();
    let mut stream = TcpStream::connect(addr)?;
    // Write the HTTP request.
    let request = b"GET / HTTP/1.1\r\nHost: www.google.com\r\n\r\n";
    stream.write_all(request)?;
    stream.flush()?;

    // Read the HTTP response.
    let mut response = Vec::new();
    stream.read_to_end(&mut response)?;

    // Print the HTTP response status code.
    let status_code = response[0] as u8;
    println!("Status code: {}", status_code);

    // Print the HTTP response headers.
    let headers = response[1..].split(|b| *b == b'\n').collect::<Vec<_>>();
    for header in &headers {
        println!("{:?}", header);
    }

    // Print the HTTP response body.
    let body = &response[headers.len() * 2..];
    println!("{:?}", body);

    Ok(())
}