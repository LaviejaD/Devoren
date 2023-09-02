pub mod macros {
    
    #[macro_export]
    macro_rules! routes {
    ($($elemen:expr),*) => {
        {
            let mut  rutas:Vec<crate::prelude::Route> = vec![];
            $(rutas.push(($elemen)());)*
            rutas
        }
    };

}
    #[macro_export]
    macro_rules! create_route {
        () => {};
        ($name:ident,Get($r:expr),$body:expr) => {
            pub fn $name() -> Route {
                Route::new(Method::Get($r.to_string()), $body)
            }
        };
        ($name:ident,Post($r:expr),$body:expr) => {
            pub fn $name() -> Route {
                Route::new(Method::Post($r.to_string()), $body)
            }
        };
        ($name:ident,Put($r:expr),$body:expr) => {
            pub fn $name() -> Route {
                Route::new(Method::Post($r.to_string()), $body)
            }
        };
    }
    
    
}
