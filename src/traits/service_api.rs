use async_trait::async_trait;
use crate::models::query::QueryResponse;
use crate::errors::service_error::ServiceError;
use crate::models::rag_config::RagServices;

#[async_trait]
pub trait ServiceApiHandler {
    async fn central_query(&self, rag_services: &RagServices) -> Result<QueryResponse, ServiceError>;
}