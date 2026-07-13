-- Add up migration script here
-- 删除旧的用户名长度约束
ALTER TABLE users
    DROP CONSTRAINT users_username_length;

-- 用户名：去除首尾空格后长度必须为 3~50
ALTER TABLE users
    ADD CONSTRAINT users_username_length
        CHECK (
            char_length(btrim(username)) BETWEEN 3 AND 50
            );

-- 邮箱：去除首尾空格后不能为空
ALTER TABLE users
    ADD CONSTRAINT users_email_not_blank
        CHECK (
            char_length(btrim(email)) > 0
            );

-- 显示名称：去除首尾空格后不能为空
ALTER TABLE users
    ADD CONSTRAINT users_display_name_not_blank
        CHECK (
            char_length(btrim(display_name)) > 0
            );