use crate::request;
use std::collections::HashMap;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;
pub fn parser(client: TcpStream) -> request::Request {
    let mut buffer = BufReader::new(client);
    let mut headers = HashMap::new();

    let buffer = buffer.fill_buf().unwrap().to_vec();

    let req = String::from_utf8(buffer).unwrap();
    let mut lines: Vec<_> = req.lines().collect();
    let yew = lines.remove(0).split(" ").collect::<Vec<_>>();

    let method = yew.get(0).unwrap().to_string();
    let endpoint = yew.get(1).unwrap().to_string();
    let version = yew
        .get(2)
        .unwrap()
        .to_string()
        .split("/")
        .collect::<Vec<_>>()
        .pop()
        .unwrap()
        .to_string();
    let mut b = 0;
    for header in lines {
        b += 1;
        if header.is_empty() {
            println!("Empty line {}", b);
            break;
        }
        let header_split = header.split_once(": ").unwrap();
        headers.insert(header_split.0.to_string(), header_split.1.to_string());
    }

    request::Request::new(method, endpoint, version, headers)
}
