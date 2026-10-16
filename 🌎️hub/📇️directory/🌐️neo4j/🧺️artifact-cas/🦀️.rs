//! 🧺️ Artifact CAS reservations, sweep candidates and delete fences.

use super::*;

impl Neo4jDirectory {
    /// 🔁️ One attempt of [`ArtifactCasDirectory::reserve_artifact_cas`]; the trait method retries it while Neo4j reports a transient fault.
    async fn reserve_artifact_cas_attempt(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let scope_key = checkpoint_key_v1(&plan.scope, plan.checkpoint_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let (coordinator_id, physical_epoch) = cas_reservation_barrier(&mut txn, &plan.scope.space_id, i64::try_from(now_ms).map_err(backend)?).await?;
        let mut historical = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key}) WHERE e.plan IS NOT NULL RETURN e.plan AS plan ORDER BY e.generation LIMIT 1").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = historical.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
            }
        }
        drop(historical);
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            let reservation = ArtifactCasReservation::fenced(
                plan.clone(),
                u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?,
                u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                i64::MAX as u64,
                coordinator_id,
                physical_epoch,
            );
            drop(published);
            txn.commit().await.map_err(backend)?;
            return Ok(reservation);
        }
        drop(published);
        let mut released = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key, operation: 'publish'}) RETURN count(e) > 0 AS released").param("key", scope_key.clone())).await.map_err(backend)?;
        let was_released: bool = released.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS released checkpoint query returned no row".into()))?.get("released").map_err(backend)?;
        drop(released);
        if was_released {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone()))
            .await
            .map_err(backend)?;
        if let Some(row) = current.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if stored.value.as_ref() != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            let expiry: i64 = row.get("expiresAtMs").map_err(backend)?;
            if expiry > i64::try_from(now_ms).map_err(backend)? {
                let reservation = ArtifactCasReservation::fenced(
                    plan.clone(),
                    u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend)?,
                    u64::try_from(row.get::<i64>("writeEpoch").map_err(backend)?).map_err(backend)?,
                    u64::try_from(expiry).map_err(backend)?,
                    coordinator_id,
                    physical_epoch,
                );
                drop(current);
                txn.commit().await.map_err(backend)?;
                return Ok(reservation);
            }
        }
        drop(current);
        let mut epoch_result = txn.execute(query("MATCH (e:ArtifactCasLedgerEvent {scopeCheckpointKey: $key}) RETURN coalesce(max(e.writeEpoch), 0) AS writeEpoch").param("key", scope_key.clone())).await.map_err(backend)?;
        let previous_epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS write epoch query returned no row".into()))?.get("writeEpoch").map_err(backend)?;
        drop(epoch_result);
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'reserve', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, plan: $plan})")
            .param("generation", generation).param("key", scope_key).param("space_id", plan.scope.space_id.clone()).param("document_id", plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&plan.checkpoint_id.0)).param("write_epoch", write_epoch).param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?).param("plan", encoded)).await.map_err(backend)?;
        let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, expires_at_ms, coordinator_id, physical_epoch);
        cas_project_reserve(&mut txn, &reservation).await?;
        txn.commit().await.map_err(backend)?;
        Ok(reservation)
    }

    /// 🔁️ One attempt of [`ArtifactCasDirectory::append_reserved_artifact_checkpoint`]; the trait method retries it while Neo4j reports a transient fault.
    async fn append_reserved_artifact_checkpoint_attempt(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        current_now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        if checkpoint.parent_checkpoint_id.is_none() || !checkpoint.baseline_frontier.is_edited_for(&checkpoint.scope) {
            return Err(DirectoryError::Conflict("ordinary artifact checkpoint publication requires established edited lineage".into()));
        }
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if let Some(event) = event {
            validate_verified_checkpoint_append(event, checkpoint)?;
        }
        let scope_key = checkpoint_key_v1(&reservation.plan.scope, reservation.plan.checkpoint_id);
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &reservation.plan.scope.space_id).await? > i64::try_from(current_now_ms).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let mut published = txn.execute(query("MATCH (r:ArtifactCasReference {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.plan AS plan").param("key", scope_key.clone())).await.map_err(backend)?;
        if let Some(row) = published.next(txn.handle()).await.map_err(backend)? {
            let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
            if event.is_some()
                || row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)?
                || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)?
                || stored.value.as_ref() != encoded
            {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        drop(published);
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", scope_key.clone()))
            .await
            .map_err(backend)?;
        let row = current.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()))?;
        let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
        if row.get::<i64>("generation").map_err(backend)? != i64::try_from(reservation.generation).map_err(backend)?
            || row.get::<i64>("writeEpoch").map_err(backend)? != i64::try_from(reservation.write_epoch).map_err(backend)?
            || row.get::<i64>("expiresAtMs").map_err(backend)? != i64::try_from(reservation.expires_at_ms).map_err(backend)?
            || reservation.expires_at_ms <= current_now_ms
            || stored.value.as_ref() != encoded
        {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        drop(current);
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let id = time_ordered_id();
        let recorded_at_ms = now_ms();
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let mut counter = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        let seq: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter query returned no row".into()))?.get("seq").map_err(backend)?;
        drop(counter);
        let public_seq = u64::try_from(seq).map_err(backend)?;
        if public_seq > DIRECTORY_WIRE_INTEGER_MAX {
            return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
        }
        let full = DirectoryEvent { seq: public_seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
        validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
        txn.run(query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
            .param("seq", seq).param("id", id.clone()).param("hlc_physical", event.hlc.physical_ms).param("hlc_logical", i64::from(event.hlc.logical)).param("actor_kind", actor_kind_to_str(event.actor.kind)).param("actor_id", event.actor.id.clone()).param("space_id", event.space_id.clone()).param("user_id", event.user_id.clone()).param("kind", kind).param("payload", payload_value.to_string()).param("recorded_at", recorded_at_ms)).await.map_err(backend)?;
        txn.run(
            query("CREATE (:ArtifactAuthorityEvent {eventSeq: $event_seq, scopeCheckpointKey: $key, payload: $payload})").param("event_seq", seq).param("key", scope_key.clone()).param("payload", semio_framework_pack_json::to_json_string(checkpoint)),
        )
        .await
        .map_err(backend)?;
        self.project(&mut txn, &full).await?;
        self.project_verified_checkpoint(&mut txn, &full, checkpoint).await?;
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'publish', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, eventSeq: $event_seq, plan: $plan})")
            .param("generation", generation).param("key", scope_key).param("space_id", reservation.plan.scope.space_id.clone()).param("document_id", reservation.plan.scope.document_id.clone()).param("checkpoint_id", hex_lower(&reservation.plan.checkpoint_id.0)).param("write_epoch", i64::try_from(reservation.write_epoch).map_err(backend)?).param("expires_at", i64::try_from(reservation.expires_at_ms).map_err(backend)?).param("event_seq", seq).param("plan", encoded)).await.map_err(backend)?;
        cas_project_publish(&mut txn, reservation, generation).await?;
        if let Some(completion) = completion {
            validate_checkpoint_publication_completion(completion)?;
            let mut updated = txn
                .execute(
                    query("MATCH (r:CheckpointPublicationReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) SET r.disposition = 'completed', r.checkpointId = $checkpoint_id, r.completedAt = $completed_at RETURN count(r) AS changed")
                        .param("key", checkpoint_publication_receipt_key(&completion.actor_user_id, &completion.correlation_id))
                        .param("command_sha256", completion.command_sha256.clone())
                        .param("checkpoint_id", hex_lower(&completion.checkpoint_id.0))
                        .param("completed_at", completion.completed_at),
                )
                .await
                .map_err(backend)?;
            let changed = updated.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("checkpoint publication completion returned no row".into()))?.get::<i64>("changed").map_err(backend)?;
            drop(updated);
            if changed != 1 {
                return Err(DirectoryError::Conflict("checkpoint publication claim is missing, substituted, or already completed".into()));
            }
        }
        txn.commit().await.map_err(backend)?;
        Ok(vec![full])
    }
}

impl ArtifactCasDirectory for Neo4jDirectory {
    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.reserve_artifact_cas_attempt(plan, expires_at_ms, now_ms).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        current_now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.append_reserved_artifact_checkpoint_attempt(event, checkpoint, reservation, completion, current_now_ms).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let mut head = self.graph.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation")).await.map_err(backend)?;
        match head.next().await.map_err(backend)? {
            Some(row) => u64::try_from(row.get::<i64>("generation").map_err(backend)?).map_err(backend),
            None => Ok(0),
        }
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        let mut result = self.graph.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
        let bytes: neo4rs::BoltBytes = row.get("coordinator").map_err(backend)?;
        bytes.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let mut head = self.graph.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation")).await.map_err(backend)?;
        let current = match head.next().await.map_err(backend)? {
            Some(row) => row.get::<i64>("generation").map_err(backend)?,
            None => 0,
        };
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (e:ArtifactCasLedgerEvent) WHERE e.generation > $after AND e.generation <= $through RETURN e.generation AS generation, e.plan AS plan ORDER BY e.generation LIMIT $limit")
                    .param("after", after)
                    .param("through", through)
                    .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut next = after;
        let mut objects = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            next = row.get("generation").map_err(backend)?;
            if let Ok(plan) = row.get::<neo4rs::BoltBytes>("plan") {
                objects.extend(decode_artifact_cas_ownership_v1(&plan.value).map_err(|error| DirectoryError::Backend(error.to_string()))?.objects);
            }
        }
        objects.sort_by_key(|object| (object.space_id.clone(), object.kind, object.digest.0));
        objects.dedup();
        Ok(ArtifactCasSweepCandidatePage { observed_generation: through_generation, next_generation: u64::try_from(next).map_err(backend)?, objects })
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        let token = cas_object_token(key);
        let mut result = self.graph.execute(
            query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation >= $observed AS observed, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved")
                .param("observed", i64::try_from(observed_generation).map_err(backend)?)
                .param("space_id", key.space_id.clone())
                .param("token", token)
                .param("now", i64::try_from(now_ms).map_err(backend)?),
        ).await.map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS sweep requires an initialized ledger".into()))?;
        if !row.get::<bool>("observed").map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        Ok(row.get::<bool>("referenced").map_err(backend)? || row.get::<bool>("reserved").map_err(backend)?)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        if lease_token == [0; 32] || expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into()));
        }
        let token = cas_object_token(key);
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        if cas_lock_space(&mut txn, &key.space_id).await? > now {
            txn.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let mut epoch_result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) SET b.fenceEpoch = coalesce(b.fenceEpoch, 0) + 1, b.leaseToken = $lease_token, b.leaseExpiresAtMs = $expires_at RETURN b.fenceEpoch AS epoch")
                    .param("space_id", key.space_id.clone())
                    .param("lease_token", lease_token.to_vec())
                    .param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let physical_epoch: i64 = epoch_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier epoch returned no row".into()))?.get("epoch").map_err(backend)?;
        drop(epoch_result);
        let mut result = txn.execute(query("MATCH (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN h.generation AS generation, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved").param("space_id", key.space_id.clone()).param("token", token).param("now", now)).await.map_err(backend)?;
        let row = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("artifact CAS sweep requires an initialized ledger".into()))?;
        let generation: i64 = row.get("generation").map_err(backend)?;
        if generation < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let referenced: bool = row.get("referenced").map_err(backend)?;
        let reserved: bool = row.get("reserved").map_err(backend)?;
        drop(result);
        if referenced || reserved {
            txn.run(query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) REMOVE b.leaseToken, b.leaseExpiresAtMs").param("space_id", key.space_id.clone())).await.map_err(backend)?;
            txn.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let mut identity = txn.execute(query("MATCH (b:ArtifactCasBarrierIdentity {id: 'singleton'}) RETURN b.coordinatorId AS coordinator")).await.map_err(backend)?;
        let identity_row = identity.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS barrier coordinator identity is missing".into()))?;
        let coordinator: neo4rs::BoltBytes = identity_row.get("coordinator").map_err(backend)?;
        let coordinator_id = coordinator.value.to_vec().try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?;
        drop(identity);
        txn.commit().await.map_err(backend)?;
        Ok(Some(ArtifactCasDeleteFence::new(key.clone(), observed_generation, coordinator_id, u64::try_from(physical_epoch).map_err(backend)?, lease_token)))
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        let token = cas_object_token(fence.object());
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}), (i:ArtifactCasBarrierIdentity {id: 'singleton'}), (h:ArtifactCasLedgerHead {id: 'singleton'}) RETURN b.fenceEpoch = $epoch AND b.leaseToken = $lease_token AND b.leaseExpiresAtMs > $now AND i.coordinatorId = $coordinator AND h.generation >= $observed AS leaseValid, EXISTS { MATCH (r:ArtifactCasReference {spaceId: $space_id}) WHERE $token IN r.objects } AS referenced, EXISTS { MATCH (r:ArtifactCasReservation {spaceId: $space_id}) WHERE r.expiresAtMs > $now AND $token IN r.objects } AS reserved")
                    .param("space_id", fence.object().space_id.clone())
                    .param("epoch", i64::try_from(fence.physical_epoch()).map_err(backend)?)
                    .param("lease_token", fence.lease_token().to_vec())
                    .param("now", i64::try_from(now_ms).map_err(backend)?)
                    .param("coordinator", fence.coordinator_id().to_vec())
                    .param("observed", i64::try_from(fence.ledger_generation()).map_err(backend)?)
                    .param("token", token),
            )
            .await
            .map_err(backend)?;
        let Some(row) = result.next(txn.handle()).await.map_err(backend)? else {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        };
        let lease_valid: bool = row.get("leaseValid").map_err(backend)?;
        let referenced: bool = row.get("referenced").map_err(backend)?;
        let reserved: bool = row.get("reserved").map_err(backend)?;
        drop(result);
        txn.commit().await.map_err(backend)?;
        Ok(lease_valid && !referenced && !reserved)
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        if expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into()));
        }
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token AND b.leaseExpiresAtMs > $now SET b.leaseExpiresAtMs = $expires_at RETURN count(b) AS renewed")
                    .param("space_id", fence.object().space_id.clone())
                    .param("lease_token", fence.lease_token().to_vec())
                    .param("now", i64::try_from(now_ms).map_err(backend)?)
                    .param("expires_at", i64::try_from(expires_at_ms).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let renewed: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease renewal returned no row".into()))?.get("renewed").map_err(backend)?;
        drop(result);
        if renewed != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        cas_lock_space(&mut txn, &fence.object().space_id).await?;
        let mut result = txn
            .execute(
                query("MATCH (b:ArtifactCasSpaceBarrier {spaceId: $space_id}) WHERE b.leaseToken = $lease_token REMOVE b.leaseToken, b.leaseExpiresAtMs RETURN count(b) AS released")
                    .param("space_id", fence.object().space_id.clone())
                    .param("lease_token", fence.lease_token().to_vec()),
            )
            .await
            .map_err(backend)?;
        let released: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact CAS deletion lease release returned no row".into()))?.get("released").map_err(backend)?;
        drop(result);
        if released != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }
}
