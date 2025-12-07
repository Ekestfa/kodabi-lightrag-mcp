use crate::errors::handler_error::HandlerError;
use crate::models::central_query::CentralQuery;
use crate::models::query::{QueryResponse, RagQuery};
use crate::models::rag_config::RagServices;
use crate::traits::config_api::ServiceConfigurationHandler;
use crate::traits::query_api::QueryApiHandler;
use crate::traits::service_api::ServiceApiHandler;
use async_trait::async_trait;
use axum::{http::StatusCode, Json};
use reqwest::Client;

#[async_trait]
impl QueryApiHandler for RagQuery {
    async fn query(&self) -> Result<QueryResponse, HandlerError> {
        if self.rag_service.rag_ip.is_empty() || self.rag_service.rag_port.is_empty() {
            return Err(HandlerError::ValidationFailed(format!(
                "Check service information {}={}:{}",
                self.rag_service.rag_name, self.rag_service.rag_ip, self.rag_service.rag_port
            )));
        }

        let url = format!(
            "http://{}:{}/query",
            self.rag_service.rag_ip, self.rag_service.rag_port
        );
       
        println!("Sending request to URL: {}", url);
        println!(
            "Request body: {}",
            serde_json::to_string(&self.query_request).unwrap()
        );

        let client = Client::new();
         // Build request with proper headers and body
        let response = client
            .post(&url)
            .json(&self.query_request)
            .send()
            .await
            .map_err(|err| {
                HandlerError::ProcessFailed(format!("Query request failed: {:?}", err))
            })?;

        // Extract response body as string
        let response_body = response.text().await.map_err(|err| {
            HandlerError::ProcessFailed(format!("Failed to read response body: {:?}", err))
        })?;

        println!("Response body: {}", response_body);
        // Parse the JSON response into QueryResponse struct
        let query_response: QueryResponse =
            serde_json::from_str(&response_body).map_err(|err| {
                HandlerError::ProcessFailed(format!(
                    "Failed to deserialize response into QueryResponse: {:?}",
                    err
                ))
            })?;

        Ok(query_response)
    }
}

#[axum::debug_handler]
pub async fn central_query_handler(
    Json(payload): axum::extract::Json<CentralQuery>,
) -> Result<axum::response::Json<QueryResponse>, (axum::http::StatusCode, String)> {
    println!("central_query_handler called");
    println!("deserialized payload: {:#?}", payload);
    // Load the rag services (uses the ServiceConfigurationHandler impl you already have)
    let rag_services = RagServices::init(payload.rag_name.as_str())
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // Call the service-layer method on CentralQuery
    let response = payload
        .central_query(&rag_services)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(response))
}

//TODO improve
#[cfg(test)]
mod tests {
    use crate::models::query::{QueryMode, QueryRequest, RagQuery};
    use crate::models::rag_config::Rag;
    use crate::traits::query_api::QueryApiHandler;

    #[tokio::test]
    async fn test_invalid_service_configuration() -> Result<(), Box<dyn std::error::Error>> {
        // Arrange
        let rag_service = Rag {
            rag_name: "software engineering".to_string(),
            rag_ip: "".to_string(),
            rag_port: "8080".to_string(),
        };

        let query_request = QueryRequest {
            query: "What is the capital of France?".to_string(),
            mode: QueryMode::LOCAL,
            only_need_context: None,
            only_need_prompt: None,
            response_type: "text".to_string(),
            top_k: 3,
            chunk_top_k: 2,
            max_entity_tokens: 50,
            max_relation_tokens: 10,
            max_total_tokens: 200,
            conversation_history: None,
            user_prompt: None,
            enable_rerank: None,
            include_references: true,
            stream: None,
        };

        let rag_query = RagQuery {
            rag_service,
            query_request: query_request,
        };

        // Act
        let result = rag_query.query().await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Check service information"));

        Ok(())
    }
}
