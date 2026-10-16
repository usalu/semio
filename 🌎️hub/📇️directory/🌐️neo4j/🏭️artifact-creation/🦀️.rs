//! 🏭️ Artifact creation claims, facts, recovery and document genesis.

use super::*;

impl Neo4jDirectory {
    /// 🔁️ One attempt of [`ArtifactCreationDirectory::claim_artifact_creation`]; the trait method retries it while Neo4j reports a transient fault.
    async fn claim_artifact_creation_attempt(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &intent.actor.user_id, &intent.request.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut txn, &intent.actor, &intent.scope.space_id, None).await?;
        if intent.accepted_at_ms > observed_now || observed_now >= intent.deadline_ms {
            return Err(DirectoryError::Conflict("artifact creation acceptance is outside its live server deadline".into()));
        }
        let facts = artifact_creation_facts(&mut txn, &request_key).await?;
        if !facts.is_empty() {
            let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
            if operation.intent.command_sha256 != intent.command_sha256 || operation.intent.scope.space_id != intent.scope.space_id {
                return Err(DirectoryError::Conflict("artifact creation request is already bound to another intent".into()));
            }
            txn.commit().await.map_err(backend)?;
            return Ok(ArtifactCreationClaimV1::Existing(operation));
        }
        let scope_key = document_scope_key_v1(&intent.scope);
        let mut occupied = txn
            .execute(
                query("OPTIONAL MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) OPTIONAL MATCH (f:ArtifactCreationFact {spaceId: $space_id, documentId: $document_id, revision: 1}) RETURN count(d) + count(f) AS count")
                    .param("scope_key", scope_key)
                    .param("space_id", intent.scope.space_id.clone())
                    .param("document_id", intent.scope.document_id.clone()),
            )
            .await
            .map_err(backend)?;
        let count: i64 = occupied.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("artifact creation occupancy returned no row".into()))?.get("count").map_err(backend)?;
        drop(occupied);
        if count != 0 {
            return Err(DirectoryError::Conflict("artifact creation document identity is already occupied".into()));
        }
        txn.run(
            query("MATCH (r:ArtifactCreationRequest {key: $key}) SET r.documentScopeKey = $scope_key, r.spaceId = $space_id, r.documentId = $document_id")
                .param("key", request_key.clone())
                .param("scope_key", document_scope_key_v1(&intent.scope))
                .param("space_id", intent.scope.space_id.clone())
                .param("document_id", intent.scope.document_id.clone()),
        )
        .await
        .map_err(backend)?;
        let fact = ArtifactCreationFactV1 {
            actor_user_id: intent.actor.user_id.clone(),
            request_id: intent.request.request_id.clone(),
            revision: 1,
            recorded_at_ms: intent.accepted_at_ms,
            body: ArtifactCreationFactBodyV1::Accepted { intent: intent.clone() },
        };
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(std::slice::from_ref(&fact))?;
        insert_artifact_creation_fact(&mut txn, &request_key, intent, &fact).await?;
        txn.commit().await.map_err(backend)?;
        Ok(ArtifactCreationClaimV1::Accepted(operation))
    }

    /// 🔁️ One attempt of [`ArtifactCreationDirectory::append_artifact_creation_fact`]; the trait method retries it while Neo4j reports a transient fault.
    ///
    /// ⏱️ A fact may not be stamped in the future. It may be stamped LATE: the durable
    /// `deadline_ms` is what closes an ABANDONED key, and refusing a `Prepared` past it here
    /// refused a genesis the guest had already produced — a 176 s honest creation on the biggest
    /// staged component (ticket 26/09/18 slice HC1, hub 7681). A key the recovery sweep did close
    /// is terminal, and `decide_artifact_creation_fact_append_v1` refuses the transition on it.
    async fn append_artifact_creation_fact_attempt(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &append.actor.user_id, &append.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut txn, &append.actor, &append.space_id, None).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if append.recorded_at_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation transition is outside its live server clock".into()));
        }
        if let Some(next) = decide_artifact_creation_fact_append_v1(&facts, append, observed_now)? {
            insert_artifact_creation_fact(&mut txn, &request_key, &operation.intent, &next).await?;
            facts.push(next);
        }
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        txn.commit().await.map_err(backend)?;
        Ok(operation)
    }

    /// 🔁️ One attempt of [`ArtifactCreationDirectory::artifact_creation_terminate_uncommitted`]; the trait method retries it while Neo4j reports a transient fault.
    async fn artifact_creation_terminate_uncommitted_attempt(&self, intent: &ArtifactCreationIntentV1, current_now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &intent.actor.user_id, &intent.request.request_id).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != *intent {
            return Err(DirectoryError::Conflict("artifact creation supervisor intent differs".into()));
        }
        if operation.receipt.is_some()
            || matches!(operation.phase, directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Cancelled | directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Failed)
        {
            txn.commit().await.map_err(backend)?;
            return Ok(operation);
        }
        let authority = validate_artifact_creation_authority(&mut txn, &intent.actor, &intent.scope.space_id, None).await;
        let observed_now = match authority {
            Ok(observed_now) => {
                if observed_now < intent.deadline_ms {
                    return Err(DirectoryError::Conflict("artifact creation still has live execution authority".into()));
                }
                observed_now
            }
            Err(DirectoryError::Unauthorized) => u64::try_from(now_ms()).map_err(backend)?,
            Err(error) => return Err(error),
        };
        if current_now_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation supervisor clock is in the future".into()));
        }
        let next = ArtifactCreationFactV1 { actor_user_id: intent.actor.user_id.clone(), request_id: intent.request.request_id.clone(), revision: operation.revision + 1, recorded_at_ms: observed_now, body: ArtifactCreationFactBodyV1::Failed };
        facts.push(next.clone());
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        insert_artifact_creation_fact(&mut txn, &request_key, intent, &next).await?;
        txn.commit().await.map_err(backend)?;
        Ok(operation)
    }

    /// 🔁️ One attempt of [`ArtifactCreationDirectory::append_document_genesis`]; the trait method retries it while Neo4j reports a transient fault.
    ///
    /// ⏱️ The durable `deadline_ms` closes an ABANDONED key; it is not a bound on how long an
    /// honest genesis may take. Refusing the publication past it stranded a creation whose
    /// pair was already prepared, in `preparing`, until the recovery sweep closed it (ticket
    /// 26/09/18 slice HC1, hub 7681: 128 s of guest genesis, then this). A closed key is
    /// terminal and `validate_document_genesis_append_v1` refuses a publication on it.
    async fn append_document_genesis_attempt(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&append.intent)?;
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let request_key = lock_artifact_creation_request(&mut txn, &append.intent.actor.user_id, &append.intent.request.request_id).await?;
        let lease_expires_at_ms = cas_lock_space(&mut txn, &append.intent.scope.space_id).await?;
        let mut writer = txn.execute(query("MERGE (c:DirectoryCounter {id: 'singleton'}) ON CREATE SET c.seq = 0 SET c.claimNonce = coalesce(c.claimNonce, 0) + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
        writer.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory writer lock returned no row".into()))?;
        drop(writer);
        #[cfg(test)]
        let observed_now_override = self.pause_genesis_before_authority_for_test().await;
        #[cfg(not(test))]
        let observed_now_override = None;
        let observed_now = validate_artifact_creation_authority(&mut txn, &append.intent.actor, &append.intent.scope.space_id, observed_now_override).await?;
        let mut facts = artifact_creation_facts(&mut txn, &request_key).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != append.intent {
            return Err(DirectoryError::Conflict("genesis creation accepted identity differs".into()));
        }
        if operation.receipt.is_some() {
            txn.commit().await.map_err(backend)?;
            return Ok(DocumentGenesisCommitV1::Existing(operation));
        }
        if append.now_ms > observed_now {
            return Err(DirectoryError::Conflict("genesis publication is outside its live server deadline".into()));
        }
        validate_document_genesis_append_v1(&operation, append)?;
        let checkpoint = &append.checkpoint;
        let reservation = &append.reservation;
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let current_now = i64::try_from(observed_now).map_err(backend)?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let token_expiry = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        if lease_expires_at_ms > current_now {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let scope_key = document_scope_key_v1(&checkpoint.scope);
        let mut occupied = txn.execute(query("OPTIONAL MATCH (d:DocumentDescriptor {scopeKey: $scope_key}) OPTIONAL MATCH (i:DocumentIndex {scopeKey: $scope_key}) OPTIONAL MATCH (c:ArtifactCheckpoint {spaceId: $space_id, documentId: $document_id}) RETURN count(d) + count(i) + count(c) AS count")
            .param("scope_key", scope_key).param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone())).await.map_err(backend)?;
        let count: i64 = occupied.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("genesis public occupancy returned no row".into()))?.get("count").map_err(backend)?;
        drop(occupied);
        if count != 0 {
            return Err(DirectoryError::Conflict("genesis scope is already publicly occupied".into()));
        }
        let mut published = txn
            .execute(query("MATCH (r:ArtifactCasReference {spaceId: $space_id, documentId: $document_id}) RETURN count(r) AS count").param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone()))
            .await
            .map_err(backend)?;
        let published_count: i64 = published.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("genesis CAS reference count returned no row".into()))?.get("count").map_err(backend)?;
        drop(published);
        if published_count != 0 {
            return Err(DirectoryError::Conflict("genesis scope has a CAS reference without its creation receipt".into()));
        }
        let checkpoint_key = checkpoint_key_v1(&checkpoint.scope, checkpoint.checkpoint_id);
        let mut current = txn
            .execute(query("MATCH (r:ArtifactCasReservation {scopeCheckpointKey: $key}) RETURN r.generation AS generation, r.writeEpoch AS writeEpoch, r.expiresAtMs AS expiresAtMs, r.plan AS plan").param("key", checkpoint_key.clone()))
            .await
            .map_err(backend)?;
        let row = current.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()))?;
        let stored: neo4rs::BoltBytes = row.get("plan").map_err(backend)?;
        if row.get::<i64>("generation").map_err(backend)? != token_generation
            || row.get::<i64>("writeEpoch").map_err(backend)? != token_epoch
            || row.get::<i64>("expiresAtMs").map_err(backend)? != token_expiry
            || token_expiry <= current_now
            || stored.value.as_ref() != encoded
        {
            return Err(DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()));
        }
        drop(current);
        let mut events = Vec::with_capacity(3);
        for event in &append.events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload = serde_json::Value::from(&event.body.to_value());
            let kind = payload.get("kind").and_then(|value| value.as_str()).unwrap_or_default().to_string();
            let mut counter = txn.execute(query("MATCH (c:DirectoryCounter {id: 'singleton'}) SET c.seq = c.seq + 1 RETURN c.seq AS seq")).await.map_err(backend)?;
            let sequence: i64 = counter.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("directory counter returned no row".into()))?.get("seq").map_err(backend)?;
            drop(counter);
            let seq = u64::try_from(sequence).map_err(backend)?;
            if seq > DIRECTORY_WIRE_INTEGER_MAX {
                return Err(DirectoryError::Conflict("directory event sequence exceeds the public integer boundary".into()));
            }
            let full = DirectoryEvent { seq, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            txn.run(query("CREATE (e:DirectoryEvent {seq: $seq, id: $id, hlcPhysical: $hlc_physical, hlcLogical: $hlc_logical, actorKind: $actor_kind, actorId: $actor_id, spaceId: $space_id, userId: $user_id, kind: $kind, payload: $payload, recordedAt: $recorded_at})")
                .param("seq", sequence).param("id", id).param("hlc_physical", event.hlc.physical_ms).param("hlc_logical", i64::from(event.hlc.logical)).param("actor_kind", actor_kind_to_str(event.actor.kind)).param("actor_id", event.actor.id.clone()).param("space_id", event.space_id.clone()).param("user_id", event.user_id.clone()).param("kind", kind).param("payload", payload.to_string()).param("recorded_at", recorded_at_ms)).await.map_err(backend)?;
            if events.len() == 2 {
                txn.run(
                    query("CREATE (:ArtifactAuthorityEvent {eventSeq: $event_seq, scopeCheckpointKey: $key, payload: $payload})")
                        .param("event_seq", sequence)
                        .param("key", checkpoint_key.clone())
                        .param("payload", semio_framework_pack_json::to_json_string(checkpoint)),
                )
                .await
                .map_err(backend)?;
            }
            self.project(&mut txn, &full).await?;
            if events.len() == 2 {
                self.project_verified_checkpoint(&mut txn, &full, checkpoint).await?;
            }
            events.push(full);
        }
        let generation = cas_generation(&mut txn).await?;
        txn.run(query("CREATE (:ArtifactCasLedgerEvent {generation: $generation, operation: 'publish', scopeCheckpointKey: $key, spaceId: $space_id, documentId: $document_id, checkpointId: $checkpoint_id, writeEpoch: $write_epoch, expiresAtMs: $expires_at, eventSeq: $event_seq, plan: $plan})")
            .param("generation", generation).param("key", checkpoint_key).param("space_id", checkpoint.scope.space_id.clone()).param("document_id", checkpoint.scope.document_id.clone()).param("checkpoint_id", hex_lower(&checkpoint.checkpoint_id.0)).param("write_epoch", token_epoch).param("expires_at", token_expiry).param("event_seq", i64::try_from(events[2].seq).map_err(backend)?).param("plan", encoded)).await.map_err(backend)?;
        cas_project_publish(&mut txn, reservation, generation).await?;
        let completion = document_genesis_completion_v1(&operation, append, &events)?;
        insert_artifact_creation_fact(&mut txn, &request_key, &append.intent, &completion).await?;
        facts.push(completion);
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        match txn.commit().await {
            Ok(()) => {
                #[cfg(test)]
                if self.genesis_test_control.fail_commit_ack.swap(false, std::sync::atomic::Ordering::SeqCst) {
                    return Ok(DocumentGenesisCommitV1::Indeterminate);
                }
                Ok(DocumentGenesisCommitV1::Committed { events, operation })
            }
            Err(_) => Ok(DocumentGenesisCommitV1::Indeterminate),
        }
    }
}

impl ArtifactCreationDirectory for Neo4jDirectory {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.claim_artifact_creation_attempt(intent).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        let key = artifact_creation_request_key(actor_user_id, request_id);
        let mut result = self.graph.execute(query("MATCH (:ArtifactCreationRequest {key: $key})-[:HAS_FACT]->(f:ArtifactCreationFact) RETURN f.payload AS payload ORDER BY f.revision LIMIT 4").param("key", key)).await.map_err(backend)?;
        let mut facts = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            facts.push(semio_framework_pack_json::from_json_str(&row.get::<String>("payload").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?);
        }
        Ok(facts)
    }

    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.append_artifact_creation_fact_attempt(append).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, current_now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.artifact_creation_terminate_uncommitted_attempt(intent, current_now_ms).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }

    async fn artifact_creation_recovery_candidates(&self, current_now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        if limit == 0 || limit > 256 {
            return Err(DirectoryError::Conflict("artifact creation recovery page is out of bounds".into()));
        }
        let mut result = self
            .graph
            .execute(
                query(
                    "MATCH (r:ArtifactCreationRequest)-[:HAS_FACT]->(a:ArtifactCreationFact {revision: 1})
             MATCH (r)-[:HAS_FACT]->(candidate:ArtifactCreationFact)
             WITH r, a, max(candidate.revision) AS latestRevision
             MATCH (r)-[:HAS_FACT]->(state:ArtifactCreationFact {revision: latestRevision})
             WHERE state.phase IN ['accepted','prepared'] AND (a.deadlineMs <= $now_ms OR state.phase = 'prepared')
             RETURN a.payload AS payload ORDER BY a.recordedAtMs, a.actorUserId, a.requestId LIMIT $limit",
                )
                .param("now_ms", i64::try_from(current_now_ms).map_err(backend)?)
                .param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut intents = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            let fact: ArtifactCreationFactV1 = semio_framework_pack_json::from_json_str(&row.get::<String>("payload").map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?;
            let ArtifactCreationFactBodyV1::Accepted { intent } = fact.body else {
                return Err(DirectoryError::Backend("artifact creation recovery row is not accepted".into()));
            };
            semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
            intents.push(intent);
        }
        Ok(intents)
    }

    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        let mut attempt = 0u32;
        loop {
            let outcome = self.append_document_genesis_attempt(append).await;
            match outcome {
                Err(error) if neo4j_transient_retry_pause(&error, &mut attempt).await => continue,
                outcome => return outcome,
            }
        }
    }
}
