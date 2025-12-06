use std::fs;
use std::env;
use serde_json;

use crate::errors::handler_error::HandlerError;
use crate::models::rag_config::RagServices;
use crate::traits::config_api::ServiceConfigurationHandler;

impl ServiceConfigurationHandler for RagServices {
    fn init(path: &str) -> Result<RagServices, HandlerError> {
        let rag_config = env::var("KODABI_RAG_SERVICES_CONFIG").unwrap_or_else(|_| "rag_config.json".to_string());

        let user_defined_rag_configs= fs::read_to_string(&rag_config)
            .map_err(|e| HandlerError::ReadFileFailed(format!("path: {}, error: {}", rag_config, e)))
            .map(|content| content); // Success case: return the content as-is
        println!("Rag config loaded from path: {}", &rag_config);

        let rag_services: Result<RagServices, HandlerError> = user_defined_rag_configs
            .and_then(|content| {
                serde_json::from_str(&content)
                    .map_err(|e| HandlerError::FileJsonParseFailed(format!("path: {}, error: {}", path, e)))
        });
        println!("Rag services parsed successfully from JSON. {}", rag_services.as_ref().map_or(0, |rs| rs.services.len()));
        rag_services
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    // Test case: Valid JSON file with correct structure
    #[test]
    fn test_init_valid_json() {
        // Arrange: Create a valid JSON file with mock data
        let valid_json = r#"
        {
            "services": [
                {
                    "rag_name": "RagService1",
                    "rag_ip": "192.168.1.10",
                    "rag_port": "8080"
                },
                {
                    "rag_name": "RagService2",
                    "rag_ip": "192.168.1.11",
                    "rag_port": "8081"
                }
            ]
        }
        "#;

        // Create a temporary file for testing
        let temp_file: &str = "test_rag_config.json";
        fs::write(temp_file, valid_json).expect("Failed to write test file");

        // Act: Call the init function
        let result = RagServices::init(temp_file);

        // Assert: Verify successful parsing and return value
        assert!(result.is_ok());
        let services: RagServices = result.unwrap();
        assert_eq!(services.services.len(), 2);

        // Validate first service details
        assert_eq!(services.services[0].rag_name, "RagService1");
        assert_eq!(services.services[0].rag_ip, "192.168.1.10");
        assert_eq!(services.services[0].rag_port, "8080");

        // Clean up
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }

    // Test case: Missing or empty file (can't reach)
    // #[test]
    // fn test_init_missing_file() {
    //     // Arrange: Create a non-existent file path
    //     let missing_path = "nonexistent_rag_config.json";

    //     // Act: Call init with missing file
    //     let result = RagServices::init();

    //     // Assert: Should return an error due to file not found
    //     assert!(result.is_err());
    //     if let Err(e) = result {
    //         assert!(e.to_string().contains("Read file failed"));
    //     }
    // }

    // Test case: Invalid JSON format
    #[test]
    fn test_init_invalid_json() {
        // Arrange: Create a file with invalid JSON
        let invalid_json = r#"
        {
             {
                "rag_name": "RagService1",
                "rag_ip": "192.168.1.10",
                "rag_port": "8080"
            }
        }"#;

        // Create a temporary file with invalid JSON
        let temp_file = "test_invalid_rag_config.json";
        fs::write(temp_file, invalid_json).expect("Failed to write test file");

        // Act: Call init function
        let result = RagServices::init(temp_file);

        // Assert: Should return error due to JSON parse failure
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Parse JSON file failed"));
        }

        // Clean up
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }

    // Test case: Empty JSON file
    #[test]
    fn test_init_empty_json() {
        // Arrange: Create an empty JSON file
        let empty_json = "{}";
        let temp_file = "test_empty_rag_config.json";
        fs::write(temp_file, empty_json).expect("Failed to write test file");

        // Act: Call init function
        let result = RagServices::init(temp_file);

        // Assert: Should return error due to empty or malformed JSON
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Parse JSON file failed"));
        }

        // Clean up
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }

    // Test case: File with incorrect structure (missing services array)
    #[test]
    fn test_init_missing_services_array() {
        // Arrange: Create a file with missing services array
        let invalid_structure = r#"
        {
            "rag_name": "RagService1",
            "rag_ip": "192.168.1.10",
            "rag_port": "8080"
        }
        "#;

        let temp_file = "test_invalid_structure.json";
        fs::write(temp_file, invalid_structure).expect("Failed to write test file");

        // Act: Call init function
        let result = RagServices::init(temp_file);

        // Assert: Should return error due to invalid JSON structure
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Parse JSON file failed"));
        }

        // Clean up
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }

    // Test case: File with malformed field names (e.g., missing quotes)
    #[test]
    fn test_init_malformed_field_names() {
        // Arrange: Create a file with malformed field names
        let malformed_json = r#"
        {
            "services": [
                {
                    rag_name: "RagService1",
                    rag_ip: "192.168.1.10",
                    rag_port: "8080"
                }
            ]
        }
        "#;

        let temp_file = "test_malformed_fields.json";
        fs::write(temp_file, malformed_json).expect("Failed to write test file");

        // Act: Call init function
        let result = RagServices::init(temp_file);

        // Assert: Should return error due to JSON parse failure
        assert!(result.is_err());
        if let Err(e) = result {
            assert!(e.to_string().contains("Parse JSON file failed"));
        }

        // Clean up
        fs::remove_file(temp_file).expect("Failed to remove test file");
    }
}