use argon2::{Argon2, PasswordHash};
use password_hash::{PasswordHasher, PasswordVerifier};
use thiserror::Error;

/// 密码允许的最小字符数。
const MIN_PASSWORD_LENGTH: usize = 12;
/// 密码允许的最大字符数。
const MAX_PASSWORD_LENGTH: usize = 128;

/// 密码哈希与验证过程中的错误。
#[derive(Debug, Error)]
pub enum PasswordError {
    /// 明文密码长度不符合系统策略。
    #[error("密码必须介于{MIN_PASSWORD_LENGTH}和{MAX_PASSWORD_LENGTH}个字符之间")]
    InvalidLength,
    /// Argon2 无法生成密码哈希。
    #[error("哈希密码失败")]
    HashError,
    /// 数据库存储的 PHC 格式密码哈希无法解析。
    #[error("存储的密码哈希无效")]
    #[allow(dead_code)]
    InvalidPasswordHash,
}

/// 将明文密码哈希为包含算法参数和随机 salt 的 PHC 字符串。
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    // 先拒绝不符合策略的密码，避免无效的哈希计算。
    validate_password_policy(password)?;

    // Argon2 默认使用随机 salt，完整结果可直接存入数据库。
    let password_hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| PasswordError::HashError)?
        .to_string();

    Ok(password_hash)
}

/// 验证明文密码是否匹配数据库保存的 PHC 哈希。
#[allow(dead_code)]
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, PasswordError> {
    // 先解析数据库中的 PHC 格式哈希。
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| PasswordError::InvalidPasswordHash)?;

    // 密码不匹配返回 false；损坏的哈希返回错误。
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(_) => Err(PasswordError::InvalidPasswordHash),
    }
}

/// 检查密码字符数是否位于允许范围内。
fn validate_password_policy(password: &str) -> Result<(), PasswordError> {
    let len = password.chars().count();

    if !(MIN_PASSWORD_LENGTH..=MAX_PASSWORD_LENGTH).contains(&len) {
        return Err(PasswordError::InvalidLength);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 正确密码应通过验证。
    #[test]
    fn test_correct_password_verify_success() {
        let plain_pwd = "MySecurePass123!";
        let hash = hash_password(plain_pwd).unwrap();

        let result = verify_password(plain_pwd, &hash).unwrap();
        assert!(result);
    }

    /// 错误密码不应通过验证。
    #[test]
    fn test_wrong_password_verify_failed() {
        let real_pwd = "HelloWorld@2026";
        let wrong_pwd = "WrongPass@1234";
        let hash = hash_password(real_pwd).unwrap();

        let ok = verify_password(wrong_pwd, &hash).unwrap();
        assert!(!ok);
    }

    /// 同一明文密码的两次哈希必须因随机 salt 而不同。
    #[test]
    fn test_two_hash_from_same_plain_not_equal() {
        let pwd = "SamePassword666#";
        let hash1 = hash_password(pwd).unwrap();
        let hash2 = hash_password(pwd).unwrap();

        assert_ne!(hash1, hash2);
    }

    /// 损坏的哈希字符串应返回错误，而不是错误地验证通过。
    #[test]
    fn test_corrupted_hash_return_error() {
        let bad_hash = "this-is-completely-wrong-hash-content-12345";
        let res = verify_password("SomePass123", bad_hash);

        assert!(matches!(res, Err(PasswordError::InvalidPasswordHash)));
    }

    /// 超出允许范围的密码长度应被拒绝。
    #[test]
    fn test_password_length_policy() {
        let short = "123";
        let long = "a".repeat(130);

        assert!(validate_password_policy(short).is_err());
        assert!(validate_password_policy(&long).is_err());
    }
}
