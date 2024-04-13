use std::{fs, io::{Read, Write}, net::TcpStream};



pub fn handle_connection (mut stream:TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Req : {}",String::from_utf8_lossy(&buffer[..]));
    let response_content = fs::read_to_string("front-end/index.html").unwrap();

    let response =  format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",response_content.len(),response_content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();


}