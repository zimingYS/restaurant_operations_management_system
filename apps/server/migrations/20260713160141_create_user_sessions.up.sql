-- Add up migration script here
CREATE TABLE user_sessions (
    id BIGSERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    token_hash CHAR(64) NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_used_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    revoked_at TIMESTAMPTZ,
    CONSTRAINT user_sessions_user_id_fk
        FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE CASCADE,

    CONSTRAINT user_sessions_token_hash_length
        CHECK (char_length(token_hash) = 64),

    CONSTRAINT user_sessions_expiry_check
       CHECK (expires_at > created_at)
);

-- 用户有效会话索引（仅未撤销）
CREATE INDEX user_sessions_active_user_idx
    ON user_sessions (user_id)
    WHERE revoked_at IS NULL;

-- 过期会话清理索引
CREATE INDEX user_sessions_expires_at_idx
    ON user_sessions (expires_at);