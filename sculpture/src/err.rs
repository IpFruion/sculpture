use std::fmt::Display;

pub trait Error {
    fn custom<T: Display + 'static>(t: T) -> Self;
}

//TODO: Better error
#[derive(Debug)]
pub struct SculptureError(String);

impl Error for SculptureError {
    fn custom<T: Display + 'static>(t: T) -> Self {
        SculptureError(format!("{}", t))
    }
}
