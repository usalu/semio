//! 📜️ The append-only directory event log and projection rebuild.

use super::*;

//#region EventLog
impl EventLog for PostgresDirectory {
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return Err(DirectoryError::Conflict("event requires its verified authority append seam".into()));
        }
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let row: (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
            if let Some(reason) = self.projection_rejection(&mut tx, &event.body).await? {
                tx.rollback().await.map_err(backend)?;
                return Ok(DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason));
            }

            let full =
                DirectoryEvent { seq: u64::try_from(row.0).map_err(backend)?, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            sqlx_core::query::query(
                "INSERT INTO hub_directory_event (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)",
            )
            .bind(row.0)
            .bind(&id)
            .bind(event.hlc.physical_ms)
            .bind(event.hlc.logical as i64)
            .bind(actor_kind_to_str(event.actor.kind))
            .bind(&event.actor.id)
            .bind(&event.space_id)
            .bind(&event.user_id)
            .bind(&kind)
            .bind(&payload_value)
            .bind(recorded_at_ms)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
            self.project(&mut tx, &full).await?;
            let release = match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => Some(("retention", retention.scope.space_id.as_str(), Some(retention.scope.document_id.as_str()), Some(retention.retained_checkpoint_id))),
                DirectoryEventBody::SpaceDeleted { space_id } => Some(("space-delete", space_id.as_str(), None, None)),
                _ => None,
            };
            if let Some((operation, space_id, document_id, checkpoint_id)) = release {
                let generation = cas_generation(&mut tx).await?;
                sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, event_seq) VALUES ($1,$2,$3,$4,$5,$6)")
                    .bind(generation)
                    .bind(operation)
                    .bind(space_id)
                    .bind(document_id)
                    .bind(checkpoint_id.map(|value| value.0.to_vec()))
                    .bind(row.0)
                    .execute(&mut *tx)
                    .await
                    .map_err(backend)?;
                cas_project_release(&mut tx, operation, space_id, document_id, checkpoint_id).await?;
            }
            persisted.push(full);
        }
        tx.commit().await.map_err(backend)?;
        Ok(DirectoryAppendOutcomeV1::Appended(persisted))
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        if events.is_empty() || events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return AdminEffectCommitV1::RejectedBeforeCommit;
        }
        let mut tx = match self.pool.begin().await {
            Ok(tx) => tx,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let persisted_result: DirectoryResult<Vec<DirectoryEvent>> = async {
            let mut persisted = Vec::with_capacity(events.len());
            for event in events {
                let id = time_ordered_id();
                let recorded_at_ms = now_ms();
                let payload_value = serde_json::Value::from(&event.body.to_value());
                let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
                let row: (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
                if self.projection_rejection(&mut tx, &event.body).await?.is_some() {
                    return Err(DirectoryError::Conflict("administrator event projection was rejected".into()));
                }
                let full = DirectoryEvent {
                    seq: u64::try_from(row.0).map_err(backend)?,
                    id: id.clone(),
                    hlc: event.hlc,
                    actor: event.actor.clone(),
                    space_id: event.space_id.clone(),
                    user_id: event.user_id.clone(),
                    body: event.body.clone(),
                    recorded_at_ms,
                };
                validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
                sqlx_core::query::query("INSERT INTO hub_directory_event (seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
                    .bind(row.0)
                    .bind(&id)
                    .bind(event.hlc.physical_ms)
                    .bind(i64::from(event.hlc.logical))
                    .bind(actor_kind_to_str(event.actor.kind))
                    .bind(&event.actor.id)
                    .bind(&event.space_id)
                    .bind(&event.user_id)
                    .bind(kind)
                    .bind(payload_value)
                    .bind(recorded_at_ms)
                    .execute(&mut *tx)
                    .await
                    .map_err(backend)?;
                self.project(&mut tx, &full).await?;
                let release = match &full.body {
                    DirectoryEventBody::ArtifactRetentionAdvanced { retention } => Some(("retention", retention.scope.space_id.as_str(), Some(retention.scope.document_id.as_str()), Some(retention.retained_checkpoint_id))),
                    DirectoryEventBody::SpaceDeleted { space_id } => Some(("space-delete", space_id.as_str(), None, None)),
                    _ => None,
                };
                if let Some((operation, space_id, document_id, checkpoint_id)) = release {
                    let generation = cas_generation(&mut tx).await?;
                    sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, event_seq) VALUES ($1,$2,$3,$4,$5,$6)")
                        .bind(generation)
                        .bind(operation)
                        .bind(space_id)
                        .bind(document_id)
                        .bind(checkpoint_id.map(|value| value.0.to_vec()))
                        .bind(row.0)
                        .execute(&mut *tx)
                        .await
                        .map_err(backend)?;
                    cas_project_release(&mut tx, operation, space_id, document_id, checkpoint_id).await?;
                }
                persisted.push(full);
            }
            insert_admin_operation_effect_receipt(&mut tx, &admin_operation_effect_receipt_v1(effect, &persisted)?).await?;
            Ok(persisted)
        }
        .await;
        let persisted = match persisted_result {
            Ok(persisted) => persisted,
            Err(_) => return admin_effect_rollback(tx).await,
        };
        match tx.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(persisted),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let rows: Vec<(i64, String, i64, i64, String, String, Option<String>, Option<String>, serde_json::Value, i64)> = sqlx_core::query_as::query_as(
            "SELECT seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, payload, recorded_at
             FROM hub_directory_event WHERE seq > $1 ORDER BY seq LIMIT $2",
        )
        .bind(since_seq)
        .bind(limit)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(event_from_row).collect()
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let row: (Option<i64>,) = sqlx_core::query_as::query_as("SELECT MAX(seq) FROM hub_directory_event").fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(row.0.unwrap_or(0)).map_err(backend)
    }
}
//#endregion
