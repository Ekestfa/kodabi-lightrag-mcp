use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

use crate::models::rag_config::Rag;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryResponseRefence {
    pub reference_id: String,
    pub file_path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RagQuery {
    pub rag_service: Rag,
    pub query_request: QueryRequest,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ConversationHistoryRole {
    USER,
    ASSISTANT,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
pub struct ConversationHistory {
    role: ConversationHistoryRole,
    content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum QueryMode {
    LOCAL, // 0
    GLOBAL,
    HYBRID,
    NAIVE,
    MIX,
    BYPASS, // 6
}

impl fmt::Display for QueryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                QueryMode::LOCAL => "local",
                QueryMode::GLOBAL => "global",
                QueryMode::HYBRID => "hybrid",
                QueryMode::NAIVE => "naive",
                QueryMode::MIX => "mix",
                QueryMode::BYPASS => "bypass",
            }
        )
    }
}

impl FromStr for QueryMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "local" => Ok(QueryMode::LOCAL),
            "global" => Ok(QueryMode::GLOBAL),
            "hybrid" => Ok(QueryMode::HYBRID),
            "naive" => Ok(QueryMode::NAIVE),
            "mix" => Ok(QueryMode::MIX),
            "bypass" => Ok(QueryMode::BYPASS),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryRequest {
    pub query: String,
    pub mode: QueryMode,
    pub only_need_context: Option<bool>,
    pub only_need_prompt: Option<bool>,
    pub response_type: String,
    pub top_k: i32,
    pub chunk_top_k: i32,
    pub max_entity_tokens: i32,
    pub max_relation_tokens: i32,
    pub max_total_tokens: i32,
    pub conversation_history: Option<Vec<ConversationHistory>>, // Format: [{'role': 'user/assistant', 'content': 'message'}]
    pub user_prompt: Option<String>,
    pub enable_rerank: Option<bool>,
    pub include_references: bool,
    pub stream: Option<bool>,
}

impl QueryRequest {
    /// Creates a new QueryRequest with all fields initialized to their default values
    ///
    /// This method initializes all fields with sensible default values, ensuring that
    /// the struct is always in a valid state when constructed. It follows Rust best practices
    /// by using a builder pattern and providing clear, readable default values.
    ///
    /// # Returns
    /// A new QueryRequest instance with all fields set to default values
    pub fn new() -> Self {
        Self {
            query: String::new(),
            mode: QueryMode::HYBRID,
            only_need_context: None,
            only_need_prompt: None,
            response_type: "Multiple Paragraphs".to_string(),
            top_k: 40,
            chunk_top_k: 20,
            max_entity_tokens: 6000,
            max_relation_tokens: 10000,
            max_total_tokens: 30000,
            conversation_history: None,
            user_prompt: None,
            enable_rerank: None,
            include_references: true,
            stream: None,
        }
    }

    /// Sets the query field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `query` - The search query string
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_query(mut self, query: String) -> Self {
        self.query = query;
        self
    }

    /// Sets the mode field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `mode` - The query mode (e.g., HYBRID, RAG, etc.)
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_mode(mut self, mode: QueryMode) -> Self {
        self.mode = mode;
        self
    }

    /// Sets the response type field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `response_type` - The desired response format (e.g., "Multiple Paragraphs", "Bullet Points")
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_response_type(mut self, response_type: impl Into<String>) -> Self {
        self.response_type = response_type.into();
        self
    }

    /// Sets the top_k field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `top_k` - The number of top results to return
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_top_k(mut self, top_k: i32) -> Self {
        self.top_k = top_k;
        self
    }

    /// Sets the chunk_top_k field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `chunk_top_k` - The number of top results per chunk
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_chunk_top_k(mut self, chunk_top_k: i32) -> Self {
        self.chunk_top_k = chunk_top_k;
        self
    }

    /// Sets the max_entity_tokens field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `max_entity_tokens` - The maximum number of tokens for entity extraction
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_max_entity_tokens(mut self, max_entity_tokens: i32) -> Self {
        self.max_entity_tokens = max_entity_tokens;
        self
    }

    /// Sets the max_relation_tokens field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `max_relation_tokens` - The maximum number of tokens for relation extraction
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_max_relation_tokens(mut self, max_relation_tokens: i32) -> Self {
        self.max_relation_tokens = max_relation_tokens;
        self
    }

    /// Sets the max_total_tokens field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `max_total_tokens` - The maximum total number of tokens in the response
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_max_total_tokens(mut self, max_total_tokens: i32) -> Self {
        self.max_total_tokens = max_total_tokens;
        self
    }

    /// Sets the conversation_history field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `conversation_history` - The history of previous conversations
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_conversation_history(
        mut self,
        conversation_history: Option<Vec<ConversationHistory>>,
    ) -> Self {
        self.conversation_history = conversation_history;
        self
    }

    /// Sets the user_prompt field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `user_prompt` - The user's prompt for the query
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_user_prompt(mut self, user_prompt: Option<String>) -> Self {
        self.user_prompt = user_prompt;
        self
    }

    /// Sets the enable_rerank field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `enable_rerank` - Whether to enable reranking of results
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_enable_rerank(mut self, enable_rerank: Option<bool>) -> Self {
        self.enable_rerank = enable_rerank;
        self
    }

    /// Sets the stream field and returns a mutable reference to self
    ///
    /// This method allows for fluent API construction by enabling method chaining.
    /// It is designed to be used in conjunction with other builder methods.
    ///
    /// # Parameters
    /// * `stream` - Whether to stream the response
    ///
    /// # Returns
    /// A mutable reference to the QueryRequest instance
    pub fn with_stream(mut self, stream: Option<bool>) -> Self {
        self.stream = stream;
        self
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: String,
    pub references: Vec<QueryResponseRefence>,
}