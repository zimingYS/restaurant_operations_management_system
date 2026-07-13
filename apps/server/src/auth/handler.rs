use axum::{Json, extract::State, http::StatusCode};

use crate::{
    auth::{
        dto::{BootstrapOwnerRequest, BootstrapOwnerResponse},
        error::AuthHttpError,
        service,
    },
    state::AppState,
};

/// 处理首个店主账号初始化请求。
pub async fn bootstrap_owner(
    // 从 Axum 路由状态中提取数据库连接池。
    State(state): State<AppState>,
    // 将 JSON 请求体反序列化为认证请求 DTO。
    Json(request): Json<BootstrapOwnerRequest>,
) -> Result<(StatusCode, Json<BootstrapOwnerResponse>), AuthHttpError> {
    // 业务层负责密码哈希、事务与角色分配。
    let response = service::bootstrap_owner(&state.db, request)
        .await
        .map_err(AuthHttpError::from)?;

    // 初始化成功时返回 201 和不含敏感字段的响应体。
    Ok((StatusCode::CREATED, Json(response)))
}
