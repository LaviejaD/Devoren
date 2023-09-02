use std::net::TcpListener;

pub fn find_port() -> i32 {
    let mut port: i32 = 0;
    for p in 8080..9000 {
        match TcpListener::bind(("127.0.0.1", p)) {
            Ok(_) => {
                port = p as i32;
                break;
            }
            Err(_) => continue,
        }
    }
    port
}
