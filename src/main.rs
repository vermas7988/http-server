#![allow(dead_code)]
mod http;
mod server;

use crate::server::Server;

fn main() {
    println!("Hello, world!");

    let server_address = "127.0.0.1:5000".to_string();
    let server = Server::new(server_address);
    server.run();
}
