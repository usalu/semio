//! 🏘️ Spaces, memberships, document descriptors and artifact checkpoint reads.

use super::*;

//#region Spaces
impl SpaceDirectory for Neo4jDirectory {
    async fn get_space(&self, space_id: &str) -> DirectoryResult<Option<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space {id: $space_id}) RETURN s AS s").param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => Ok(Some(space_from_node(&row)?)),
            None => Ok(None),
        }
    }

    async fn list_spaces_for_user(&self, user_id: &str) -> DirectoryResult<Vec<(SpaceRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(s:Space) RETURN s AS s, m.role AS role ORDER BY s.createdAt").param("user_id", user_id)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let studio = space_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                studios.push((studio, role));
            }
        }
        Ok(studios)
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let mut result = self.graph.execute(query("MATCH (s:Space) RETURN s AS s ORDER BY s.createdAt SKIP $offset LIMIT $limit").param("limit", limit).param("offset", offset)).await.map_err(backend)?;
        let mut studios = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            studios.push(space_from_node(&row)?);
        }
        Ok(studios)
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let cypher = "MATCH (s:Space) WHERE $space_id IS NULL OR s.id = $space_id
                      WITH s ORDER BY s.id SKIP $offset LIMIT $limit
                      CALL { WITH s OPTIONAL MATCH (:User)-[membership:MEMBER_OF]->(s) RETURN count(membership) AS memberCount }
                      CALL { WITH s OPTIONAL MATCH (s)-[:CONTAINS_DOCUMENT]->(document:DocumentDescriptor) RETURN count(document) AS documentCount }
                      CALL { WITH s OPTIONAL MATCH (session:SyncSession {spaceId: s.id}) WHERE session.disconnectedAt IS NULL RETURN count(session) AS activeConnections }
                      CALL { WITH s OPTIONAL MATCH (event:DirectoryEvent {spaceId: s.id}) RETURN max(event.recordedAt) AS lastEventAt }
                      RETURN s AS s, memberCount, documentCount, activeConnections, coalesce(lastEventAt, s.createdAt) AS updatedAt ORDER BY s.id";
        let mut result = self.graph.execute(query(cypher).param("space_id", space_id.map(str::to_string)).param("offset", i64::try_from(offset).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?)).await.map_err(backend)?;
        let mut summaries = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            summaries.push(AdminSpaceSummaryRecord {
                space: space_from_node(&row)?,
                member_count: u64::try_from(row.get::<i64>("memberCount").map_err(backend)?).map_err(backend)?,
                document_count: u64::try_from(row.get::<i64>("documentCount").map_err(backend)?).map_err(backend)?,
                active_connections: u64::try_from(row.get::<i64>("activeConnections").map_err(backend)?).map_err(backend)?,
                updated_at: row.get("updatedAt").map_err(backend)?,
            });
        }
        Ok(summaries)
    }

    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>> {
        let cypher = "MATCH (s:Space) WHERE s.visibility = 'public' OR EXISTS { MATCH (:User {id: $user_id})-[:MEMBER_OF]->(s) }
                      CALL { WITH s OPTIONAL MATCH (:User)-[membership:MEMBER_OF]->(s) RETURN count(membership) AS memberCount }
                      CALL { WITH s OPTIONAL MATCH (s)-[:CONTAINS_DOCUMENT]->(document:DocumentDescriptor) RETURN count(document) AS documentCount }
                      CALL { WITH s OPTIONAL MATCH (session:SyncSession {spaceId: s.id}) WHERE session.disconnectedAt IS NULL RETURN count(session) AS activeConnections }
                      CALL { WITH s OPTIONAL MATCH (event:DirectoryEvent {spaceId: s.id}) RETURN max(event.recordedAt) AS lastEventAt }
                      CALL { WITH s OPTIONAL MATCH (:User {id: $user_id})-[own:MEMBER_OF]->(s) RETURN head(collect(own.role)) AS role }
                      RETURN s AS s, memberCount, documentCount, activeConnections, coalesce(lastEventAt, s.createdAt) AS updatedAt, role ORDER BY s.id";
        let mut result = self.graph.execute(query(cypher).param("user_id", user_id.map(str::to_string))).await.map_err(backend)?;
        let mut summaries = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: Option<String> = row.get("role").map_err(backend)?;
            summaries.push((
                AdminSpaceSummaryRecord {
                    space: space_from_node(&row)?,
                    member_count: u64::try_from(row.get::<i64>("memberCount").map_err(backend)?).map_err(backend)?,
                    document_count: u64::try_from(row.get::<i64>("documentCount").map_err(backend)?).map_err(backend)?,
                    active_connections: u64::try_from(row.get::<i64>("activeConnections").map_err(backend)?).map_err(backend)?,
                    updated_at: row.get("updatedAt").map_err(backend)?,
                },
                role.as_deref().and_then(SpaceRole::parse),
            ));
        }
        Ok(summaries)
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (u:User)-[membership:MEMBER_OF]->(:Space {id: $space_id})
                     RETURN u AS u, membership.role AS role ORDER BY u.id SKIP $offset LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("offset", i64::try_from(offset).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            members.push((user_from_node(&row)?, SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))?));
        }
        Ok(members)
    }

    async fn list_members(&self, space_id: &str) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        let mut result = self.graph.execute(query("MATCH (u:User)-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN u AS u, m.role AS role ORDER BY m.createdAt").param("space_id", space_id)).await.map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let user = user_from_node(&row)?;
            let role: String = row.get("role").map_err(backend)?;
            if let Some(role) = SpaceRole::parse(&role) {
                members.push((user, role));
            }
        }
        Ok(members)
    }

    async fn get_role(&self, space_id: &str, user_id: &str) -> DirectoryResult<Option<SpaceRole>> {
        let mut result = self.graph.execute(query("MATCH (:User {id: $user_id})-[m:MEMBER_OF]->(:Space {id: $space_id}) RETURN m.role AS role").param("user_id", user_id).param("space_id", space_id)).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => {
                let role: String = row.get("role").map_err(backend)?;
                Ok(SpaceRole::parse(&role))
            }
            None => Ok(None),
        }
    }

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (u:User)-[membership:MEMBER_OF]->(:Space {id: $space_id})
                     WHERE $after IS NULL OR u.id > $after
                     RETURN u.id AS userId, u.email AS email, u.displayName AS displayName, membership.role AS role
                     ORDER BY u.id LIMIT $limit",
                )
                .param("space_id", space_id)
                .param("after", after_user_id.map(str::to_owned))
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut members = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let role: String = row.get("role").map_err(backend)?;
            members.push(SpaceAdministrationMemberRow {
                user_id: row.get("userId").map_err(backend)?,
                email: row.get("email").map_err(backend)?,
                display_name: row.get("displayName").map_err(backend)?,
                role: SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))?,
            });
        }
        Ok(members)
    }
}

impl DocumentDirectory for Neo4jDirectory {
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        let scope_key = document_scope_key_v1(scope);
        let mut result = self.graph.execute(query("MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) RETURN d.descriptor AS descriptor").param("scope_key", scope_key)).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let encoded: String = row.get("descriptor").map_err(backend)?;
        semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Some).map_err(backend)
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let mut result = self.graph.execute(query("MATCH (d:DocumentDescriptor {spaceId: $space_id}) RETURN d.descriptor AS descriptor ORDER BY d.documentId").param("space_id", space_id)).await.map_err(backend)?;
        let mut descriptors = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let encoded: String = row.get("descriptor").map_err(backend)?;
            descriptors.push(semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?);
        }
        Ok(descriptors)
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (d:DocumentDescriptor)
                     WHERE $space_id IS NULL OR d.spaceId = $space_id
                     RETURN d.descriptor AS descriptor
                     ORDER BY d.spaceId, d.documentId SKIP $offset LIMIT $limit",
                )
                .param("space_id", space_id.map(str::to_string))
                .param("offset", i64::try_from(offset).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut descriptors = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let encoded: String = row.get("descriptor").map_err(backend)?;
            descriptors.push(semio_framework_pack_json::from_json_str(&encoded, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?);
        }
        Ok(descriptors)
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let mut result = self.graph.execute(query("MATCH (c:ArtifactCheckpoint {scopeCheckpointKey: $key}) RETURN c.payload AS payload").param("key", checkpoint_key_v1(scope, checkpoint_id))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Some).map_err(backend)
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        let mut result = self.graph.execute(query("MATCH (p:ArtifactCheckpointPrivate {scopeCheckpointKey: $key}) RETURN p.payload AS payload").param("key", checkpoint_key_v1(scope, checkpoint_id))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Some).map_err(backend)
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let mut result =
            self.graph.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:ACTIVE_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Some).map_err(backend)
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        let mut result = self.graph.execute(query("MATCH (r:ArtifactRetention {scopeKey: $scope_key}) RETURN r.payload AS payload").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let payload: String = row.get("payload").map_err(backend)?;
        semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map(Some).map_err(backend)
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let mut result = self.graph.execute(query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN count(c) AS count").param("scope_key", document_scope_key_v1(scope))).await.map_err(backend)?;
        let count: i64 = result.next().await.map_err(backend)?.and_then(|row| row.get("count").ok()).unwrap_or(0);
        u64::try_from(count).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (:DocumentDescriptor {scopeKey: $scope_key})-[:HAS_CHECKPOINT]->(c:ArtifactCheckpoint) RETURN c.payload AS payload ORDER BY c.eventSeq LIMIT $limit")
                    .param("scope_key", document_scope_key_v1(scope))
                    .param("limit", limit as i64),
            )
            .await
            .map_err(backend)?;
        let mut checkpoints = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let payload: String = row.get("payload").map_err(backend)?;
            checkpoints.push(semio_framework_pack_json::from_json_str(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?);
        }
        Ok(checkpoints)
    }
}
//#endregion
