-- Add up migration script here
-- 创建角色表
CREATE TABLE roles (
                       id BIGSERIAL PRIMARY KEY,

                       code VARCHAR(50) NOT NULL,
                       name VARCHAR(100) NOT NULL,
                       description TEXT,

                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

                       CONSTRAINT roles_code_unique UNIQUE (code)
);

-- 创建用户角色关联表（多对多）
CREATE TABLE user_roles (
                            user_id BIGINT NOT NULL,
                            role_id BIGINT NOT NULL,

                            assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- 联合主键，防止重复分配
                            PRIMARY KEY (user_id, role_id),

    -- 删除用户时自动删除关联
                            CONSTRAINT fk_user_roles_user
                                FOREIGN KEY (user_id)
                                    REFERENCES users(id)
                                    ON DELETE CASCADE,

    -- 删除角色时禁止删除
                            CONSTRAINT fk_user_roles_role
                                FOREIGN KEY (role_id)
                                    REFERENCES roles(id)
                                    ON DELETE RESTRICT
);

-- 按角色查询用户时使用
CREATE INDEX user_roles_role_id_idx
    ON user_roles(role_id);

-- 初始化系统角色
INSERT INTO roles (code, name, description)
VALUES
    ('owner', '店主', '系统最高权限'),
    ('manager', '店长', '负责门店日常管理'),
    ('clerk', '店员', '负责日常营业');