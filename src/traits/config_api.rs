use crate::errors::handler_error::HandlerError;
use crate::models::rag_config::RagServices;

pub trait ServiceConfigurationHandler {
    fn init(path: &str) -> Result<RagServices, HandlerError>;
}