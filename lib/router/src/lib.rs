use client::Client;
use http::{has_dinamy_params, url_split, Method, Request, Response};

use std::{collections::HashMap, thread};

pub trait Route {
    fn run(&self, request: Request, client: Client) -> thread::JoinHandle<()>;
    fn endpoint(&self) -> (Method, String);
    fn callback(&self, request: Request) -> Response;
}

pub struct Routes {
    routes: HashMap<String, Box<dyn Route>>,
    routes_dinamy: HashMap<String, Box<dyn Route>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
            routes_dinamy: HashMap::new(),
        }
    }
    pub fn insert(&mut self, route: impl Route + 'static) {
        let (method, endpoint) = route.endpoint();
        let format = format!("{} {}", method.to_string(), endpoint.to_lowercase());

        match has_dinamy_params(endpoint.clone()) {
            true => self.routes_dinamy.insert(format, Box::new(route)),
            false => self.routes.insert(format, Box::new(route)),
        };
    }

    pub fn get(&self, request: &Request) -> Option<&Box<dyn Route>> {
        let endpoint = request.endpoint.clone();
        println!("{endpoint}");
        let method = request.method.to_string();
        let get = format!("{} {}", method, endpoint);

        let r = &self.routes.get(&get);
        *r
    }
}
