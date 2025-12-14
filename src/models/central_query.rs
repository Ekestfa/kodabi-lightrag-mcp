use serde::{Deserialize, Serialize, Deserializer};
use serde::de;
use serde_json::Value;

use crate::models::query::QueryRequest;
use crate::models::rag_mcp::LlmQueryRequest;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CentralQuery {
    pub rag_name: String,

    // Accept either an object (QueryRequest) or a bare string for convenience.
    #[serde(deserialize_with = "deserialize_query_request")]
    pub query: QueryRequest,
}

impl CentralQuery {
    pub fn from_llm_query_request(query: &LlmQueryRequest) -> Result<Self, String> {
        println!(
            "Transforming LlmQueryRequest to CentralQuery for RAG: {}",
            query.rag_name
        );

        // Start from defaults and apply optional values
        let mut query_request = QueryRequest::new().with_query(query.query.clone());

        // Only set mode if provided; otherwise keep default from QueryRequest::new()
        if let Some(mode) = query.mode.as_ref() {
            query_request = query_request.with_mode(mode.clone());
        }

        // Optional fields
        query_request = query_request
            .with_user_prompt(query.user_prompt.clone())
            .with_conversation_history(query.history.clone());

        Ok(CentralQuery {
            rag_name: query.rag_name.clone(),
            query: query_request,
        })
    }
}

/// Custom deserializer for `CentralQuery::query`
/// Accepts either:
/// - an object deserializable to `QueryRequest`
/// - OR a string which will be used as `QueryRequest.query` with defaults for everything else.
fn deserialize_query_request<'de, D>(deserializer: D) -> Result<QueryRequest, D::Error>
where
    D: Deserializer<'de>,
{
    // First deserialize into a serde_json::Value (single use of `deserializer`)
    let v = Value::deserialize(deserializer).map_err(de::Error::custom)?;

    match v {
        Value::String(s) => Ok(QueryRequest::new().with_query(s)),
        Value::Object(_) => serde_json::from_value(v).map_err(de::Error::custom),
        other => Err(de::Error::custom(format!(
            "expected string or object for query field, got: {}",
            other
        ))),
    }
}