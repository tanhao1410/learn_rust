use std::net::TcpStream;
use std::io::{Read, Write};
use std::fs;

pub fn handle_connection(mut stream:TcpStream){
    // let mut buff = [0u8;500];
    // stream.read(&mut buff).unwrap();
    // let request = String::from_utf8_lossy(& buff);
    //
    // println!("{}",request);
    // let content = fs::read_to_string("hello.html").unwrap();
    // let respone = format!{"HTTP/1.1 200 OK\r\n\r\n{}",content};
    // stream.write(respone.as_bytes());
    // stream.flush().unwrap();
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();

    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    println!("{}",response);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();

}