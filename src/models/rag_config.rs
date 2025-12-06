use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rag {
    pub rag_name: String,
    pub rag_ip: String,
    pub rag_port: String,
}

impl Rag {
    pub fn get_full_path(&self) -> String {
        return format!("{}:{}", self.rag_ip, self.rag_port);
    }

    pub fn get_service_detail(&self) -> String {
        return format!("{}: [{}:{}]", self.rag_name, self.rag_ip, self.rag_port);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagServices {
    pub services: Vec<Rag>,
}

impl RagServices {
    pub fn get_service_by_name(&self, service_name: &str) -> Option<Rag> {
        self.services
            .iter()
            .find(|service| service.rag_name.as_str() == service_name)
            .cloned()
    }
}
