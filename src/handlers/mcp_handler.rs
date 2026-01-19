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
    let stream = tokio_stream::iter(vec![
        format!("data: {{\"message_id\": \"123\", \"content\": \"Processing request...\", \"is_final\": false}}\
"),
        format!("data: {{\"message_id\": \"123\", \"content\": \"Querying database...\", \"is_final\": false}}\
"),
        format!("data: {{\"message_id\": \"123\", \"content\": \"Final result: Data retrieved\", \"is_final\": true}}\
"),
    ]);
    let body = Body::from_stream(stream);
    Response::builder()
        .header("Content-Type", "text/event-stream")
        .header("Cache-Control", "no-cache")
        .header("Connection", "keep-alive")
        .body(body)
        .unwrap()
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
async fn announcement_handler() -> Response {\n    let stream = tokio_stream::iter(vec![\n        format!("data: {{\\\"event\\\": \\\"cpu_alert\\\", \\\"message\\\": \\\"CPU usage 95%\\\"}}\\\n"),\n        format!("data: {{\\\"event\\\": \\\"new_message\\\", \\\"message\\\": \\\"User sent: Hello!\\\"}}\\\n"),\n    ]);\n    let body = Body::from_stream(stream);\n    Response::builder()\n        .header("Content-Type", "text/event-stream")\n        .header("Cache-Control", "no-cache")\n        .header("Connection", "keep-alive")\n        .body(body)\n        .unwrap()\n}
