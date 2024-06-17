pub(crate) mod utilities {
    use regex::{Error, Regex};

    #[allow(dead_code)]
    pub(crate) trait RegexStringConversions {
        fn into_regex(self) -> Result<Regex, Error>;
    }

    impl RegexStringConversions for &str {
        fn into_regex(self) -> Result<Regex, Error> {
            Regex::new(self)
        }
    }
}