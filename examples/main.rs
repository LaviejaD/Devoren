use devonrex::prelude::*;
use std::{fs, thread};

#[route(get,/)]
fn Index(request: Request) -> Response {
    let mut response = Response::default();
    if let Ok(html) = fs::read_to_string("./public/index.html") {
        response.body(html)
    }
    response
}
#[route(get,/test/<name>)]
fn Test(request: Request) -> Response {
    let response = Response::default().body("test".to_string());
    response
}
#[route(get,/test/<lastname>)]
fn Test2(request: Request) -> Response {
    let response = Response::default().body("test2".to_string());
    response
}
fn main() {
    let port = 8080;
    println!("http://127.0.0.1:{0}/", port);
    Rex::new(port, 5)
        .add_routes(Index)
        .add_routes(Test2)
        .add_routes(Test)
        .run();
}
