pub mod auth;

use axum::routing::get;
use axum::{Router, serve};
use dotenvy::dotenv;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db = init_db().await?;
    let pool = Arc::new(db);

    let app = Router::new()
        .route("/health", get(|| async { "healthy" }))
        .with_state(pool);

    println!("服务器启动于'http://127.0.0.1:3000/'");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    serve(listener, app).await?;

    Ok(())
}

async fn init_db() -> anyhow::Result<PgPool> {
    // 加载.env
    dotenv().ok();

    // 读取DATABASE_URL
    let database_url = env::var("DATABASE_URL")
        .map_err(|_| anyhow::anyhow!("环境变量 DATABASE_URL 未配置，请检查 .env 文件"))?;

    // 创建最多5个连接的PgPool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // 校验连接
    let _check: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    println!("数据库连接校验成功");
    Ok(pool)
}
