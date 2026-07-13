use sqlx::{Postgres, Transaction};

/// 首店主初始化专用的 PostgreSQL advisory lock 键。
const BOOTSTRAP_OWNER_LOCK_KEY: i64 = 2_026_071_301;

/// 获取事务级 advisory lock，避免并发请求创建多个首店主。
pub async fn lock_bootstrap(tx: &mut Transaction<'_, Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        SELECT pg_advisory_xact_lock($1)
        "#,
    )
    .bind(BOOTSTRAP_OWNER_LOCK_KEY)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

/// 判断系统中是否已经存在任意用户。
pub async fn has_any_user(tx: &mut Transaction<'_, Postgres>) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM users
        )
        "#,
    )
    .fetch_one(&mut **tx)
    .await
}

/// 根据稳定的角色 code 查询角色主键。
pub async fn find_role_id_by_code(
    tx: &mut Transaction<'_, Postgres>,
    code: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT id
        FROM roles
        WHERE code = $1
        "#,
    )
    .bind(code)
    .fetch_one(&mut **tx)
    .await
}

/// 插入用户记录并返回数据库生成的用户主键。
pub async fn insert_user(
    tx: &mut Transaction<'_, Postgres>,
    username: &str,
    email: &str,
    display_name: &str,
    password_hash: &str,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        INSERT INTO users (
            username,
            email,
            display_name,
            password_hash
        )
        VALUES ($1, $2, $3, $4)
        RETURNING id
        "#,
    )
    // 所有外部输入使用绑定参数，避免 SQL 注入。
    .bind(username)
    .bind(email)
    .bind(display_name)
    .bind(password_hash)
    .fetch_one(&mut **tx)
    .await
}

/// 为指定用户创建角色关联记录。
pub async fn assign_role(
    tx: &mut Transaction<'_, Postgres>,
    user_id: i64,
    role_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO user_roles (
            user_id,
            role_id
        )
        VALUES ($1, $2)
        "#,
    )
    .bind(user_id)
    .bind(role_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}
