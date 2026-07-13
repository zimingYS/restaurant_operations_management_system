use crate::auth::dto::{LoginRequest, LoginResponse};
use crate::{
    auth::{
        dto::{BootstrapOwnerRequest, BootstrapOwnerResponse},
        error::AuthHttpError,
        service,
    },
    state::AppState,
};
use axum::{Json, extract::State, http::StatusCode};
use axum_extra::extract::CookieJar;
use axum_extra::extract::cookie::{Cookie, SameSite};

const SESSION_COOKIE_NAME: &str = "restaurant_session";

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

pub async fn login(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(request): Json<LoginRequest>,
) -> Result<(CookieJar, Json<LoginResponse>), AuthHttpError> {
    let login_success = service::login(&state.db, request).await?;

    let cookie = Cookie::build((
        SESSION_COOKIE_NAME,
        login_success.session_token.raw().to_owned(),
    ))
    .path("/")
    .http_only(true)
    .same_site(SameSite::Lax)
    .secure(false)
    .max_age(time::Duration::days(7))
    .build();

    Ok((jar.add(cookie), Json(login_success.response)))
}
