use serde_json;
use std::env;
use std::fs;

use crate::errors::handler_error::HandlerError;
use crate::models::rag_config::RagServices;
use crate::traits::config_api::ServiceConfigurationHandler;

impl ServiceConfigurationHandler for RagServices {
    fn init(path: &str) -> Result<RagServices, HandlerError> {
        let rag_config = env::var("KODABI_RAG_SERVICES_CONFIG")
            .unwrap_or_else(|_| "rag_config.json".to_string());

        let user_defined_rag_configs = fs::read_to_string(&rag_config)
            .map_err(|e| {
                HandlerError::ReadFileFailed(format!("path: {}, error: {}", rag_config, e))
            })
            .map(|content| content); // Success case: return the content as-is
        println!("Rag config loaded from path: {}", &rag_config);

        let rag_services: Result<RagServices, HandlerError> =
            user_defined_rag_configs.and_then(|content| {
                serde_json::from_str(&content).map_err(|e| {
                    HandlerError::FileJsonParseFailed(format!("path: {}, error: {}", path, e))
                })
            });
        println!(
            "Rag services parsed successfully from JSON. {}",
            rag_services.as_ref().map_or(0, |rs| rs.services.len())
        );
        rag_services
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use temp_env::with_var;

    // Test case: Valid JSON file with correct structure
    #[test]
    fn test_init_valid_json() {
        // Arrange: Mock the environment variable to point to a valid JSON string
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

        // Set the environment variable to point to a mock configuration
        let mock_config_path = "test_rag_config.json";

        // Mock the file content by simulating a file read operation
        let mock_file_content = valid_json.to_string();

        // Create a temporary file for testing
        let result = fs::write(mock_config_path, mock_file_content)
            .map_err(|e| panic!("Failed to write valid JSON file: {}", e));

        // Ensure the file was written successfully
        assert!(result.is_ok());

        with_var(
            "KODABI_RAG_SERVICES_CONFIG",
            Some(&mock_config_path),
            || {
                // Act: Call the init function
                let result = RagServices::init(mock_config_path);

                // Assert: Verify successful parsing and return value
                assert!(result.is_ok());
                let services: RagServices = result.unwrap();
                assert_eq!(services.services.len(), 2);

                // Validate first service details
                assert_eq!(services.services[0].rag_name, "RagService1");
                assert_eq!(services.services[0].rag_ip, "192.168.1.10");
                assert_eq!(services.services[0].rag_port, "8080");
            },
        );

        // Clean up: Remove the mock file if it exists
        if let Err(e) = fs::remove_file(mock_config_path) {
            eprintln!("Warning: Failed to remove test file: {}", e);
        }
    }

    // Test case: Invalid JSON format
    #[test]
    fn test_init_invalid_json() {
        // Arrange: Set up a mock environment variable with invalid JSON
        let invalid_json = r#"
        {
             {
                "rag_name": "RagService1",
                "rag_ip": "192.168.1.10",
                "rag_port": "8080"
            }
        }"#;

        let mock_config_path = "test_invalid_rag_config.json";

        // Mock the file content by simulating a file read operation
        let mock_file_content = invalid_json.to_string();

        // Create a temporary file for testing
        let result = fs::write(mock_config_path, mock_file_content)
            .map_err(|e| panic!("Failed to write invalid JSON file: {}", e));

        // Ensure the file was written successfully
        assert!(result.is_ok());

        with_var(
            "KODABI_RAG_SERVICES_CONFIG",
            Some(&mock_config_path),
            || {
                // Act: Call the init function
                let result = RagServices::init(mock_config_path);

                // Assert: Should return error due to JSON parse failure
                assert!(result.is_err());
                if let Err(e) = result {
                    assert!(e.to_string().contains("Parse JSON file failed"));
                }
            },
        );

        // Clean up: Remove the mock file
        if let Err(e) = fs::remove_file(mock_config_path) {
            eprintln!("Warning: Failed to remove test file: {}", e);
        }
    }

    // Test case: Empty JSON file
    #[test]
    fn test_init_empty_json() {
        // Arrange: Set up a mock environment variable with empty JSON
        let empty_json = "{}";
        let mock_config_path = "test_empty_rag_config.json";

        // Mock the file content by simulating a file read operation
        let mock_file_content = empty_json.to_string();

        // Create a temporary file for testing
        let result = fs::write(mock_config_path, mock_file_content)
            .map_err(|e| panic!("Failed to write empty JSON file: {}", e));

        // Ensure the file was written successfully
        assert!(result.is_ok());

        with_var(
            "KODABI_RAG_SERVICES_CONFIG",
            Some(&mock_config_path),
            || {
                // Act: Call the init function
                let result = RagServices::init(mock_config_path);

                // Assert: Should return error due to empty or malformed JSON
                assert!(result.is_err());
                if let Err(e) = result {
                    assert!(e.to_string().contains("Parse JSON file failed"));
                }
            },
        );

        // Clean up: Remove the mock file
        if let Err(e) = fs::remove_file(mock_config_path) {
            eprintln!("Warning: Failed to remove test file: {}", e);
        }
    }

    // Test case: File with incorrect structure (missing services array)
    #[test]
    fn test_init_missing_services_array() {
        // Arrange: Set up a mock environment variable with invalid structure
        let invalid_structure = r#"
        {
            "rag_name": "RagService1",
            "rag_ip": "192.168.1.10",
            "rag_port": "8080"
        }
        "#;

        let mock_config_path = "test_invalid_structure.json";

        // Mock file content with invalid structure
        let mock_file_content = invalid_structure.to_string();
        let result = fs::write(mock_config_path, mock_file_content)
            .map_err(|e| panic!("Failed to write invalid structure file: {}", e));

        // Ensure the file was written successfully
        assert!(result.is_ok());

        with_var(
            "KODABI_RAG_SERVICES_CONFIG",
            Some(&mock_config_path),
            || {
                // Act: Call the init function
                let result = RagServices::init(mock_config_path);

                // Assert: Should return error due to invalid JSON structure
                assert!(result.is_err());
                if let Err(e) = result {
                    assert!(e.to_string().contains("Parse JSON file failed"));
                }
            },
        );

        // Clean up: Remove the mock file
        if let Err(e) = fs::remove_file(mock_config_path) {
            eprintln!("Warning: Failed to remove test file: {}", e);
        }
    }

    // Test case: File with malformed field names (e.g., missing quotes)
    #[test]
    fn test_init_malformed_field_names() {
        // Arrange: Set up a mock environment variable with malformed JSON
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

        let mock_config_path = "test_malformed_fields.json";

        // Mock file content with malformed field names
        let mock_file_content = malformed_json.to_string();
        let result = fs::write(mock_config_path, mock_file_content)
            .map_err(|e| panic!("Failed to write malformed JSON file: {}", e));

        // Ensure the file was written successfully
        assert!(result.is_ok());

        with_var(
            "KODABI_RAG_SERVICES_CONFIG",
            Some(&mock_config_path),
            || {
                // Act: Call the init function
                let result = RagServices::init(mock_config_path);

                // Assert: Should return error due to JSON parse failure
                assert!(result.is_err());
                if let Err(e) = result {
                    assert!(e.to_string().contains("Parse JSON file failed"));
                }
            },
        );

        // Clean up: Remove the mock file
        if let Err(e) = fs::remove_file(mock_config_path) {
            eprintln!("Warning: Failed to remove test file: {}", e);
        }
    }
}
