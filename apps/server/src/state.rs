use sqlx::PgPool;

// 全局应用状态
#[derive(Clone)]
pub struct AppState {
    // Postgres连接池
    #[allow(dead_code)]
    pub db: PgPool,
}

impl AppState {
    // 初始化AppState
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}
