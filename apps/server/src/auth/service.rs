use crate::auth::dto::{
    BootstrapOwnerRequest, BootstrapOwnerResponse, LoginRequest, LoginResponse,
};
use crate::auth::password::{PasswordError, hash_password, verify_password};
use crate::auth::repository;
use crate::auth::session::SessionToken;
use sqlx::PgPool;
use thiserror::Error;

/// 会话有效期
const SESSION_TTL_SECONDS: i64 = 60 * 60 * 24 * 7;

/// 初始化首个店主账号时可能发生的业务错误。
#[derive(Debug, Error)]
pub enum BootstrapOwnerError {
    /// 密码不符合安全策略或哈希失败。
    #[error(transparent)]
    InvalidPassword(#[from] PasswordError),

    /// 系统已经存在用户，不能重复初始化首店主。
    #[error("System has already been initialized")]
    AlreadyInitialized,

    /// 执行认证相关数据库操作失败。
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    /// 在阻塞线程池中执行密码哈希任务失败。
    #[error(transparent)]
    PasswordTask(#[from] tokio::task::JoinError),
}

/// 在一个事务中创建首个用户并分配 owner 角色。
pub async fn bootstrap_owner(
    pool: &PgPool,
    request: BootstrapOwnerRequest,
) -> Result<BootstrapOwnerResponse, BootstrapOwnerError> {
    // 拆分请求，避免密码哈希后继续保留整个请求对象。
    let BootstrapOwnerRequest {
        username,
        email,
        display_name,
        password,
    } = request;

    // Argon2 是 CPU 密集操作，放入 Tokio 阻塞线程池执行。
    let password_hash = tokio::task::spawn_blocking(move || hash_password(&password)).await??;

    // 后续数据库操作必须处于同一个事务中。
    let mut tx = pool.begin().await?;
    // 先加锁，再检查用户数量，保证初始化操作只能成功一次。
    repository::lock_bootstrap(&mut tx).await?;

    if repository::has_any_user(&mut tx).await? {
        // 未提交的事务会在离开作用域时自动回滚并释放锁。
        return Err(BootstrapOwnerError::AlreadyInitialized);
    }

    // 查询系统预置的 owner 角色。
    let role_id = repository::find_role_id_by_code(&mut tx, "owner").await?;
    // 插入用户后取得数据库分配的主键。
    let user_id =
        repository::insert_user(&mut tx, &username, &email, &display_name, &password_hash).await?;

    // 建立用户与 owner 角色的多对多关联。
    repository::assign_role(&mut tx, user_id, role_id).await?;
    // 所有步骤成功后才提交事务。
    tx.commit().await?;

    // 响应只返回安全的公开信息。
    Ok(BootstrapOwnerResponse {
        id: user_id,
        username,
        display_name,
    })
}

/// 登录成功
pub struct LoginSuccess {
    pub response: LoginResponse,
    pub session_token: SessionToken,
}

/// 登录出错
#[derive(Error, Debug)]
pub enum LoginError {
    #[error("无效凭证")]
    InvalidCredentials,

    #[error(transparent)]
    PasswordVerification(#[from] PasswordError),

    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    PasswordTask(#[from] tokio::task::JoinError),
}

pub async fn login(pool: &PgPool, request: LoginRequest) -> Result<LoginSuccess, LoginError> {
    // 获取用户名和密码
    let LoginRequest { username, password } = request;

    // 查询用户
    let user = match repository::find_user_by_username(pool, &username).await? {
        Some(user) => user,
        None => return Err(LoginError::InvalidCredentials),
    };

    if !user.is_active {
        return Err(LoginError::InvalidCredentials);
    }

    // 密码验证放到阻塞线程池
    let password_hash = user.password_hash;

    let verified =
        tokio::task::spawn_blocking(move || verify_password(&password, &password_hash)).await??;

    if !verified {
        return Err(LoginError::InvalidCredentials);
    }

    // 开启事务
    let mut tx = pool.begin().await?;

    // 生成新的 Session Token
    let session_token = SessionToken::generate();

    // 创建 Session
    repository::create_session(&mut tx, user.id, session_token.hash(), SESSION_TTL_SECONDS).await?;

    // 提交事务
    tx.commit().await?;

    Ok(LoginSuccess {
        response: LoginResponse {
            id: user.id,
            username: user.username,
        },
        session_token,
    })
}
