use std::{env, net::SocketAddr};

use dotenvy::dotenv;

/// 应用启动所需的全局配置。
pub struct AppConfig {
    /// PostgreSQL 数据库连接地址。
    pub database_url: String,
    /// HTTP 服务绑定的监听地址。
    pub bind_addr: SocketAddr,
}

impl AppConfig {
    /// 从 .env 文件和系统环境变量加载配置。
    pub fn load() -> anyhow::Result<Self> {
        // 尝试加载本地 .env；部署环境可直接使用系统环境变量。
        dotenv().ok();

        // 数据库地址必须显式配置。
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("环境变量 DATABASE_URL 未配置，请检查 .env 文件"))?;
        // 未配置时仅监听本机，避免开发服务意外暴露到局域网。
        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse()?;

        Ok(Self {
            database_url,
            bind_addr,
        })
    }
}
