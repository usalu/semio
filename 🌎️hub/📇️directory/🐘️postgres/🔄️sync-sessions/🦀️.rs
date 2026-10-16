//! 🔄️ Document sync session bookkeeping.

use super::*;

//#region SyncSessions
impl SyncSessionDirectory for PostgresDirectory {
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
        sqlx_core::query::query(
            "INSERT INTO hub_sync_session (id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)",
        )
        .bind(&id)
        .bind(auth_session_id)
        .bind(i64::try_from(authorization_generation).map_err(backend)?)
        .bind(actor_id)
        .bind(space_id)
        .bind(document_id)
        .bind(surface)
        .bind(user_id)
        .bind(authenticated_email)
        .bind(role_str)
        .bind(client_label)
        .bind(connected_at)
        .execute(&self.pool)
        .await
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
        sqlx_core::query::query("UPDATE hub_sync_session SET disconnected_at = $2 WHERE id = $1").bind(sync_session_id).bind(now_ms()).execute(&self.pool).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let rows: Vec<SyncSessionRow> = sqlx_core::query_as::query_as(
            "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
             WHERE document_id = $1 ORDER BY connected_at DESC",
        )
        .bind(document_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        Ok(rows.into_iter().map(sync_session_from_row).collect())
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > semio_hub_directory_contract::ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={}", semio_hub_directory_contract::ACTIVE_SYNC_SESSION_READ_MAX)));
        }
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        let rows: Vec<SyncSessionRow> = match space_id {
            Some(space_id) => sqlx_core::query_as::query_as(
                "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE space_id = $1 AND disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT $2 OFFSET $3",
            )
            .bind(space_id)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?,
            None => sqlx_core::query_as::query_as(
                "SELECT id, auth_session_id, authorization_generation, actor_id, space_id, document_id, surface, user_id, authenticated_email, space_role, client_label, connected_at, disconnected_at FROM hub_sync_session
                 WHERE disconnected_at IS NULL ORDER BY connected_at DESC, id ASC LIMIT $1 OFFSET $2",
            )
            .bind(limit)
            .bind(offset)
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
}
//#endregion
