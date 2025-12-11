use crate::models::rag_mcp::LlmQueryRequest;
use crate::models::rag_mcp::RagMcp;
use axum::{
    Extension, Json as AxumJson
};
use reqwest::StatusCode;
use rmcp::{
    model::{ServerCapabilities, ServerInfo},
    tool_handler,
};
use std::sync::Arc;

#[tool_handler]
impl rmcp::ServerHandler for RagMcp {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            instructions: Some(
                "Provided LightRAG microservices capabilities that includes ".into(),
            ),
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[axum::debug_handler]
pub async fn mcp_info_handler(
    Extension(rag_mcp): Extension<Arc<RagMcp>>,
    AxumJson(payload): AxumJson<LlmQueryRequest>,
) -> Result<axum::Json<rmcp::model::CallToolResult>, (StatusCode, String)> {
        println!("Received MCP info request: {:?}", payload);
    // Call the MCP tool wrapper and convert errors to HTTP 500.
    let result = rag_mcp
        .as_ref()
        .call_ask_to_software_engineer(payload)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(axum::Json(result))
}

// #[tool_handler]
// #[prompt_handler(router = self.prompt_router)]
// impl ServerHandler for MicropubMcp {
//     fn get_info(&self) -> ServerInfo {
//         ServerInfo {
//             protocol_version: ProtocolVersion::V_2024_11_05,
//             capabilities: ServerCapabilities::builder()
//                 .enable_tools()
//                 .enable_prompts()
//                 .build(),
//             server_info: Implementation::from_build_env(),
//             instructions: Some(
//                 "Micropub MCP server for posting and managing micropub content via AI assistants.\n\n\
//                  IMAGE UPLOADS:\n\
//                  - Use 'upload_media' tool to upload images explicitly (supports file paths or base64 data)\n\
//                  - Or use 'publish_post' with local image paths (e.g., ![alt](~/photo.jpg)) - they'll auto-upload\n\n\
//                  SERVER-SIDE DRAFTS:\n\
//                  - Use 'push_draft' tool to save drafts to server with post-status: draft\n\
//                  - Drafts remain editable locally and can be re-pushed to update\n\
//                  - Use 'publish_post' to change server draft to published status\n\
//                  - Supports media upload and backdating when pushing drafts\n\n\
//                  All uploads and draft operations require authentication via 'micropub auth <domain>' first."
//                     .to_string(),
//             ),
//         }
//     }
// }