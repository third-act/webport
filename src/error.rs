use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    Unspecified(String),
    ParseError(String),
    SerializationError(String),
    NetworkError(String),
    ApiError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            Error::Unspecified(g) => g,
            Error::ParseError(g) => g,
            Error::SerializationError(g) => g,
            Error::NetworkError(g) => g,
            Error::ApiError(g) => g,
        };
        write!(f, "{}", text)
    }
}
