//! 🔄️ Document sync session bookkeeping.

use super::*;

//#region SyncSessions
impl SyncSessionDirectory for SqliteDirectory {
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
    ) -> DirectoryResult<SyncSessionRecord> {
        validate_bounded_auth_text(actor_id, "sync actor", AUTH_TEXT_MAX_BYTES)?;
        let id = time_ordered_id();
        let connected_at = now_ms();
        let role_str = space_role.map(|r| r.as_str());
        self.lock()?
            .execute(
                "INSERT INTO hub_sync_session (id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12)",
                rusqlite::params![id, auth_session_id, i64::try_from(authorization_generation).map_err(backend)?, actor_id, space_id, document_id, surface, user_id, authenticated_email, role_str, client_label, connected_at],
            )
            .map_err(backend)?;
        Ok(SyncSessionRecord {
            id,
            auth_session_id: auth_session_id.map(str::to_string),
            authorization_generation,
            actor_id: actor_id.to_string(),
            space_id: space_id.to_string(),
            document_id: document_id.to_string(),
            surface: surface.to_string(),
            user_id: user_id.map(str::to_string),
            authenticated_email: authenticated_email.map(str::to_string),
            space_role,
            client_label: client_label.to_string(),
            connected_at,
            disconnected_at: None,
        })
    }

    async fn record_sync_session_close(&self, sync_session_id: &str) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?2 WHERE id = ?1", rusqlite::params![sync_session_id, now_ms()]).map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE document_id = ?1 ORDER BY connected_at DESC").map_err(backend)?;
        let rows = stmt.query_map([document_id], sync_session_row).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={ACTIVE_SYNC_SESSION_READ_MAX}")));
        }
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        let conn = self.lock()?;
        let rows: Vec<SyncSessionRecord> = match space_id {
            Some(space_id) => {
                let mut stmt = conn
                    .prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE space_id = ?1 AND disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT ?2 OFFSET ?3")
                    .map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![space_id, limit, offset], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
            None => {
                let mut stmt =
                    conn.prepare("SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session WHERE disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT ?1 OFFSET ?2").map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![limit, offset], sync_session_row).map_err(backend)?;
                mapped.filter_map(|row| row.ok()).collect()
            }
        };
        Ok(rows)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.lock()?.execute("UPDATE hub_sync_session SET disconnected_at = ?1 WHERE disconnected_at IS NULL", rusqlite::params![now_ms()]).map_err(backend)?;
        Ok(())
    }
}
//#endregion
