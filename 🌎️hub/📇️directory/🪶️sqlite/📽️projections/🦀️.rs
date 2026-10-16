//! 📽️ Event persistence and the projection of directory events into the read tables.

use super::*;

impl SqliteDirectory {
    pub(super) fn persist_event_with_identity(&self, tx: &Transaction<'_>, event: &NewDirectoryEvent, id: String, recorded_at_ms: i64) -> DirectoryResult<DirectoryEvent> {
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        tx.execute(
            "INSERT INTO hub_directory_event (id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            rusqlite::params![id, event.hlc.physical_ms, event.hlc.logical, actor_kind_to_str(event.actor.kind), event.actor.id, event.space_id, event.user_id, kind, payload_value.to_string(), recorded_at_ms],
        )
        .map_err(backend)?;
        let persisted =
            DirectoryEvent { seq: u64::try_from(tx.last_insert_rowid()).map_err(backend)?, id, hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
        validate_directory_event_page_event(&persisted).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        Ok(persisted)
    }

    pub(super) fn persist_event(&self, tx: &Transaction<'_>, event: &NewDirectoryEvent) -> DirectoryResult<DirectoryEvent> {
        self.persist_event_with_identity(tx, event, time_ordered_id(), now_ms())
    }

    pub(super) fn project_verified_checkpoint(&self, tx: &Transaction<'_>, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        let payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
        let event_seq = i64::try_from(event.seq).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_checkpoint_private (space_id, document_id, checkpoint_id, event_seq, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), event_seq, payload],
        )
        .map_err(backend)?;
        Ok(())
    }
}

//#region 🔖️Projections
impl SqliteDirectory {
    /// 🏛️ Reads projection authority inside the current SQLite writer transaction.
    pub(super) fn projection_rejection(&self, tx: &Transaction<'_>, body: &DirectoryEventBody) -> DirectoryResult<Option<DirectoryProjectionRejectionV1>> {
        let Some(space_id) = directory_projection_space_v1(body) else {
            return Ok(None);
        };
        let kind: Option<String> = tx.query_row("SELECT kind FROM hub_space WHERE id = ?1", rusqlite::params![space_id], |row| row.get(0)).optional().map_err(backend)?;
        Ok(directory_projection_rejection_v1(body, kind.as_deref()))
    }

    /// 🧮️ Applies one event and enforces intrinsic archive authority in the same transaction.
    pub(super) fn project(&self, tx: &Transaction<'_>, event: &DirectoryEvent) -> DirectoryResult<()> {
        if let Some(reason) = self.projection_rejection(tx, &event.body)? {
            return Err(reason.into_error());
        }
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                tx.execute("INSERT OR IGNORE INTO hub_user (id, email, display_name, created_at) VALUES (?1, ?2, ?3, ?4)", rusqlite::params![user_id, email, display_name, event.recorded_at_ms]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                    rusqlite::params![space_id, name, owner_user_id, event.recorded_at_ms, kind_to_str(*space_kind), visibility_to_str(*visibility)],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                tx.execute("UPDATE hub_space SET name = ?2 WHERE id = ?1", rusqlite::params![space_id, name]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                tx.execute("UPDATE hub_space SET visibility = ?2 WHERE id = ?1", rusqlite::params![space_id, visibility_to_str(*visibility)]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                tx.execute("UPDATE hub_space SET kind = 'archive' WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("UPDATE hub_space_membership SET role = 'spectator' WHERE space_id = ?1 AND role = 'author'", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                tx.execute("DELETE FROM hub_share_grant WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_retention WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_checkpoint_private WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_checkpoint WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_document_descriptor WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space_invite WHERE space_id = ?1", rusqlite::params![space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_space WHERE id = ?1", rusqlite::params![space_id]).map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                tx.execute("DELETE FROM hub_space_membership WHERE space_id = ?1 AND user_id = ?2", rusqlite::params![space_id, user_id]).map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                tx.execute(
                    "INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES (?1, ?2, ?3, ?4)
                     ON CONFLICT(space_id, user_id) DO UPDATE SET role = ?3",
                    rusqlite::params![space_id, user_id, role_from_wire(*role).as_str(), event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                tx.execute(
                    "INSERT OR IGNORE INTO hub_document_descriptor (space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash, announced_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)",
                    rusqlite::params![descriptor.space_id, descriptor.document_id, descriptor.artifact_kind, descriptor.artifact_schema, descriptor.owner.plugin_id, descriptor.owner.package_id, descriptor.owner.version, descriptor.owner.package_hash, descriptor.pack_schema_hash, descriptor.bootstrap_version as i64, descriptor.bootstrap_frontier.head_seq as i64, descriptor.bootstrap_frontier.commit_seq as i64, descriptor.bootstrap_frontier.epoch as i64, descriptor.bootstrap_snapshot_hash, event.recorded_at_ms],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let descriptor = tx.query_row(
                    "SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2",
                    rusqlite::params![scope.space_id, scope.document_id], document_descriptor_row,
                ).optional().map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?;
                let row = semio_hub_directory::document_index_projection_v1(event, &descriptor)?;
                let payload = semio_framework_pack_json::to_json_string(&row);
                let previous: Option<String> = tx.query_row("SELECT payload FROM hub_document_index WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![scope.space_id, scope.document_id], |row| row.get(0)).optional().map_err(backend)?;
                if previous.as_ref().is_some_and(|stored| stored != &payload) {
                    return Err(DirectoryError::Conflict("document index is already bound".into()));
                }
                tx.execute("INSERT OR IGNORE INTO hub_document_index (space_id, document_id, payload) VALUES (?1, ?2, ?3)", rusqlite::params![scope.space_id, scope.document_id, payload]).map_err(backend)?;
            }
            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let descriptor = tx.query_row("SELECT space_id, document_id, artifact_kind, artifact_schema, owner_plugin_id, owner_package_id, owner_version, owner_package_hash, pack_schema_hash, bootstrap_version, bootstrap_head_seq, bootstrap_commit_seq, bootstrap_epoch, bootstrap_snapshot_hash FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], document_descriptor_row).optional().map_err(backend)?.ok_or_else(|| DirectoryError::NotFound("checkpoint document descriptor".into()))?;
                let index_payload: Option<String> =
                    tx.query_row("SELECT payload FROM hub_document_index WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], |row| row.get(0)).optional().map_err(backend)?;
                let index = index_payload.as_deref().map(semio_framework_pack_json::from_json_str::<directory::os_directory::DirectoryIndexedDocumentViewV1>).transpose().map_err(backend)?;
                semio_hub_directory::validate_checkpoint_index_v1(index.as_ref(), &descriptor, checkpoint)?;
                let active = tx
                    .query_row("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2 AND active = 1", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], published_checkpoint_row)
                    .optional()
                    .map_err(backend)?;
                let count: i64 = tx.query_row("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], |row| row.get(0)).map_err(backend)?;
                semio_hub_directory::validate_published_checkpoint_lineage(&descriptor, active.as_ref(), u64::try_from(count).map_err(backend)?, checkpoint)?;
                let payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
                tx.execute(
                    "UPDATE hub_artifact_checkpoint SET active = 0 WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id <> ?3",
                    rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice()],
                )
                .map_err(backend)?;
                tx.execute(
                    "INSERT INTO hub_artifact_checkpoint (space_id, document_id, checkpoint_id, parent_checkpoint_id, descriptor_digest, frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash, pack_sha256, pack_byte_length, spr_sha256, spr_byte_length, aggregate_sha256, published_at, event_seq, active, payload)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, 1, ?18)
                     ON CONFLICT(space_id, document_id, checkpoint_id) DO UPDATE SET active = 1, payload = excluded.payload",
                    rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), checkpoint.parent_checkpoint_id.map(|id| id.0.to_vec()), checkpoint.descriptor_digest_v1.0.as_slice(), checkpoint.baseline_frontier.document_id, i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?, checkpoint.baseline_frontier.head_edit_id, i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?, checkpoint.baseline_frontier.chain_hash.0.as_slice(), checkpoint.pack.sha256.0.as_slice(), i64::try_from(checkpoint.pack.byte_length).map_err(backend)?, checkpoint.spr.sha256.0.as_slice(), i64::try_from(checkpoint.spr.byte_length).map_err(backend)?, checkpoint.aggregate_sha256.0.as_slice(), i64::try_from(checkpoint.published_at_ms).map_err(backend)?, i64::try_from(event.seq).map_err(backend)?, payload],
                )
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let payload = serde_json::Value::from(&retention.to_value()).to_string();
                tx.execute(
                    "INSERT INTO hub_artifact_retention (space_id, document_id, retained_checkpoint_id, floor_document_id, floor_head_edit_ordinal, floor_head_edit_id, floor_last_commit_seq, floor_chain_hash, checkpoint_lineage_head, event_seq, payload)
                     VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
                     ON CONFLICT(space_id, document_id) DO UPDATE SET retained_checkpoint_id = excluded.retained_checkpoint_id, floor_document_id = excluded.floor_document_id, floor_head_edit_ordinal = excluded.floor_head_edit_ordinal, floor_head_edit_id = excluded.floor_head_edit_id, floor_last_commit_seq = excluded.floor_last_commit_seq, floor_chain_hash = excluded.floor_chain_hash, checkpoint_lineage_head = excluded.checkpoint_lineage_head, event_seq = excluded.event_seq, payload = excluded.payload",
                    rusqlite::params![retention.scope.space_id, retention.scope.document_id, retention.retained_checkpoint_id.0.as_slice(), retention.retained_floor.document_id, i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?, retention.retained_floor.head_edit_id, i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?, retention.retained_floor.chain_hash.0.as_slice(), retention.checkpoint_lineage_head.0.as_slice(), i64::try_from(event.seq).map_err(backend)?, payload],
                )
                .map_err(backend)?;
            }
        }
        Ok(())
    }
}

impl ProjectionRebuild for SqliteDirectory {
    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let event_count: i64 = tx.query_row("SELECT COUNT(*) FROM hub_directory_event", [], |row| row.get(0)).map_err(backend)?;
        let total = u64::try_from(event_count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        tx.execute_batch(
            "CREATE TEMP TABLE hub_rebuild_user AS SELECT * FROM hub_user;
             CREATE TEMP TABLE hub_rebuild_auth_session AS SELECT * FROM hub_auth_session;
             CREATE TEMP TABLE hub_rebuild_sync_binding AS SELECT id, auth_session_id, user_id FROM hub_sync_session;
             CREATE TEMP TABLE hub_rebuild_space_invite AS SELECT * FROM hub_space_invite;",
        )
        .map_err(|error| DirectoryError::Backend(format!("SQLite projection rebuild credential snapshot: {error}")))?;
        tx.execute_batch(
            "DELETE FROM hub_artifact_cas_reservation; DELETE FROM hub_artifact_cas_reference; DELETE FROM hub_artifact_retention; DELETE FROM hub_artifact_checkpoint_private; DELETE FROM hub_artifact_checkpoint; DELETE FROM hub_document_index; DELETE FROM hub_document_descriptor; DELETE FROM hub_space_membership; DELETE FROM hub_space; DELETE FROM hub_user;",
        )
        .map_err(|error| DirectoryError::Backend(format!("SQLite projection rebuild truncation: {error}")))?;
        tx.execute_batch(
            "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at)
             SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_rebuild_user;",
        )
        .map_err(|error| DirectoryError::Backend(format!("SQLite projection rebuild identity restore: {error}")))?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        const PAGE: i64 = 512;
        while replayed < total {
            let events: Vec<DirectoryEvent> = {
                let mut stmt = tx.prepare("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > ?1 ORDER BY seq LIMIT ?2").map_err(backend)?;
                let mapped = stmt.query_map(rusqlite::params![cursor, PAGE], event_row).map_err(backend)?;
                mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
            };
            if events.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for event in &events {
                cursor = i64::try_from(event.seq).map_err(backend)?;
                self.project(&tx, event).map_err(|error| DirectoryError::Backend(format!("SQLite projection rebuild event {}: {error}", event.seq)))?;
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let checkpoint = tx
                        .query_row("SELECT payload FROM hub_artifact_authority_journal WHERE event_seq = ?1", rusqlite::params![cursor], verified_checkpoint_row)
                        .optional()
                        .map_err(backend)?
                        .ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq)))?;
                    self.project_verified_checkpoint(&tx, event, &checkpoint)?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        let ledger_rows: Vec<(i64, String, String, Option<String>, Option<Vec<u8>>, Option<i64>, Option<i64>, Option<Vec<u8>>)> = {
            let mut statement = tx.prepare("SELECT generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_ledger_journal ORDER BY generation").map_err(backend)?;
            let mapped = statement.query_map([], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?))).map_err(backend)?;
            mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
        };
        for (generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) in ledger_rows {
            match operation.as_str() {
                "reserve" => {
                    let plan = decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?;
                    let reservation = ArtifactCasReservation::unfenced(
                        plan,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal expiry missing".into()))?).map_err(backend)?,
                    );
                    Self::cas_project_reserve(&tx, &reservation)?;
                }
                "publish" => {
                    let plan = decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?;
                    let reservation = ArtifactCasReservation::unfenced(
                        plan,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal expiry missing".into()))?).map_err(backend)?,
                    );
                    Self::cas_project_publish(&tx, &reservation, generation)?;
                }
                "retention" | "space-delete" => {
                    let checkpoint_id = checkpoint_id.map(|bytes| bytes.try_into().map(ArtifactHash).map_err(|_: Vec<u8>| DirectoryError::Backend("artifact CAS release journal checkpoint width".into()))).transpose()?;
                    Self::cas_project_release(&tx, &operation, &space_id, document_id.as_deref(), checkpoint_id)?;
                }
                _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
            }
        }
        tx.execute_batch(
            "INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind)
             SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_rebuild_auth_session;
             UPDATE hub_sync_session
             SET auth_session_id = (SELECT auth_session_id FROM hub_rebuild_sync_binding WHERE hub_rebuild_sync_binding.id = hub_sync_session.id),
                 user_id = (SELECT user_id FROM hub_rebuild_sync_binding WHERE hub_rebuild_sync_binding.id = hub_sync_session.id)
             WHERE id IN (SELECT id FROM hub_rebuild_sync_binding);
             INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id)
             SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_rebuild_space_invite;
             DROP TABLE hub_rebuild_user;
             DROP TABLE hub_rebuild_auth_session;
             DROP TABLE hub_rebuild_sync_binding;
             DROP TABLE hub_rebuild_space_invite;",
        )
        .map_err(|error| DirectoryError::Backend(format!("SQLite projection rebuild credential restore: {error}")))?;
        tx.commit().map_err(backend)?;
        Ok(replayed)
    }
}
//#endregion 🔖️Projections
