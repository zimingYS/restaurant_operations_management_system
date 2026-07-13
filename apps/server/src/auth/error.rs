use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::auth::service::BootstrapOwnerError;

/// 对客户端公开的统一错误响应体。
#[derive(Serialize)]
struct ErrorResponse {
    /// 不包含数据库或密码细节的安全错误消息。
    message: &'static str,
}

/// 将认证业务错误转换为 HTTP 响应的包装类型。
pub struct AuthHttpError(BootstrapOwnerError);

impl From<BootstrapOwnerError> for AuthHttpError {
    /// 接收 service 层错误，交由 IntoResponse 统一映射。
    fn from(error: BootstrapOwnerError) -> Self {
        Self(error)
    }
}

impl IntoResponse for AuthHttpError {
    /// 将业务错误映射为客户端可理解且不泄露内部信息的响应。
    fn into_response(self) -> Response {
        let (status, message) = match self.0 {
            // 密码策略不通过属于客户端输入问题。
            BootstrapOwnerError::InvalidPassword(_) => (
                StatusCode::BAD_REQUEST,
                "Password does not meet the security policy",
            ),
            // 初始化只能执行一次。
            BootstrapOwnerError::AlreadyInitialized => {
                (StatusCode::CONFLICT, "System has already been initialized")
            }
            // 数据库与阻塞任务错误不暴露具体原因。
            BootstrapOwnerError::Database(_) | BootstrapOwnerError::PasswordTask(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
            }
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
