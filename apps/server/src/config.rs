use std::{env, net::SocketAddr};

use dotenvy::dotenv;

/// 全局设置
pub struct AppConfig {
    pub database_url: String,
    pub bind_addr: SocketAddr,
}

impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        // 读取配置文件
        dotenv().ok();

        // 数据库链接
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| anyhow::anyhow!("环境变量 DATABASE_URL 未配置，请检查 .env 文件"))?;
        // 地址绑定
        let bind_addr = env::var("BIND_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_owned())
            .parse()?;

        Ok(Self {
            database_url,
            bind_addr,
        })
    }
}
