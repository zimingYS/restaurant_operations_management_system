use crate::{auth, health, state::AppState};
use axum::Router;

/// 组装应用中的全部路由，并注入共享状态。
pub fn build(state: AppState) -> Router {
    // 注册无需认证的健康检查接口。
    health::router()
        // 将认证接口统一挂载到 /auth 前缀下。
        .nest("/auth", auth::router())
        // 为需要数据库的路由提供 AppState。
        .with_state(state)
}
