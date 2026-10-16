//! 📜️ The append-only directory event log and projection rebuild.

use super::*;

impl Neo4jDirectory {
    /// 🔁️ One attempt of [`EventLog::append_decided_events`]; the trait method retries it while Neo4j reports a transient fault.
    async fn append_decided_events_attempt(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        if events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return Err(DirectoryError::Conflict("event requires its verified authority append seam".into()));
        }
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut persisted = Vec::with_capacity(events.len());
        for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(seq).map_err(backend)?;
            if let Some(reason) = self.projection_rejection(&mut txn, &event.body).await? {
                txn.rollback().await.map_err(backend)?;
                return Ok(DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason));
            }
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(
                query(
                    "CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id,
                                                spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})",
                )
                .param("seq", i64::try_from(seq).map_err(backend)?)
                .param("id", id.clone())
                .param("hlc_physical", event.hlc.physical_ms)
                .param("hlc_logical", event.hlc.logical as i64)
                .param("actor_kind", actor_kind_to_str(event.actor.kind))
                .param("actor_id", event.actor.id.clone())
                .param("space_id", event.space_id.clone())
                .param("user_id", event.user_id.clone())
                .param("kind", kind)
                .param("payload", payload_value.to_string())
                .param("recorded_at", recorded_at_ms),
            )
            .await
            .map_err(backend)?;
            self.project(&mut txn, &full).await?;
            match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'retention', spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", retention.scope.space_id.clone())
                            .param("document_id", retention.scope.document_id.clone())
                            .param("checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "retention", &retention.scope.space_id, Some(&retention.scope), Some(retention.retained_checkpoint_id)).await?;
                }
                DirectoryEventBody::SpaceDeleted { space_id } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'space-delete', spaceId: $space_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", space_id.clone())
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "space-delete", space_id, None, None).await?;
                }
                _ => {}
            }
            persisted.push(full);
        }
        txn.commit().await.map_err(backend)?;
        Ok(DirectoryAppendOutcomeV1::Appended(persisted))
    }
}

//#region EventLog
impl EventLog for Neo4jDirectory {
    /// ➕️ Assigns a dense `seq` via a `(:DirectoryCounter {id:'singleton'})` node
    /// incremented in the same transaction as the `(:DirectoryEvent)` node and the projection —
    /// the write's atomicity comes from `Txn`, not from any Neo4j auto-increment primitive (Neo4j
    /// has none).
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.append_decided_events_attempt(events).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>> {
        if events.is_empty() || events.iter().any(|event| matches!(&event.body, DirectoryEventBody::ArtifactCheckpointPublished { .. } | DirectoryEventBody::InviteRedeemed { .. })) {
            return AdminEffectCommitV1::RejectedBeforeCommit;
        }
        let mut txn = match self.graph.start_txn().await {
            Ok(txn) => txn,
            Err(_) => return AdminEffectCommitV1::Indeterminate,
        };
        let persisted_result: DirectoryResult<Vec<DirectoryEvent>> = async {
            let mut persisted = Vec::with_capacity(events.len());
            for event in events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload_value = serde_json::Value::from(&event.body.to_value());
            let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(seq).map_err(backend)?;
            if self.projection_rejection(&mut txn, &event.body).await?.is_some() {
                return Err(DirectoryError::Conflict("administrator event projection was rejected".into()));
            }
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(
                query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                    .param("seq", i64::try_from(seq).map_err(backend)?)
                    .param("id", id)
                    .param("hlc_physical", event.hlc.physical_ms)
                    .param("hlc_logical", i64::from(event.hlc.logical))
                    .param("actor_kind", actor_kind_to_str(event.actor.kind))
                    .param("actor_id", event.actor.id.clone())
                    .param("space_id", event.space_id.clone())
                    .param("user_id", event.user_id.clone())
                    .param("kind", kind)
                    .param("payload", payload_value.to_string())
                    .param("recorded_at", recorded_at_ms),
            )
            .await
            .map_err(backend)?;
            self.project(&mut txn, &full).await?;
            match &full.body {
                DirectoryEventBody::ArtifactRetentionAdvanced { retention } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'retention', spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", retention.scope.space_id.clone())
                            .param("document_id", retention.scope.document_id.clone())
                            .param("checkpoint_id", hex_lower(&retention.retained_checkpoint_id.0))
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "retention", &retention.scope.space_id, Some(&retention.scope), Some(retention.retained_checkpoint_id)).await?;
                }
                DirectoryEventBody::SpaceDeleted { space_id } => {
                    let generation = cas_generation(&mut txn).await?;
                    txn.run(
                        query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'space-delete', spaceId: $space_id, eventSeq: $event_seq})")
                            .param("generation", generation)
                            .param("space_id", space_id.clone())
                            .param("event_seq", i64::try_from(seq).map_err(backend)?),
                    )
                    .await
                    .map_err(backend)?;
                    cas_project_release(&mut txn, "space-delete", space_id, None, None).await?;
                }
                _ => {}
            }
                persisted.push(full);
            }
            insert_admin_operation_effect_receipt(&mut txn, &admin_operation_effect_receipt_v1(effect, &persisted)?).await?;
            Ok(persisted)
        }
        .await;
        let persisted = match persisted_result {
            Ok(persisted) => persisted,
            Err(_) => return admin_effect_rollback(txn).await,
        };
        match txn.commit().await {
            Ok(()) => AdminEffectCommitV1::Applied(persisted),
            Err(_) => AdminEffectCommitV1::Indeterminate,
        }
    }

    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>> {
        let (since_seq, limit) = bounded_event_read(since_seq, limit)?;
        let mut result = self.graph.execute(query("MATCH (e:DirectoryEvent) WHERE e.seq > $since_seq RETURN e AS e ORDER BY e.seq LIMIT $limit").param("since_seq", since_seq).param("limit", limit)).await.map_err(backend)?;
        let mut events = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            events.push(event_from_node(&row)?);
        }
        Ok(events)
    }

    async fn head_seq(&self) -> DirectoryResult<u64> {
        let mut result = self.graph.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) RETURN c.seq AS seq")).await.map_err(backend)?;
        match result.next().await.map_err(backend)? {
            Some(row) => u64::try_from(row.get::<i64>("seq").map_err(backend)?).map_err(backend),
            None => Ok(0),
        }
    }
}
//#endregion
