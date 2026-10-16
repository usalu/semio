//! 🔄️ The capability trait of realtime sync sessions.

use super::*;

//#region SyncSessionDirectory
/// 🔄️ Realtime sync sessions: the open and close bookkeeping and the presence reads.
pub trait SyncSessionDirectory: Send + Sync + 'static {
    /// 🔴️ Widened over the pre-ticket signature with `space_id`/`surface` (contract's
    /// presence scope is `(space_id, document_id, surface)`).
    async fn record_sync_session_open(
        &self,
        auth_session_id: Option<&str>,
        authorization_generation: u64,
        actor_id: &str,
        space_id: &str,
        document_id: &str,
        surface: &str,
        user_id: Option<&str>,
        authenticated_email: Option<&str>,
        space_role: Option<SpaceRole>,
        client_label: &str,
    ) -> DirectoryResult<SyncSessionRecord>;
    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()>;
    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>>;
    /// 🟢️ Every still-open session, optionally scoped to one space — the admin connections
    /// view and the per-space presence roster both read this instead of iterating documents.
    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>>;
    async fn list_active_sync_sessions(&self, space_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        self.list_active_sync_sessions_page(space_id, 0, limit).await
    }
    /// 🧹️ Marks every still-open session closed — called once at hub boot, before any real
    /// connection lands, to clear crash residue from the previous process (a session that never got
    /// its `disconnected_at` because the hub was killed mid-connection).
    async fn close_all_sync_sessions(&self) -> DirectoryResult<()>;
}
//#endregion SyncSessionDirectory
