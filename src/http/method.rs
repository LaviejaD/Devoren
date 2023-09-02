#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
    Get(String),
    Post(String),
    Put(String),
    Head(String),
    Delete(String),
    Trace(String),
    Connect(String),
}

impl Method {
    pub fn convert(&self) -> String {
        let r = match self {
            Method::Get(_) => "GET",
            Method::Post(_) => "POST",
            Method::Put(_) => "PUT",
            Method::Head(_) => "HEAD",
            Method::Trace(_) => "TRACE",
            Method::Delete(_) => "DELETE",
            Method::Connect(_) => "CONNECT",
        };
        r.to_string()
    }
}
