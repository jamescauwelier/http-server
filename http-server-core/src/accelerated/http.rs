use http::{Request, Response, StatusCode};
use crate::accelerated::social;

pub struct Http {}

impl Http {
    pub fn process(request: Request<String>) -> Response<String> {
        return Response::builder()
            .status(StatusCode::OK)
            .body(social::hello::process(request.body()))
            .unwrap();
    }
}