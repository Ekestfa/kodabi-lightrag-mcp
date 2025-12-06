use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    ValidationFailed(String),
    NotFound(String),
    QueryFailed(String)
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ValidationFailed(msg) => write!(f, "Validation failed: {}", msg),
            ServiceError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ServiceError::QueryFailed(msg) => write!(f, "Query failed: {}", msg)
        }
    }
}

impl std::error::Error for ServiceError {}