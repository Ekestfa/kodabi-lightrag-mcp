use crate::errors::handler_error::HandlerError;
use crate::models::query::QueryResponse;
use async_trait::async_trait;

#[async_trait]
pub trait QueryApiHandler {
    async fn query(&self) -> Result<QueryResponse, HandlerError>;
}
