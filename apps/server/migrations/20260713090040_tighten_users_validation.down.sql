-- Add down migration script here
-- 删除本次新增的约束
ALTER TABLE users
    DROP CONSTRAINT users_display_name_not_blank;

ALTER TABLE users
    DROP CONSTRAINT users_email_not_blank;

ALTER TABLE users
    DROP CONSTRAINT users_username_length;

-- 恢复旧的用户名长度约束
ALTER TABLE users
    ADD CONSTRAINT users_username_length
        CHECK (
            char_length(username) BETWEEN 3 AND 50
            );