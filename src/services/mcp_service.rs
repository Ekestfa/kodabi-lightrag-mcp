use crate::models::central_query::CentralQuery;
use crate::models::rag_mcp::LlmQueryRequest;
use crate::models::rag_mcp::RagMcp;
use crate::traits::service_api::ServiceApiHandler;
use crate::models::rag_config::RagServices;
use crate::models::rag_config::Rag;

use rmcp::tool;
use rmcp::handler::server::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::model::{
    CallToolResult, Content};
use rmcp::ErrorData as McpError;

/// The main entry point for the RAG MCP server.
impl RagMcp {
    /// Creates a new instance of the RAG MCP server.
    /// 
    /// Returns a Result with the initialized server or an error if setup fails.
    pub fn new() -> Result<Self, McpError> {
        let tool_router = ToolRouter::new();
        Ok(Self { tool_router })
    }

    /// Asks the software engineering RAG service to process a query via MCP.
    /// 
    /// This tool converts an LLM query into a central query format and routes it to
    /// one or more RAG services. The response is returned as a text content block.
    ///
    /// # Arguments
    /// - `query`: A parameterized LLM query request containing the query details.
    ///
    /// # Returns
    /// - A successful response with the RAG service output, or an error if processing fails.
    ///
    /// # Errors
    /// - `invalid_params`: If the input query structure is invalid.
    /// - `internal_error`: If communication with the RAG service fails.
    ///
    #[tool(
        name = "software engineering rag query tool",
        description = "Asks the software engineering RAG service via MCP to process a query"
    )]
    async fn ask_to_software_engineer(
        &self,
        query: Parameters<LlmQueryRequest>,
    ) -> Result<CallToolResult, McpError> {
        // Validate and convert the input query
        let query_data = CentralQuery::from_llm_query_request(&query.0)
            .map_err(|e| McpError::invalid_params(e, None))?;

        // Configure a dummy RAG service (in production, this would be dynamic)
        let dummy_rag = Rag {
            rag_name: "software engineering".to_string(),
            rag_ip: "host.docker.internal".to_string(),
            rag_port: "9621".to_string(),
        };

        println!("Dummy service configured!");
        // Build a list of RAG services
        let rag_services = RagServices {
            services: vec![dummy_rag],
        };

        println!("Execute the central query against the RAG services");
        // Execute the central query against the RAG services
        let query_response = query_data
            .central_query(&rag_services)
            .await
            .map_err(|e| McpError::internal_error(e.to_string(), None))?;

        println!("RAG service response received: {}", query_response.response);
        // Return success result with formatted response
        Ok(CallToolResult::success(vec![Content::text(query_response.response)]))
    }

        /// Public wrapper so external callers (e.g. HTTP handlers) can invoke the
    /// MCP tool logic. This wraps the provided `LlmQueryRequest` into the
    /// `Parameters` type and forwards the call to the private tool method.
    pub async fn call_ask_to_software_engineer(
        &self,
        req: LlmQueryRequest,
    ) -> Result<CallToolResult, McpError> {
        let params = Parameters(req);
        self.ask_to_software_engineer(params).await
    }
}