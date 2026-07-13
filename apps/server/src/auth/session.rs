use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use rand::RngExt;
use sha2::{Digest, Sha256};

#[allow(dead_code)]
pub struct SessionToken {
    raw: String,
    hash: String,
}

impl SessionToken {
    pub fn generate() -> Self {
        let mut bytes = [0u8; 32];
        rand::rng().fill(&mut bytes);

        // Base64 URL-safe
        let raw = URL_SAFE_NO_PAD.encode(bytes);

        // 十六进制字符串
        let hash = hash_session_token(&raw);

        Self { raw, hash }
    }

    /// 获取浏览器Cookie中保存的原始Token
    pub fn raw(&self) -> &str {
        &self.raw
    }

    /// 获取数据库保存的SHA-256哈希
    pub fn hash(&self) -> &str {
        &self.hash
    }
}

pub fn hash_session_token(raw_token: &str) -> String {
    let digest = Sha256::digest(raw_token.as_bytes());
    hex::encode(digest)
}

#[cfg(test)]
mod tests {
    use super::{SessionToken, hash_session_token};

    #[test]
    fn generated_raw_tokens_are_different() {
        let token1 = SessionToken::generate();
        let token2 = SessionToken::generate();

        assert_ne!(token1.raw(), token2.raw());
    }

    #[test]
    fn generated_token_hashes_are_different() {
        let token1 = SessionToken::generate();
        let token2 = SessionToken::generate();

        assert_ne!(token1.hash(), token2.hash());
    }

    #[test]
    fn same_raw_token_has_same_hash() {
        let raw_token = "this_is_a_fixed_session_token";

        let hash1 = hash_session_token(raw_token);
        let hash2 = hash_session_token(raw_token);

        assert_eq!(hash1, hash2);
    }

    #[test]
    fn token_hash_is_64_hex_characters() {
        let token = SessionToken::generate();
        let hash = token.hash();

        assert_eq!(hash.len(), 64);
        assert!(hash.chars().all(|c| c.is_ascii_hexdigit()));
    }
}
