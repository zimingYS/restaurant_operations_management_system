// 应用路由组装模块。
mod app;
// 认证相关功能模块。
mod auth;
// 环境配置读取模块。
mod config;
// 数据库连接初始化模块。
mod database;
// 健康检查接口模块。
mod health;
// 全局共享应用状态模块。
mod state;

use axum::serve;

/// 启动 HTTP 服务。
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 读取环境变量与本地配置。
    let config = config::AppConfig::load()?;
    // 创建并校验 PostgreSQL 连接池。
    let db = database::init::init_db(&config.database_url).await?;
    // 将共享依赖放入应用状态。
    let state = state::AppState::new(db);
    // 组装全部业务路由。
    let app = app::build(state);

    // 绑定 HTTP 监听地址并开始提供服务。
    println!("Server listening on http://{}", config.bind_addr);
    let listener = tokio::net::TcpListener::bind(config.bind_addr).await?;
    serve(listener, app).await?;

    Ok(())
}
