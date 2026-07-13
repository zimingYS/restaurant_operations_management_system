use argon2::{Argon2, PasswordHash};
use password_hash::{PasswordHasher, PasswordVerifier};
use thiserror::Error;

const MIN_PASSWORD_LENGTH: usize = 12;
const MAX_PASSWORD_LENGTH: usize = 128;

#[derive(Debug, Error)]
pub enum PasswordError {
    #[error("密码必须介于{MIN_PASSWORD_LENGTH}和{MAX_PASSWORD_LENGTH}个字符之间")]
    InvalidLength,
    #[error("哈希密码失败")]
    HashError,
    #[error("存储的密码哈希无效")]
    InvalidPasswordHash,
}

/// 将明文密码哈希为PHC格式字符串
pub fn hash_password(password: &str) -> Result<String, PasswordError> {
    validate_password_policy(password)?;

    let password_hash = Argon2::default()
        .hash_password(password.as_bytes())
        .map_err(|_| PasswordError::HashError)?
        .to_string();

    Ok(password_hash)
}

/// 验证用户输入的密码是否与存储的哈希匹配
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, PasswordError> {
    let parsed_hash =
        PasswordHash::new(password_hash).map_err(|_| PasswordError::InvalidPasswordHash)?;

    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
        Err(_) => Err(PasswordError::InvalidPasswordHash),
    }
}

// 校验密码是否符合长度规则
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

    // 正确密码验证测试
    #[test]
    fn test_correct_password_verify_success() {
        // 准备正确密码
        let plain_pwd = "MySecurePass123!";
        let hash = hash_password(plain_pwd).unwrap();
        // 校验返回true
        let result = verify_password(plain_pwd, &hash).unwrap();
        assert_eq!(result, true);
    }

    // 错误密码验证失败测试
    #[test]
    fn test_wrong_password_verify_failed() {
        let real_pwd = "HelloWorld@2026";
        let wrong_pwd = "WrongPass@1234";
        let hash = hash_password(real_pwd).unwrap();

        let ok = verify_password(wrong_pwd, &hash).unwrap();
        assert_eq!(ok, false);
    }

    // 同一明文两次哈希结果不同测试
    #[test]
    fn test_two_hash_from_same_plain_not_equal() {
        // Argon2会自动生成随机盐，两次哈希结果不一样
        let pwd = "SamePassword666#";
        let hash1 = hash_password(pwd).unwrap();
        let hash2 = hash_password(pwd).unwrap();
        assert_ne!(hash1, hash2);
    }

    // 损坏的哈希字符串返回错误测试
    #[test]
    fn test_corrupted_hash_return_error() {
        let bad_hash = "this-is-completely-wrong-hash-content-12345";
        let res = verify_password("SomePass123", bad_hash);
        assert!(matches!(res, Err(PasswordError::InvalidPasswordHash)));
    }

    // 测试长度校验 超出范围返回错误
    #[test]
    fn test_password_length_policy() {
        let short = "123";
        let long = "a".repeat(130);
        assert!(validate_password_policy(short).is_err());
        assert!(validate_password_policy(&long).is_err());
    }
}
