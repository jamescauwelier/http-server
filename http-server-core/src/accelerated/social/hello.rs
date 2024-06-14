// a simple mapping between an input type and output type
// for the http server
pub(crate) fn process(name: &str) -> String {
    return format!("Hello, {}!", name);
}