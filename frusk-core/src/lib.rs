// use futures::executor::block_on;
use futures::executor::ThreadPool;
use futures::prelude::*;
use mio::net::{TcpListener, TcpStream};
use std::fs;
use std::io::prelude::*;
use std::io::ErrorKind;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

mod tcp;
use tcp::Listener;

pub fn create_listener(host: &str, port: u16) -> TcpListener {
    let host_vec: Vec<u8> = host.split(".").map(|x| x.parse::<u8>().unwrap()).collect();
    println!("host_vec {:?}", host_vec);
    let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    TcpListener::bind(socket).unwrap()
}

pub async fn handle_connection(mut stream: TcpStream) {
    println!("handle_connection");

    let mut buffer = [0; 2048];
    loop {
        match stream.read(&mut buffer) {
            Ok(n) => {
                println!("n: {}", n);
                break;
            }
            Err(err) => match err.kind() {
                ErrorKind::WouldBlock => {
                    continue;
                }
                d => {
                    println!("stream error: {:?}", d);
                    return;
                }
            },
        }
    }

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
