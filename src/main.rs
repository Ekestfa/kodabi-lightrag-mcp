use axum::{
    routing::get,
    Router
};
use tokio::net::TcpListener;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a TCP listener on port 3000
    let listener = TcpListener::bind("0.0.0.0:3000").await?;
    
    // Create a router with multiple routes
    let app = Router::new()
        .route("/health", get(|| async { "OK" }));

    
    // Start serving requests on the listener
    println!("Server is running on http://0.0.0.0:3000");
    
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