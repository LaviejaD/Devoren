#![macro_use]
use crate::http::Method;
pub struct Route {
    pub callback: Box<dyn Fn() -> String + 'static>,
    method: Method,
}

impl Route {
    pub fn new<F: Fn() -> String + 'static>(method: Method, callback: F) -> Self {
        Route {
            callback: Box::new(callback),
            method,
        }
    }
    pub fn println(self) {
        println!("{0}", self.method.convert())
    }
}
#[allow(dead_code)]
pub struct Routes {
    vec_routes: Vec<Route>,
}
