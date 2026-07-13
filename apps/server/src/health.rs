use axum::{Router, routing::get};

use crate::state::AppState;

/// 创建健康检查路由。
pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(health_check))
}

/// 返回服务存活状态，不访问数据库或业务数据。
async fn health_check() -> &'static str {
    "healthy"
}
