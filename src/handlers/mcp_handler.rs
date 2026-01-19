use crate::models::rag_mcp::RagMcp;
use crate::models::rag_mcp::RagQueryTool;
use crate::traits::tool::Tool;
use axum::{response::Json, Extension};
use reqwest::StatusCode;
use rmcp::model::Implementation;
use rmcp::model::ProtocolVersion;
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    tool_handler,
};
use serde_json::Value;
use std::sync::Arc;

#[tool_handler]
impl rmcp::ServerHandler for RagMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::LATEST,
            server_info: Implementation::from_build_env(),
            instructions: Some(
                "Provided LightRAG microservices capabilities that includes:\n\n\
                - Querying RAG knowledge bases\n\
                - Retrieving and summarizing information\n\
                - Generating responses based on context\n\
                - Managing document embeddings and retrieval\n\
                - Supporting various query modes (e.g., semantic, hybrid, etc.)\n\
                - Handling user prompts and history for context-aware responses"
                    .to_string(),
            ),
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_tools()
                .build(),
            ..Default::default()
        }
    }
}

#[axum::debug_handler]
pub async fn mcp_info_handler(
    Extension(rag_mcp): Extension<Arc<RagMcp>>,
    Json(payload): Json<RagQueryTool>,
) -> Result<axum::Json<rmcp::model::CallToolResult>, (StatusCode, String)> {
    println!("Received MCP info request: {:?}", payload);
    
    // Validate input parameters before processing
    if payload.query.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Query parameter is required".to_string()));
    }
    
    // Call the MCP tool wrapper with proper error handling
    let result = rag_mcp
        .as_ref()
        .ask_to_software_engineer(rmcp::handler::server::wrapper::Parameters(payload))
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(result))
}

pub async fn list_tools() -> Json<Value> {
    // Initialize RagQueryTool with proper schema and description
    let rag_tool = RagQueryTool {
        rag_name: String::new(),
        query: String::new(),
        mode: None,
        user_prompt: None,
        history: None,
    };
    
    Json(
        serde_json::json!({
            "tools": [
                {
                    "name": "RAG Query Tool",
                    "description": rag_tool.description(),
                    "inputSchema": rag_tool.schema()
                }
            ]
        }),
    )
}
