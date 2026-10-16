//! 🔄️ Document sync session bookkeeping.

use super::*;

//#region SyncSessions
impl SyncSessionDirectory for Neo4jDirectory {
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
        let role_str = space_role.map(|r| r.as_str().to_string()).unwrap_or_default();
        match user_id {
            Some(user_id) => {
                self.graph
                    .run(
                        query(
                            "MATCH (u:User {id: $user_id})
                             CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, authenticatedEmail: $authenticated_email, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})
                             CREATE (s)-[:AS_USER]->(u)",
                        )
                        .param("space_id", space_id)
                        .param("document_id", document_id)
                        .param("surface", surface)
                        .param("user_id", user_id)
                        .param("authenticated_email", authenticated_email.unwrap_or_default())
                        .param("id", id.clone())
                        .param("auth_session_id", auth_session_id.unwrap_or_default())
                        .param("generation", i64::try_from(authorization_generation).map_err(backend)?)
                        .param("actor_id", actor_id)
                        .param("client_label", client_label)
                        .param("role", role_str.clone())
                        .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
            None => {
                self.graph
                    .run(
                        query("CREATE (s:SyncSession {id: $id, authSessionId: $auth_session_id, authorizationGeneration: $generation, actorId: $actor_id, spaceId: $space_id, documentId: $document_id, surface: $surface, authenticatedEmail: $authenticated_email, clientLabel: $client_label, spaceRole: $role, connectedAt: $connected_at})")
                            .param("space_id", space_id)
                            .param("document_id", document_id)
                            .param("surface", surface)
                            .param("authenticated_email", authenticated_email.unwrap_or_default())
                            .param("id", id.clone())
                            .param("auth_session_id", auth_session_id.unwrap_or_default())
                            .param("generation", i64::try_from(authorization_generation).map_err(backend)?)
                            .param("actor_id", actor_id)
                            .param("client_label", client_label)
                            .param("role", role_str.clone())
                            .param("connected_at", connected_at),
                    )
                    .await
                    .map_err(backend)?;
            }
        }
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
        self.graph.run(query("MATCH (s:SyncSession {id: $id}) SET s.disconnectedAt = $disconnected_at").param("id", sync_session_id).param("disconnected_at", now_ms())).await.map_err(backend)?;
        Ok(())
    }

    async fn list_sync_sessions_for_document(&self, document_id: &str) -> DirectoryResult<Vec<SyncSessionRecord>> {
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (s:SyncSession {documentId: $document_id})
                     OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                     RETURN s.id AS id, s.authSessionId AS authSessionId, s.authorizationGeneration AS generation, s.actorId AS actorId,
                            s.spaceId AS spaceId, s.surface AS surface, u.id AS userId, s.authenticatedEmail AS authenticatedEmail, s.spaceRole AS role, s.clientLabel AS clientLabel,
                            s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                     ORDER BY s.connectedAt DESC",
                )
                .param("document_id", document_id),
            )
            .await
            .map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            sessions.push(sync_session_from_row(&row, document_id)?);
        }
        Ok(sessions)
    }

    async fn list_active_sync_sessions_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<SyncSessionRecord>> {
        if limit == 0 || limit > semio_hub_directory_contract::ACTIVE_SYNC_SESSION_READ_MAX {
            return Err(DirectoryError::Conflict(format!("active sync-session limit must be 1..={}", semio_hub_directory_contract::ACTIVE_SYNC_SESSION_READ_MAX)));
        }
        let cypher = "MATCH (s:SyncSession)
                       WHERE s.disconnectedAt IS NULL AND ($space_id IS NULL OR s.spaceId = $space_id)
                       OPTIONAL MATCH (s)-[:AS_USER]->(u:User)
                       RETURN s.id AS id, s.authSessionId AS authSessionId, s.authorizationGeneration AS generation, s.actorId AS actorId,
                              s.spaceId AS spaceId, s.documentId AS documentId, s.surface AS surface, u.id AS userId, s.authenticatedEmail AS authenticatedEmail,
                              s.spaceRole AS role, s.clientLabel AS clientLabel, s.connectedAt AS connectedAt, s.disconnectedAt AS disconnectedAt
                       ORDER BY s.connectedAt DESC, s.id ASC SKIP $offset LIMIT $limit";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string)).param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?)).await.map_err(backend)?;
        let mut sessions = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let document_id: String = row.get("documentId").map_err(backend)?;
            sessions.push(sync_session_from_row(&row, &document_id)?);
        }
        Ok(sessions)
    }

    async fn close_all_sync_sessions(&self) -> DirectoryResult<()> {
        self.graph.run(query("MATCH (s:SyncSession) WHERE s.disconnectedAt IS NULL SET s.disconnectedAt = $now").param("now", now_ms())).await.map_err(backend)?;
        Ok(())
    }
}
//#endregion
