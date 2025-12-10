use std::fmt;

#[derive(Debug)]
pub enum ServiceError {
    ValidationFailed(String),
    NotFound(String),
    QueryFailed(String),
    McpError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::ValidationFailed(msg) => write!(f, "[SE]:Validation failed: {:?}", msg),
            ServiceError::NotFound(msg) => write!(f, "[SE]:Not found: {:?}", msg),
            ServiceError::QueryFailed(msg) => write!(f, "[SE]:Query failed: {:?}", msg),
            ServiceError::McpError(msg) => write!(f, "[SE]:MCP error: {:?}", msg),
        }
    }
}

impl std::error::Error for ServiceError {}
