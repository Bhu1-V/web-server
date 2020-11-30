use std::net::{TcpListener,TcpStream};
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::fs;
use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let tp = ThreadPool::new(4);
    for stream in listener.incoming(){
        let _stream = stream.unwrap();

        tp.execute(||{
            handle_connection(_stream);
        });
    }
}
fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    // print!("{}", String::from_utf8_lossy(&buffer));
    let status_line:&str;
    let content = if String::from_utf8_lossy(&buffer).starts_with("GET / "){
        status_line = "200 OK";
        fs::read_to_string("hello.html").unwrap()
    }else if  String::from_utf8_lossy(&buffer).starts_with("GET /sleep "){
        status_line = "200 OK";
        thread::sleep(Duration::from_secs(10));
        fs::read_to_string("hello.html").unwrap()
    }
    else{
        status_line = "404 NOT FOUND";
        fs::read_to_string("404.html").unwrap()
    };
    let response = format!("HTTP/1.1 {}\r\nContent-Length: {}\r\n\r\n{}",status_line,content.len(),content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}