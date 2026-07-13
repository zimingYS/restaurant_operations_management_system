-- Add up migration script here
CREATE TABLE users (
                       id BIGSERIAL PRIMARY KEY,

                       username VARCHAR(50) NOT NULL,
                       email VARCHAR(255) NOT NULL,

                       password_hash TEXT NOT NULL,

                       display_name VARCHAR(100) NOT NULL,

                       is_active BOOLEAN NOT NULL DEFAULT TRUE,

                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                       CONSTRAINT users_username_length
                           CHECK (char_length(username) BETWEEN 3 AND 50)
);

-- 用户名忽略大小写唯一
CREATE UNIQUE INDEX users_username_unique_idx
    ON users (LOWER(username));

-- 邮箱忽略大小写唯一
CREATE UNIQUE INDEX users_email_unique_idx
    ON users (LOWER(email));