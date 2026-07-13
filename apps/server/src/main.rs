use axum::{serve, Router};
use axum::routing::get;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/health",get(|| async { "healthy" }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    serve(listener, app).await?;
    
    Ok(())
}
