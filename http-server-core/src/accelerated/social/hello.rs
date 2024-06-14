use http::{Request, Response};

pub struct HelloName(String);

impl From<Request<String>> for HelloName {
    fn from(request: Request<String>) -> Self {
        HelloName(request.body().clone())
    }
}

pub struct Greeting(String);

impl <T> Into<Response<T>> for Greeting
where
    T: From<String>
{
    fn into(self) -> Response<T> {
        Response::builder()
            .status(200)
            .body(self.0.into())
            .unwrap()
    }
}

// a simple mapping between an input type and output type
// for the http server
pub(crate) fn process(name: HelloName) -> Greeting {
    return Greeting(
        format!("Hello, {}!", name.0)
    );
}