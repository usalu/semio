//! 🏘️ Spaces, memberships, document descriptors and artifact checkpoint reads.

use super::*;

//#region Spaces
impl SpaceDirectory for SqliteDirectory {
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
        let rows =
            stmt.query_map([user_id], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, i64>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, row.get::<_, String>(6)?))).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).filter_map(|(id, name, owner_user_id, created_at, kind, visibility, role)| SpaceRole::parse(&role).map(|role| (SpaceRecord { id, name, owner_user_id, created_at, kind, visibility }, role))).collect())
    }

    async fn list_spaces(&self, limit: i64, offset: i64) -> DirectoryResult<Vec<SpaceRecord>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT id, name, owner_user_id, created_at, kind, visibility FROM hub_space ORDER BY created_at LIMIT ?1 OFFSET ?2").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![limit, offset], |row| Ok(SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? })).map_err(backend)?;
        Ok(rows.filter_map(|row| row.ok()).collect())
    }

    async fn list_admin_space_summaries_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<AdminSpaceSummaryRecord>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator space page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility,
                        (SELECT COUNT(*) FROM hub_space_membership m WHERE m.space_id = s.id),
                        (SELECT COUNT(*) FROM hub_document_descriptor d WHERE d.space_id = s.id),
                        (SELECT COUNT(*) FROM hub_sync_session y WHERE y.space_id = s.id AND y.disconnected_at IS NULL),
                        COALESCE((SELECT MAX(e.recorded_at) FROM hub_directory_event e WHERE e.space_id = s.id), s.created_at)
                 FROM hub_space s WHERE (?1 IS NULL OR s.id = ?1) ORDER BY s.id LIMIT ?2 OFFSET ?3",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![space_id, i64::try_from(limit).map_err(backend)?, i64::try_from(offset).map_err(backend)?], |row| {
                Ok((
                    SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? },
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, i64>(9)?,
                ))
            })
            .map_err(backend)?;
        rows.map(|row| {
            let (space, member_count, document_count, active_connections, updated_at) = row.map_err(backend)?;
            Ok(AdminSpaceSummaryRecord {
                space,
                member_count: u64::try_from(member_count).map_err(backend)?,
                document_count: u64::try_from(document_count).map_err(backend)?,
                active_connections: u64::try_from(active_connections).map_err(backend)?,
                updated_at,
            })
        })
        .collect()
    }

    async fn list_visible_space_summaries(&self, user_id: Option<&str>) -> DirectoryResult<Vec<(AdminSpaceSummaryRecord, Option<SpaceRole>)>> {
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT s.id, s.name, s.owner_user_id, s.created_at, s.kind, s.visibility,
                        (SELECT COUNT(*) FROM hub_space_membership m WHERE m.space_id = s.id),
                        (SELECT COUNT(*) FROM hub_document_descriptor d WHERE d.space_id = s.id),
                        (SELECT COUNT(*) FROM hub_sync_session y WHERE y.space_id = s.id AND y.disconnected_at IS NULL),
                        COALESCE((SELECT MAX(e.recorded_at) FROM hub_directory_event e WHERE e.space_id = s.id), s.created_at),
                        (SELECT r.role FROM hub_space_membership r WHERE r.space_id = s.id AND r.user_id = ?1)
                 FROM hub_space s
                 WHERE s.visibility = 'public' OR EXISTS (SELECT 1 FROM hub_space_membership v WHERE v.space_id = s.id AND v.user_id = ?1)
                 ORDER BY s.id",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![user_id], |row| {
                Ok((
                    SpaceRecord { id: row.get(0)?, name: row.get(1)?, owner_user_id: row.get(2)?, created_at: row.get(3)?, kind: row.get(4)?, visibility: row.get(5)? },
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, i64>(9)?,
                    row.get::<_, Option<String>>(10)?,
                ))
            })
            .map_err(backend)?;
        rows.map(|row| {
            let (space, member_count, document_count, active_connections, updated_at, role) = row.map_err(backend)?;
            let summary = AdminSpaceSummaryRecord {
                space,
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
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT u.id, u.email, u.display_name, u.password_hash, u.sso_subject, u.sso_provider, u.created_at, m.role
                 FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id
                 WHERE m.space_id = ?1 ORDER BY u.id LIMIT ?2 OFFSET ?3",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![space_id, i64::try_from(limit).map_err(backend)?, i64::try_from(offset).map_err(backend)?], |row| {
                Ok((UserRecord { id: row.get(0)?, email: row.get(1)?, display_name: row.get(2)?, password_hash: row.get(3)?, sso_subject: row.get(4)?, sso_provider: row.get(5)?, created_at: row.get(6)? }, row.get::<_, String>(7)?))
            })
            .map_err(backend)?;
        rows.map(|row| {
            let (user, role) = row.map_err(backend)?;
            SpaceRole::parse(&role).map(|role| (user, role)).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))
        })
        .collect()
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

    async fn list_space_administration_members_page(&self, space_id: &str, after_user_id: Option<&str>, limit: usize) -> DirectoryResult<Vec<SpaceAdministrationMemberRow>> {
        if limit == 0 || limit > semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("space administration member page limit must be 1..={}", semio_hub_directory_contract::SPACE_ADMINISTRATION_PAGE_FETCH_MAX)));
        }
        let conn = self.lock()?;
        let mut statement = conn
            .prepare(
                "SELECT u.id, u.email, u.display_name, m.role
                 FROM hub_space_membership m JOIN hub_user u ON u.id = m.user_id
                 WHERE m.space_id = ?1 AND (?2 IS NULL OR u.id > ?2) ORDER BY u.id LIMIT ?3",
            )
            .map_err(backend)?;
        let rows = statement
            .query_map(rusqlite::params![space_id, after_user_id, i64::try_from(limit).map_err(backend)?], |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?)))
            .map_err(backend)?;
        rows.map(|row| {
            let (user_id, email, display_name, role) = row.map_err(backend)?;
            let role = SpaceRole::parse(&role).ok_or_else(|| DirectoryError::Backend("stored member role is invalid".into()))?;
            Ok(SpaceAdministrationMemberRow { user_id, email, display_name, role })
        })
        .collect()
    }
}

impl DocumentDirectory for SqliteDirectory {
    async fn get_document_descriptor(&self, scope: &DocumentScope) -> DirectoryResult<Option<DocumentDescriptor>> {
        self.lock()?
            .query_row(
                "SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2",
                rusqlite::params![scope.space_id, scope.document_id],
                document_descriptor_row,
            )
            .optional()
            .map_err(backend)
    }

    async fn list_document_descriptors(&self, space_id: &str) -> DirectoryResult<Vec<DocumentDescriptor>> {
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 ORDER BY document_id").map_err(backend)?;
        let rows = stmt.query_map([space_id], document_descriptor_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn list_document_descriptors_page(&self, space_id: Option<&str>, offset: usize, limit: usize) -> DirectoryResult<Vec<DocumentDescriptor>> {
        if limit == 0 || limit > semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX {
            return Err(DirectoryError::Conflict(format!("administrator document page limit must be 1..={}", semio_hub_directory_contract::ADMIN_PAGE_FETCH_MAX)));
        }
        let conn = self.lock()?;
        let limit = i64::try_from(limit).map_err(backend)?;
        let offset = i64::try_from(offset).map_err(backend)?;
        match space_id {
            Some(space_id) => {
                let mut statement = conn.prepare("SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 ORDER BY space_id, document_id LIMIT ?2 OFFSET ?3").map_err(backend)?;
                let rows = statement.query_map(rusqlite::params![space_id, limit, offset], document_descriptor_row).map_err(backend)?;
                rows.collect::<Result<Vec<_>, _>>().map_err(backend)
            }
            None => {
                let mut statement = conn.prepare("SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor ORDER BY space_id, document_id LIMIT ?1 OFFSET ?2").map_err(backend)?;
                let rows = statement.query_map(rusqlite::params![limit, offset], document_descriptor_row).map_err(backend)?;
                rows.collect::<Result<Vec<_>, _>>().map_err(backend)
            }
        }
    }

    async fn get_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        self.lock()?
            .query_row("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![scope.space_id, scope.document_id, checkpoint_id.0.as_slice()], published_checkpoint_row)
            .optional()
            .map_err(backend)
    }

    async fn get_verified_artifact_checkpoint(&self, scope: &DocumentScope, checkpoint_id: ArtifactHash) -> DirectoryResult<Option<ArtifactCheckpoint>> {
        self.lock()?
            .query_row("SELECT payload FROM hub_artifact_checkpoint_private WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3", rusqlite::params![scope.space_id, scope.document_id, checkpoint_id.0.as_slice()], verified_checkpoint_row)
            .optional()
            .map_err(backend)
    }

    async fn get_active_artifact_checkpoint(&self, scope: &DocumentScope) -> DirectoryResult<Option<PublishedArtifactCheckpoint>> {
        self.lock()?.query_row("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 AND active = 1", rusqlite::params![scope.space_id, scope.document_id], published_checkpoint_row).optional().map_err(backend)
    }

    async fn get_artifact_retention(&self, scope: &DocumentScope) -> DirectoryResult<Option<ArtifactRetention>> {
        self.lock()?.query_row("SELECT payload FROM hub_artifact_retention WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], artifact_retention_row).optional().map_err(backend)
    }

    async fn artifact_checkpoint_count(&self, scope: &DocumentScope) -> DirectoryResult<u64> {
        let count: i64 = self.lock()?.query_row("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).map_err(backend)?;
        u64::try_from(count).map_err(backend)
    }

    async fn list_artifact_checkpoint_lineage(&self, scope: &DocumentScope, limit: usize) -> DirectoryResult<Vec<PublishedArtifactCheckpoint>> {
        if limit == 0 || limit as u64 > ARTIFACT_CHECKPOINT_LINEAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact checkpoint lineage limit must be 1..={ARTIFACT_CHECKPOINT_LINEAGE_MAX}")));
        }
        let conn = self.lock()?;
        let mut stmt = conn.prepare("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 ORDER BY event_seq LIMIT ?3").map_err(backend)?;
        let rows = stmt.query_map(rusqlite::params![scope.space_id, scope.document_id, limit as i64], published_checkpoint_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }
}
//#endregion
