use sqlx::PgPool;

/// 在多个路由之间共享的应用依赖。
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL 连接池句柄；克隆状态不会复制实际连接。
    pub db: PgPool,
}

impl AppState {
    /// 使用已创建的数据库连接池构造应用状态。
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
