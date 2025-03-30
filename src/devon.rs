use crate::utils::find_port;
use client::Client;
use http::{parser, Request, Response};
use router::{Route, Routes};
use std::{
    io::prelude::*,
    net::{TcpListener, TcpStream},
    thread::{self, JoinHandle},
    time,
};
fn handle_client(client: &mut TcpStream, response: String) {
    println!("new connection");
    // let content = fs::read_to_string("./index.html").unwrap();
    // let len = content.len();
    // let content_type = "text/html";
    // let foram = format!(
    // "HTTP/1.1 200 OK\r\nContent-Length:{len}\r\nContent-Type:{content_type}\r\n\r\n{content}"
    // );

    client.write_all(response.as_bytes()).unwrap();
    // client.write(b"Hello, World from devonrex").unwrap();
    client.flush().unwrap();
}

pub struct Rex {
    port: u32,
    routes: Routes,
}

impl Rex {
    pub fn new() -> Self {
        Rex {
            port: find_port(),
            routes: Routes::new(),
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
        // let waitin = Vec::new();
        if let Ok(lister) = TcpListener::bind(format!("127.0.0.1:{0}", self.port)) {
            //       let mut hilos = Vec::new();

            for stream in lister.incoming() {
                // let start = time::Instant::now();

                match stream {
                    Ok(client_stream) => {
                        let client = Client::new(client_stream);
                        let request = parser(&client);
                        if let Some(route) = self.routes.get(&request) {
                            let r = route.run(request, client);

                            // println!("hola ?{r}");
                            // handle_client(&mut client, r);
                        }
                        // println!("no existe")
                    }
                    Err(_) => println!("Error"),
                }
                // println!("{:#?}", start.elapsed());
            }

            // let request = parser(&client);
            // self.routes.get(request);

            // handle_client(&mut client);
        }
    }
}

// for stream in lister.incoming() {
//     //     let mut client = stream.unwrap();

//     match stream {
//         Ok(mut client) => {
//             let request = parser(&client);
//             if let Some(response) = self.routes.get(&request) {
//                 // handle_client(&mut client, response.http());

//                 handle_client(&mut client, response.call(request).http());
//                 let hilo = thread::spawn(move || {});
//                 println!("{}", hilos.len());
//                 hilos.push(hilo);
//             }
//         }
//         Err(_) => println!("Error"),
//     }
// }

//   let request = parser(&client);
//  self.routes.get(request);

//      handle_client(&mut client);
