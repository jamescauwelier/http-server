use http::{Request, Response, StatusCode};

pub struct MultiplyInput (String);

impl From<Request<String>> for MultiplyInput {
    fn from(value: Request<String>) -> Self {
        MultiplyInput(value.body().clone())
    }
}

pub struct MultiplyOutput(String);

impl <T> From<MultiplyOutput> for Response<T>
where
    T: From<String>
{
    fn from(value: MultiplyOutput) -> Self {
        Response::builder()
            .status(StatusCode::OK)
            .body(value.0.into())
            .unwrap()
    }
}

pub fn process(input: MultiplyInput) -> MultiplyOutput {
    MultiplyOutput(vec![input.0; 3].join(" "))
}