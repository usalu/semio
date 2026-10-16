//! 🧺️ Artifact CAS reservations, sweep candidates and delete fences.

use super::*;

impl SqliteDirectory {
    pub(super) fn cas_generation(tx: &Transaction<'_>) -> DirectoryResult<i64> {
        let current: i64 = tx.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        let next = current.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS ledger generation overflow".into()))?;
        tx.execute("UPDATE hub_artifact_cas_ledger_head SET generation = ?1 WHERE singleton = 1 AND generation = ?2", rusqlite::params![next, current]).map_err(backend)?;
        Ok(next)
    }

    fn cas_reservation_barrier(tx: &Transaction<'_>, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
        let current: Option<(i64, Option<i64>)> = tx.query_row("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = ?1", [space_id], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(backend)?;
        if current.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now_ms) {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let epoch = match current {
            Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
            None => 1,
        };
        tx.execute(
            "INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES (?1, ?2, NULL, NULL) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = NULL, expires_at_ms = NULL",
            rusqlite::params![space_id, epoch],
        )
        .map_err(backend)?;
        let coordinator: Vec<u8> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        Ok((coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?, u64::try_from(epoch).map_err(backend)?))
    }

    fn cas_insert_objects(tx: &Transaction<'_>, table: &str, plan: &ArtifactCasOwnershipPlanV1) -> DirectoryResult<()> {
        if !matches!(table, "hub_artifact_cas_reservation_object" | "hub_artifact_cas_reference_object") {
            return Err(DirectoryError::Backend("invalid artifact CAS projection table".into()));
        }
        let sql = format!("INSERT INTO {table}(space_id, document_id, checkpoint_id, kind, object_digest) VALUES (?1, ?2, ?3, ?4, ?5)");
        for object in &plan.objects {
            tx.execute(&sql, rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice(), object.kind.name(), object.digest.0.as_slice()]).map_err(backend)?;
        }
        Ok(())
    }

    pub(super) fn cas_project_reserve(tx: &Transaction<'_>, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
        let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let generation = i64::try_from(reservation.generation).map_err(backend)?;
        let write_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let expires_at_ms = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        tx.execute(
            "DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
        )
        .map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_reservation(space_id, document_id, checkpoint_id, generation, write_epoch, expires_at_ms, plan) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), generation, write_epoch, expires_at_ms, plan],
        )
        .map_err(backend)?;
        Self::cas_insert_objects(tx, "hub_artifact_cas_reservation_object", &reservation.plan)
    }

    pub(super) fn cas_project_publish(tx: &Transaction<'_>, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
        let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let write_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        tx.execute(
            "DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
        )
        .map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_reference(space_id, document_id, checkpoint_id, generation, write_epoch, plan) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), generation, write_epoch, plan],
        )
        .map_err(backend)?;
        Self::cas_insert_objects(tx, "hub_artifact_cas_reference_object", &reservation.plan)
    }

    pub(super) fn cas_project_release(tx: &Transaction<'_>, operation: &str, space_id: &str, document_id: Option<&str>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
        match operation {
            "retention" => {
                let document_id = document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?;
                let checkpoint_id = checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?;
                tx.execute(
                    "DELETE FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id IN (
                        SELECT checkpoint_id FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND event_seq < (
                            SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3
                        )
                    )",
                    rusqlite::params![space_id, document_id, checkpoint_id.0.as_slice()],
                )
                .map_err(backend)?;
                tx.execute(
                    "DELETE FROM hub_artifact_checkpoint_private WHERE space_id = ?1 AND document_id = ?2 AND event_seq < (
                        SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3
                    )",
                    rusqlite::params![space_id, document_id, checkpoint_id.0.as_slice()],
                )
                .map_err(backend)?;
            }
            "space-delete" => {
                tx.execute("DELETE FROM hub_artifact_cas_reservation WHERE space_id = ?1", [space_id]).map_err(backend)?;
                tx.execute("DELETE FROM hub_artifact_cas_reference WHERE space_id = ?1", [space_id]).map_err(backend)?;
            }
            _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
        }
        Ok(())
    }

    pub(super) fn append_cas_release(&self, tx: &Transaction<'_>, event: &DirectoryEvent) -> DirectoryResult<()> {
        let (operation, space_id, document_id, checkpoint_id) = match &event.body {
            DirectoryEventBody::ArtifactRetentionAdvanced { retention } => ("retention", retention.scope.space_id.as_str(), Some(retention.scope.document_id.as_str()), Some(retention.retained_checkpoint_id)),
            DirectoryEventBody::SpaceDeleted { space_id } => ("space-delete", space_id.as_str(), None, None),
            _ => return Ok(()),
        };
        let generation = Self::cas_generation(tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, event_seq) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![generation, operation, space_id, document_id, checkpoint_id.map(|value| value.0.to_vec()), i64::try_from(event.seq).map_err(backend)?],
        )
        .map_err(backend)?;
        Self::cas_project_release(tx, operation, space_id, document_id, checkpoint_id)
    }
}

impl ArtifactCasDirectory for SqliteDirectory {
    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let expires_at = i64::try_from(expires_at_ms).map_err(backend)?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let (coordinator_id, physical_epoch) = Self::cas_reservation_barrier(&tx, &plan.scope.space_id, now)?;
        let historical: Option<Vec<u8>> = tx
            .query_row(
                "SELECT plan FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3 AND plan IS NOT NULL ORDER BY generation LIMIT 1",
                rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
                |row| row.get(0),
            )
            .optional()
            .map_err(backend)?;
        if historical.as_ref().is_some_and(|value| value != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = tx
            .query_row(
                "SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()
            .map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, i64::MAX as u64, coordinator_id, physical_epoch);
            tx.commit().map_err(backend)?;
            return Ok(reservation);
        }
        let released: i64 = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3 AND operation = 'publish')",
                rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
                |row| row.get(0),
            )
            .map_err(backend)?;
        if released != 0 {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = tx
            .query_row(
                "SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(backend)?;
        if let Some((generation, write_epoch, current_expiry, stored)) = current {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            if current_expiry > now {
                let reservation =
                    ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, u64::try_from(current_expiry).map_err(backend)?, coordinator_id, physical_epoch);
                tx.commit().map_err(backend)?;
                return Ok(reservation);
            }
        }
        let previous_epoch: i64 = tx
            .query_row(
                "SELECT COALESCE(MAX(write_epoch), 0) FROM hub_artifact_cas_ledger_journal WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice()],
                |row| row.get(0),
            )
            .map_err(backend)?;
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = Self::cas_generation(&tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) VALUES (?1, 'reserve', ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![generation, plan.scope.space_id, plan.scope.document_id, plan.checkpoint_id.0.as_slice(), write_epoch, expires_at, encoded],
        )
        .map_err(backend)?;
        let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, expires_at_ms, coordinator_id, physical_epoch);
        Self::cas_project_reserve(&tx, &reservation)?;
        tx.commit().map_err(backend)?;
        Ok(reservation)
    }

    async fn append_reserved_artifact_checkpoint(
        &self,
        event: Option<&NewDirectoryEvent>,
        checkpoint: &ArtifactCheckpoint,
        reservation: &ArtifactCasReservation,
        completion: Option<&CheckpointPublicationCompletionV1>,
        now_ms: u64,
    ) -> DirectoryResult<Vec<DirectoryEvent>> {
        validate_artifact_cas_publication_v1(&reservation.plan, checkpoint).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if checkpoint.parent_checkpoint_id.is_none() || !checkpoint.baseline_frontier.is_edited_for(&checkpoint.scope) {
            return Err(DirectoryError::Conflict("ordinary checkpoint publication requires an edited child of committed genesis".into()));
        }
        if let Some(event) = event {
            validate_verified_checkpoint_append(event, checkpoint)?;
        }
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let live_lease: i64 = tx.query_row("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_delete_lease WHERE space_id = ?1 AND expires_at_ms > ?2)", rusqlite::params![reservation.plan.scope.space_id, now], |row| row.get(0)).map_err(backend)?;
        if live_lease != 0 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = tx
            .query_row(
                "SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .optional()
            .map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if event.is_some() || generation != token_generation || write_epoch != token_epoch || stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = tx
            .query_row(
                "SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = ?1 AND document_id = ?2 AND checkpoint_id = ?3",
                rusqlite::params![reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice()],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?)),
            )
            .optional()
            .map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, stored)| *generation != token_generation || *epoch != token_epoch || *expiry != i64::try_from(reservation.expires_at_ms).unwrap_or(-1) || *expiry <= now || stored != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let full = self.persist_event(&tx, event)?;
        let checkpoint_payload = serde_json::Value::from(&checkpoint.to_value()).to_string();
        let event_seq = i64::try_from(full.seq).map_err(backend)?;
        tx.execute(
            "INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES (?1, ?2, ?3, ?4, ?5)",
            rusqlite::params![event_seq, checkpoint.scope.space_id, checkpoint.scope.document_id, checkpoint.checkpoint_id.0.as_slice(), checkpoint_payload],
        )
        .map_err(backend)?;
        self.project(&tx, &full)?;
        self.project_verified_checkpoint(&tx, &full, checkpoint)?;
        let generation = Self::cas_generation(&tx)?;
        tx.execute(
            "INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES (?1, 'publish', ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            rusqlite::params![generation, reservation.plan.scope.space_id, reservation.plan.scope.document_id, reservation.plan.checkpoint_id.0.as_slice(), token_epoch, i64::try_from(reservation.expires_at_ms).map_err(backend)?, event_seq, encoded],
        )
        .map_err(backend)?;
        Self::cas_project_publish(&tx, reservation, generation)?;
        if let Some(completion) = completion {
            validate_checkpoint_publication_completion(completion)?;
            let changed = tx
                .execute(
                    "UPDATE hub_checkpoint_publication_receipt SET disposition = 'completed', checkpoint_id = ?4, completed_at = ?5 WHERE actor_user_id = ?1 AND correlation_id = ?2 AND command_sha256 = ?3 AND disposition = 'pending'",
                    rusqlite::params![completion.actor_user_id, completion.correlation_id, completion.command_sha256, completion.checkpoint_id.0.as_slice(), completion.completed_at],
                )
                .map_err(backend)?;
            if changed != 1 {
                return Err(DirectoryError::Conflict("checkpoint publication claim is missing, substituted, or already completed".into()));
            }
        }
        tx.commit().map_err(backend)?;
        Ok(vec![full])
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let generation: i64 = self.lock()?.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        u64::try_from(generation).map_err(backend)
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        let bytes: Vec<u8> = self.lock()?.query_row("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        bytes.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        let limit = i64::try_from(limit).map_err(backend)?;
        let conn = self.lock()?;
        let current: i64 = conn.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let rows: Vec<(i64, Option<Vec<u8>>)> = {
            let mut statement = conn.prepare("SELECT generation, plan FROM hub_artifact_cas_ledger_journal WHERE generation > ?1 AND generation <= ?2 ORDER BY generation LIMIT ?3").map_err(backend)?;
            let mapped = statement.query_map(rusqlite::params![after, through, limit], |row| Ok((row.get(0)?, row.get(1)?))).map_err(backend)?;
            mapped.collect::<Result<Vec<_>, _>>().map_err(backend)?
        };
        let mut objects = Vec::new();
        let mut next = after;
        for (generation, plan) in rows {
            next = generation;
            if let Some(plan) = plan {
                objects.extend(decode_artifact_cas_ownership_v1(&plan).map_err(|error| DirectoryError::Backend(error.to_string()))?.objects);
            }
        }
        objects.sort_by_key(|object| (object.space_id.clone(), object.kind, object.digest.0));
        objects.dedup();
        Ok(ArtifactCasSweepCandidatePage { observed_generation: through_generation, next_generation: u64::try_from(next).map_err(backend)?, objects })
    }

    async fn artifact_cas_delete_preview_protected(&self, key: &ArtifactCasObjectKey, observed_generation: u64, now_ms: u64) -> DirectoryResult<bool> {
        let conn = self.lock()?;
        let current: i64 = conn.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        if current < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let referenced: i64 = conn
            .query_row("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = ?1 AND kind = ?2 AND object_digest = ?3)", rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice()], |row| row.get(0))
            .map_err(backend)?;
        let reserved: i64 = conn.query_row(
            "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = ?1 AND object.kind = ?2 AND object.object_digest = ?3 AND reservation.expires_at_ms > ?4)",
            rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice(), i64::try_from(now_ms).map_err(backend)?],
            |row| row.get(0),
        ).map_err(backend)?;
        Ok(referenced != 0 || reserved != 0)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        if lease_token == [0; 32] || expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into()));
        }
        let observed = i64::try_from(observed_generation).map_err(backend)?;
        let now = i64::try_from(now_ms).map_err(backend)?;
        let expires_at = i64::try_from(expires_at_ms).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let current_lease: Option<(i64, Option<i64>)> =
            tx.query_row("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = ?1", [key.space_id.as_str()], |row| Ok((row.get(0)?, row.get(1)?))).optional().map_err(backend)?;
        if current_lease.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now) {
            tx.commit().map_err(backend)?;
            return Ok(None);
        }
        let physical_epoch = match current_lease {
            Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
            None => 1,
        };
        tx.execute("INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES (?1, ?2, ?3, ?4) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = excluded.lease_token, expires_at_ms = excluded.expires_at_ms", rusqlite::params![key.space_id, physical_epoch, lease_token.as_slice(), expires_at]).map_err(backend)?;
        let current: i64 = tx.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        if current < observed {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let referenced: i64 = tx
            .query_row("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = ?1 AND kind = ?2 AND object_digest = ?3)", rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice()], |row| row.get(0))
            .map_err(backend)?;
        let reserved: i64 = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = ?1 AND object.kind = ?2 AND object.object_digest = ?3 AND reservation.expires_at_ms > ?4)",
            rusqlite::params![key.space_id, key.kind.name(), key.digest.0.as_slice(), now],
            |row| row.get(0),
        ).map_err(backend)?;
        if referenced != 0 || reserved != 0 {
            tx.execute("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = ?1 AND lease_token = ?2", rusqlite::params![key.space_id, lease_token.as_slice()]).map_err(backend)?;
            tx.commit().map_err(backend)?;
            return Ok(None);
        }
        let coordinator: Vec<u8> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        let coordinator_id = coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?;
        tx.commit().map_err(backend)?;
        Ok(Some(ArtifactCasDeleteFence::new(key.clone(), observed_generation, coordinator_id, u64::try_from(physical_epoch).map_err(backend)?, lease_token)))
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction_with_behavior(rusqlite::TransactionBehavior::Immediate).map_err(backend)?;
        let lease: Option<(i64, Option<Vec<u8>>, Option<i64>)> =
            tx.query_row("SELECT fence_epoch, lease_token, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = ?1", [fence.object().space_id.as_str()], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).optional().map_err(backend)?;
        let lease_valid = lease.is_some_and(|(epoch, token, expiry)| u64::try_from(epoch).ok() == Some(fence.physical_epoch()) && token.as_deref() == Some(fence.lease_token().as_slice()) && expiry.is_some_and(|expiry| expiry > now));
        let current: i64 = tx.query_row("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        let referenced: i64 = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = ?1 AND kind = ?2 AND object_digest = ?3)",
                rusqlite::params![fence.object().space_id, fence.object().kind.name(), fence.object().digest.0.as_slice()],
                |row| row.get(0),
            )
            .map_err(backend)?;
        let reserved: i64 = tx
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = ?1 AND object.kind = ?2 AND object.object_digest = ?3 AND reservation.expires_at_ms > ?4)",
                rusqlite::params![fence.object().space_id, fence.object().kind.name(), fence.object().digest.0.as_slice(), now],
                |row| row.get(0),
            )
            .map_err(backend)?;
        let coordinator: Vec<u8> = tx.query_row("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton = 1", [], |row| row.get(0)).map_err(backend)?;
        let coordinator_valid = coordinator.as_slice() == fence.coordinator_id().as_slice();
        tx.commit().map_err(backend)?;
        Ok(lease_valid && coordinator_valid && u64::try_from(current).map_err(backend)? >= fence.ledger_generation() && referenced == 0 && reserved == 0)
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        if expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into()));
        }
        let conn = self.lock()?;
        let changed = conn
            .execute(
                "UPDATE hub_artifact_cas_delete_lease SET expires_at_ms = ?3 WHERE space_id = ?1 AND lease_token = ?2 AND expires_at_ms > ?4",
                rusqlite::params![fence.object().space_id, fence.lease_token().as_slice(), i64::try_from(expires_at_ms).map_err(backend)?, i64::try_from(now_ms).map_err(backend)?],
            )
            .map_err(backend)?;
        if changed != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let key = fence.object().clone();
        let conn = self.lock()?;
        if conn.execute("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = ?1 AND lease_token = ?2", rusqlite::params![key.space_id, fence.lease_token().as_slice()]).map_err(backend)? != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }
}
