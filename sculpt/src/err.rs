use std::fmt::Display;

//TODO: make better error
#[derive(Debug)]
pub struct Error(String);

impl sculptor::err::Error for Error {
    fn custom<T: std::fmt::Display + 'static>(t: T) -> Self {
        Self(t.to_string())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for Error {}
