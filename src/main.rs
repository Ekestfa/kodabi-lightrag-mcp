use axum::{routing::get, Router};
use std::env;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::from_filename(".env").ok();
    let ip = env::var("KODABI_BASE_IP").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("KODABI_BASE_PORT").unwrap_or_else(|_| "8080".to_string());
    // Create a TCP listener on port 3000
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).await?;

    // Create a router with multiple routes
    let app = Router::new().route("/health", get(|| async { "OK" }));

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