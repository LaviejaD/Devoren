use crate::utils::find_port;
use client::Client;
use http::{parser_http_client, Request};
use router::{Route, Routes};
use std::{default, net::TcpListener};
use thread::ThreadManager;
// fn handle_client(client: &mut TcpStream, response: String) {
//     println!("new connection");
//     let content = fs::read_to_string("./index.html").unwrap();
//     let len = content.len();
//     let content_type = "text/html";
//     let foram = format!(
//     "HTTP/1.1 200 OK\r\nContent-Length:{len}\r\nContent-Type:{content_type}\r\n\r\n{content}"
//     );

//     client.write_all(response.as_bytes()).unwrap();
//     client.write(b"Hello, World from devonrex").unwrap();
//     client.flush().unwrap();
// }
pub struct Rex {
    port: u32,
    routes: Routes,
    middleware: Routes,
    threadmanager: ThreadManager,
}
impl default::Default for Rex {
    fn default() -> Self {
        Rex {
            port: find_port(),
            routes: Routes::new(),
            middleware: Routes::new(),
            threadmanager: ThreadManager::new(10),
        }
    }
}

impl Rex {
    pub fn new(port: u32, thread_size: usize) -> Self {
        Rex {
            port,
            routes: Routes::new(),
            middleware: Routes::new(),
            threadmanager: ThreadManager::new(thread_size),
        }
    }

    pub fn set_port(mut self, p: u32) -> Self {
        self.port = p;
        self
    }
    pub fn add_routes(&mut self, r: impl Route + 'static) -> &mut Self {
        self.routes.insert(r);
        self
    }
    pub fn add_middleware(&mut self, m: impl Route + 'static) -> &mut Self {
        self.middleware.insert(m);
        self
    }
    pub fn middleware_handle(&mut self, request: &mut Request) {
        if let Some(middleware) = self.routes.get(request) {}
    }
    pub fn routes_handle(&mut self, request: &mut Request, client: Client) {
        if let Some(route) = self.routes.get(request) {
            let r = route.run(request.clone(), client);
            self.threadmanager.add(r);
        }
    }
    pub fn run(&mut self) {
        if let Ok(lister) = TcpListener::bind(format!("127.0.0.1:{0}", self.port)) {
            // lister
            //     .set_nonblocking(true)
            //     .expect("Cannot set non-blocking");
            for stream in lister.incoming() {
                match stream {
                    Ok(client_stream) => {
                        let client = Client::new(client_stream);
                        let mut request = parser_http_client(&client);
                        self.middleware_handle(&mut request);
                        self.routes_handle(&mut request, client);
                    }

                    Err(e) => println!("Error {:#?}", e),
                }
            }
            //lister
        }
    }
}
