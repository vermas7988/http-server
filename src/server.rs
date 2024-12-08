use std::io::Read;
use std::net::TcpListener;
use crate::http::request::Request;

pub struct Server {
    address: String,
}

impl Server {
    pub fn new(address: String) -> Self {
        Server { address }
    }

    pub fn run(self) {
        println!("Listening on {}", self.address);
        let listener = TcpListener::bind(&self.address).unwrap();
        loop {
            match listener.accept() {
                Ok((mut stream, address)) => {
                    println!("OK from: {}", address);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(size) => {
                            println!("Received request: {}, size {}", String::from_utf8_lossy(&buffer[..size]), size);
                            match Request::try_from(&buffer[..]) {
                                Ok(_request) => {}
                                Err(e) => {
                                    println!("Failed to parse request: {}", e)
                                }
                            }
                        }
                        Err(e) => {
                            println!("Failed to read from connection: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("error in connection: {}", e);
                }
            }
        }
    }
}