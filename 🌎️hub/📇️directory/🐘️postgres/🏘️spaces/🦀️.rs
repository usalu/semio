//! 🏘️ Spaces, memberships, document descriptors and artifact checkpoint reads.

use super::*;

//#region Spaces
impl SpaceDirectory for PostgresDirectory {
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

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, String, i64, String, String, i64, i64, i64, i64)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility,
                    (SELECT COUNT(*) FROM hub_space_membership m WHERE m.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_document_descriptor d WHERE d.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_sync_session y WHERE y.space_id = s.id AND y.disconnected_at IS NULL),
                    COALESCE((SELECT MAX(e.recorded_at) FROM hub_directory_event e WHERE e.space_id = s.id), s.created_at)
             FROM hub_space s WHERE ($1::TEXT IS NULL OR s.id = $1) ORDER BY s.id LIMIT $2 OFFSET $3",
        )
        .bind(space_id)
        .bind(i64::try_from(limit).map_err(backend)?)
        .bind(i64::try_from(offset).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(id, name, owner_user_id, created_at, kind, visibility, member_count, document_count, active_connections, updated_at)| {
                Ok(AdminSpaceSummaryRecord {
                    space: SpaceRecord { id, name, owner_user_id, created_at, kind, visibility },
                    member_count: u64::try_from(member_count).map_err(backend)?,
                    document_count: u64::try_from(document_count).map_err(backend)?,
                    active_connections: u64::try_from(active_connections).map_err(backend)?,
                    updated_at,
                })
            })
            .collect()
    }

    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>> {
        let rows: Vec<(String, String, String, i64, String, String, i64, i64, i64, i64, Option<String>)> = sqlx_core::query_as::query_as(
            "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility,
                    (SELECT COUNT(*) FROM hub_space_membership m WHERE m.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_document_descriptor d WHERE d.space_id = s.id),
                    (SELECT COUNT(*) FROM hub_sync_session y WHERE y.space_id = s.id AND y.disconnected_at IS NULL),
                    COALESCE((SELECT MAX(e.recorded_at) FROM hub_directory_event e WHERE e.space_id = s.id), s.created_at),
                    (SELECT r.role FROM hub_space_membership r WHERE r.space_id = s.id AND r.user_id = $1)
             FROM hub_space s
             WHERE s.visibility = 'public' OR EXISTS (SELECT 1 FROM hub_space_membership v WHERE v.space_id = s.id AND v.user_id = $1)
             ORDER BY s.id",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(id, name, owner_user_id, created_at, kind, visibility, member_count, document_count, active_connections, updated_at, role)| {
                let summary = AdminSpaceSummaryRecord {
                    space: SpaceRecord { id, name, owner_user_id, created_at, kind, visibility },
                    member_count: u64::try_from(member_count).map_err(backend)?,
                    document_count: u64::try_from(document_count).map_err(backend)?,
                    active_connections: u64::try_from(active_connections).map_err(backend)?,
                    updated_at,
                };
                Ok((summary, role.as_deref().and_then(SpaceRole::parse)))
            })
            .collect()
    }

    async fn list_admin_space_members_page(&self, space_id: &str, offset: usize, limit: usize) -> DirectoryResult<Vec<(UserRecord, SpaceRole)>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator member page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, String, Option<String>, Option<String>, Option<String>, i64, String)> = sqlx_core::query_as::query_as(
            "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
             FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id
             WHERE m.space_id = $1 ORDER BY u.id LIMIT $2 OFFSET $3",
        )
        .bind(space_id)
        .bind(i64::try_from(limit).map_err(backend)?)
        .bind(i64::try_from(offset).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(id, email, display_name, password_hash, sso_subject, sso_provider, created_at, role)| {
                SpaceRole::parse(&role).map(|role| (UserRecord { id, email, display_name, password_hash, sso_subject, sso_provider, created_at }, role)).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))
            })
            .collect()
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

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let rows: Vec<(String, String, String, String)> = sqlx_core::query_as::query_as(
            "SELECT u.id, u.email, u.display_name, m.role
             FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id
             WHERE m.space_id = $1 AND ($2::text IS NULL OR u.id > $2) ORDER BY u.id LIMIT $3",
        )
        .bind(space_id)
        .bind(after_user_id)
        .bind(i64::try_from(limit).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(user_id, email, display_name, role)| SpaceRole::parse(&role).map(|role| SpaceAdministrationMemberRow { user_id, email, display_name, role }).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into())))
            .collect()
    }
}

impl DocumentDirectory for PostgresDirectory {
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 ORDER BY document_id").bind(space_id).fetch_all(&self.pool).await.map_err(backend)?;
        rows.into_iter().map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).collect()
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        let rows: Vec<(serde_json::Value,)> = match space_id {
            Some(space_id) => sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 ORDER BY space_id, document_id LIMIT $2 OFFSET $3")
                .bind(space_id)
                .bind(limit)
                .bind(offset)
                .fetch_all(&self.pool)
                .await
                .map_err(backend)?,
            None => sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor ORDER BY space_id, document_id LIMIT $1 OFFSET $2").bind(limit).bind(offset).fetch_all(&self.pool).await.map_err(backend)?,
        };
        rows.into_iter().map(|(value,)| DocumentDescriptor::from_value(DslValue::from(value)).map_err(backend)).collect()
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(checkpoint_id.0.as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        row.map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint_private WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(checkpoint_id.0.as_slice())
            .fetch_optional(&self.pool)
            .await
            .map_err(backend)?;
        row.map(|(value,)| ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 AND active").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        let row: Option<(serde_json::Value,)> =
            sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_retention WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&self.pool).await.map_err(backend)?;
        row.map(|(value,)| ArtifactRetention::from_value(DslValue::from(value)).map_err(backend)).transpose()
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let row: (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(row.0).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 ORDER BY event_seq LIMIT $3")
            .bind(&scope.space_id)
            .bind(&scope.document_id)
            .bind(limit as i64)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
        rows.into_iter().map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).collect()
    }
}
//#endregion
