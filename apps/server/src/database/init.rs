use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

/// 根据数据库连接地址创建并校验 PostgreSQL 连接池。
pub async fn init_db(database_url: &str) -> anyhow::Result<PgPool> {
    // 限制连接数，避免单个应用实例耗尽数据库连接。
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // 启动阶段执行轻量查询，数据库不可用时立即失败。
    let _check: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    println!("数据库连接校验成功");
    Ok(pool)
}
