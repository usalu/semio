//! 🐘️ `HubDirectory` over PostgreSQL — direct `sqlx-postgres`/`sqlx-core` (not the `sqlx` facade),
//! matching the exact precedent in `compose/server/hub/rs/bin.rs`. The scale-out backend for
//! multi-node self-hosted deployments. `#[cfg(feature = "postgres")]`-gated as a whole by the
//! parent `directory` module (see `📇️directory/🦀️component.rs`'s `//#region 🔖️Backends`).
//!
//! 🌳️ `SCHEMA` is inlined as a `const` string rather than `include_str!`-ed from a sibling
//! `.sql` file: Shape V2 tree purity allows only `component.<ext>` files, `📦️packages`, and plain
//! component folders below an owner root, so a standalone `🛢️schema.sql` asset has no home in this
//! tree (it is neither example/fixture/generated data for `rootDataDirNames` nor packaging code)
//! — folding it into a string literal is a zero-behavior-change mechanical transform (see
//! `📋️TEMPLATE-FAMILY.md`'s "non-source assets" section for the general rule this establishes).

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{kind_to_str, role_from_wire, visibility_to_str, HubClock, HubDirectory, NewDirectoryEvent};
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, Hlc};
pub use sqlx_core::row::Row;
pub use sqlx_postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

//#region 🔖️Schema
// 🛢️ os-hub directory Postgres schema (identity/tenancy only) — idempotent bootstrap
// (CREATE ... IF NOT EXISTS), no migration framework (greenfield: there are no users yet, so
// schema changes are edited in place, not migrated). Document persistence and blobs are
// `db::Database`'s tables, not this schema's.
const SCHEMA: &str = "
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
    space_id TEXT NOT NULL,
    document_id TEXT NOT NULL,
    surface TEXT NOT NULL,
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

CREATE TABLE IF NOT EXISTS hub_space_invite (
    id TEXT PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at BIGINT NOT NULL,
    expires_at BIGINT NOT NULL,
    revoked_at BIGINT
);

CREATE TABLE IF NOT EXISTS hub_directory_event (
    seq BIGSERIAL PRIMARY KEY,
    id TEXT NOT NULL UNIQUE,
    hlc_physical BIGINT NOT NULL,
    hlc_logical BIGINT NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT,
    user_id TEXT,
    kind TEXT NOT NULL,
    payload JSONB NOT NULL,
    recorded_at BIGINT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_sync_session_space ON hub_sync_session (space_id, disconnected_at);
CREATE INDEX IF NOT EXISTS idx_space_invite_space ON hub_space_invite (space_id);
";
//#endregion 🔖️Schema

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

fn actor_kind_to_str(kind: DirectoryActorKind) -> &'static str {
    match kind {
        DirectoryActorKind::User => "user",
        DirectoryActorKind::Admin => "admin",
        DirectoryActorKind::System => "system",
    }
}

fn actor_kind_from_str(value: &str) -> DirectoryActorKind {
    match value {
        "admin" => DirectoryActorKind::Admin,
        "system" => DirectoryActorKind::System,
        _ => DirectoryActorKind::User,
    }
}

/// @emoji 🐘️ PostgreSQL-backed `HubDirectory`, pooled via `PgPool`.
pub struct PostgresDirectory {
    pool: PgPool,
}

impl PostgresDirectory {
    /// @emoji 🔌️ Connects to `database_url` and bootstraps the schema (idempotent, no migration framework).
    pub async fn connect(database_url: &str) -> DirectoryResult<Self> {
        let pool = PgPoolOptions::new().max_connections(20).connect(database_url).await.map_err(backend)?;
        for statement in SCHEMA.split(';').map(str::trim).filter(|s| !s.is_empty() && !s.starts_with("--")) {
            sqlx_core::query::query(statement).execute(&pool).await.map_err(backend)?;
        }
        Ok(Self { pool })
    }

    /// @emoji 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write. The system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6).
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'").fetch_one(&self.pool).await.map_err(backend)?;
        if user_exists.0 > 0 {
            return Ok(());
        }
        let actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:seed".into() };
        let mut clock = HubClock::new();
        let events = vec![
            NewDirectoryEvent { hlc: clock.tick(), actor: actor.clone(), space_id: None, user_id: Some("seed".into()), body: DirectoryEventBody::UserCreated { user_id: "seed".into(), email: "seed@localhost".into(), display_name: "System".into() } },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::SpaceCreated { space_id: "default".into(), name: "Space".into(), space_kind: DirectorySpaceKind::Studio, visibility: DirectorySpaceVisibility::Private, owner_user_id: "seed".into() },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor,
                space_id: Some("default".into()),
                user_id: Some("seed".into()),
                body: DirectoryEventBody::MemberUpserted { space_id: "default".into(), user_id: "seed".into(), role: DirectorySpaceRole::Author },
            },
        ];
        self.append_events(&events).await?;
        Ok(())
    }

    //#region 🔖️Projections
    /// @emoji 🧮️ The only place `hub_user`/`hub_space`/`hub_space_membership` rows are written —
    /// see the sqlite backend's twin for the full rationale (unconditional: `decide` already
    /// enforced every law before this event existed).
    async fn project(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
                    .bind(user_id)
                    .bind(email)
                    .bind(display_name)
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                sqlx_core::query::query("INSERT INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING")
                    .bind(space_id)
                    .bind(name)
                    .bind(owner_user_id)
                    .bind(event.recorded_at_ms)
                    .bind(kind_to_str(*space_kind))
                    .bind(visibility_to_str(*visibility))
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                sqlx_core::query::query("UPDATE hub_space SET name = $2 WHERE id = $1").bind(space_id).bind(name).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                sqlx_core::query::query("UPDATE hub_space SET visibility = $2 WHERE id = $1").bind(space_id).bind(visibility_to_str(*visibility)).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                sqlx_core::query::query("UPDATE hub_space SET kind = 'archive' WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space_invite WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

impl HubDirectory for PostgresDirectory {
    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        let token = Uuid::now_v7().to_string();
        sqlx_core::query::query("INSERT INTO hub_share_token (token, document_id, created_at) VALUES ($1, $2, $3)").bind(&token).bind(document_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool> {
        let has_tokens: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_share_token WHERE document_id = $1").bind(document_id).fetch_one(&self.pool).await.map_err(backend)?;
        if has_tokens.0 == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_share_token WHERE document_id = $1 AND token = $2").bind(document_id).bind(token).fetch_one(&self.pool).await.map_err(backend)?;
                Ok(valid.0 > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7)")
            .bind(&id)
            .bind(email)
            .bind(display_name)
            .bind(password_hash)
            .bind(sso_subject)
            .bind(sso_provider)
            .bind(created_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(UserRecord {
            id,
            email: email.to_string(),
            display_name: display_name.to_string(),
            password_hash: password_hash.map(str::to_string),
            sso_subject: sso_subject.map(str::to_string),
            sso_provider: sso_provider.map(str::to_string),
            created_at,
        })
    }

    async fn get_user(&self, user_id: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = $1").bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = $1").bind(email).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        let row: Option<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = $1 AND sso_subject = $2")
                .bind(provider)
                .bind(subject)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        Ok(row.map(user_from_row))
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64)> =
            sqlx_core::query_as::query_as("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT $1 OFFSET $2")
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?;
        Ok(rows.into_iter().map(user_from_row).collect())
    }
    //#endregion

    //#region Spaces
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        let row: Option<(String, String, String, i64, String, String)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space WHERE id = $1").bind(space_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(|(id, name, owner_user_id, created_at, kind, visibility)| SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }))
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let rows: Vec<(String, String, String, i64, String, String, String)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility, m.role FROM hub_space s
             JOIN hub_space_membership m ON m.space_id = s.id WHERE m.user_id = $1 ORDER BY s.created_at",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role))).collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let rows: Vec<(String, String, String, i64, String, String)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT $1 OFFSET $2").bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(backend)?;
        Ok(rows.into_iter().map(|(id, name, owner_user_id, created_at, kind, visibility)| SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }).collect())
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64, String)> = sqlx_core::query_as::query_as(
            "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
             FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id WHERE m.space_id = $1 ORDER BY m.created_at",
        )
        .bind(space_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .filter_map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at, role)| SpaceRole::parse(&role).map(|role| (UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }, role)))
            .collect())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let row: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.and_then(|(role,)| SpaceRole::parse(&role)))
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        sqlx_core::query::query("INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES ($1, $2, $3, $4, $5)")
            .bind(&id)
            .bind(user_id)
            .bind(created_at)
            .bind(expires_at)
            .bind(sso_provider)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>> {
        let row: Option<(String, String, i64, i64, Option<String>)> =
            sqlx_core::query_as::query_as("SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = $1").bind(id).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(|(id, user_id, created_at, expires_at, sso_provider)| AuthSessionRecord { id, user_id, created_at, expires_at, sso_provider }))
    }

    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("DELETE FROM hub_auth_session WHERE id = $1").bind(id).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region Invites
    async fn create_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64) -> DirectoryResult<InviteRecord> {
        let id = Uuid::now_v7().to_string();
        let token = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        sqlx_core::query::query("INSERT INTO hub_space_invite (id, token, space_id, role, created_at, expires_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&id)
            .bind(&token)
            .bind(space_id)
            .bind(role.as_str())
            .bind(created_at)
            .bind(expires_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(InviteRecord { id, token, space_id: space_id.to_string(), role, created_at, expires_at, revoked_at: None })
    }

    async fn get_invite_by_token(&self, token: &str) -> DirectoryResult<Option<InviteRecord>> {
        let row: Option<(String, String, String, String, i64, i64, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT id, token, space_id, role, created_at, expires_at, revoked_at FROM hub_space_invite WHERE token = $1").bind(token).fetch_optional(&self.pool).await.map_err(backend)?;
        Ok(row.map(invite_from_row))
    }

    async fn revoke_invite(&self, invite_id: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("UPDATE hub_space_invite SET revoked_at = $2 WHERE id = $1").bind(invite_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let rows: Vec<(String, String, String, String, i64, i64, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT id, token, space_id, role, created_at, expires_at, revoked_at FROM hub_space_invite WHERE space_id = $1 ORDER BY created_at DESC").bind(space_id).fetch_all(&self.pool).await.map_err(backend)?;
        Ok(rows.into_iter().map(invite_from_row).collect())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(&self, space_id: &str, document_id: &str, surface: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str());
        sqlx_core::query::query("INSERT INTO hub_sync_session (id, space_id, document_id, surface, user_id, space_role, client_label, connected_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)")
            .bind(&id)
            .bind(space_id)
            .bind(document_id)
            .bind(surface)
            .bind(user_id)
            .bind(role_str)
            .bind(client_label)
            .bind(connected_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(SyncSessionRecord {
            id,
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            surface: surface.to_string(),
            user_id: user_id.map(str::to_string),
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $2 WHERE id = $1").bind(sync_session_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let rows: Vec<(String, String, String, String, Option<String>, Option<String>, String, i64, Option<i64>)> = sqlx_core::query_as::query_as(
            "SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
             WHERE document_id = $1 ORDER BY connected_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().map(sync_session_from_row).collect())
    }

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let rows: Vec<(String, String, String, String, Option<String>, Option<String>, String, i64, Option<i64>)> = match space_id {
            Some(space_id) => sqlx_core::query_as::query_as(
                "SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE space_id = $1 AND disconnected_at IS NULL ORDER BY connected_at DESC",
            )
            .bind(space_id)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
            None => sqlx_core::query_as::query_as(
                "SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE disconnected_at IS NULL ORDER BY connected_at DESC",
            )
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
        };
        Ok(rows.into_iter().map(sync_session_from_row).collect())
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $1 WHERE disconnected_at IS NULL").bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region EventLog
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = Uuid::now_v7().to_string();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::to_value(&event.body).map_err(backend)?;
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let row: (i64,) = sqlx_core::query_as::query_as(
                "INSERT INTO hub_directory_event (id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10) RETURNING seq",
            )
            .bind(&id)
            .bind(event.hlc.physical_ms)
            .bind(event.hlc.logical as i64)
            .bind(actor_kind_to_str(event.actor.kind))
            .bind(&event.actor.id)
            .bind(&event.space_id)
            .bind(&event.user_id)
            .bind(&kind)
            .bind(&payload_value)
            .bind(recorded_at_ms)
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
            let full = DirectoryEvent { seq: row.0 as u64, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            self.project(&mut tx, &full).await?;
            persisted.push(full);
        }
        tx.commit().await.map_err(backend)?;
        Ok(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> = sqlx_core::query_as::query_as(
            "SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at
             FROM hub_directory_event WHERE seq > $1 ORDER BY seq LIMIT $2",
        )
        .bind(since_seq as i64)
        .bind(limit as i64)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(event_from_row).collect()
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let row: (Option<i64>,) = sqlx_core::query_as::query_as("SELECT MAX(seq) FROM hub_directory_event").fetch_one(&self.pool).await.map_err(backend)?;
        Ok(row.0.unwrap_or(0) as u64)
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space_membership").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_user").execute(&mut *tx).await.map_err(backend)?;
        let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> =
            sqlx_core::query_as::query_as("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event ORDER BY seq").fetch_all(&mut *tx).await.map_err(backend)?;
        let events = rows.into_iter().map(event_from_row).collect::<DirectoryResult<Vec<_>>>()?;
        let mut replayed = 0u64;
        for event in &events {
            self.project(&mut tx, event).await?;
            replayed += 1;
        }
        tx.commit().await.map_err(backend)?;
        Ok(replayed)
    }
    //#endregion
}

fn user_from_row(row: (String, String, String, Option<String>, Option<String>, Option<String>, i64)) -> UserRecord {
    let (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) = row;
    UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }
}

fn invite_from_row(row: (String, String, String, String, i64, i64, Option<i64>)) -> InviteRecord {
    let (id, token, space_id, role, created_at, expires_at, revoked_at) = row;
    InviteRecord { id, token, space_id, role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator), created_at, expires_at, revoked_at }
}

fn sync_session_from_row(row: (String, String, String, String, Option<String>, Option<String>, String, i64, Option<i64>)) -> SyncSessionRecord {
    let (id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at) = row;
    SyncSessionRecord { id, space_id, document_id, surface, user_id, space_role: space_role.and_then(|r| SpaceRole::parse(&r)), client_label, connected_at, disconnected_at }
}

fn event_from_row(row: (i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)) -> DirectoryResult<DirectoryEvent> {
    let (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at_ms) = row;
    let body: DirectoryEventBody = serde_json::from_value(payload).map_err(backend)?;
    Ok(DirectoryEvent { seq: seq as u64, id, hlc: Hlc { physical_ms: hlc_physical, logical: hlc_logical as u32 }, actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: actor_id }, space_id, user_id, body, recorded_at_ms })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use testcontainers_modules::postgres::Postgres;
    use testcontainers_modules::testcontainers;
    use testcontainers_modules::testcontainers::runners::AsyncRunner;

    async fn test_directory() -> (PostgresDirectory, testcontainers::ContainerAsync<Postgres>) {
        let container = Postgres::default().start().await.expect("start postgres container");
        let port = container.get_host_port_ipv4(5432).await.expect("port");
        let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
        let directory = PostgresDirectory::connect(&url).await.expect("connect");
        (directory, container)
    }

    fn actor(id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
    }

    /// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
    /// `append_events` — see the module root's `//#region 🔖️Decider`); rebuilds just enough of a
    /// `create-space` decision by hand so these backend tests do not need a full `DirectoryService`.
    async fn seed_space(dir: &PostgresDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
        let space_id = Uuid::now_v7().to_string();
        let owner_role = if kind == DirectorySpaceKind::Archive { DirectorySpaceRole::Spectator } else { DirectorySpaceRole::Author };
        let events = vec![
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(owner_user_id),
                space_id: Some(space_id.clone()),
                user_id: Some(owner_user_id.to_string()),
                body: DirectoryEventBody::SpaceCreated { space_id: space_id.clone(), name: "Space".into(), space_kind: kind, visibility: DirectorySpaceVisibility::Private, owner_user_id: owner_user_id.to_string() },
            },
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(owner_user_id),
                space_id: Some(space_id.clone()),
                user_id: Some(owner_user_id.to_string()),
                body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: owner_user_id.to_string(), role: owner_role },
            },
        ];
        dir.append_events(&events).await.expect("seed space");
        space_id
    }

    // 🔬️ Users, spaces, and role-based membership round-trip against a real Postgres.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let (directory, _container) = test_directory().await;
        let mut clock = HubClock::new();
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
        assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));
    }

    // 🔬️ Schema bootstrap + seed grow a default space the seed user authors — proves the DDL
    // (`hub_space`/`hub_space_membership`, `author`/`spectator` role CHECK) matches this crate's
    // own queries against a real Postgres instance.
    #[tokio::test]
    async fn seed_creates_default_space_and_membership() {
        let (directory, _container) = test_directory().await;
        directory.seed().await.expect("seed");
        let space = directory.get_space("default").await.unwrap().expect("default space");
        assert_eq!(space.kind, "studio");
        assert_eq!(space.visibility, "private");
        assert_eq!(directory.get_role("default", "seed").await.unwrap(), Some(SpaceRole::Author));
    }

    // 🔬️ Event log replay reproduces the same projections against a real Postgres, mirroring the
    // sqlite backend's `event_log_replay_matches_projections`.
    #[tokio::test]
    async fn event_log_replay_matches_projections() {
        let (directory, _container) = test_directory().await;
        directory.seed().await.expect("seed");
        let head = directory.head_seq().await.expect("head seq");
        let before = directory.get_space("default").await.unwrap();
        let replayed = directory.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, head);
        assert_eq!(directory.get_space("default").await.unwrap(), before);
    }
}
//#endregion 🧪️Tests
