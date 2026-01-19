use crate::models::query::ConversationHistory;
use crate::models::query::QueryMode;
use crate::traits::tool::Tool;
use async_trait::async_trait;
use rmcp::handler::server::router::tool::ToolRouter;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct RagMcp {
    pub tool_router: ToolRouter<Self>,
    // pub prompt_router: PromptRouter<Self>,
}

#[derive(Debug, Clone, JsonSchema, Serialize, Deserialize)]
pub struct RagQueryTool {
    pub rag_name: String,
    pub query: String,
    pub mode: Option<QueryMode>,
    pub user_prompt: Option<String>,
    pub history: Option<Vec<ConversationHistory>>,
}

#[async_trait]
impl Tool for RagQueryTool {
    fn description(&self) -> &str {
        "Retrieve data from RAG services. Choose service by rag_name."
    }

    fn schema(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "rag_name": {
                    "type": "string",
                    "description": "Indicates related RAG microservice. (e.g. Software Engineering tool: `software engineering`)"
                },
                "query": {
                    "type": "string",
                    "description": "Query text to RAG service, according to query, the selected microservice will be returning result.)"
                },
                "mode": {
                    "type": "int",
                    "description": "Indicates nullable query mode of RAG, {0-6}: LOCAL, GLOBAL, HYBRID (default), NAIVE, MIX, BYPASS"
                },
                "user_prompt": {
                    "type": "string",
                    "description": "Indicates nullable prompt that specify the query result."
                },
                "history": {
                    "type": "object array",
                    "description": "Indicates user/assistant chat history array, which could be provided more context to get detailed RAG result."
                },
            }
        })
    }

    async fn call(&self, arguments: &Value) -> Result<Value, String> {
        // Here you would implement the logic to call the RAG service.
        // For demonstration, we will just echo back the arguments.
        Ok(arguments.clone())
    }
}
