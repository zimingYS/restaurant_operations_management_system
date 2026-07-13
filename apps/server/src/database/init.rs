use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

pub async fn init_db(database_url: &str) -> anyhow::Result<PgPool> {
    // 创建数据库连接池
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    // 数据库检查
    let _check: i32 = sqlx::query_scalar("SELECT 1").fetch_one(&pool).await?;

    println!("数据库连接校验成功");
    Ok(pool)
}
