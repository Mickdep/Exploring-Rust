use chapter_20::ThreadPool;
use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(||{
            handle_connection(stream);
        });
    }

    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(s) => handle_connection(s),
    //         Err(e) => println!("Error occurred: {}", e),
    //     }
    // }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get){
        ("HTTP/1.1 200 OK \r\n\r\n", "hello.html")
    }else{
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let file_content = fs::read_to_string(filename).unwrap();

    let response = format!("{}{}", status_line, file_content);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
