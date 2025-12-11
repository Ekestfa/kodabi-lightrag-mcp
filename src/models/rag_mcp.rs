use crate::models::query::ConversationHistory;
use crate::models::query::QueryMode;
use rmcp::handler::server::router::tool::ToolRouter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

pub struct RagMcp {
    pub tool_router: ToolRouter<Self>,
    // pub prompt_router: PromptRouter<Self>,
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct LlmQueryRequest {
    pub rag_name: String,
    pub query: String,
    pub mode: Option<QueryMode>,
    pub user_prompt: Option<String>,
    pub history: Option<Vec<ConversationHistory>>,
}