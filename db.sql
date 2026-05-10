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
