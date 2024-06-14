use http::{Request, Response, StatusCode};

pub struct Http {}

impl Http {
    pub fn process(request: Request<String>) -> Response<String> {
        return Response::builder()
            .status(StatusCode::OK)
            .body("Hello, world!".to_string())
            .unwrap();
    }
}