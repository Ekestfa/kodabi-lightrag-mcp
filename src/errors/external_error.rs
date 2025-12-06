use std::fmt;

#[derive(Debug)]
pub enum ExternalCallError {
    ValidationFailed(String),
    HealthFailed(String),
    ResponseFailed(String)
}

impl fmt::Display for ExternalCallError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExternalCallError::ValidationFailed(msg) => write!(f, "ExternalCallError::ValidationFailed: {}", msg),
            ExternalCallError::HealthFailed(msg) => write!(f, "ExternalCallError::HealthFailed: {}", msg),
            ExternalCallError::ResponseFailed(msg) => write!(f, "ExternalCallError::ResponseFailed: {}", msg)
        }
    }
}

impl std::error::Error for ExternalCallError {}