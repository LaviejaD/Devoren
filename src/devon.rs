use crate::utils::find_port;
use http::parser;
use router::{Route, Routes};
use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};
fn handle_client(client: &mut TcpStream) {
    println!("new connection");
    let content = fs::read_to_string("./index.html").unwrap();
    let len = content.len();
    let content_type = "text/html";
    let foram = format!(
        "HTTP/1.1 200 OK\r\nContent-Length:{len}\r\nContent-Type:{content_type}\r\n\r\n{content}"
    );

    //  client.read(&mut buffer).unwrap();
    client.write(foram.as_bytes()).unwrap();
    client.write(b"Hello, World from devonrex").unwrap();
    client.flush().unwrap();
    // println!("{:#?}", lines);
}

pub struct Rex {
    port: i32,
    routes: Routes,
}

impl Rex {
    pub fn new() -> Self {
        Rex {
            port: find_port(),
            routes: Routes::new(),
        }
    }
    pub fn set_port(mut self, p: i32) -> Self {
        self.port = p;
        self
    }
    pub fn add_routes(mut self, r: impl Route + 'static) -> Self {
        self.routes.insert(r);
        self
    }
    pub fn run(self) {
        let lister = TcpListener::bind(format!("127.0.0.1:{0}", self.port)).unwrap();
        println!("http://127.0.0.1:{0}/", self.port);
        for stream in lister.incoming() {
            let mut client = stream.unwrap();
            let request = parser(client.try_clone().unwrap());
            //handle_client(&mut client);
        }
    }
}
