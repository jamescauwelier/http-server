pub mod uri {
    pub mod path {
        use http::Request;
        use regex::Regex;

        pub fn is<T>(path: &'static str) -> Box<dyn Fn(&Request<T>) -> bool + 'static >{
            Box::new(
                move |req: &Request<T>| -> bool {
                    req.uri().path() == path
                }
            )
        }

        pub fn matches<T>(regex: Regex) -> Box<dyn Fn(&Request<T>) -> bool + 'static> {
            Box::new(
                move |req: &Request<T>| {
                    regex.is_match(req.uri().path())
                }
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use http::Request;
    use super::uri;
    use crate::accelerated::regex::utilities::RegexStringConversions;

    fn request_hello() -> Request<String> {
        Request::builder()
            .uri("http://example.com/hello")
            .body("world".to_string())
            .unwrap()
    }

    fn request_multiply() -> Request<String> {
        Request::builder()
            .uri("http://example.com/multiply")
            .body("world".to_string())
            .unwrap()
    }

    #[test]
    fn test_name_matching() {

        let hello = request_hello();
        let multiply = request_multiply();

        // let exact_hello_route_matcher = uri::path::is("/hello");
        // let regex_hello_route_matcher = uri::path::matches(r"/hello".into_regex().unwrap());
        // let exact_multiply_route_matcher = uri::path::is("/multiply");
        // let regex_multiply_route_matcher = uri::path::matches(r"/multiply".into_regex().unwrap());

        assert!(uri::path::is("/hello")(&hello));
        assert!(!uri::path::matches(r"/hello".into_regex().unwrap())(&multiply));
        assert!(!uri::path::is("/multiply")(&hello));
        assert!(uri::path::is("/multiply")(&multiply));
        assert!(uri::path::matches(r"/multiply".into_regex().unwrap())(&multiply));
    }
}