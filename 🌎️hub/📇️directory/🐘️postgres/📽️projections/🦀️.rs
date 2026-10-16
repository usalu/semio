//! 📽️ Event persistence and the projection of directory events into the read tables.

use super::*;

impl PostgresDirectory {
    pub(super) async fn project_verified_checkpoint(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &DirectoryEvent, checkpoint: &ArtifactCheckpoint) -> DirectoryResult<()> {
        let new_event = NewDirectoryEvent { hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone() };
        validate_verified_checkpoint_append(&new_event, checkpoint)?;
        sqlx_core::query::query("INSERT INTO hub_artifact_checkpoint_private (space_id, document_id, checkpoint_id, event_seq, payload) VALUES ($1, $2, $3, $4, $5)")
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .bind(i64::try_from(event.seq).map_err(backend)?)
            .bind(serde_json::Value::from(&checkpoint.to_value()))
            .execute(&mut **tx)
            .await
            .map_err(backend)?;
        Ok(())
    }
}

//#region 🔖️Projections
impl PostgresDirectory {
    /// 🏛️ Reads projection authority after the PostgreSQL directory-head writer lock.
    pub(super) async fn projection_rejection(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, body: &DirectoryEventBody) -> DirectoryResult<Option<DirectoryProjectionRejectionV1>> {
        let Some(space_id) = directory_projection_space_v1(body) else {
            return Ok(None);
        };
        let kind: Option<(String,)> = sqlx_core::query_as::query_as("SELECT kind FROM hub_space WHERE id = $1").bind(space_id).fetch_optional(&mut **tx).await.map_err(backend)?;
        Ok(directory_projection_rejection_v1(body, kind.as_ref().map(|row| row.0.as_str())))
    }

    /// 🧮️ Applies one event and enforces intrinsic archive authority in the same transaction.
    pub(super) async fn project(&self, tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, event: &DirectoryEvent) -> DirectoryResult<()> {
        if let Some(reason) = self.projection_rejection(tx, &event.body).await? {
            return Err(reason.into_error());
        }
        match &event.body {
            DirectoryEventBody::UserCreated { user_id, email, display_name } => {
                sqlx_core::query::query("INSERT INTO hub_user (id, email, display_name, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (id) DO NOTHING")
                    .bind(user_id)
                    .bind(email)
                    .bind(display_name)
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceCreated { space_id, name, space_kind, visibility, owner_user_id } => {
                sqlx_core::query::query("INSERT INTO hub_space (id, name, owner_user_id, created_at, kind, visibility) VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (id) DO NOTHING")
                    .bind(space_id)
                    .bind(name)
                    .bind(owner_user_id)
                    .bind(event.recorded_at_ms)
                    .bind(kind_to_str(*space_kind))
                    .bind(visibility_to_str(*visibility))
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::SpaceRenamed { space_id, name } => {
                sqlx_core::query::query("UPDATE hub_space SET name = $2 WHERE id = $1").bind(space_id).bind(name).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceVisibilityChanged { space_id, visibility } => {
                sqlx_core::query::query("UPDATE hub_space SET visibility = $2 WHERE id = $1").bind(space_id).bind(visibility_to_str(*visibility)).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceArchived { space_id } => {
                sqlx_core::query::query("UPDATE hub_space SET kind = 'archive' WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("UPDATE hub_space_membership SET role = 'spectator' WHERE space_id = $1 AND role = 'author'").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::SpaceDeleted { space_id } => {
                sqlx_core::query::query("DELETE FROM hub_share_grant WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_retention WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint_private WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_document_descriptor WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space_invite WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
                sqlx_core::query::query("DELETE FROM hub_space WHERE id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::MemberUpserted { space_id, user_id, role } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::MemberRemoved { space_id, user_id } => {
                sqlx_core::query::query("DELETE FROM hub_space_membership WHERE space_id = $1 AND user_id = $2").bind(space_id).bind(user_id).execute(&mut **tx).await.map_err(backend)?;
            }
            DirectoryEventBody::InviteRedeemed { space_id, user_id, role, .. } => {
                sqlx_core::query::query("INSERT INTO hub_space_membership (space_id, user_id, role, created_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, user_id) DO UPDATE SET role = $3")
                    .bind(space_id)
                    .bind(user_id)
                    .bind(role_from_wire(*role).as_str())
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::DocumentAnnounced { descriptor } => {
                sqlx_core::query::query("INSERT INTO hub_document_descriptor (space_id, document_id, descriptor, announced_at) VALUES ($1, $2, $3, $4) ON CONFLICT (space_id, document_id) DO NOTHING")
                    .bind(&descriptor.space_id)
                    .bind(&descriptor.document_id)
                    .bind(serde_json::Value::from(&descriptor.to_value()))
                    .bind(event.recorded_at_ms)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::DocumentIndexed { scope, .. } => {
                let descriptor: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2 FOR UPDATE")
                    .bind(&scope.space_id)
                    .bind(&scope.document_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(backend)?;
                let descriptor = DocumentDescriptor::from_value(DslValue::from(descriptor.ok_or_else(|| DirectoryError::NotFound("indexed document descriptor".into()))?.0)).map_err(backend)?;
                let row = semio_hub_directory::document_index_projection_v1(event, &descriptor)?;
                let payload = semio_framework_pack_json::to_json_string(&row);
                let previous: Option<(String,)> =
                    sqlx_core::query_as::query_as("SELECT payload FROM hub_document_index WHERE space_id = $1 AND document_id = $2").bind(&scope.space_id).bind(&scope.document_id).fetch_optional(&mut **tx).await.map_err(backend)?;
                if previous.as_ref().is_some_and(|(stored,)| stored != &payload) {
                    return Err(DirectoryError::Conflict("document index is already bound".into()));
                }
                sqlx_core::query::query("INSERT INTO hub_document_index (space_id, document_id, payload) VALUES ($1, $2, $3) ON CONFLICT (space_id, document_id) DO NOTHING")
                    .bind(&scope.space_id)
                    .bind(&scope.document_id)
                    .bind(payload)
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
            }
            DirectoryEventBody::UserPreferenceRecorded { .. } => {}
            DirectoryEventBody::ArtifactCheckpointPublished { checkpoint } => {
                let descriptor: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT descriptor FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2 FOR UPDATE")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(backend)?;
                let descriptor = DocumentDescriptor::from_value(DslValue::from(descriptor.ok_or_else(|| DirectoryError::NotFound("checkpoint document descriptor".into()))?.0)).map_err(backend)?;
                let index: Option<(String,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_document_index WHERE space_id = $1 AND document_id = $2 FOR UPDATE")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(backend)?;
                let index = index.map(|(payload,)| semio_framework_pack_json::from_json_str::<directory::os_directory::DirectoryIndexedDocumentViewV1>(&payload, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)).transpose()?;
                semio_hub_directory::validate_checkpoint_index_v1(index.as_ref(), &descriptor, checkpoint)?;
                let active: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2 AND active FOR UPDATE")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(backend)?;
                let active = active.map(|(value,)| PublishedArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend)).transpose()?;
                let (count,): (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .fetch_one(&mut **tx)
                    .await
                    .map_err(backend)?;
                semio_hub_directory::validate_published_checkpoint_lineage(&descriptor, active.as_ref(), u64::try_from(count).map_err(backend)?, checkpoint)?;
                let payload = serde_json::Value::from(&checkpoint.to_value());
                sqlx_core::query::query("UPDATE hub_artifact_checkpoint SET active = FALSE WHERE space_id = $1 AND document_id = $2 AND checkpoint_id <> $3")
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .bind(checkpoint.checkpoint_id.0.as_slice())
                    .execute(&mut **tx)
                    .await
                    .map_err(backend)?;
                sqlx_core::query::query(
                    "INSERT INTO hub_artifact_checkpoint (space_id, document_id, checkpoint_id, parent_checkpoint_id, descriptor_digest, frontier_document_id, head_edit_ordinal, head_edit_id, last_commit_seq, chain_hash, pack_sha256, pack_byte_length, spr_sha256, spr_byte_length, aggregate_sha256, published_at, event_seq, active, payload)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, TRUE, $18)
                     ON CONFLICT (space_id, document_id, checkpoint_id) DO UPDATE SET active = TRUE, payload = EXCLUDED.payload",
                )
                .bind(&checkpoint.scope.space_id)
                .bind(&checkpoint.scope.document_id)
                .bind(checkpoint.checkpoint_id.0.as_slice())
                .bind(checkpoint.parent_checkpoint_id.map(|id| id.0.to_vec()))
                .bind(checkpoint.descriptor_digest_v1.0.as_slice())
                .bind(&checkpoint.baseline_frontier.document_id)
                .bind(i64::try_from(checkpoint.baseline_frontier.head_edit_ordinal).map_err(backend)?)
                .bind(&checkpoint.baseline_frontier.head_edit_id)
                .bind(i64::try_from(checkpoint.baseline_frontier.last_commit_seq).map_err(backend)?)
                .bind(checkpoint.baseline_frontier.chain_hash.0.as_slice())
                .bind(checkpoint.pack.sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.pack.byte_length).map_err(backend)?)
                .bind(checkpoint.spr.sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.spr.byte_length).map_err(backend)?)
                .bind(checkpoint.aggregate_sha256.0.as_slice())
                .bind(i64::try_from(checkpoint.published_at_ms).map_err(backend)?)
                .bind(i64::try_from(event.seq).map_err(backend)?)
                .bind(payload)
                .execute(&mut **tx)
                .await
                .map_err(backend)?;
            }
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                let payload = serde_json::Value::from(&retention.to_value());
                sqlx_core::query::query(
                    "INSERT INTO hub_artifact_retention (space_id, document_id, retained_checkpoint_id, floor_document_id, floor_head_edit_ordinal, floor_head_edit_id, floor_last_commit_seq, floor_chain_hash, checkpoint_lineage_head, event_seq, payload)
                     VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
                     ON CONFLICT (space_id, document_id) DO UPDATE SET retained_checkpoint_id = EXCLUDED.retained_checkpoint_id, floor_document_id = EXCLUDED.floor_document_id, floor_head_edit_ordinal = EXCLUDED.floor_head_edit_ordinal, floor_head_edit_id = EXCLUDED.floor_head_edit_id, floor_last_commit_seq = EXCLUDED.floor_last_commit_seq, floor_chain_hash = EXCLUDED.floor_chain_hash, checkpoint_lineage_head = EXCLUDED.checkpoint_lineage_head, event_seq = EXCLUDED.event_seq, payload = EXCLUDED.payload",
                )
                .bind(&retention.scope.space_id)
                .bind(&retention.scope.document_id)
                .bind(retention.retained_checkpoint_id.0.as_slice())
                .bind(&retention.retained_floor.document_id)
                .bind(i64::try_from(retention.retained_floor.head_edit_ordinal).map_err(backend)?)
                .bind(&retention.retained_floor.head_edit_id)
                .bind(i64::try_from(retention.retained_floor.last_commit_seq).map_err(backend)?)
                .bind(retention.retained_floor.chain_hash.0.as_slice())
                .bind(retention.checkpoint_lineage_head.0.as_slice())
                .bind(i64::try_from(event.seq).map_err(backend)?)
                .bind(payload)
                .execute(&mut **tx)
                .await
                .map_err(backend)?;
            }
        }
        Ok(())
    }
}

impl ProjectionRebuild for PostgresDirectory {
    async fn rebuild_projections(&self) -> DirectoryResult<u64> {
        self.rebuild_projections_controlled(&UNCONTROLLED_PROJECTION_REBUILD).await
    }

    async fn rebuild_projections_controlled(&self, control: &dyn ProjectionRebuildControl) -> DirectoryResult<u64> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let _: (i64,) = sqlx_core::query_as::query_as("SELECT seq FROM hub_directory_event_head WHERE singleton FOR UPDATE").fetch_one(&mut *tx).await.map_err(backend)?;
        let (event_count,): (i64,) = sqlx_core::query_as::query_as("SELECT COUNT(*) FROM hub_directory_event").fetch_one(&mut *tx).await.map_err(backend)?;
        let total = u64::try_from(event_count).map_err(backend)?;
        checkpoint_projection_rebuild(control, 0, total)?;
        sqlx_core::query::query(
            "CREATE TEMP TABLE hub_rebuild_space_invite ON COMMIT DROP AS SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_space_invite",
        )
        .execute(&mut *tx)
        .await
        .map_err(backend)?;
        sqlx_core::query::query("CREATE TEMP TABLE hub_rebuild_user ON COMMIT DROP AS SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_user").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query(
            "CREATE TEMP TABLE hub_rebuild_auth_session ON COMMIT DROP AS SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_auth_session",
        )
        .execute(&mut *tx)
        .await
        .map_err(backend)?;
        sqlx_core::query::query("CREATE TEMP TABLE hub_rebuild_sync_binding ON COMMIT DROP AS SELECT id, auth_session_id, user_id FROM hub_sync_session").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_retention").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint_private").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_artifact_checkpoint").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_document_index").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_document_descriptor").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space_membership").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_space").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query("DELETE FROM hub_user").execute(&mut *tx).await.map_err(backend)?;
        sqlx_core::query::query(
            "INSERT INTO hub_user (id, email, display_name, password_hash, sso_subject, sso_provider, created_at)
             SELECT id, email, display_name, password_hash, sso_subject, sso_provider, created_at FROM hub_rebuild_user",
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| DirectoryError::Backend(format!("PostgreSQL projection rebuild identity restore: {error}")))?;
        let mut replayed = 0u64;
        let mut cursor = 0i64;
        while replayed < total {
            let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> =
                sqlx_core::query_as::query_as("SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at FROM hub_directory_event WHERE seq > $1 ORDER BY seq LIMIT 512")
                    .bind(cursor)
                    .fetch_all(&mut *tx)
                    .await
                    .map_err(backend)?;
            if rows.is_empty() {
                return Err(DirectoryError::Backend("directory event replay ended before its counted head".into()));
            }
            for row in rows {
                cursor = row.0;
                let event = event_from_row(row)?;
                if let Err(error) = self.project(&mut tx, &event).await {
                    tx.rollback().await.map_err(backend)?;
                    return Err(DirectoryError::Backend(format!("PostgreSQL projection rebuild event {}: {error}", event.seq)));
                }
                if matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. }) {
                    let private: Option<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_authority_journal WHERE event_seq = $1").bind(cursor).fetch_optional(&mut *tx).await.map_err(backend)?;
                    let checkpoint =
                        private.ok_or_else(|| DirectoryError::Backend(format!("missing private authority journal for checkpoint event {}", event.seq))).and_then(|(value,)| ArtifactCheckpoint::from_value(DslValue::from(value)).map_err(backend))?;
                    self.project_verified_checkpoint(&mut tx, &event, &checkpoint).await?;
                }
                replayed += 1;
                checkpoint_projection_rebuild(control, replayed, total)?;
            }
        }
        sqlx_core::query::query(
            "INSERT INTO hub_space_invite (id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id)
             SELECT id, selector, secret_digest, space_id, role, created_at, expires_at, revoked_at, revoked_reason, accepted_at, accepted_event_id FROM hub_rebuild_space_invite",
        )
        .execute(&mut *tx)
        .await
        .map_err(backend)?;
        sqlx_core::query::query(
            "INSERT INTO hub_auth_session (id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind)
             SELECT id, selector, secret_digest, user_id, identity_provider, identity_subject_digest, issued_at, expires_at, revoked_at, revoked_reason, authorization_generation, device_instance_id, session_kind FROM hub_rebuild_auth_session",
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| DirectoryError::Backend(format!("PostgreSQL projection rebuild session restore: {error}")))?;
        sqlx_core::query::query(
            "UPDATE hub_sync_session SET auth_session_id = binding.auth_session_id, user_id = binding.user_id
             FROM hub_rebuild_sync_binding AS binding WHERE hub_sync_session.id = binding.id",
        )
        .execute(&mut *tx)
        .await
        .map_err(|error| DirectoryError::Backend(format!("PostgreSQL projection rebuild sync binding restore: {error}")))?;
        type CasLedgerRow = (i64, String, String, Option<String>, Option<Vec<u8>>, Option<i64>, Option<i64>, Option<Vec<u8>>);
        let ledger_rows: Vec<CasLedgerRow> =
            sqlx_core::query_as::query_as("SELECT generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_ledger_journal ORDER BY generation").fetch_all(&mut *tx).await.map_err(backend)?;
        for (generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) in ledger_rows {
            match operation.as_str() {
                "reserve" => {
                    let reservation = ArtifactCasReservation::unfenced(
                        decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS reserve journal expiry missing".into()))?).map_err(backend)?,
                    );
                    cas_project_reserve(&mut tx, &reservation).await?;
                }
                "publish" => {
                    let reservation = ArtifactCasReservation::unfenced(
                        decode_artifact_cas_ownership_v1(plan.as_deref().ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal plan missing".into()))?).map_err(|error| DirectoryError::Backend(error.to_string()))?,
                        u64::try_from(generation).map_err(backend)?,
                        u64::try_from(write_epoch.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal epoch missing".into()))?).map_err(backend)?,
                        u64::try_from(expires_at_ms.ok_or_else(|| DirectoryError::Backend("artifact CAS publish journal expiry missing".into()))?).map_err(backend)?,
                    );
                    cas_project_publish(&mut tx, &reservation, generation).await?;
                }
                "retention" | "space-delete" => {
                    let checkpoint_id = checkpoint_id.map(|bytes| array32(bytes, "artifact CAS release checkpoint").map(ArtifactHash)).transpose()?;
                    cas_project_release(&mut tx, &operation, &space_id, document_id.as_deref(), checkpoint_id).await?;
                }
                _ => return Err(DirectoryError::Backend("artifact CAS ledger operation is invalid".into())),
            }
        }
        tx.commit().await.map_err(backend)?;
        Ok(replayed)
    }
}
//#endregion 🔖️Projections
