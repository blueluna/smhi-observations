#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    JsonSerialization(serde_json::Error),
    InvalidValue,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Request(ref error) => write!(f, "Request: {}", error),
            Self::JsonSerialization(ref error) => write!(f, "Json: {}", error),
            Self::InvalidValue => write!(f, "Invalid value"),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        "SmhiObservationError"
    }

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::Request(ref error) => Some(error),
            Self::JsonSerialization(ref error) => Some(error),
            Self::InvalidValue => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Error {
        Self::Request(error)
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Self::JsonSerialization(error)
    }
}
