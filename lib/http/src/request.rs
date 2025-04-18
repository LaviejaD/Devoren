use crate::method::Method;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub endpoint: String,
    pub parameters: HashMap<String, String>,
    pub query: HashMap<String, String>,
    pub http_version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Request {
    pub fn new(
        method: String,
        endpoint: String,
        http_version: String,
        headers: HashMap<String, String>,
        parameters: HashMap<String, String>,
        query: HashMap<String, String>,
    ) -> Self {
        Self {
            method: Method::from(method),
            endpoint,
            http_version,
            headers,
            parameters,
            query,
            body: String::new(),
        }
    }
}
