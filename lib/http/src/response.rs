use std::collections::HashMap;
struct Response {
    pub endpoint: String,
    pub version: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl Response {}
