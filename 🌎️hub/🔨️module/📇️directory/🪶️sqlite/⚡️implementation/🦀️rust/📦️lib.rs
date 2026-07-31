mod header {
    // 🧲️Header
    // HubDirectory over SQLite (rusqlite) — the zero-touch default for local dev and single-user
    // self-hosting; no external database service required.
    //
    // Uses synchronous `rusqlite` behind `Arc<Mutex<Connection>>`, not an async SQLite driver:
    // a Cargo workspace may link only one native `sqlite3` (`links = "sqlite3"`), and `rusqlite`
    // is already the sqlite binding used elsewhere in this workspace (`vcs`, `db_storage_sqlite`,
    // compose's unrelated `compose/client/lib/rs`) — adding `sqlx-sqlite`'s `libsqlite3-sys`
    // alongside it is a hard `cargo` resolution conflict, not a style choice. Trait methods stay
    // `async fn` (satisfying the shared `HubDirectory` interface) but their bodies are synchronous
    // rusqlite calls: queries are short, the mutex guard is never held across an `.await`, so
    // nothing here blocks the executor for longer than a real query takes.
}

use async_trait::async_trait;
use os_hub_directory::error::{DirectoryError, DirectoryResult};
use os_hub_directory::model::*;
use os_hub_directory::HubDirectory;
use rusqlite::{Connection, OptionalExtension};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

//#region 🔖️Schema
const SCHEMA: &str = "\
CREATE TABLE IF NOT EXISTS node (
    id TEXT PRIMARY KEY,
    space_id TEXT NOT NULL,
    parent_id TEXT REFERENCES node(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    kind TEXT NOT NULL
);
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
    created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS hub_space_membership (
    space_id TEXT NOT NULL REFERENCES hub_space(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES hub_user(id) ON DELETE CASCADE,
    role TEXT NOT NULL,
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
    document_id TEXT NOT NULL,
    user_id TEXT REFERENCES hub_user(id) ON DELETE SET NULL,
    space_role TEXT,
    client_label TEXT NOT NULL,
    connected_at INTEGER NOT NULL,
    disconnected_at INTEGER
);
CREATE INDEX IF NOT EXISTS idx_membership_user ON hub_space_membership (user_id);
CREATE INDEX IF NOT EXISTS idx_node_space_parent ON node (space_id, parent_id);
CREATE INDEX IF NOT EXISTS idx_sync_session_document ON hub_sync_session (document_id, disconnected_at);
";
//#endregion 🔖️Schema

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
}

/// @emoji 🗄️ SQLite-backed `HubDirectory`. One `rusqlite::Connection` behind a `Mutex` — see
/// `header` for why this isn't an async SQLite driver.
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

    /// @emoji 🌱️ Seeds a placeholder `seed` system user, a default space it owns, and a
    /// `Documents/default` node. The system user satisfies `hub_space.owner_user_id`'s foreign
    /// key until a real bootstrap admin claims ownership through `/admin` (HP-6). Document
    /// existence itself is `db::Database`'s concern (see `os-hub`'s `bin.rs`), not seeded here.
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: i64 =
            self.lock()?.query_row("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'", [], |row| row.get(0)).map_err(backend)?;
        if user_exists == 0 {
            self.lock()?
                .execute(
                    "INSERT INTO hub_user (id, email, display_name, created_at) VALUES ('seed', 'seed@localhost', 'System', ?1)",
                    rusqlite::params![now_ms()],
                )
                .map_err(backend)?;
        }
        let studio_exists: i64 = self
            .lock()?
            .query_row("SELECT COUNT(*) FROM hub_space WHERE id = 'default'", [], |row| row.get(0))
            .map_err(backend)?;
        if studio_exists == 0 {
            self.lock()?
                .execute(
                    "INSERT INTO hub_space (id, name, owner_user_id, created_at) VALUES ('default', 'Space', 'seed', ?1)",
                    rusqlite::params![now_ms()],
                )
                .map_err(backend)?;
        }
        let node_count: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM node", [], |row| row.get(0)).map_err(backend)?;
        if node_count == 0 {
            let folder = self.create_node("default", None, "Documents", "folder").await?;
            self.create_node("default", Some(&folder.id), "default", "document").await?;
        }
        Ok(())
    }
}

#[async_trait]
impl HubDirectory for SqliteDirectory {
    //#region Vfs
    async fn list_nodes(&self, space_id: &str, parent: Option<&str>) -> DirectoryResult<Vec<NodeRecord>> {
        let conn = self.lock()?;
        let row_mapper = |row: &rusqlite::Row| -> rusqlite::Result<NodeRecord> {
            Ok(NodeRecord { id: row.get(0)?, space_id: space_id.to_string(), parent_id: row.get(1)?, name: row.get(2)?, kind: row.get(3)? })
        };
        let rows = match parent {
            Some(parent) => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE space_id = ?1 AND parent_id = ?2 ORDER BY name")
                    .map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![space_id, parent], row_mapper).map_err(backend)?;
                let collected: Vec<_> = mapped.filter_map(|row| row.ok()).collect();
                collected
            }
            None => {
                let mut stmt = conn
                    .prepare("SELECT id, parent_id, name, kind FROM node WHERE space_id = ?1 AND parent_id IS NULL ORDER BY name")
                    .map_err(backend)?;
                let mapped = stmt.query_map([space_id], row_mapper).map_err(backend)?;
                let collected: Vec<_> = mapped.filter_map(|row| row.ok()).collect();
                collected
            }
        };
        Ok(rows)
    }

    async fn create_node(&self, space_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> DirectoryResult<NodeRecord> {
        let id = Uuid::now_v7().to_string();
        self.lock()?
            .execute(
                "INSERT INTO node (id, space_id, parent_id, name, kind) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, space_id, parent_id, name, kind],
            )
            .map_err(backend)?;
        Ok(NodeRecord { id, space_id: space_id.to_string(), parent_id: parent_id.map(str::to_string), name: name.to_string(), kind: kind.to_string() })
    }
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String> {
        let token = Uuid::now_v7().to_string();
        self.lock()?
            .execute(
                "INSERT INTO share_token (token, document_id, created_at) VALUES (?1, ?2, ?3)",
                rusqlite::params![token, document_id, now_ms()],
            )
            .map_err(backend)?;
        Ok(token)
    }

    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool> {
        let conn = self.lock()?;
        let has_tokens: i64 = conn
            .query_row("SELECT COUNT(*) FROM share_token WHERE document_id = ?1", [document_id], |row| row.get(0))
            .map_err(backend)?;
        if has_tokens == 0 {
            return Ok(true);
        }
        match token {
            None => Ok(false),
            Some(token) => {
                let valid: i64 = conn
                    .query_row(
                        "SELECT COUNT(*) FROM share_token WHERE document_id = ?1 AND token = ?2",
                        rusqlite::params![document_id, token],
                        |row| row.get(0),
                    )
                    .map_err(backend)?;
                Ok(valid > 0)
            }
        }
    }
    //#endregion

    //#region Users
    async fn create_user(
        &self,
        email: &str,
        display_name: &str,
        password_hash: Option<&str>,
        sso_subject: Option<&str>,
        sso_provider: Option<&str>,
    ) -> DirectoryResult<UserRecord> {
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

    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?
            .query_row(
                "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE email = ?1",
                [email],
                user_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>> {
        self.lock()?
            .query_row(
                "SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user WHERE sso_provider = ?1 AND sso_subject = ?2",
                rusqlite::params![provider, subject],
                user_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user ORDER BY created_at LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], user_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion

    //#region Spaces
    async fn create_space(&self, name: &str, owner_user_id: &str) -> DirectoryResult<SpaceRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        self.lock()?
            .execute(
                "INSERT INTO hub_space (id, name, owner_user_id, created_at) VALUES (?1, ?2, ?3, ?4)",
                rusqlite::params![id, name, owner_user_id, created_at],
            )
            .map_err(backend)?;
        self.upsert_membership(&id, owner_user_id, SpaceRole::Owner).await?;
        Ok(SpaceRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at })
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, m.role FROM hub_space s
                 JOIN hub_space_membership m ON m.space_id = s.id WHERE m.user_id = ?1 ORDER BY s.created_at",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([user_id], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, String>(4)?,
                ))
            })
            .map_err(backend)?;
        Ok(rows
            .filter_map(|row| row.ok())
            .filter_map(|(id, name, owner_user_id, created_at, role)| {
                SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at }, role))
            })
            .collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, name, owner_user_id, created_at FROM hub_space ORDER BY created_at LIMIT ?1 OFFSET ?2")
            .map_err(backend)?;
        let rows = stmt
            .query_map(rusqlite::params![limit, offset], |row| {
                Ok(SpaceRecord {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    owner_user_id: row.get(2)?,
                    created_at: row.get(3)?,
                })
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn upsert_membership(&self, space_id: &str, user_id: &str, role: SpaceRole) -> DirectoryResult<()> {
        self.lock()?
            .execute(
                "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                 ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                rusqlite::params![space_id, user_id, role.as_str(), now_ms()],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, space_id: &str, user_id: &str) -> DirectoryResult<()> {
        self.lock()?
            .execute(
                "DELETE FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2",
                rusqlite::params![space_id, user_id],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let role: Option<String> = self
            .lock()?
            .query_row(
                "SELECT role FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2",
                rusqlite::params![space_id, user_id],
                |row| row.get(0),
            )
            .optional()
            .map_err(backend)?;
        Ok(role.and_then(|r| SpaceRole::parse(&r)))
    }
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        let expires_at = created_at + ttl_secs * 1000;
        self.lock()?
            .execute(
                "INSERT INTO hub_auth_session (id, user_id, created_at, expires_at, sso_provider) VALUES (?1, ?2, ?3, ?4, ?5)",
                rusqlite::params![id, user_id, created_at, expires_at, sso_provider],
            )
            .map_err(backend)?;
        Ok(AuthSessionRecord { id, user_id: user_id.to_string(), created_at, expires_at, sso_provider: sso_provider.map(str::to_string) })
    }

    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>> {
        self.lock()?
            .query_row(
                "SELECT id, user_id, created_at, expires_at, sso_provider FROM hub_auth_session WHERE id = ?1",
                [id],
                |row| {
                    Ok(AuthSessionRecord {
                        id: row.get(0)?,
                        user_id: row.get(1)?,
                        created_at: row.get(2)?,
                        expires_at: row.get(3)?,
                        sso_provider: row.get(4)?,
                    })
                },
            )
            .optional()
            .map_err(backend)
    }

    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("DELETE FROM hub_auth_session WHERE id = ?1", [id]).map_err(backend)?;
        Ok(())
    }
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(
        &self,
        document_id: &str,
        user_id: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str());
        self.lock()?
            .execute(
                "INSERT INTO hub_sync_session (id, document_id, user_id, space_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                rusqlite::params![id, document_id, user_id, role_str, client_label, connected_at],
            )
            .map_err(backend)?;
        Ok(SyncSessionRecord {
            id,
            document_id: document_id.to_string(),
            user_id: user_id.map(str::to_string),
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.lock()?
            .execute(
                "UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1",
                rusqlite::params![sync_session_id, now_ms()],
            )
            .map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT id, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE document_id = ?1 ORDER BY connected_at DESC",
            )
            .map_err(backend)?;
        let rows = stmt
            .query_map([document_id], |row| {
                let space_role: Option<String> = row.get(2)?;
                Ok(SyncSessionRecord {
                    id: row.get(0)?,
                    document_id: document_id.to_string(),
                    user_id: row.get(1)?,
                    space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
                    client_label: row.get(3)?,
                    connected_at: row.get(4)?,
                    disconnected_at: row.get(5)?,
                })
            })
            .map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }
    //#endregion
}

fn user_row(row: &rusqlite::Row) -> rusqlite::Result<UserRecord> {
    Ok(UserRecord {
        id: row.get(0)?,
        email: row.get(1)?,
        display_name: row.get(2)?,
        password_hash: row.get(3)?,
        sso_subject: row.get(4)?,
        sso_provider: row.get(5)?,
        created_at: row.get(6)?,
    })
}

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    // 🔬️ Users, studios, and role-based membership round-trip.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let studio = directory.create_space("Space A", &user.id).await.expect("create space");
        assert_eq!(directory.get_role(&studio.id, &user.id).await.unwrap(), Some(SpaceRole::Owner));
        let member = directory.create_user("b@example.com", "Bob", None, None, None).await.expect("create user 2");
        directory.upsert_membership(&studio.id, &member.id, SpaceRole::Viewer).await.expect("add member");
        assert_eq!(directory.get_role(&studio.id, &member.id).await.unwrap(), Some(SpaceRole::Viewer));
        let studios = directory.list_spaces_for_user(&member.id).await.unwrap();
        assert_eq!(studios.len(), 1);
        directory.remove_membership(&studio.id, &member.id).await.expect("remove");
        assert_eq!(directory.get_role(&studio.id, &member.id).await.unwrap(), None);
    }

    // 🔬️ SyncSession open/close is durable and listable.
    #[tokio::test]
    async fn sync_session_lifecycle() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let session = directory.record_sync_session_open("default", None, None, "test-client").await.expect("open");
        assert!(session.disconnected_at.is_none());
        directory.record_sync_session_close(&session.id).await.expect("close");
        let sessions = directory.list_sync_sessions_for_document("default").await.unwrap();
        assert_eq!(sessions.len(), 1);
        assert!(sessions[0].disconnected_at.is_some());
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

    // 🔬️ VFS nodes are durable and creatable, seeded with a Documents/default tree.
    #[tokio::test]
    async fn nodes_seeded_and_creatable() {
        let directory = SqliteDirectory::connect(":memory:").await.expect("connect");
        directory.seed().await.expect("seed");
        let roots = directory.list_nodes("default", None).await.unwrap();
        assert_eq!(roots.len(), 1);
        assert_eq!(roots[0].name, "Documents");
        let children = directory.list_nodes("default", Some(&roots[0].id)).await.unwrap();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].name, "default");
    }
}
//#endregion 🔖️Tests
