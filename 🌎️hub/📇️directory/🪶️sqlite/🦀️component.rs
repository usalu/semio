//! 🪶️ `HubDirectory` over SQLite (rusqlite) — the zero-touch default for local dev and single-user
//! self-hosting; no external database service required. `#[cfg(feature = "sqlite")]`-gated as a
//! whole by the parent `directory` module (see `📇️directory/🦀️component.rs`'s `//#region 🔖️Backends`).
//!
//! Uses synchronous `rusqlite` behind `Arc<Mutex<Connection>>`, not an async SQLite driver: a
//! Cargo workspace may link only one native `sqlite3` (`links = "sqlite3"`), and `rusqlite` is
//! already the sqlite binding used elsewhere in this workspace (`vcs`, `db_storage_sqlite`,
//! semio_compose_rs's unrelated client lib) — adding `sqlx-sqlite`'s `libsqlite3-sys` alongside it
//! is a hard `cargo` resolution conflict, not a style choice. Trait methods stay `async fn`
//! (satisfying the shared `HubDirectory` interface) but their bodies are synchronous rusqlite
//! calls: queries are short, the mutex guard is never held across an `.await`, so nothing here
//! blocks the executor for longer than a real query takes.

use crate::directory::error::{DirectoryError, DirectoryResult};
use crate::directory::model::*;
use crate::directory::{kind_to_str, role_from_wire, visibility_to_str, HubClock, HubDirectory, NewDirectoryEvent};
use async_trait::async_trait;
use directory::os_directory::{DirectoryActor, DirectoryActorKind, DirectoryEvent, DirectoryEventBody, DirectorySpaceKind, DirectorySpaceRole, DirectorySpaceVisibility, Hlc};
use rusqlite::{Connection, OptionalExtension, Transaction};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

//#region 🔖️Schema
const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS share_token (
    token TEXT PRIMARY KEY,
    document_id TEXT NOT NULL,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_user (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    password_hash TEXT,
    sso_subject TEXT,
    sso_provider TEXT,
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_space (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    owner_user_id TEXT NOT NULL REFERENCES hub_user(id),
    created_at INTEGER NOT NULL,
    kind TEXT NOT NULL CHECK (kind IN ('atelier', 'studio', 'archive')),
    visibility TEXT NOT NULL CHECK (visibility IN ('private', 'public'))
);
CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    PRIMARY KEY (space_id, user_id)
);
CREATE TABLE IF NOT EXISTS hub_auth_session (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
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
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE TABLE IF NOT EXISTS hub_space_invite (
    id TEXT PRIMARY KEY,
    token TEXT NOT NULL UNIQUE,
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    role TEXT NOT NULL CHECK (role IN ('author', 'spectator')),
    created_at INTEGER NOT NULL,
    expires_at INTEGER NOT NULL,
    revoked_at INTEGER
);
CREATE TABLE IF NOT EXISTS hub_directory_event (
    seq INTEGER PRIMARY KEY AUTOINCREMENT,
    id TEXT NOT NULL UNIQUE,
    hlc_physical INTEGER NOT NULL,
    hlc_logical INTEGER NOT NULL,
    actor_kind TEXT NOT NULL,
    actor_id TEXT NOT NULL,
    space_id TEXT,
    user_id TEXT,
    kind TEXT NOT NULL,
    payload TEXT NOT NULL,
    recorded_at INTEGER NOT NULL
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

/// @emoji 🗄️ SQLite-backed `HubDirectory`. One `rusqlite::Connection` behind a `Mutex` — see this
/// module's own doc for why this isn't an async SQLite driver.
pub struct SqliteDirectory {
    conn: Arc<Mutex<Connection>>,
}

impl SqliteDirectory {
    /// @emoji 🔌️ Opens (creating if absent) the SQLite database at `path` and bootstraps the schema.
    /// `path` may be `:memory:` for tests.
    pub async fn connect(path: &str) -> DirectoryResult<Self> {
        let conn = Connection::open(path).map_err(backend)?;
        conn.execute_batch(SCHEMA).map_err(backend)?;
        Ok(Self { conn: Arc::new(Mutex::new(conn)) })
    }

    fn lock(&self) -> DirectoryResult<std::sync::MutexGuard<'_, Connection>> {
        self.conn.lock().map_err(|_| DirectoryError::Backend("sqlite connection lock poisoned".into()))
    }

    /// @emoji 🌱️ Seeds a placeholder `seed` system user and a default `studio`/`private` space it
    /// owns, through the event log (`user.created` + `space.created` + `member.upserted`) like any
    /// other write — the system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6). Document existence itself is
    /// `db::Database`'s concern (see `bin.rs`), not seeded here.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'", [], |row| row.get(0)).map_err(backend)?;
        if user_exists > 0 {
            return Ok(());
        }
        let actor = DirectoryActor { kind: DirectoryActorKind::System, id: "system:seed".into() };
        let mut clock = HubClock::new();
        let events = vec![
            NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor.clone(),
                space_id: None,
                user_id: Some("seed".into()),
                body: DirectoryEventBody::UserCreated { user_id: "seed".into(), email: "seed@localhost".into(), display_name: "System".into() },
            },
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
    /// applies one already-persisted `DirectoryEvent`'s effect inside the same transaction
    /// `append_events`/`rebuild_projections` run in. Unconditional: by the time an event exists in
    /// the log, `decide` (`../🦀️component.rs`) already enforced every law, so this never rejects.
    fn project(&self, tx: &Transaction<'_>, event: &DirectoryEvent) -> DirectoryResult<()> {
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_user (id, email, display_name, created_at) VALUES (?1, ?2, ?3, ?4)",
                    rusqlite::params![user_id, email, display_name, event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![space_id, name, owner_user_id, event.recorded_at_ms, kind_to_str(*space_kind), visibility_to_str(*visibility)],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                tx.execute("UPDATE hub_space SET name = ?2 WHERE id = ?1", rusqlite::params![space_id, name]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                tx.execute("UPDATE hub_space SET visibility = ?2 WHERE id = ?1", rusqlite::params![space_id, visibility_to_str(*visibility)]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                tx.execute("UPDATE hub_space SET kind = 'archive' WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space_invite WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id]).map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
        }
        Ok(())
    }
    //#endregion 🔖️Projections
}

#[async_trait]
impl HubDirectory for SqliteDirectory {
    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        let token = Uuid::now_v7().to_string();
        self.lock()?.execute("INSERT INTO share_token (token, document_id, created_at) VALUES (?1, ?2, ?3)", rusqlite::params![token, document_id, now_ms()]).map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool> {
        let conn = self.lock()?;
        let has_tokens: i64 = conn.query_row("SELECT COUNT(*) FROM share_token WHERE document_id = ?1", [document_id], |row| row.get(0)).map_err(backend)?;
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: i64 = conn.query_row("SELECT COUNT(*) FROM share_token WHERE document_id = ?1 AND token = ?2", rusqlite::params![document_id, token], |row| row.get(0)).map_err(backend)?;
                Ok(valid > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(&self, email: &str, display_name: &str, password_hash: Option<&str>, sso_subject: Option<&str>, sso_provider: Option<&str>) -> DirectoryResult<UserRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.lock()?
            .execute(
                "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                rusqlite::params![id, email, display_name, password_hash, sso_subject, sso_provider, created_at],
            )
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
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE id = ?1", [user_id], user_row).optional().map_err(backend)
    }

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?.query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1", [email], user_row).optional().map_err(backend)
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?
            .query_row("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2", rusqlite::params![provider, subject], user_row)
            .optional()
            .map_err(backend)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], user_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region Spaces
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        self.lock()?
            .query_row("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space WHERE id = ?1", [space_id], |row| {
                Ok(SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? })
            })
            .optional()
            .map_err(backend)
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility, m.role FROM hub_space s
                 JOIN hub_space_membership m ON m.space_id = s.id WHERE m.user_id = ?1 ORDER BY s.created_at",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([user_id], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, row.get::<_, String>(6)?))
            })
            .map_err(backend)?;
        Ok(rows
            .filter_map(|row| row.ok())
            .filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role)))
            .collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt
            .query_map(rusqlite::params![limit, offset], |row| {
                Ok(SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? })
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
                 FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id WHERE m.space_id = ?1 ORDER BY m.created_at",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([space_id], |row| {
                let role: String = row.get(7)?;
                Ok((UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? }, role))
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).filter_map(|(user, role)| SpaceRole::parse(&role).map(|role| (user, role))).collect())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let role: Option<String> = self.lock()?.query_row("SELECT role FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id], |row| row.get(0)).optional().map_err(backend)?;
        Ok(role.and_then(|r| SpaceRole::parse(&r)))
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.lock()?.execute("INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES (?1, ?2, ?3, ?4, ?5)", rusqlite::params![id, user_id, created_at, expires_at, sso_provider]).map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>> {
        self.lock()?
            .query_row("SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = ?1", [id], |row| {
                Ok(AuthSessionRecord { id: row.get(0)?, user_id: row.get(1)?, created_at: row.get(2)?, expires_at: row.get(3)?, sso_provider: row.get(4)? })
            })
            .optional()
            .map_err(backend)
    }

    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("DELETE FROM hub_auth_session WHERE id = ?1", [id]).map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region Invites
    async fn create_invite(&self, space_id: &str, role: SpaceRole, ttl_secs: i64) -> DirectoryResult<InviteRecord> {
        let id = Uuid::now_v7().to_string();
        let token = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.lock()?
            .execute("INSERT INTO hub_space_invite (id, token, space_id, role, created_at, expires_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)", rusqlite::params![id, token, space_id, role.as_str(), created_at, expires_at])
            .map_err(backend)?;
        Ok(InviteRecord { id, token, space_id: space_id.to_string(), role, created_at, expires_at, revoked_at: None })
    }

    async fn get_invite_by_token(&self, token: &str) -> DirectoryResult<Option<InviteRecord>> {
        self.lock()?.query_row("SELECT id, token, space_id, role, created_at, expires_at, revoked_at FROM hub_space_invite WHERE token = ?1", [token], invite_row).optional().map_err(backend)
    }

    async fn revoke_invite(&self, invite_id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_space_invite SET revoked_at = ?2 WHERE id = ?1", rusqlite::params![invite_id, now_ms()]).map_err(backend)?;
        Ok(())
    }

    async fn list_invites(&self, space_id: &str) -> DirectoryResult<Vec<InviteRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, token, space_id, role, created_at, expires_at, revoked_at FROM hub_space_invite WHERE space_id = ?1 ORDER BY created_at DESC").map_err(backend)?;
        let rows = stmt.query_map([space_id], invite_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(&self, space_id: &str, document_id: &str, surface: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str());
        self.lock()?
            .execute(
                "INSERT INTO hub_sync_session (id, space_id, document_id, surface, user_id, space_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                rusqlite::params![id, space_id, document_id, surface, user_id, role_str, client_label, connected_at],
            )
            .map_err(backend)?;
        Ok(SyncSessionRecord { id, space_id: space_id.to_string(), document_id: document_id.to_string(), surface: surface.to_string(), user_id: user_id.map(str::to_string), space_role, client_label: client_label.to_string(), connected_at, disconnected_at: None })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1", rusqlite::params![sync_session_id, now_ms()]).map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE document_id = ?1 ORDER BY connected_at DESC")
            .map_err(backend)?;
        let rows = stmt.query_map([document_id], sync_session_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_active_sync_sessions(&self, space_id: Option<&str>) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let rows: Vec<SyncSessionRecord> = match space_id {
            Some(space_id) => {
                let mut stmt = conn
                    .prepare("SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE space_id = ?1 AND disconnected_at IS NULL ORDER BY connected_at DESC")
                    .map_err(backend)?;
                let mapped = stmt.query_map([space_id], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
            None => {
                let mut stmt = conn
                    .prepare("SELECT id, space_id, document_id, surface, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE disconnected_at IS NULL ORDER BY connected_at DESC")
                    .map_err(backend)?;
                let mapped = stmt.query_map([], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
        };
        Ok(rows)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?1 WHERE disconnected_at IS NULL", rusqlite::params![now_ms()]).map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region EventLog
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = Uuid::now_v7().to_string();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::to_value(&event.body).map_err(backend)?;
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            tx.execute(
                "INSERT INTO hub_directory_event (id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
                rusqlite::params![id, event.hlc.physical_ms, event.hlc.logical, actor_kind_to_str(event.actor.kind), event.actor.id, event.space_id, event.user_id, kind, payload_value.to_string(), recorded_at_ms],
            )
            .map_err(backend)?;
            let seq = tx.last_insert_rowid() as u64;
            let full = DirectoryEvent { seq, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            self.project(&tx, &full)?;
            persisted.push(full);
        }
        tx.commit().map_err(backend)?;
        Ok(persisted)
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > ?1 ORDER BY seq LIMIT ?2")
            .map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![since_seq as i64, limit as i64], event_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let seq: Option<i64> = self.lock()?.query_row("SELECT MAX(seq) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        Ok(seq.unwrap_or(0) as u64)
    }

    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        tx.execute_batch("DELETE FROM hub_space_membership; DELETE FROM hub_space; DELETE FROM hub_user;").map_err(backend)?;
        let events: Vec<DirectoryEvent> = {
            let mut stmt = tx
                .prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event ORDER BY seq")
                .map_err(backend)?;
            let mapped = stmt.query_map([], event_row).map_err(backend)?;
            let collected: Result<Vec<_>, _> = mapped.collect();
            collected.map_err(backend)?
        };
        let mut replayed = 0u64;
        for event in &events {
            self.project(&tx, event)?;
            replayed += 1;
        }
        tx.commit().map_err(backend)?;
        Ok(replayed)
    }
    //#endregion
}

fn user_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? })
}

fn sync_session_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<SyncSessionRecord> {
    let space_role: Option<String> = row.get(5)?;
    Ok(SyncSessionRecord {
        id: row.get(0)?,
        space_id: row.get(1)?,
        document_id: row.get(2)?,
        surface: row.get(3)?,
        user_id: row.get(4)?,
        space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
        client_label: row.get(6)?,
        connected_at: row.get(7)?,
        disconnected_at: row.get(8)?,
    })
}

fn invite_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<InviteRecord> {
    let role: String = row.get(3)?;
    Ok(InviteRecord { id: row.get(0)?, token: row.get(1)?, space_id: row.get(2)?, role: SpaceRole::parse(&role).unwrap_or(SpaceRole::Spectator), created_at: row.get(4)?, expires_at: row.get(5)?, revoked_at: row.get(6)? })
}

fn event_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<DirectoryEvent> {
    let seq: i64 = row.get(0)?;
    let id: String = row.get(1)?;
    let hlc_physical: i64 = row.get(2)?;
    let hlc_logical: i64 = row.get(3)?;
    let actor_kind: String = row.get(4)?;
    let actor_id: String = row.get(5)?;
    let space_id: Option<String> = row.get(6)?;
    let user_id: Option<String> = row.get(7)?;
    let payload: String = row.get(8)?;
    let recorded_at_ms: i64 = row.get(9)?;
    let body: DirectoryEventBody =
        serde_json::from_str(&payload).map_err(|error| rusqlite::Error::FromSqlConversionFailure(8, rusqlite::types::Type::Text, Box::new(error)))?;
    Ok(DirectoryEvent {
        seq: seq as u64,
        id,
        hlc: Hlc { physical_ms: hlc_physical, logical: hlc_logical as u32 },
        actor: DirectoryActor { kind: actor_kind_from_str(&actor_kind), id: actor_id },
        space_id,
        user_id,
        body,
        recorded_at_ms,
    })
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn actor(id: &str) -> DirectoryActor {
        DirectoryActor { kind: DirectoryActorKind::User, id: id.to_string() }
    }

    /// 🌱️ `create_space`/`upsert_membership` were removed (writes now go through
    /// `append_events` — see the module root's `//#region 🔖️Decider`); this recreates just enough
    /// of a `create-space` decision by hand so backend tests do not need a full `DirectoryService`.
    async fn seed_space(dir: &SqliteDirectory, clock: &mut HubClock, owner_user_id: &str, kind: DirectorySpaceKind) -> String {
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

    // 🔬️ Users, spaces, and role-based membership round-trip over the event log.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        let mut clock = HubClock::new();
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let space_id = seed_space(&directory, &mut clock, &user.id, DirectorySpaceKind::Studio).await;
        assert_eq!(directory.get_role(&space_id, &user.id).await.unwrap(), Some(SpaceRole::Author));

        let member = directory.create_user("b@example.com", "Bob", None, None, None).await.expect("create user 2");
        directory
            .append_events(&[NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(&user.id),
                space_id: Some(space_id.clone()),
                user_id: Some(member.id.clone()),
                body: DirectoryEventBody::MemberUpserted { space_id: space_id.clone(), user_id: member.id.clone(), role: DirectorySpaceRole::Spectator },
            }])
            .await
            .expect("add member");
        assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), Some(SpaceRole::Spectator));
        let spaces = directory.list_spaces_for_user(&member.id).await.unwrap();
        assert_eq!(spaces.len(), 1);

        directory
            .append_events(&[NewDirectoryEvent {
                hlc: clock.tick(),
                actor: actor(&user.id),
                space_id: Some(space_id.clone()),
                user_id: Some(member.id.clone()),
                body: DirectoryEventBody::MemberRemoved { space_id: space_id.clone(), user_id: member.id.clone() },
            }])
            .await
            .expect("remove member");
        assert_eq!(directory.get_role(&space_id, &member.id).await.unwrap(), None);
    }

    // 🔬️ SyncSession open/close is durable, listable, filterable by space, and boot-time cleanup
    // closes every still-open session at once.
    #[tokio::test]
    async fn sync_session_lifecycle() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let session = directory.record_sync_session_open("default", "default", "s.space@1/*#editor", None, None, "test-client").await.expect("open");
        assert!(session.disconnected_at.is_none());
        assert_eq!(directory.list_active_sync_sessions(Some("default")).await.unwrap().len(), 1);

        directory.record_sync_session_close(&session.id).await.expect("close");
        let sessions = directory.list_sync_sessions_for_document("default").await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].disconnected_at.is_some());
        assert!(directory.list_active_sync_sessions(Some("default")).await.unwrap().is_empty());

        directory.record_sync_session_open("default", "default", "s.space@1/*#viewer", None, None, "test-client-2").await.expect("open 2");
        directory.close_all_sync_sessions().await.expect("close all");
        assert!(directory.list_active_sync_sessions(None).await.unwrap().is_empty());
    }

    // 🔬️ Share tokens: tokenless is open; once issued, only a valid token authorizes.
    #[tokio::test]
    async fn share_token_gating() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        assert!(directory.authorized_by_token("default", None).await.unwrap());
        let token = directory.create_share_token("default").await.expect("mint token");
        assert!(!directory.authorized_by_token("default", None).await.unwrap());
        assert!(directory.authorized_by_token("default", Some(&token)).await.unwrap());
    }

    // 🔬️ `seed()` (now event-sourced) still leaves a dense, replayable log.
    #[tokio::test]
    async fn seed_is_replayable() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        assert_eq!(directory.head_seq().await.unwrap(), 3);
        let before = directory.get_space("default").await.unwrap();
        let replayed = directory.rebuild_projections().await.expect("rebuild");
        assert_eq!(replayed, 3);
        assert_eq!(directory.get_space("default").await.unwrap(), before);
    }
}
//#endregion 🧪️Tests
