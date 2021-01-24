use futures::executor::block_on;
use futures::executor::ThreadPool;
use futures_timer::Delay;
use mio::net::{TcpListener, TcpStream};
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;

pub async fn run() {
    let pool = ThreadPool::new().expect("Failed to build pool");

    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8080);
    let listener = TcpListener::bind(socket).unwrap();
    loop {
        match listener.accept() {
            Ok((stream, _)) => {
                println!("Ok");
                pool.spawn_ok(handle_connection(stream));
                println!("done");
            }
            Err(err) => {
                match err.kind() {
                    ErrorKind::WouldBlock => {
                        continue;
                    }
                    d => {
                        println!("error: {:?}", d);
                    }
                }
                if err.kind() == ErrorKind::WouldBlock {
                    continue;
                }
            }
        }
    }
}

async fn handle_connection(mut stream: TcpStream) {
    println!("handle_connection");
    Delay::new(Duration::from_secs(3)).await;

    // Read the first 1024 bytes of data from the stream
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";

    // Respond with greetings or a 404,
    // depending on the data in the request
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    // Write response back to the stream,
    // and flush the stream to ensure the response is sent back to the client
    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, contents.len(), contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
