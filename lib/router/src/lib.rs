use client::Client;
use http::{Method, Request, Response};

use std::{collections::HashMap, thread};

// pub struct Route {
//     pub callback: Arc<dyn Fn(Request) -> Response + Send>,
//     pub endpoint: String,
//     pub method: Method,
// }

// impl Route {
//     pub fn new<T: Fn(Request) -> Response + 'static + Send>(
//         method: Method,
//         endpoint: String,
//         callback: T,
//     ) -> Self {
//         Route {
//             callback: Arc::new(callback),

//             endpoint,
//             method,
//         }
//     }
//     pub fn endpoint() {
//         todo!()
//     }
// }

pub trait Route {
    fn run(&self, request: Request, client: Client) -> thread::JoinHandle<()>;
    fn endpoint(&self) -> (Method, String);
    fn callback(&self, request: Request) -> Response;
}

pub struct Routes {
    routes_map: HashMap<String, Box<dyn Route>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            routes_map: HashMap::new(),
        }
    }
    pub fn insert(&mut self, route: impl Route + 'static) {
        let (method, endpoint) = route.endpoint();
        let format = format!("{} {}", method.to_string(), endpoint.to_lowercase());
        self.routes_map.insert(format, Box::new(route));
    }

    pub fn get(&self, request: &Request) -> Option<&Box<dyn Route>> {
        let endpoint = request.endpoint.clone();
        let method = request.method.to_string();
        let get = format!("{} {}", method, endpoint);

        let r = &self.routes_map.get(&get);
        *r
    }
}
