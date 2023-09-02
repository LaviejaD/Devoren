use crate::http::parser;
use crate::utils::find_port;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Rex {
    port: i32,
}
fn handle_client(mut client: TcpStream) {
    println!("new connection");
    let content = fs::read_to_string("./src/index.html").expect("error a leer xd");
    let _len = content.len();
    let foram = format!("HTTP/1.1 200 OK\r\n\r\n{content}");

    let mut buffer = BufReader::new(&mut client);

    let buffer = buffer.fill_buf().unwrap().to_vec();
    //  client.read(&mut buffer).unwrap();

    client.write(foram.as_bytes()).unwrap();
    client.write(b"Hello, World from devonrex").unwrap();
    client.flush().unwrap();
    let req = String::from_utf8(buffer.to_vec()).expect("error xd");

    println!("{}", req);
}

impl Rex {
    pub fn new() -> Self {
        Rex { port: find_port() }
    }
    pub fn set_port(mut self, p: i32) -> Self {
        self.port = p;
        self
    }
    pub fn run(self) {
        let lister = TcpListener::bind(format!("127.0.0.1:{0}", self.port)).unwrap();
        println!("http://127.0.0.1:{0}/", self.port);
        for stream in lister.incoming() {
            let client = stream.unwrap();

            parser("hola".to_string());
            handle_client(client);
        }
    }
}
