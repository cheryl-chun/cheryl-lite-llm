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
CREATE TABLE virtual_keys (
    id CHAR(36) PRIMARY KEY,
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at DATETIME NULL,
    models JSON NOT NULL,
    user_id VARCHAR(255) NULL,
    team_id VARCHAR(255) NULL,
    created_by CHAR(36) NOT NULL,  -- FK to master_keys.id
    description TEXT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME NULL,
    
    INDEX idx_key_hash (key_hash),
    INDEX idx_enabled (enabled),
    INDEX idx_user_id (user_id),
    INDEX idx_team_id (team_id),
    FOREIGN KEY (created_by) REFERENCES master_keys(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;

CREATE TABLE master_keys (
    id CHAR(36) PRIMARY KEY,
    key_hash VARCHAR(64) NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT TRUE,
    expires_at DATETIME NULL,
    description VARCHAR(255) NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    last_used_at DATETIME NULL,
    
    INDEX idx_key_hash (key_hash),
    INDEX idx_enabled (enabled)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4;