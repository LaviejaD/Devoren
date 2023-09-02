#[cfg(test)]
pub mod tests {

    use crate::prelude::*;
    use crate::routes;

    #[test]
    fn macro_route_test() {
        let r1 = Route::new(Method::Get("Hola".to_string()), || {
            println!("1");
            String::new()
        });
        let r2 = Route::new(Method::Get("Hola".to_string()), || {
            println!("2");
            String::new()
        });
        let r3 = Route::new(Method::Get("Hola".to_string()), || {
            println!("3");
            String::new()
        });
        let r4 = Route::new(Method::Get("Hola".to_string()), || {
            println!("4");
            String::new()
        });

        let vr: Vec<Route> = routes!(|| r1, || r2, || r3, || r4);

        for r in &vr {
            (r.callback)();
        }
        //  assert_eq!(vr.len(), 4)
    }
}
