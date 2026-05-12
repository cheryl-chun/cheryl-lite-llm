-- Pg
CREATE TABLE virtual_keys (
    id uuid PRIMARY KEY,
    key_hash text NOT NULL UNIQUE,
    key_prefix text,
    key_alias text,
    enabled boolean NOT NULL DEFAULT true,
    expires_at timestamptz,
    models jsonb NOT NULL DEFAULT '[]',
    user_id uuid,
    team_id uuid,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now()
);
-- MySQL
CREATE TABLE IF NOT EXISTS virtual_keys (
    id CHAR(36) PRIMARY KEY,
    key_hash VARCHAR(255) NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at DATETIME NULL,
    models JSON NOT NULL,
    user_id CHAR(36) NULL,
    team_id CHAR(36) NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    INDEX idx_key_hash (key_hash),
    INDEX idx_user_id (user_id),
    INDEX idx_team_id (team_id)
);