extern crate integration;
use integration::ThreadPool;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

fn main() {
    env_logger::init();

    const HOST: &str = "127.0.0.1";
    const PORT: &str = "8477";

    // Concating Host address and Port to Create Final Endpoint
    let end_point: String = HOST.to_owned() + ":" + PORT;
    let pool = ThreadPool::new(4);

    // Creating TCP Listener at our end point
    let listener = TcpListener::bind(end_point).unwrap();
    log::info!("Server is listening at {}:{}", HOST, PORT);

    // Conneting to any incoming connections
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, contents) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "Hello World")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(10));
        ("HTTP/1.1 200 OK\r\n", "Good Mornig!")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "Not Found")
    };

    let response = format!(
        "{}Content-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
