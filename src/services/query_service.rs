use async_trait::async_trait;
use crate::errors::service_error::ServiceError;
use crate::models::central_query::CentralQuery;
use crate::models::query::{QueryResponse, RagQuery};
use crate::models::rag_config::RagServices;
use crate::traits::query_api::QueryApiHandler;
use crate::traits::service_api::ServiceApiHandler;

#[async_trait]
impl ServiceApiHandler for CentralQuery {
    async fn central_query(
        &self,
        rag_services: &RagServices,
    ) -> Result<QueryResponse, ServiceError> {
        if self.rag_name.is_empty() {
            return Err(ServiceError::ValidationFailed(format!(
                "Rag service is empty: {}",
                &self.rag_name
            )));
        }

        let found_rag_service = rag_services.get_service_by_name(&self.rag_name);
        if found_rag_service.is_none() {
            return Err(ServiceError::NotFound(format!(
                "Service not found: {}",
                &self.rag_name
            )));
        }

        let query_request = RagQuery {
            rag_service: found_rag_service.unwrap(),
            query_request: self.query.clone(),
        };

        let query_response = query_request.query().await;

        match query_response {
            Ok(contents) => Ok(contents),
            Err(e) => Err(ServiceError::QueryFailed(format!("Query failed: {:?}", e))),
        }
    }
}
