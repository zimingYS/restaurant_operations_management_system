-- Add down migration script here
-- 删除关联表
DROP TABLE IF EXISTS user_roles;

-- 删除角色表
DROP TABLE IF EXISTS roles;