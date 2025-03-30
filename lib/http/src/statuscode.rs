pub enum Status {
    Ok,

    Custom(u16, String),
}

impl Status {
    pub fn to_string(&self) -> String {
        match self {
            Status::Ok => format!("{} {}", 200, "Ok"),
            Status::Custom(e, m) => format!("{} {}", e, m),
        }
    }
}
