use axum::{
    routing::{get, post},
    Router,
    Extension
};
use std::env;
use std::sync::Arc;
use tokio::net::TcpListener;

use kodabi_lightrag_mcp::handlers::query_handler::central_query_handler;
use kodabi_lightrag_mcp::handlers::mcp_handler::mcp_info_handler;
use kodabi_lightrag_mcp::models::rag_mcp::RagMcp;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").ok();
    let ip = env::var("KODABI_BASE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("KODABI_BASE_PORT").unwrap_or_else(|_| "9699".to_string());

    // Initialize RagMcp implementation
    let rag_mcp = match RagMcp::new() {
        Ok(r) => Arc::new(r),
        Err(e) => {
            eprintln!("Failed to initialize RagMcp: {:?}", e);
            std::process::exit(1);
        }
    };

    // Create a TCP listener on port 3000
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;

    // Prepare shared rag config path as Arc
    // let rag_config_path = Arc::new(rag_config);

    // Create a router with multiple routes
    let app = Router::new()
        // .layer(Extension(rag_config_path))
        .route("/health", get(|| async { "OK" }))
        .route("/central/query", post(central_query_handler))
        .route("/mcp/info", post(mcp_info_handler))
        .layer(Extension(rag_mcp.clone()));

    // Start serving requests on the listener
    println!("Server is running on {}:{}", ip, port);

    // Serve requests using the router
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}

async fn shutdown_signal() {
    // Wait for a shutdown signal (e.g., Ctrl+C)
    tokio::signal::ctrl_c().await.unwrap();
}
