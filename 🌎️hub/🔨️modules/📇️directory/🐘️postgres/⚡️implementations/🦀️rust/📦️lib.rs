mod header {
    // 🧲️Header
    // HubDirectory over PostgreSQL — direct `sqlx-postgres`/`sqlx-core` (not the `sqlx` facade),
    // matching the exact precedent in `semio_compose_rs/server/semio_hub/rs/bin.rs`. The scale-out backend for
    // multi-node self-hosted deployments; schema bootstrap is `directory/postgres/🛢️schema.sql`.
}

use async_trait::async_trait;
use os_hub_directory::error::{DirectoryError, DirectoryResult};
use os_hub_directory::model::*;
use os_hub_directory::HubDirectory;
pub use sqlx_core::row::Row;
pub use sqlx_postgres::{PgPool, PgPoolOptions};
use uuid::Uuid;

const SCHEMA: &str = include_str!("../../../🐘️postgres/⚡️implementations/🦀️rust/🛢️schema.sql");

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_millis() as i64)
}

fn backend<E: std::fmt::Display>(err: E) -> DirectoryError {
    DirectoryError::Backend(err.to_string())
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
    /// owns. The system user satisfies `hub_space.owner_user_id`'s foreign key until a real
    /// bootstrap admin claims ownership through `/admin` (HP-6).
    pub async fn seed(&self) -> DirectoryResult<()> {
        let user_exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_user WHERE id = 'seed'").fetch_one(&self.pool).await.map_err(backend)?;
        if user_exists.0 == 0 {
            sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, created_at) VALUES ('seed', 'seed@localhost', 'System', $1)").bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        }
        let exists: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_space WHERE id = 'default'").fetch_one(&self.pool).await.map_err(backend)?;
        if exists.0 == 0 {
            sqlx_core::query::query("INSERT INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES ('default', 'Space', 'seed', $1, 'studio', 'private')").bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
            self.upsert_membership("default", "seed", SpaceRole::Author).await?;
        }
        Ok(())
    }
}

#[async_trait]
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
    async fn create_space(&self, name: &str, owner_user_id: &str, kind: &str, visibility: &str) -> DirectoryResult<SpaceRecord> {
        let id = Uuid::now_v7().to_string();
        let created_at = now_ms();
        sqlx_core::query::query("INSERT INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&id)
            .bind(name)
            .bind(owner_user_id)
            .bind(created_at)
            .bind(kind)
            .bind(visibility)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        // 🎯️ An archive is frozen — even its owner is a spectator, never an author (matches
        // `upsert_membership`'s own archive-rejects-author law below).
        let owner_role = if kind == "archive" { SpaceRole::Spectator } else { SpaceRole::Author };
        self.upsert_membership(&id, owner_user_id, owner_role).await?;
        Ok(SpaceRecord { id, name: name.to_string(), owner_user_id: owner_user_id.to_string(), created_at, kind: kind.to_string(), visibility: visibility.to_string() })
    }

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
        Ok(rows
            .into_iter()
            .filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role)))
            .collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let rows: Vec<(String, String, String, i64, String, String)> =
            sqlx_core::query_as::query_as("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT $1 OFFSET $2").bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(backend)?;
        Ok(rows.into_iter().map(|(id, name, owner_user_id, created_at, kind, visibility)| SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }).collect())
    }

    async fn upsert_membership(&self, space_id: &str, user_id: &str, role: SpaceRole) -> DirectoryResult<()> {
        if role == SpaceRole::Author {
            if let Some(space) = self.get_space(space_id).await? {
                if space.kind == "archive" {
                    return Err(DirectoryError::Conflict(format!("space '{space_id}' is an archive; no author memberships are allowed")));
                }
                if space.kind == "atelier" {
                    let other_authors: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_space_membership WHERE space_id = $1 AND role = 'author' AND user_id != $2")
                        .bind(space_id)
                        .bind(user_id)
                        .fetch_one(&self.pool)
                        .await
                        .map_err(backend)?;
                    if other_authors.0 > 0 {
                        return Err(DirectoryError::Conflict(format!("space '{space_id}' is an atelier; it already has a distinct author")));
                    }
                }
            }
        }
        sqlx_core::query::query(
            "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4)
             ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3",
        )
        .bind(space_id)
        .bind(user_id)
        .bind(role.as_str())
        .bind(now_ms())
        .execute(&self.pool)
        .await
        .map_err(backend)?;
        Ok(())
    }

    async fn remove_membership(&self, space_id: &str, user_id: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).execute(&self.pool).await.map_err(backend)?;
        Ok(())
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

    //#region SyncSessions
    async fn record_sync_session_open(&self, document_id: &str, user_id: Option<&str>, space_role: Option<SpaceRole>, client_label: &str) -> DirectoryResult<SyncSessionRecord> {
        let id = Uuid::now_v7().to_string();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str());
        sqlx_core::query::query("INSERT INTO hub_sync_session (id, document_id, user_id, space_role, client_label, connected_at) VALUES ($1, $2, $3, $4, $5, $6)")
            .bind(&id)
            .bind(document_id)
            .bind(user_id)
            .bind(role_str)
            .bind(client_label)
            .bind(connected_at)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(SyncSessionRecord { id, document_id: document_id.to_string(), user_id: user_id.map(str::to_string), space_role, client_label: client_label.to_string(), connected_at, disconnected_at: None })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $2 WHERE id = $1").bind(sync_session_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let rows: Vec<(String, Option<String>, Option<String>, String, i64, Option<i64>)> = sqlx_core::query_as::query_as(
            "SELECT id, user_id, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
             WHERE document_id = $1 ORDER BY connected_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows
            .into_iter()
            .map(|(id, user_id, space_role, client_label, connected_at, disconnected_at)| SyncSessionRecord {
                id,
                document_id: document_id.to_string(),
                user_id,
                space_role: space_role.and_then(|r| SpaceRole::parse(&r)),
                client_label,
                connected_at,
                disconnected_at,
            })
            .collect())
    }
    //#endregion
}

fn user_from_row(row: (String, String, String, Option<String>, Option<String>, Option<String>, i64)) -> UserRecord {
    let (id, email, display_name, password_hash, sso_subject, sso_provider, created_at) = row;
    UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }
}

//#region 🔖️Tests
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

    // 🔬️ Users, studios, and role-based membership round-trip against a real Postgres.
    #[tokio::test]
    async fn user_space_membership_round_trip() {
        let (directory, _container) = test_directory().await;
        let user = directory.create_user("a@example.com", "Ada", None, None, None).await.expect("create user");
        let studio = directory.create_space("Space A", &user.id, "studio", "private").await.expect("create space");
        assert_eq!(directory.get_role(&studio.id, &user.id).await.unwrap(), Some(SpaceRole::Author));
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

    // 🔬️ The postgres DDL (`🛢️schema.sql`) matches this crate's own queries: archive/atelier
    // membership laws round-trip against a real Postgres, same as the sqlite backend's law test.
    #[tokio::test]
    async fn space_kind_membership_laws_are_enforced() {
        let (directory, _container) = test_directory().await;
        let owner = directory.create_user("owner@example.com", "Owner", None, None, None).await.expect("create user");
        let other = directory.create_user("other@example.com", "Other", None, None, None).await.expect("create user 2");

        let archive = directory.create_space("Archive", &owner.id, "archive", "private").await.expect("create archive");
        let err = directory.upsert_membership(&archive.id, &owner.id, SpaceRole::Author).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));

        let atelier = directory.create_space("Atelier", &owner.id, "atelier", "private").await.expect("create atelier");
        directory.upsert_membership(&atelier.id, &owner.id, SpaceRole::Author).await.expect("re-upsert sole author");
        let err = directory.upsert_membership(&atelier.id, &other.id, SpaceRole::Author).await.unwrap_err();
        assert!(matches!(err, DirectoryError::Conflict(_)));

        let studio = directory.create_space("Studio", &owner.id, "studio", "private").await.expect("create studio");
        directory.upsert_membership(&studio.id, &other.id, SpaceRole::Author).await.expect("studio allows many authors");
    }
}
//#endregion 🔖️Tests
