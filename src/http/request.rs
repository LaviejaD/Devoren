use crate::http::method::Method;
use std::collections::HashMap;
#[derive(Debug)]
pub struct Request {
    pub method: Method,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}
