use std::fmt::{self};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum HandlerError {
    ValidationFailed(String),
    ReadFileFailed(String),
    FileJsonParseFailed(String),
    ProcessFailed(String)
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HandlerError::ValidationFailed(msg) => write!(f, "[HE]:Validation failed: {}", msg),
            HandlerError::ReadFileFailed(msg) => write!(f, "[HE]:Read file failed: {}", msg),
            HandlerError::FileJsonParseFailed(msg) => write!(f, "[HE]:Parse JSON file failed: {}", msg),
            HandlerError::ProcessFailed(msg) => write!(f, "[HE]:Process failed: {}", msg)
        }
    }
}

impl std::error::Error for HandlerError {}