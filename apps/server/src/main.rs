mod app;
// Authentication primitives are wired when the first auth route is added.
#[allow(dead_code)]
mod auth;
mod config;
mod database;
mod health;
mod state;

use axum::serve;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = config::AppConfig::load()?;
    let db = database::init::init_db(&config.database_url).await?;
    let state = state::AppState::new(db);
    let app = app::build(state);

    println!("服务器启动于 http://{}", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    serve(listener, app).await?;

    Ok(())
}
