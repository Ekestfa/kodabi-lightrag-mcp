use serde:: {Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

use crate::models::rag_config::Rag;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QueryResponseRefence {
	pub reference_id: String,
	pub file_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RagQuery {
    pub rag_service: Rag,
    pub query_request: QueryRequest
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ConversationHistoryRole {
    USER,
    ASSISTANT
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConversationHistory {
    role: ConversationHistoryRole,
    content: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum QueryMode {
    LOCAL, // 0
    GLOBAL,
    HYBRID,
    NAIVE,
    MIX,
    BYPASS // 6
}

impl fmt::Display for QueryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            QueryMode::LOCAL => "local",
            QueryMode::GLOBAL => "global",
            QueryMode::HYBRID => "hybrid",
            QueryMode::NAIVE => "naive",
            QueryMode::MIX => "mix",
            QueryMode::BYPASS => "bypass",
        })
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
    pub stream: Option<bool>
}

impl QueryRequest {
    /// Initializes optional fields in QueryRequest with default values
    /// 
    /// This method sets default values for optional fields in the QueryRequest struct
    /// when they are not provided. This ensures that all fields have valid values
    /// and prevents potential runtime errors from missing or null values.
    ///
    /// # Returns
    /// A new QueryRequest with default values for optional fields
    pub fn with_default_options(mut self) -> Self {
        // Initialize only_need_context with default value (false)
        if self.only_need_context.is_none() {
            self.only_need_context = Some(false);
        }
        
        // Initialize only_need_prompt with default value (false)
        if self.only_need_prompt.is_none() {
            self.only_need_prompt = Some(false);
        }
        
        // Initialize conversation_history with empty vector if not provided
        if self.conversation_history.is_none() {
            self.conversation_history = Some(Vec::new());
        }
        
        // Initialize user_prompt with empty string if not provided
        if self.user_prompt.is_none() {
            self.user_prompt = Some(String::new());
        }
        
        // Initialize enable_rerank with default value (false)
        if self.enable_rerank.is_none() {
            self.enable_rerank = Some(false);
        }
        
        // Initialize stream with default value (false)
        if self.stream.is_none() {
            self.stream = Some(false);
        }
        
        self
    }
    
    /// Creates a new QueryRequest with all optional fields initialized to their defaults
    ///
    /// # Arguments
    /// * `query` - The search query string
    /// * `mode` - The query execution mode (LOCAL, GLOBAL, etc.)
    /// * `response_type` - The expected response format
    /// * `top_k` - Maximum number of results to return
    /// * `chunk_top_k` - Maximum number of chunks per result
    /// * `max_entity_tokens` - Maximum tokens for entity extraction
    /// * `max_relation_tokens` - Maximum tokens for relation extraction
    /// * `max_total_tokens` - Maximum total tokens for the response
    ///
    /// # Returns
    /// A fully initialized QueryRequest with default values for optional fields
    pub fn new(
        query: String,
        mode: QueryMode,
        response_type: String,
        top_k: i32,
        chunk_top_k: i32,
        max_entity_tokens: i32,
        max_relation_tokens: i32,
        max_total_tokens: i32
    ) -> Self {
        Self {
            query,
            mode,
            only_need_context: Some(false),
            only_need_prompt: Some(false),
            response_type,
            top_k,
            chunk_top_k,
            max_entity_tokens,
            max_relation_tokens,
            max_total_tokens,
            conversation_history: Some(Vec::new()),
            user_prompt: Some(String::new()),
            enable_rerank: Some(false),
            include_references: true,
            stream: Some(false),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QueryResponse {
    pub response: String,
    pub references: Vec<QueryResponseRefence>
}

