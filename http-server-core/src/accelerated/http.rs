use http::{Request, Response, StatusCode};
use crate::accelerated::social;

pub struct Http {}

impl Http {
    pub fn process(request: Request<String>) -> Response<String> {
        match request.uri().path() {
            "/hello" => {
                return social::hello::process(request.into()).into()
            }
            "/multiply" => {
                return social::multiply::process(request.into()).into()
            }
            _ => {
                return Response::builder()
                    .status(StatusCode::NOT_FOUND)
                    .body("Not Found".to_string())
                    .unwrap();
            }
        }
    }
}