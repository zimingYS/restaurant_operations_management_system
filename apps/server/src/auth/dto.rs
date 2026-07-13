use serde::{Deserialize, Serialize};

/// 初始化首个店主账号的请求体。
#[derive(Debug, Deserialize)]
pub struct BootstrapOwnerRequest {
    /// 登录账号。
    pub username: String,
    /// 店主邮箱。
    pub email: String,
    /// 系统中显示的姓名。
    pub display_name: String,
    /// 明文密码；只在本次请求的内存中短暂存在。
    pub password: String,
}

/// 初始化首个店主账号后的安全响应体。
#[derive(Debug, Serialize)]
pub struct BootstrapOwnerResponse {
    /// 新创建用户的数据库主键。
    pub id: i64,
    /// 新创建用户的登录账号。
    pub username: String,
    /// 新创建用户的显示名称。
    pub display_name: String,
}
