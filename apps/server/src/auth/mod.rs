use crate::state::AppState;
use axum::Router;
use axum::routing::post;

pub mod dto;
pub mod error;
pub mod handler;
pub mod password;
pub mod repository;
pub mod service;
pub mod session;

/// 创建认证模块的子路由。
pub fn router() -> Router<AppState> {
    // 此路由会在 app.rs 中挂载到 /auth 前缀。
    Router::new()
        .route("/bootstrap-owner", post(handler::bootstrap_owner))
        .route("/login", post(handler::login))
}
