mod header {
    // 🧲️Header
    // Backend-agnostic os-hub identity/tenancy directory. HubDirectory is the single seam every
    // persistence backend (sqlite/postgres/neo4j) implements; bin.rs never sees a driver type
    // (sqlx/neo4rs/rusqlite), only this trait and the DTOs in `model` — satisfies the "external
    // libraries stay behind an interface" rule for a trait three backends must share.
    //
    // 🎯️ Design choice (split from the pre-CW6 `HubStorage`): document persistence (snapshots,
    // operations) and content-addressed blobs are no longer this crate's concern — `db::Database`
    // (server-side document authority) and `db`'s own `PayloadStorage` own that now (see
    // `os-hub`'s `bin.rs`). This crate keeps exactly the identity/tenancy surface that has no `db`
    // counterpart: users, spaces, memberships, auth sessions, share tokens, VFS nodes, and
    // realtime sync sessions.
}

//#region 🔖️Error
pub mod error {
    /// @emoji 🧯️ Opaque directory error — never wraps a backend driver's error type, so no `sqlx`/
    /// `neo4rs`/`rusqlite` type ever crosses this crate's public API.
    #[derive(Debug, thiserror::Error)]
    pub enum DirectoryError {
        #[error("not found: {0}")]
        NotFound(String),
        #[error("conflict: {0}")]
        Conflict(String),
        #[error("unauthorized")]
        Unauthorized,
        #[error("backend error: {0}")]
        Backend(String),
    }

    pub type DirectoryResult<T> = Result<T, DirectoryError>;
}
//#endregion 🔖️Error

//#region 🔖️Model
pub mod model {
    use serde::{Deserialize, Serialize};

    /// @emoji 🗂️ VFS tree entry, scoped to a space.
    #[derive(Clone, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodeRecord {
        pub id: String,
        pub space_id: String,
        pub parent_id: Option<String>,
        pub name: String,
        pub kind: String,
    }

    /// @emoji 🔗️ An anonymous per-document bearer token (existing auth-lite scheme, kept as-is).
    /// `document_id` is opaque here — the directory has no FK relationship to a `db::Database`
    /// document; it never persists document content itself.
    pub struct ShareTokenRecord {
        pub token: String,
        pub document_id: String,
        pub created_at: i64,
    }

    /// @emoji 🙋️ A platform user — local password login and/or one linked SSO identity.
    pub struct UserRecord {
        pub id: String,
        pub email: String,
        pub display_name: String,
        pub password_hash: Option<String>,
        pub sso_subject: Option<String>,
        pub sso_provider: Option<String>,
        pub created_at: i64,
    }

    /// @emoji 🏛️ A space: the tenant/workspace unit that owns documents, nodes, and memberships.
    pub struct SpaceRecord {
        pub id: String,
        pub name: String,
        pub owner_user_id: String,
        pub created_at: i64,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum SpaceRole {
        Owner,
        Member,
        Viewer,
    }

    impl SpaceRole {
        pub fn as_str(&self) -> &'static str {
            match self {
                SpaceRole::Owner => "owner",
                SpaceRole::Member => "member",
                SpaceRole::Viewer => "viewer",
            }
        }

        pub fn parse(value: &str) -> Option<Self> {
            match value {
                "owner" => Some(SpaceRole::Owner),
                "member" => Some(SpaceRole::Member),
                "viewer" => Some(SpaceRole::Viewer),
                _ => None,
            }
        }
    }

    pub struct SpaceMembershipRecord {
        pub space_id: String,
        pub user_id: String,
        pub role: SpaceRole,
        pub created_at: i64,
    }

    /// @emoji 🍪️ A browser login session (distinct from {@link SyncSessionRecord}'s realtime connection).
    pub struct AuthSessionRecord {
        pub id: String,
        pub user_id: String,
        pub created_at: i64,
        pub expires_at: i64,
        pub sso_provider: Option<String>,
    }

    /// @emoji 🔴️ A realtime document connection — the "session as live-features backend" record;
    /// written by `os-hub`'s wire-v2 WS handler on Hello/disconnect, not per-operation.
    pub struct SyncSessionRecord {
        pub id: String,
        pub document_id: String,
        pub user_id: Option<String>,
        pub space_role: Option<SpaceRole>,
        pub client_label: String,
        pub connected_at: i64,
        pub disconnected_at: Option<i64>,
    }
}
//#endregion 🔖️Model

//#region 🔖️Trait
use error::DirectoryResult;
use model::*;

/// @emoji 🗄️ Backend-agnostic os-hub identity/tenancy directory. Implemented once per backend
/// (sqlite/postgres/neo4j); `HubState` holds an `Arc<dyn HubDirectory>` so the directory backend is
/// a deploy-time choice, not a compile-time one — independent of `db::Database`'s own storage
/// backend choice (see `os-hub`'s `bin.rs`, `OS_HUB_DIRECTORY_BACKEND` vs `OS_HUB_STORAGE_BACKEND`).
#[async_trait::async_trait]
pub trait HubDirectory: Send + Sync + 'static {
    //#region Vfs
    async fn list_nodes(&self, space_id: &str, parent: Option<&str>) -> DirectoryResult<Vec<NodeRecord>>;
    async fn create_node(&self, space_id: &str, parent_id: Option<&str>, name: &str, kind: &str) -> DirectoryResult<NodeRecord>;
    //#endregion

    //#region ShareTokens
    async fn create_share_token(&self, document_id: &str) -> DirectoryResult<String>;
    async fn authorized_by_token(&self, document_id: &str, token: Option<&str>) -> DirectoryResult<bool>;
    //#endregion

    //#region Users
    async fn create_user(
        &self,
        email: &str,
        display_name: &str,
        password_hash: Option<&str>,
        sso_subject: Option<&str>,
        sso_provider: Option<&str>,
    ) -> DirectoryResult<UserRecord>;
    async fn get_user_by_email(&self, email: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn get_user_by_sso_subject(&self, provider: &str, subject: &str) -> DirectoryResult<Option<UserRecord>>;
    async fn list_users(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<UserRecord>>;
    //#endregion

    //#region Spaces
    async fn create_space(&self, name: &str, owner_user_id: &str) -> DirectoryResult<SpaceRecord>;
    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>>;
    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>>;
    async fn upsert_membership(&self, space_id: &str, user_id: &str, role: SpaceRole) -> DirectoryResult<()>;
    async fn remove_membership(&self, space_id: &str, user_id: &str) -> DirectoryResult<()>;
    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>>;
    //#endregion

    //#region AuthSessions
    async fn create_auth_session(&self, user_id: &str, ttl_secs: i64, sso_provider: Option<&str>) -> DirectoryResult<AuthSessionRecord>;
    async fn get_auth_session(&self, id: &str) -> DirectoryResult<Option<AuthSessionRecord>>;
    async fn revoke_auth_session(&self, id: &str) -> DirectoryResult<()>;
    //#endregion

    //#region SyncSessions
    async fn record_sync_session_open(
        &self,
        document_id: &str,
        user_id: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
    //#endregion
}
//#endregion 🔖️Trait
