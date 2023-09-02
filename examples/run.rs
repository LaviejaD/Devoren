use devonrex::create_route;
use devonrex::prelude::{Method, Rex, Route};
fn ruta() -> String {
    "Hello".to_string()
}

create_route!(hola, Post(&"/index.html"), ruta);

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    stream.write(b"HTTP/1.1 200 OK\r\n\r\n").unwrap();
    stream.write(b"Hello, Worldasdddddd!").unwrap();
    stream.flush().unwrap();
}

fn main() {
    Rex::new().run();

    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        // handle_client(stream.unwrap());
        break;
    }
}
