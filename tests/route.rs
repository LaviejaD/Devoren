use client::Client;
use devonrex::{client, http, router, router_macro};
use http::{Method, Request, Response};
use router::Route;
use router_macro::route;
use std::thread;

#[route(get,/)]
fn _Test(request: http::Request) -> Response {
    http::Response::default()
}
#[route(post,/upload/<id>)]
fn _Test1(request: http::Request) -> Response {
    http::Response::default()
}

#[test]
fn test_endpoint_route() {
    let point = "GET /".to_string();
    let (method, point1) = _Test.endpoint();

    assert_eq!(format!("{} {}", method.to_string(), point1), point);
}
#[test]
fn test_len_route() {
    let mut _router = router::Routes::new();
    _router.insert(_Test);
    _router.insert(_Test1);
    assert!(_router.routes.len() == _router.routes_dinamy.len())
}
