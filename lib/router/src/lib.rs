use http::Method;
pub trait Route {
    fn call(&self) -> String;
    fn enpoint(&self) -> (Method, String);
}

pub struct Routes {
    vec_routes: Vec<Box<dyn Route>>,
}

impl Routes {
    pub fn new() -> Self {
        Self {
            vec_routes: Vec::new(),
        }
    }
    pub fn insert(&mut self, r: impl Route + 'static) {
        self.vec_routes.push(Box::new(r));
    }
}
