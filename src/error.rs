pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

#[derive(Debug)]
pub enum BedrockError {
    ModelTimeout,
    ModelNotReady,
    InvalidRequest(String),
    ServiceError(String),
    ParseError(String),
}

impl std::fmt::Display for BedrockError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ModelTimeout => write!(f, "Model took too long to respond"),
            Self::ModelNotReady => write!(f, "Model is not ready"),
            Self::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            Self::ServiceError(msg) => write!(f, "Service error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for BedrockError {}

impl From<BedrockError> for Error {
    fn from(err: BedrockError) -> Self {
        Error::new(&err.to_string())
    }
}