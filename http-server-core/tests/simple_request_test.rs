use http::{Request, StatusCode};
use http_server_core::prelude::*;

#[test]
fn test_hello_request() {
    let request = Request::builder()
        .uri("http://example.com/hello")
        .body("world".to_string())
        .unwrap();
    let response = Http::process(request);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.body(), "Hello, world!");
}

#[test]
fn test_multiply_request() {
    let request = Request::builder()
        .uri("http://example.com/multiply")
        .body("world".to_string())
        .unwrap();
    let response = Http::process(request);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.body(), "world world world");
}