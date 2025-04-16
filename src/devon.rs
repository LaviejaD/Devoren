use crate::utils::find_port;
use client::Client;
use http::parser_http_client;
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
    thread_size: usize,
}
impl default::Default for Rex {
    fn default() -> Self {
        Rex {
            port: find_port(),
            routes: Routes::new(),
            thread_size: 10,
        }
    }
}

impl Rex {
    pub fn new(port: u32, thread_size: usize) -> Self {
        Rex {
            port,
            routes: Routes::new(),
            thread_size,
        }
    }

    pub fn set_port(mut self, p: u32) -> Self {
        self.port = p;
        self
    }
    pub fn add_routes(mut self, r: impl Route + 'static) -> Self {
        self.routes.insert(r);
        self
    }
    pub fn run(self) {
        let mut thread_manager = ThreadManager::new(self.thread_size);
        if let Ok(lister) = TcpListener::bind(format!("127.0.0.1:{0}", self.port)) {
            // lister
            //     .set_nonblocking(true)
            //     .expect("Cannot set non-blocking");
            for stream in lister.incoming() {
                match stream {
                    Ok(client_stream) => {
                        let client = Client::new(client_stream);
                        let mut request = parser_http_client(&client);
                        if let Some(route) = self.routes.get(&mut request) {
                            let r = route.run(request, client);
                            thread_manager.add(r);
                        }
                    }

                    Err(e) => println!("Error {:#?}", e),
                }
            }
            //lister
        }
    }
}
