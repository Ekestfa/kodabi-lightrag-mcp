//! Tool definitions for MCP.
//!
//! Tools are functions that agents can call to perform actions. Each tool must implement
//! the `Tool` trait, providing a description, JSON Schema for parameters, and an async
//! execution function.
//!
//! # Example
//!
//! ```rust,no_run
//! use axum_mcp::Tool;
//! use async_trait::async_trait;
//! use serde_json::Value;
//!
//! struct EchoTool;
//!
//! #[async_trait]
//! impl Tool for EchoTool {
//!     fn description(&self) -> &str {
//!         "Echo back the input text"
//!     }
//!
//!     fn schema(&self) -> Value {
//!         serde_json::json!({
//!             "type": "object",
//!             "properties": {
//!                 "text": {
//!                     "type": "string",
//!                     "description": "Text to echo back"
//!                 }
//!             },
//!             "required": ["text"]
//!         })
//!     }
//!
//!     async fn call(&self, arguments: &Value) -> Result<Value, String> {
//!         let text = arguments
//!             .get("text")
//!             .and_then(|v| v.as_str())
//!             .ok_or_else(|| "Missing 'text' parameter".to_string())?;
//!         Ok(serde_json::json!({ "echoed": text }))
//!     }
//! }
//! ```

use async_trait::async_trait;
use serde_json::Value;

/// A tool that can be called by MCP clients.
///
/// Tools are the primary mechanism for agents to interact with external systems.
/// Each tool must provide:
///
/// - A human-readable description (used by agents to understand when to call the tool)
/// - A JSON Schema defining the tool's parameters
/// - An async execution function that processes arguments and returns results
///
/// # JSON Schema Format
///
/// The schema should follow [JSON Schema Draft 7](https://json-schema.org/).
/// Parameters without defaults are automatically marked as required.
///
/// # Error Handling
///
/// Return `Err(String)` to indicate tool execution failure. The error message
/// will be returned to the agent.
#[async_trait]
pub trait Tool: Send + Sync {
    /// Get the tool's description.
    ///
    /// This description is shown to agents to help them understand when and how
    /// to use this tool. Be clear and specific about what the tool does.
    fn description(&self) -> &str;

    /// Get the JSON Schema for the tool's input parameters.
    ///
    /// This schema defines the structure of the `arguments` parameter passed to `call`.
    /// It should follow JSON Schema Draft 7 format.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use serde_json::{json, Value};
    ///
    /// let schema: Value = json!({
    ///     "type": "object",
    ///     "properties": {
    ///         "query": {
    ///             "type": "string",
    ///             "description": "Search query"
    ///         },
    ///         "limit": {
    ///             "type": "integer",
    ///             "description": "Maximum results",
    ///             "default": 10
    ///         }
    ///     },
    ///     "required": ["query"]
    /// });
    /// ```
    fn schema(&self) -> Value;

    /// Call the tool with the given arguments.
    ///
    /// # Arguments
    ///
    /// * `arguments` - A JSON object containing the tool's parameters, validated
    ///   against the schema returned by `schema()`.
    ///
    /// # Returns
    ///
    /// * `Ok(Value)` - The tool's result as JSON
    /// * `Err(String)` - An error message describing what went wrong
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use serde_json::{json, Value};
    ///
    /// # async fn example(arguments: &Value) -> Result<Value, String> {
    /// let query = arguments
    ///     .get("query")
    ///     .and_then(|v| v.as_str())
    ///     .ok_or_else(|| "Missing required parameter 'query'".to_string())?;
    ///
    /// // Perform the tool's action...
    /// Ok(json!({ "result": "success" }))
    /// # }
    /// ```
    async fn call(&self, arguments: &Value) -> Result<Value, String>;
}