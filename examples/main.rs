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

fn main() {
    let port = 8080;
    println!("http://127.0.0.1:{0}/", port);
    Rex::new(port, 5).add_routes(Index).run();
}
