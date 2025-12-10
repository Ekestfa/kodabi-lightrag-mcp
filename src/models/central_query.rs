use serde::{Deserialize, Serialize};

use crate::models::query::QueryRequest;
use crate::models::rag_mcp::LlmQueryRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CentralQuery {
    pub rag_name: String,
    pub query: QueryRequest,
}

impl CentralQuery {
    pub fn from_llm_query_request(query: &LlmQueryRequest) -> Result<Self, String> {
        let query_request = QueryRequest::new()
            .with_query(query.query.clone())
            .with_mode(query.mode.as_ref().unwrap().clone())
            .with_user_prompt(query.user_prompt.clone())
            .with_conversation_history(query.history.clone());

        Ok(CentralQuery {
            rag_name: query.rag_name.clone(),
            query: query_request,
        })
    }
}
