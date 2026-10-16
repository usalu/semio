//! 🧺️ Artifact CAS reservations, sweep candidates and delete fences.

use super::*;

pub(super) async fn cas_generation(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>) -> DirectoryResult<i64> {
    let (generation,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_artifact_cas_ledger_head SET generation = generation + 1 WHERE singleton RETURNING generation").fetch_one(&mut **tx).await.map_err(backend)?;
    Ok(generation)
}

pub(super) async fn cas_lock_space(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, space_id: &str) -> DirectoryResult<()> {
    sqlx_core::query::query("SELECT pg_advisory_xact_lock(hashtextextended($1, 1162037326))").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
    Ok(())
}

async fn cas_reservation_barrier(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, space_id: &str, now_ms: i64) -> DirectoryResult<([u8; 32], u64)> {
    cas_lock_space(tx, space_id).await?;
    let current: Option<(i64, Option<i64>)> = sqlx_core::query_as::query_as("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(space_id).fetch_optional(&mut **tx).await.map_err(backend)?;
    if current.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now_ms) {
        return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
    }
    let epoch = match current {
        Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
        None => 1,
    };
    sqlx_core::query::query(
        "INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES ($1,$2,NULL,NULL) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = NULL, expires_at_ms = NULL",
    )
    .bind(space_id)
    .bind(epoch)
    .execute(&mut **tx)
    .await
    .map_err(backend)?;
    let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut **tx).await.map_err(backend)?;
    Ok((coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?, u64::try_from(epoch).map_err(backend)?))
}

async fn cas_insert_objects(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, table: &str, plan: &ArtifactCasOwnershipPlanV1) -> DirectoryResult<()> {
    let sql = match table {
        "reservation" => "INSERT INTO hub_artifact_cas_reservation_object(space_id, document_id, checkpoint_id, kind, object_digest) VALUES ($1, $2, $3, $4, $5)",
        "reference" => "INSERT INTO hub_artifact_cas_reference_object(space_id, document_id, checkpoint_id, kind, object_digest) VALUES ($1, $2, $3, $4, $5)",
        _ => return Err(DirectoryError::Backend("invalid artifact CAS projection table".into())),
    };
    for object in &plan.objects {
        sqlx_core::query::query(sql).bind(&plan.scope.space_id).bind(&plan.scope.document_id).bind(plan.checkpoint_id.0.as_slice()).bind(object.kind.name()).bind(object.digest.0.as_slice()).execute(&mut **tx).await.map_err(backend)?;
    }
    Ok(())
}

pub(super) async fn cas_project_reserve(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, reservation: &ArtifactCasReservation) -> DirectoryResult<()> {
    let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    sqlx_core::query::query("INSERT INTO hub_artifact_cas_reservation(space_id, document_id, checkpoint_id, generation, write_epoch, expires_at_ms, plan) VALUES ($1, $2, $3, $4, $5, $6, $7)")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .bind(i64::try_from(reservation.generation).map_err(backend)?)
        .bind(i64::try_from(reservation.write_epoch).map_err(backend)?)
        .bind(i64::try_from(reservation.expires_at_ms).map_err(backend)?)
        .bind(plan)
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    cas_insert_objects(tx, "reservation", &reservation.plan).await
}

pub(super) async fn cas_project_publish(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, reservation: &ArtifactCasReservation, generation: i64) -> DirectoryResult<()> {
    let plan = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
    sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    sqlx_core::query::query("INSERT INTO hub_artifact_cas_reference(space_id, document_id, checkpoint_id, generation, write_epoch, plan) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(&reservation.plan.scope.space_id)
        .bind(&reservation.plan.scope.document_id)
        .bind(reservation.plan.checkpoint_id.0.as_slice())
        .bind(generation)
        .bind(i64::try_from(reservation.write_epoch).map_err(backend)?)
        .bind(plan)
        .execute(&mut **tx)
        .await
        .map_err(backend)?;
    cas_insert_objects(tx, "reference", &reservation.plan).await
}

pub(super) async fn cas_project_release(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, operation: &str, space_id: &str, document_id: Option<&str>, checkpoint_id: Option<ArtifactHash>) -> DirectoryResult<()> {
    match operation {
        "retention" => {
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id IN (SELECT checkpoint_id FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND event_seq < (SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3))")
                .bind(space_id).bind(document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?).bind(checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?.0.as_slice()).execute(&mut **tx).await.map_err(backend)?;
            sqlx_core::query::query(
                "DELETE FROM hub_artifact_checkpoint_private WHERE space_id = $1 AND document_id = $2 AND event_seq < (SELECT event_seq FROM hub_artifact_authority_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3)",
            )
            .bind(space_id)
            .bind(document_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention document missing".into()))?)
            .bind(checkpoint_id.ok_or_else(|| DirectoryError::Backend("artifact CAS retention checkpoint missing".into()))?.0.as_slice())
            .execute(&mut **tx)
            .await
            .map_err(backend)?;
        }
        "space-delete" => {
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reservation WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
            sqlx_core::query::query("DELETE FROM hub_artifact_cas_reference WHERE space_id = $1").bind(space_id).execute(&mut **tx).await.map_err(backend)?;
        }
        _ => return Err(DirectoryError::Backend("invalid artifact CAS release operation".into())),
    }
    Ok(())
}

impl ArtifactCasDirectory for PostgresDirectory {
    async fn reserve_artifact_cas(&self, plan: &ArtifactCasOwnershipPlanV1, expires_at_ms: u64, now_ms: u64) -> DirectoryResult<ArtifactCasReservation> {
        let encoded = encode_artifact_cas_ownership_v1(plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        if expires_at_ms <= now_ms || expires_at_ms.checked_sub(now_ms).is_none_or(|ttl| ttl > ARTIFACT_CAS_RESERVATION_MAX_TTL_MS) {
            return Err(DirectoryError::Conflict(format!("artifact CAS reservation ttl must be 1..={ARTIFACT_CAS_RESERVATION_MAX_TTL_MS} milliseconds")));
        }
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let (coordinator_id, physical_epoch) = cas_reservation_barrier(&mut tx, &plan.scope.space_id, now).await?;
        let historical: Option<(Vec<u8>,)> = sqlx_core::query_as::query_as("SELECT plan FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 AND plan IS NOT NULL ORDER BY generation LIMIT 1")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if historical.as_ref().is_some_and(|(value,)| value != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS checkpoint identity names a different ownership plan".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, write_epoch, stored)) = published {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published ownership conflict".into()));
            }
            let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, i64::MAX as u64, coordinator_id, physical_epoch);
            tx.commit().await.map_err(backend)?;
            return Ok(reservation);
        }
        let (released,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 AND operation = 'publish')")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        if released {
            return Err(DirectoryError::Conflict("artifact CAS released checkpoint cannot be reserved again".into()));
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, write_epoch, expiry, stored)) = current {
            if stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS reservation identity conflict".into()));
            }
            if expiry > now {
                let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, u64::try_from(expiry).map_err(backend)?, coordinator_id, physical_epoch);
                tx.commit().await.map_err(backend)?;
                return Ok(reservation);
            }
        }
        let (previous_epoch,): (i64,) = sqlx_core::query_as::query_as("SELECT COALESCE(MAX(write_epoch), 0) FROM hub_artifact_cas_ledger_journal WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let write_epoch = previous_epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS write epoch overflow".into()))?;
        let generation = cas_generation(&mut tx).await?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, plan) VALUES ($1, 'reserve', $2, $3, $4, $5, $6, $7)")
            .bind(generation)
            .bind(&plan.scope.space_id)
            .bind(&plan.scope.document_id)
            .bind(plan.checkpoint_id.0.as_slice())
            .bind(write_epoch)
            .bind(i64::try_from(expires_at_ms).map_err(backend)?)
            .bind(encoded)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        let reservation = ArtifactCasReservation::fenced(plan.clone(), u64::try_from(generation).map_err(backend)?, u64::try_from(write_epoch).map_err(backend)?, expires_at_ms, coordinator_id, physical_epoch);
        cas_project_reserve(&mut tx, &reservation).await?;
        tx.commit().await.map_err(backend)?;
        Ok(reservation)
    }

    async fn append_reserved_artifact_checkpoint(
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
        let encoded = encode_artifact_cas_ownership_v1(&reservation.plan).map_err(|error| DirectoryError::Conflict(error.to_string()))?;
        let token_generation = i64::try_from(reservation.generation).map_err(backend)?;
        let token_epoch = i64::try_from(reservation.write_epoch).map_err(backend)?;
        let token_expiry = i64::try_from(reservation.expires_at_ms).map_err(backend)?;
        let now = i64::try_from(current_now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &reservation.plan.scope.space_id).await?;
        let (leased,): (bool,) =
            sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_delete_lease WHERE space_id = $1 AND expires_at_ms > $2)").bind(&reservation.plan.scope.space_id).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        if leased {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is active for this space".into()));
        }
        let published: Option<(i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, plan FROM hub_artifact_cas_reference WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3")
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some((generation, epoch, stored)) = published {
            if event.is_some() || generation != token_generation || epoch != token_epoch || stored != encoded {
                return Err(DirectoryError::Conflict("artifact CAS published reservation conflict".into()));
            }
            return Ok(Vec::new());
        }
        let current: Option<(i64, i64, i64, Vec<u8>)> = sqlx_core::query_as::query_as("SELECT generation, write_epoch, expires_at_ms, plan FROM hub_artifact_cas_reservation WHERE space_id = $1 AND document_id = $2 AND checkpoint_id = $3 FOR UPDATE")
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if current.as_ref().is_none_or(|(generation, epoch, expiry, stored)| *generation != token_generation || *epoch != token_epoch || *expiry != token_expiry || *expiry <= now || stored != &encoded) {
            return Err(DirectoryError::Conflict("artifact CAS reservation is missing, expired, or superseded".into()));
        }
        let event = event.ok_or_else(|| DirectoryError::Conflict("new artifact CAS publication requires one public event".into()))?;
        let id = time_ordered_id();
        let recorded_at_ms = now_ms();
        let payload_value = serde_json::Value::from(&event.body.to_value());
        let kind = payload_value.get("kind").and_then(|value| value.as_str()).unwrap_or_default();
        let (event_seq,): (i64,) = sqlx_core::query_as::query_as("UPDATE hub_directory_event_head SET seq = seq + 1 WHERE singleton RETURNING seq").fetch_one(&mut *tx).await.map_err(backend)?;
        let full =
            DirectoryEvent { seq: u64::try_from(event_seq).map_err(backend)?, id: id.clone(), hlc: event.hlc, actor: event.actor.clone(), space_id: event.space_id.clone(), user_id: event.user_id.clone(), body: event.body.clone(), recorded_at_ms };
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
            .bind(&payload_value)
            .bind(recorded_at_ms)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        sqlx_core::query::query("INSERT INTO hub_artifact_authority_journal(event_seq, space_id, document_id, checkpoint_id, payload) VALUES ($1,$2,$3,$4,$5)")
            .bind(event_seq)
            .bind(&checkpoint.scope.space_id)
            .bind(&checkpoint.scope.document_id)
            .bind(checkpoint.checkpoint_id.0.as_slice())
            .bind(serde_json::Value::from(&checkpoint.to_value()))
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        self.project(&mut tx, &full).await?;
        self.project_verified_checkpoint(&mut tx, &full, checkpoint).await?;
        let generation = cas_generation(&mut tx).await?;
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_ledger_journal(generation, operation, space_id, document_id, checkpoint_id, write_epoch, expires_at_ms, event_seq, plan) VALUES ($1,'publish',$2,$3,$4,$5,$6,$7,$8)")
            .bind(generation)
            .bind(&reservation.plan.scope.space_id)
            .bind(&reservation.plan.scope.document_id)
            .bind(reservation.plan.checkpoint_id.0.as_slice())
            .bind(token_epoch)
            .bind(token_expiry)
            .bind(event_seq)
            .bind(encoded)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        cas_project_publish(&mut tx, reservation, generation).await?;
        if let Some(completion) = completion {
            validate_checkpoint_publication_completion(completion)?;
            let updated = sqlx_core::query::query(
                "UPDATE hub_checkpoint_publication_receipt SET disposition = 'completed', checkpoint_id = $4, completed_at = $5 WHERE actor_user_id = $1 AND correlation_id = $2 AND command_sha256 = $3 AND disposition = 'pending'",
            )
            .bind(&completion.actor_user_id)
            .bind(&completion.correlation_id)
            .bind(&completion.command_sha256)
            .bind(completion.checkpoint_id.0.as_slice())
            .bind(completion.completed_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
            if updated.rows_affected() != 1 {
                return Err(DirectoryError::Conflict("checkpoint publication claim is missing, substituted, or already completed".into()));
            }
        }
        tx.commit().await.map_err(backend)?;
        Ok(vec![full])
    }

    async fn artifact_cas_ledger_generation(&self) -> DirectoryResult<u64> {
        let (generation,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        u64::try_from(generation).map_err(backend)
    }

    async fn artifact_cas_coordinator_id(&self) -> DirectoryResult<[u8; 32]> {
        let (bytes,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        bytes.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))
    }

    async fn artifact_cas_sweep_candidates(&self, after_generation: u64, through_generation: u64, limit: usize) -> DirectoryResult<ArtifactCasSweepCandidatePage> {
        if limit == 0 || limit > ARTIFACT_CAS_SWEEP_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("artifact CAS sweep page requires limit 1..={ARTIFACT_CAS_SWEEP_PAGE_MAX}")));
        }
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        let after = i64::try_from(after_generation).map_err(backend)?;
        let through = i64::try_from(through_generation).map_err(backend)?;
        if through > current || after > through {
            return Err(DirectoryError::Conflict("artifact CAS sweep bounds are outside the ledger".into()));
        }
        let rows: Vec<(i64, Option<Vec<u8>>)> = sqlx_core::query_as::query_as("SELECT generation, plan FROM hub_artifact_cas_ledger_journal WHERE generation > $1 AND generation <= $2 ORDER BY generation LIMIT $3")
            .bind(after)
            .bind(through)
            .bind(i64::try_from(limit).map_err(backend)?)
            .fetch_all(&self.pool)
            .await
            .map_err(backend)?;
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
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&self.pool).await.map_err(backend)?;
        if current < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&key.space_id)
            .bind(key.kind.name())
            .bind(key.digest.0.as_slice())
            .fetch_one(&self.pool)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)")
            .bind(&key.space_id).bind(key.kind.name()).bind(key.digest.0.as_slice()).bind(i64::try_from(now_ms).map_err(backend)?).fetch_one(&self.pool).await.map_err(backend)?;
        Ok(referenced || reserved)
    }

    async fn acquire_artifact_cas_delete_fence(&self, key: &ArtifactCasObjectKey, observed_generation: u64, lease_token: [u8; 32], now_ms: u64, expires_at_ms: u64) -> DirectoryResult<Option<ArtifactCasDeleteFence>> {
        if observed_generation == 0 {
            return Err(DirectoryError::Conflict("artifact CAS sweep requires a nonzero observed generation".into()));
        }
        if lease_token == [0; 32] || expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is invalid".into()));
        }
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &key.space_id).await?;
        let current_lease: Option<(i64, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT fence_epoch, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(&key.space_id).fetch_optional(&mut *tx).await.map_err(backend)?;
        if current_lease.as_ref().and_then(|(_, expiry)| *expiry).is_some_and(|expiry| expiry > now) {
            tx.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let physical_epoch = match current_lease {
            Some((epoch, _)) => epoch.checked_add(1).ok_or_else(|| DirectoryError::Conflict("artifact CAS fence epoch overflow".into()))?,
            None => 1,
        };
        sqlx_core::query::query("INSERT INTO hub_artifact_cas_delete_lease(space_id, fence_epoch, lease_token, expires_at_ms) VALUES ($1,$2,$3,$4) ON CONFLICT(space_id) DO UPDATE SET fence_epoch = excluded.fence_epoch, lease_token = excluded.lease_token, expires_at_ms = excluded.expires_at_ms").bind(&key.space_id).bind(physical_epoch).bind(lease_token.as_slice()).bind(i64::try_from(expires_at_ms).map_err(backend)?).execute(&mut *tx).await.map_err(backend)?;
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        if current < i64::try_from(observed_generation).map_err(backend)? {
            return Err(DirectoryError::Conflict("artifact CAS sweep observation is ahead of the ledger".into()));
        }
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&key.space_id)
            .bind(key.kind.name())
            .bind(key.digest.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)").bind(&key.space_id).bind(key.kind.name()).bind(key.digest.0.as_slice()).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        if referenced || reserved {
            sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = $1 AND lease_token = $2")
                .bind(&key.space_id)
                .bind(lease_token.as_slice())
                .execute(&mut *tx)
                .await
                .map_err(backend)?;
            tx.commit().await.map_err(backend)?;
            return Ok(None);
        }
        let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        let coordinator_id = coordinator.try_into().map_err(|_| DirectoryError::Backend("artifact CAS barrier coordinator identity is invalid".into()))?;
        tx.commit().await.map_err(backend)?;
        Ok(Some(ArtifactCasDeleteFence::new(key.clone(), observed_generation, coordinator_id, u64::try_from(physical_epoch).map_err(backend)?, lease_token)))
    }

    async fn validate_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64) -> DirectoryResult<bool> {
        let now = i64::try_from(now_ms).map_err(backend)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        cas_lock_space(&mut tx, &fence.object().space_id).await?;
        let lease: Option<(i64, Option<Vec<u8>>, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT fence_epoch, lease_token, expires_at_ms FROM hub_artifact_cas_delete_lease WHERE space_id = $1 FOR UPDATE").bind(&fence.object().space_id).fetch_optional(&mut *tx).await.map_err(backend)?;
        let lease_valid = lease.is_some_and(|(epoch, token, expiry)| u64::try_from(epoch).ok() == Some(fence.physical_epoch()) && token.as_deref() == Some(fence.lease_token().as_slice()) && expiry.is_some_and(|expiry| expiry > now));
        let (current,): (i64,) = sqlx_core::query_as::query_as("SELECT generation FROM hub_artifact_cas_ledger_head WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        let (referenced,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reference_object WHERE space_id = $1 AND kind = $2 AND object_digest = $3)")
            .bind(&fence.object().space_id)
            .bind(fence.object().kind.name())
            .bind(fence.object().digest.0.as_slice())
            .fetch_one(&mut *tx)
            .await
            .map_err(backend)?;
        let (reserved,): (bool,) = sqlx_core::query_as::query_as("SELECT EXISTS(SELECT 1 FROM hub_artifact_cas_reservation_object object JOIN hub_artifact_cas_reservation reservation USING(space_id, document_id, checkpoint_id) WHERE object.space_id = $1 AND object.kind = $2 AND object.object_digest = $3 AND reservation.expires_at_ms > $4)")
            .bind(&fence.object().space_id).bind(fence.object().kind.name()).bind(fence.object().digest.0.as_slice()).bind(now).fetch_one(&mut *tx).await.map_err(backend)?;
        let (coordinator,): (Vec<u8>,) = sqlx_core::query_as::query_as("SELECT coordinator_id FROM hub_artifact_cas_barrier_identity WHERE singleton").fetch_one(&mut *tx).await.map_err(backend)?;
        tx.commit().await.map_err(backend)?;
        Ok(lease_valid && coordinator.as_slice() == fence.coordinator_id().as_slice() && u64::try_from(current).map_err(backend)? >= fence.ledger_generation() && !referenced && !reserved)
    }

    async fn renew_artifact_cas_delete_fence(&self, fence: &ArtifactCasDeleteFence, now_ms: u64, expires_at_ms: u64) -> DirectoryResult<()> {
        if expires_at_ms <= now_ms {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease renewal is invalid".into()));
        }
        let result = sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET expires_at_ms = $3 WHERE space_id = $1 AND lease_token = $2 AND expires_at_ms > $4")
            .bind(&fence.object().space_id)
            .bind(fence.lease_token().as_slice())
            .bind(i64::try_from(expires_at_ms).map_err(backend)?)
            .bind(i64::try_from(now_ms).map_err(backend)?)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        if result.rows_affected() != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }

    async fn release_artifact_cas_delete_fence(&self, fence: ArtifactCasDeleteFence) -> DirectoryResult<()> {
        let result = sqlx_core::query::query("UPDATE hub_artifact_cas_delete_lease SET lease_token = NULL, expires_at_ms = NULL WHERE space_id = $1 AND lease_token = $2")
            .bind(&fence.object().space_id)
            .bind(fence.lease_token().as_slice())
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        if result.rows_affected() != 1 {
            return Err(DirectoryError::Conflict("artifact CAS deletion lease is no longer owned".into()));
        }
        Ok(())
    }
}
