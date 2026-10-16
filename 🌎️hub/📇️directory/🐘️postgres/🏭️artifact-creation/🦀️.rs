//! 🏭️ Artifact creation claims, facts, recovery and document genesis.

use super::*;

fn artifact_creation_phase(body: &ArtifactCreationFactBodyV1) -> &'static str {
    match body {
        ArtifactCreationFactBodyV1::Accepted { .. } => "accepted",
        ArtifactCreationFactBodyV1::Prepared { .. } => "prepared",
        ArtifactCreationFactBodyV1::Committed { .. } => "committed",
        ArtifactCreationFactBodyV1::Cancelled => "cancelled",
        ArtifactCreationFactBodyV1::Failed => "failed",
    }
}

async fn lock_artifact_creation_request(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, actor_user_id: &str, request_id: &str) -> DirectoryResult<()> {
    let key = format!("v1:{}:{}:{}{}", actor_user_id.len(), request_id.len(), actor_user_id, request_id);
    sqlx_core::query::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 1634562391))").bind(key).execute(&mut **tx).await.map_err(backend)?;
    Ok(())
}

async fn artifact_creation_facts(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
    let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as("SELECT payload FROM hub_artifact_creation_fact WHERE actor_user_id = $1 AND request_id = $2 ORDER BY revision LIMIT 4 FOR UPDATE")
        .bind(actor_user_id)
        .bind(request_id)
        .fetch_all(&mut **tx)
        .await
        .map_err(backend)?;
    rows.into_iter().map(|(payload,)| ArtifactCreationFactV1::from_value(DslValue::from(payload)).map_err(backend)).collect()
}

async fn validate_artifact_creation_authority(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, actor: &ArtifactCreationActorV1, space_id: &str, observed_now_override: Option<u64>) -> DirectoryResult<u64> {
    let user: Option<(String,)> = sqlx_core::query_as::query_as("SELECT id FROM hub_user WHERE id = $1 FOR UPDATE").bind(&actor.user_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    if user.is_none() {
        return Err(DirectoryError::Unauthorized);
    }
    let session: Option<(String, i64, i64, Option<i64>)> =
        sqlx_core::query_as::query_as("SELECT user_id, authorization_generation, expires_at, revoked_at FROM hub_auth_session WHERE id = $1 FOR UPDATE").bind(&actor.session_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    let generation = i64::try_from(actor.authorization_generation).map_err(backend)?;
    if session.as_ref().is_none_or(|(user_id, established_generation, _, revoked_at)| user_id != &actor.user_id || *established_generation != generation || revoked_at.is_some()) {
        return Err(DirectoryError::Unauthorized);
    }
    let space: Option<(String,)> = sqlx_core::query_as::query_as("SELECT kind FROM hub_space WHERE id = $1 FOR UPDATE").bind(space_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    if space.as_ref().is_none_or(|(kind,)| kind == "archive") {
        return Err(DirectoryError::Unauthorized);
    }
    let membership: Option<(String,)> = sqlx_core::query_as::query_as("SELECT role FROM hub_space_membership WHERE space_id = $1 AND user_id = $2 FOR UPDATE").bind(space_id).bind(&actor.user_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    if membership.as_ref().is_none_or(|(role,)| role != "author") {
        return Err(DirectoryError::Unauthorized);
    }
    let observed_now = observed_now_override.unwrap_or(u64::try_from(now_ms()).map_err(backend)?);
    let observed_now_i64 = i64::try_from(observed_now).map_err(backend)?;
    if session.is_none_or(|(_, _, expires_at, _)| expires_at <= observed_now_i64) {
        return Err(DirectoryError::Unauthorized);
    }
    Ok(observed_now)
}

async fn insert_artifact_creation_fact(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, intent: &ArtifactCreationIntentV1, fact: &ArtifactCreationFactV1) -> DirectoryResult<()> {
    let payload = serde_json::Value::from(&fact.to_value());
    if payload.to_string().len() > 8 * 1024 * 1024 {
        return Err(DirectoryError::Conflict("artifact creation fact exceeds its bounded envelope".into()));
    }
    sqlx_core::query::query("INSERT INTO hub_artifact_creation_fact(actor_user_id, request_id, revision, phase, space_id, document_id, deadline_ms, recorded_at_ms, payload) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9)")
        .bind(&fact.actor_user_id)
        .bind(&fact.request_id)
        .bind(i64::try_from(fact.revision).map_err(backend)?)
        .bind(artifact_creation_phase(&fact.body))
        .bind(&intent.scope.space_id)
        .bind(&intent.scope.document_id)
        .bind(i64::try_from(intent.deadline_ms).map_err(backend)?)
        .bind(i64::try_from(fact.recorded_at_ms).map_err(backend)?)
        .bind(payload)
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    Ok(())
}

impl ArtifactCreationDirectory for PostgresDirectory {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        lock_artifact_creation_request(&mut tx, &intent.actor.user_id, &intent.request.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut tx, &intent.actor, &intent.scope.space_id, None).await?;
        if intent.accepted_at_ms > observed_now || observed_now >= intent.deadline_ms {
            return Err(DirectoryError::Conflict("artifact creation acceptance is outside its live server deadline".into()));
        }
        let facts = artifact_creation_facts(&mut tx, &intent.actor.user_id, &intent.request.request_id).await?;
        if !facts.is_empty() {
            let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
            if operation.intent.command_sha256 != intent.command_sha256 || operation.intent.scope.space_id != intent.scope.space_id {
                return Err(DirectoryError::Conflict("artifact creation request is already bound to another intent".into()));
            }
            tx.commit().await.map_err(backend)?;
            return Ok(ArtifactCreationClaimV1::Existing(operation));
        }
        let (occupied,): (bool,) =
            sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2) OR EXISTS(SELECT 1 FROM hub_artifact_creation_fact WHERE space_id = $1 AND document_id = $2)")
                .bind(&intent.scope.space_id)
                .bind(&intent.scope.document_id)
                .fetch_one(&mut *tx)
                .await
                .map_err(backend)?;
        if occupied {
            return Err(DirectoryError::Conflict("artifact creation document identity is already occupied".into()));
        }
        let fact = ArtifactCreationFactV1 {
            actor_user_id: intent.actor.user_id.clone(),
            request_id: intent.request.request_id.clone(),
            revision: 1,
            recorded_at_ms: intent.accepted_at_ms,
            body: ArtifactCreationFactBodyV1::Accepted { intent: intent.clone() },
        };
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(std::slice::from_ref(&fact))?;
        insert_artifact_creation_fact(&mut tx, intent, &fact).await?;
        tx.commit().await.map_err(backend)?;
        Ok(ArtifactCreationClaimV1::Accepted(operation))
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let facts = artifact_creation_facts(&mut tx, actor_user_id, request_id).await?;
        tx.commit().await.map_err(backend)?;
        Ok(facts)
    }

    /// ⏱️ A fact may not be stamped in the future. It may be stamped LATE: the durable
    /// `deadline_ms` is what closes an ABANDONED key, and refusing a `Prepared` past it here
    /// refused a genesis the guest had already produced — a 176 s honest creation on the biggest
    /// staged component (ticket 26/09/18 slice HC1, hub 7681). A key the recovery sweep did close
    /// is terminal, and `decide_artifact_creation_fact_append_v1` refuses the transition on it.
    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut tx = self.pool.begin().await.map_err(backend)?;
        lock_artifact_creation_request(&mut tx, &append.actor.user_id, &append.request_id).await?;
        let observed_now = validate_artifact_creation_authority(&mut tx, &append.actor, &append.space_id, None).await?;
        let mut facts = artifact_creation_facts(&mut tx, &append.actor.user_id, &append.request_id).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if append.recorded_at_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation transition is outside its live server clock".into()));
        }
        if let Some(next) = decide_artifact_creation_fact_append_v1(&facts, append, observed_now)? {
            insert_artifact_creation_fact(&mut tx, &operation.intent, &next).await?;
            facts.push(next);
        }
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        tx.commit().await.map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, current_now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        lock_artifact_creation_request(&mut tx, &intent.actor.user_id, &intent.request.request_id).await?;
        let mut facts = artifact_creation_facts(&mut tx, &intent.actor.user_id, &intent.request.request_id).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != *intent {
            return Err(DirectoryError::Conflict("artifact creation supervisor intent differs".into()));
        }
        if operation.receipt.is_some()
            || matches!(operation.phase, directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Cancelled | directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Failed)
        {
            tx.commit().await.map_err(backend)?;
            return Ok(operation);
        }
        let authority = validate_artifact_creation_authority(&mut tx, &intent.actor, &intent.scope.space_id, None).await;
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
        insert_artifact_creation_fact(&mut tx, intent, &next).await?;
        tx.commit().await.map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_recovery_candidates(&self, current_now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        if limit == 0 || limit > 256 {
            return Err(DirectoryError::Conflict("artifact creation recovery page is out of bounds".into()));
        }
        let rows: Vec<(serde_json::Value,)> = sqlx_core::query_as::query_as(
            "SELECT a.payload FROM hub_artifact_creation_fact AS a WHERE a.revision = 1 AND NOT EXISTS(SELECT 1 FROM hub_artifact_creation_fact AS t WHERE t.actor_user_id = a.actor_user_id AND t.request_id = a.request_id AND t.phase IN ('committed','cancelled','failed')) AND (a.deadline_ms <= $1 OR EXISTS(SELECT 1 FROM hub_artifact_creation_fact AS p WHERE p.actor_user_id = a.actor_user_id AND p.request_id = a.request_id AND p.phase = 'prepared')) ORDER BY a.recorded_at_ms, a.actor_user_id, a.request_id LIMIT $2",
        )
        .bind(i64::try_from(current_now_ms).map_err(backend)?)
        .bind(i64::try_from(limit).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter()
            .map(|(payload,)| {
                let fact = ArtifactCreationFactV1::from_value(DslValue::from(payload)).map_err(backend)?;
                let ArtifactCreationFactBodyV1::Accepted { intent } = fact.body else {
                    return Err(DirectoryError::Backend("artifact creation recovery row is not accepted".into()));
                };
                semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
                Ok(intent)
            })
            .collect()
    }

    /// ⏱️ The durable `deadline_ms` closes an ABANDONED key; it is not a bound on how long an
    /// honest genesis may take. Refusing the publication past it stranded a creation whose
    /// pair was already prepared, in `preparing`, until the recovery sweep closed it (ticket
    /// 26/09/18 slice HC1, hub 7681: 128 s of guest genesis, then this). A closed key is
    /// terminal and `validate_document_genesis_append_v1` refuses a publication on it.
    async fn append_document_genesis(&self, append: &DocumentGenesisAppendV1) -> DirectoryResult<DocumentGenesisCommitV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&append.intent)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        lock_artifact_creation_request(&mut tx, &append.intent.actor.user_id, &append.intent.request.request_id).await?;
        cas_lock_space(&mut tx, &append.intent.scope.space_id).await?;
        let _: (i64,) = sqlx_core::query_as::query_as("SELECT seq FROM hub_directory_event_head WHERE singleton FOR UPDATE").fetch_one(&mut *tx).await.map_err(backend)?;
        #[cfg(test)]
        let observed_now_override = self.pause_genesis_before_authority_for_test().await;
        #[cfg(not(test))]
        let observed_now_override = None;
        let observed_now = validate_artifact_creation_authority(&mut tx, &append.intent.actor, &append.intent.scope.space_id, observed_now_override).await?;
        let mut facts = artifact_creation_facts(&mut tx, &append.intent.actor.user_id, &append.intent.request.request_id).await?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != append.intent {
            return Err(DirectoryError::Conflict("genesis creation accepted identity differs".into()));
        }
        if operation.receipt.is_some() {
            tx.commit().await.map_err(backend)?;
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
        let (occupied,): (bool,) = sqlx_core::query_as::query_as(
            "SELECT EXISTS(SELECT 1 FROM hub_document_descriptor WHERE space_id = $1 AND document_id = $2) OR EXISTS(SELECT 1 FROM hub_document_index WHERE space_id = $1 AND document_id = $2) OR EXISTS(SELECT 1 FROM hub_artifact_checkpoint WHERE space_id = $1 AND document_id = $2)",
        )
        .bind(&checkpoint.scope.space_id)
        .bind(&checkpoint.scope.document_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(backend)?;
        if occupied {
            return Err(DirectoryError::Conflict("genesis scope is already publicly occupied".into()));
        }
        let (live_lease,): (bool,) =
            sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_delete_lease WHERE space_id = $1 AND expires_at_ms > $2)").bind(&checkpoint.scope.space_id).bind(current_now).fetch_one(&mut *tx).await.map_err(backend)?;
        if live_lease {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let (published,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2)")
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        if published {
            return Err(DirectoryError::Conflict("genesis scope has a CAS reference without its creation receipt".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 FOR UPDATE")
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, plan)| *generation != token_generation || *epoch != token_epoch || *expiry != token_expiry || *expiry <= current_now || plan != &encoded) {
            return Err(DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()));
        }
        let mut events = Vec::with_capacity(3);
        for event in &append.events {
            let id = time_ordered_id();
            let recorded_at_ms = now_ms();
            let payload = serde_json::Value::from(&event.body.to_value());
            let kind = payload.get("kind").and_then(|value| value.as_str()).unwrap_or_default();
            let (event_seq,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
            let full = DirectoryEvent {
                seq: u64::try_from(event_seq).map_err(backend)?,
                id: id.clone(),
                hlc: event.hlc,
                actor: event.actor.clone(),
                space_id: event.space_id.clone(),
                user_id: event.user_id.clone(),
                body: event.body.clone(),
                recorded_at_ms,
            };
            validate_directory_event_page_event(&full).map_err(|_| DirectoryError::Conflict("directory event violates the bounded event-page contract".into()))?;
            sqlx_core::query::query("INSERT INTO hub_directory_event(seq, id, hlc_physical, hlc_logical, actor_kind, actor_id, space_id, user_id, kind, payload, recorded_at) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11)")
                .bind(event_seq)
                .bind(&id)
                .bind(event.hlc.physical_ms)
                .bind(i64::from(event.hlc.logical))
                .bind(actor_kind_to_str(event.actor.kind))
                .bind(&event.actor.id)
                .bind(&event.space_id)
                .bind(&event.user_id)
                .bind(kind)
                .bind(&payload)
                .bind(recorded_at_ms)
                .execute(&mut *tx)
                .await
                .map_err(backend)?;
            if events.len() == 2 {
                sqlx_core::query::query("INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES ($1,$2,$3,$4,$5)")
                    .bind(event_seq)
                    .bind(&checkpoint.scope.space_id)
                    .bind(&checkpoint.scope.document_id)
                    .bind(checkpoint.checkpoint_id.0.as_slice())
                    .bind(serde_json::Value::from(&checkpoint.to_value()))
                    .execute(&mut *tx)
                    .await
                    .map_err(backend)?;
            }
            self.project(&mut tx, &full).await?;
            if events.len() == 2 {
                self.project_verified_checkpoint(&mut tx, &full, checkpoint).await?;
            }
            events.push(full);
        }
        let generation = cas_generation(&mut tx).await?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES ($1,'publish',$2,$3,$4,$5,$6,$7,$8)")
            .bind(generation)
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .bind(token_epoch)
            .bind(token_expiry)
            .bind(i64::try_from(events[2].seq).map_err(backend)?)
            .bind(encoded)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        cas_project_publish(&mut tx, reservation, generation).await?;
        let completion = document_genesis_completion_v1(&operation, append, &events)?;
        insert_artifact_creation_fact(&mut tx, &append.intent, &completion).await?;
        facts.push(completion);
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        match tx.commit().await {
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
