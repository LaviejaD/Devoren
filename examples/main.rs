use devonrex::prelude::*;
#[route(get,/index.html)]
fn Ruta(hola: String) -> String {
    "Hello".to_string()
}
#[route(get,/hola)]
fn Hola(Queloquees: i64) -> String {
    "hola".to_string()
}
fn main() {
    Rex::new().add_routes(Ruta).run();
}
