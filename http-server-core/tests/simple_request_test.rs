use http::{Request, StatusCode};
use http_server_core::prelude::*;

#[test]
fn simple_request() {
    let request = Request::builder()
        .uri("http://example.com/hello")
        .body("world".to_string())
        .unwrap();
    let response = Http::process(request);

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.body(), "Hello, world!");
}

