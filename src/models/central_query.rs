use serde:: {Serialize, Deserialize};

use crate::models::query::{QueryRequest};


#[derive(Debug, Serialize, Deserialize)]
pub struct CentralQuery {
    pub rag_name: String,
    pub query: QueryRequest
}