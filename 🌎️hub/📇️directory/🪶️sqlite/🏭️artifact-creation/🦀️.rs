//! 🏭️ Artifact creation claims, facts, recovery and document genesis.

use super::*;

impl SqliteDirectory {
    fn creation_facts(conn: &Connection, user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        let mut query = conn.prepare("SELECT payload FROM hub_artifact_creation_fact WHERE actor_user_id = ?1 AND request_id = ?2 ORDER BY revision LIMIT 4").map_err(backend)?;
        let rows = query.query_map(rusqlite::params![user_id, request_id], |row| row.get::<_, String>(0)).map_err(backend)?;
        rows.map(|row| semio_framework_pack_json::from_json_str(&row.map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)).collect()
    }

    fn creation_authority(conn: &Connection, actor: &ArtifactCreationActorV1, space_id: &str, now_ms: u64) -> DirectoryResult<()> {
        let allowed: i64 = conn.query_row("SELECT EXISTS(SELECT 1 FROM hub_auth_session AS a JOIN hub_user AS u ON u.id = a.user_id JOIN hub_space AS s ON s.id = ?4 JOIN hub_space_membership AS m ON m.space_id = s.id AND m.user_id = u.id WHERE a.id = ?1 AND a.user_id = ?2 AND a.authorization_generation = ?3 AND a.revoked_at IS NULL AND a.expires_at > ?5 AND s.kind <> 'archive' AND m.role = 'author')", rusqlite::params![actor.session_id, actor.user_id, i64::try_from(actor.authorization_generation).map_err(backend)?, space_id, i64::try_from(now_ms).map_err(backend)?], |row| row.get(0)).map_err(backend)?;
        if allowed != 1 {
            return Err(DirectoryError::Unauthorized);
        }
        Ok(())
    }

    fn insert_creation_fact(tx: &Transaction<'_>, intent: &ArtifactCreationIntentV1, fact: &ArtifactCreationFactV1) -> DirectoryResult<()> {
        let phase = match fact.body {
            ArtifactCreationFactBodyV1::Accepted { .. } => "accepted",
            ArtifactCreationFactBodyV1::Prepared { .. } => "prepared",
            ArtifactCreationFactBodyV1::Committed { .. } => "committed",
            ArtifactCreationFactBodyV1::Cancelled => "cancelled",
            ArtifactCreationFactBodyV1::Failed => "failed",
        };
        let payload = semio_framework_pack_json::to_json_string(fact);
        if payload.len() > 8 * 1024 * 1024 {
            return Err(DirectoryError::Conflict("artifact creation fact exceeds its bounded envelope".into()));
        }
        tx.execute(
            "INSERT INTO hub_artifact_creation_fact(actor_user_id, request_id, revision, phase, space_id, document_id, deadline_ms, recorded_at_ms, payload) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            rusqlite::params![
                fact.actor_user_id,
                fact.request_id,
                i64::try_from(fact.revision).map_err(backend)?,
                phase,
                intent.scope.space_id,
                intent.scope.document_id,
                i64::try_from(intent.deadline_ms).map_err(backend)?,
                i64::try_from(fact.recorded_at_ms).map_err(backend)?,
                payload
            ],
        )
        .map_err(backend)?;
        Ok(())
    }
}

impl ArtifactCreationDirectory for SqliteDirectory {
    async fn claim_artifact_creation(&self, intent: &ArtifactCreationIntentV1) -> DirectoryResult<ArtifactCreationClaimV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let observed_now = u64::try_from(now_ms()).map_err(backend)?;
        if intent.accepted_at_ms > observed_now || observed_now >= intent.deadline_ms {
            return Err(DirectoryError::Conflict("artifact creation acceptance is outside its live server deadline".into()));
        }
        Self::creation_authority(&tx, &intent.actor, &intent.scope.space_id, observed_now)?;
        let facts = Self::creation_facts(&tx, &intent.actor.user_id, &intent.request.request_id)?;
        if !facts.is_empty() {
            let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
            if operation.intent.command_sha256 != intent.command_sha256 || operation.intent.scope.space_id != intent.scope.space_id {
                return Err(DirectoryError::Conflict("artifact creation request is already bound to another intent".into()));
            }
            return Ok(ArtifactCreationClaimV1::Existing(operation));
        }
        let occupied: i64 = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2 UNION ALL SELECT 1 FROM hub_artifact_creation_fact WHERE space_id = ?1 AND document_id = ?2)",
                rusqlite::params![intent.scope.space_id, intent.scope.document_id],
                |row| row.get(0),
            )
            .map_err(backend)?;
        if occupied != 0 {
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
        Self::insert_creation_fact(&tx, intent, &fact)?;
        tx.commit().map_err(backend)?;
        Ok(ArtifactCreationClaimV1::Accepted(operation))
    }

    async fn read_artifact_creation(&self, actor_user_id: &str, request_id: &str) -> DirectoryResult<Vec<ArtifactCreationFactV1>> {
        #[cfg(test)]
        if self.creation_read_failure.swap(false, std::sync::atomic::Ordering::SeqCst) {
            return Err(DirectoryError::Backend("injected creation receipt read failure".into()));
        }
        let conn = self.lock()?;
        Self::creation_facts(&conn, actor_user_id, request_id)
    }

    /// ⏱️ A fact may not be stamped in the future. It may be stamped LATE: the durable
    /// `deadline_ms` is what closes an ABANDONED key, and refusing a `Prepared` past it here
    /// refused a genesis the guest had already produced — a 176 s honest creation on the biggest
    /// staged component (ticket 26/09/18 slice HC1, hub 7681). A key the recovery sweep did close
    /// is terminal, and `decide_artifact_creation_fact_append_v1` refuses the transition on it.
    async fn append_artifact_creation_fact(&self, append: &ArtifactCreationFactAppendV1) -> DirectoryResult<ArtifactCreationOperationV1> {
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let observed_now = u64::try_from(now_ms()).map_err(backend)?;
        Self::creation_authority(&tx, &append.actor, &append.space_id, observed_now)?;
        let mut facts = Self::creation_facts(&tx, &append.actor.user_id, &append.request_id)?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if append.recorded_at_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation transition is outside its live server clock".into()));
        }
        if let Some(next) = decide_artifact_creation_fact_append_v1(&facts, append, observed_now)? {
            Self::insert_creation_fact(&tx, &operation.intent, &next)?;
            facts.push(next);
        }
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        tx.commit().map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_terminate_uncommitted(&self, intent: &ArtifactCreationIntentV1, current_now_ms: u64) -> DirectoryResult<ArtifactCreationOperationV1> {
        semio_hub_artifact_creation::io::validate_artifact_creation_intent_v1(&intent)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let observed_now = u64::try_from(now_ms()).map_err(backend)?;
        if current_now_ms > observed_now {
            return Err(DirectoryError::Conflict("artifact creation supervisor clock is in the future".into()));
        }
        let mut facts = Self::creation_facts(&tx, &intent.actor.user_id, &intent.request.request_id)?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != *intent {
            return Err(DirectoryError::Conflict("artifact creation supervisor intent differs".into()));
        }
        if operation.receipt.is_some()
            || matches!(operation.phase, directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Cancelled | directory::os_directory::schema::space_artifact_creation::SpaceArtifactCreationPhaseV1::Failed)
        {
            return Ok(operation);
        }
        match Self::creation_authority(&tx, &intent.actor, &intent.scope.space_id, observed_now) {
            Ok(()) if observed_now < intent.deadline_ms => return Err(DirectoryError::Conflict("artifact creation still has live execution authority".into())),
            Ok(()) | Err(DirectoryError::Unauthorized) => {}
            Err(error) => return Err(error),
        }
        let next = ArtifactCreationFactV1 { actor_user_id: intent.actor.user_id.clone(), request_id: intent.request.request_id.clone(), revision: operation.revision + 1, recorded_at_ms: observed_now, body: ArtifactCreationFactBodyV1::Failed };
        facts.push(next.clone());
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        Self::insert_creation_fact(&tx, intent, &next)?;
        tx.commit().map_err(backend)?;
        Ok(operation)
    }

    async fn artifact_creation_recovery_candidates(&self, now_ms: u64, limit: usize) -> DirectoryResult<Vec<ArtifactCreationIntentV1>> {
        if limit == 0 || limit > 256 {
            return Err(DirectoryError::Conflict("artifact creation recovery page is out of bounds".into()));
        }
        let conn = self.lock()?;
        let mut query = conn.prepare("SELECT a.payload FROM hub_artifact_creation_fact AS a WHERE a.revision = 1 AND NOT EXISTS(SELECT 1 FROM hub_artifact_creation_fact AS t WHERE t.actor_user_id = a.actor_user_id AND t.request_id = a.request_id AND t.phase IN ('committed','cancelled','failed')) AND (a.deadline_ms <= ?1 OR EXISTS(SELECT 1 FROM hub_artifact_creation_fact AS p WHERE p.actor_user_id = a.actor_user_id AND p.request_id = a.request_id AND p.phase = 'prepared')) ORDER BY a.recorded_at_ms, a.actor_user_id, a.request_id LIMIT ?2").map_err(backend)?;
        let rows = query.query_map(rusqlite::params![i64::try_from(now_ms).map_err(backend)?, limit as i64], |row| row.get::<_, String>(0)).map_err(backend)?;
        rows.map(|row| {
            let fact: ArtifactCreationFactV1 = semio_framework_pack_json::from_json_str(&row.map_err(backend)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(backend)?;
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
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let observed_now = u64::try_from(now_ms()).map_err(backend)?;
        Self::creation_authority(&tx, &append.intent.actor, &append.intent.scope.space_id, observed_now)?;
        let mut facts = Self::creation_facts(&tx, &append.intent.actor.user_id, &append.intent.request.request_id)?;
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if operation.intent != append.intent {
            return Err(DirectoryError::Conflict("genesis creation accepted identity differs".into()));
        }
        if operation.receipt.is_some() {
            return Ok(DocumentGenesisCommitV1::Existing(operation));
        }
        if append.now_ms > observed_now {
            return Err(DirectoryError::Conflict("genesis publication is outside its live server deadline".into()));
        }
        semio_hub_directory::validate_document_genesis_append_v1(&operation, append)?;
        let checkpoint = &append.checkpoint;
        let reservation = &append.reservation;
        let occupied: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM hub_document_descriptor WHERE space_id = ?1 AND document_id = ?2 UNION ALL SELECT 1 FROM hub_document_index WHERE space_id = ?1 AND document_id = ?2 UNION ALL SELECT 1 FROM hub_artifact_checkpoint WHERE space_id = ?1 AND document_id = ?2)", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], |row| row.get(0)).map_err(backend)?;
        if occupied != 0 {
            return Err(DirectoryError::Conflict("genesis scope is already publicly occupied".into()));
        }
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let now = i64::try_from(observed_now).map_err(backend)?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let live_lease: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_delete_lease WHERE space_id = ?1 AND expires_at_ms > ?2)", rusqlite::params![checkpoint.scope.space_id, now], |row| row.get(0)).map_err(backend)?;
        if live_lease != 0 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let published: i64 =
            tx.query_row("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2)", rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id], |row| row.get(0)).map_err(backend)?;
        if published != 0 {
            return Err(DirectoryError::Conflict("genesis scope has a CAS reference without its creation receipt".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = tx
            .query_row(
                "SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, plan)| *generation != token_generation || *epoch != token_epoch || *expiry <= now || *expiry != i64::try_from(reservation.expires_at_ms).unwrap_or(-1) || plan != &encoded) {
            return Err(DirectoryError::Conflict("genesis CAS reservation is missing, expired or substituted".into()));
        }
        let mut events = Vec::with_capacity(3);
        for event in &append.events {
            let full = self.persist_event(&tx, event)?;
            if events.len() == 2 {
                tx.execute(
                    "INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
                    rusqlite::params![i64::try_from(full.seq).map_err(backend)?, checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), semio_framework_pack_json::to_json_string(checkpoint)],
                )
                .map_err(backend)?;
            }
            self.project(&tx, &full)?;
            if events.len() == 2 {
                self.project_verified_checkpoint(&tx, &full, checkpoint)?;
            }
            events.push(full);
        }
        let generation = Self::cas_generation(&tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES (?1, 'publish', ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![
                generation,
                checkpoint.scope.space_id,
                checkpoint.scope.document_id,
                checkpoint.checkpoint_id.0.as_slice(),
                token_epoch,
                i64::try_from(reservation.expires_at_ms).map_err(backend)?,
                i64::try_from(events[2].seq).map_err(backend)?,
                encoded
            ],
        )
        .map_err(backend)?;
        Self::cas_project_publish(&tx, reservation, generation)?;
        let completion = semio_hub_directory::document_genesis_completion_v1(&operation, append, &events)?;
        Self::insert_creation_fact(&tx, &append.intent, &completion)?;
        facts.push(completion);
        let operation = semio_hub_artifact_creation::io::fold_artifact_creation_facts_v1(&facts)?;
        if tx.commit().is_err() {
            return Ok(DocumentGenesisCommitV1::Indeterminate);
        }
        #[cfg(test)]
        {
            let failure = self.genesis_commit_ack_failure.swap(0, std::sync::atomic::Ordering::SeqCst);
            if failure == 2 {
                self.creation_read_failure.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            if failure == 3 {
                self.event_read_failure.store(true, std::sync::atomic::Ordering::SeqCst);
            }
            if failure != 0 {
                return Ok(DocumentGenesisCommitV1::Indeterminate);
            }
        }
        Ok(DocumentGenesisCommitV1::Committed { events, operation })
    }
}
