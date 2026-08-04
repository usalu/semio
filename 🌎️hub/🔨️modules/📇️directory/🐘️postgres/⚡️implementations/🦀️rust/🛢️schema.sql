-- #region 🔖️Header
-- 2026 Ueli Saluz
-- AGPL-3.0
-- os-hub directory Postgres schema (identity/tenancy only) — idempotent bootstrap
-- (CREATE ... IF NOT EXISTS), no migration framework (greenfield: there are no users yet, so
-- schema changes are edited in place, not migrated). Document persistence and blobs are
-- `db::Database`'s tables, not this schema's.
-- #endregion 🔖️Header

CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at BIGINT NOT NULL
);

CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at BIGINT NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);

CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at BIGINT NOT NULL,
    PRIMARY KEY (space_id, user_id)
);

CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    sso_provider TEXT
);

CREATE TABLE IF NOT EXISTS hub_sync_session (
    id TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at BIGINT NOT NULL,
    disconnected_at BIGINT
);

CREATE TABLE IF NOT EXISTS hub_share_token (
    token TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    created_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
