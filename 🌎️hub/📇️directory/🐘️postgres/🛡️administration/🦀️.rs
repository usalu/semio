//! 🛡️ Admin operation audit, directory command receipts and checkpoint publication claims.

use super::*;

pub(super) async fn insert_admin_operation_effect_receipt(tx: &mut sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>, receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
    validate_admin_operation_effect_receipt(receipt)?;
    let inserted = sqlx_core::query::query(
        "INSERT INTO hub_admin_operation_effect_receipt (operation_id, intent_digest, committed_at, outcome_code, event_seq_first, event_seq_last)
         VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT (operation_id) DO NOTHING",
    )
    .bind(&receipt.operation_id)
    .bind(&receipt.intent_digest)
    .bind(receipt.committed_at)
    .bind(&receipt.outcome_code)
    .bind(receipt.event_seq_first.map(i64::try_from).transpose().map_err(backend)?)
    .bind(receipt.event_seq_last.map(i64::try_from).transpose().map_err(backend)?)
    .execute(&mut **tx)
    .await
    .map_err(backend)?;
    if inserted.rows_affected() == 1 {
        return Ok(());
    }
    let established: Option<(String, String, i64, String, Option<i64>, Option<i64>)> =
        sqlx_core::query_as::query_as("SELECT operation_id, intent_digest, committed_at, outcome_code, event_seq_first, event_seq_last FROM hub_admin_operation_effect_receipt WHERE operation_id = $1 FOR UPDATE")
            .bind(&receipt.operation_id)
            .fetch_optional(&mut **tx)
            .await
            .map_err(backend)?;
    let Some((operation_id, intent_digest, committed_at, outcome_code, first, last)) = established else {
        return Err(DirectoryError::Conflict("admin operation effect receipt disappeared".into()));
    };
    let established =
        AdminOperationEffectReceiptV1 { operation_id, intent_digest, committed_at, outcome_code, event_seq_first: first.map(u64::try_from).transpose().map_err(backend)?, event_seq_last: last.map(u64::try_from).transpose().map_err(backend)? };
    if &established == receipt { Ok(()) } else { Err(DirectoryError::Conflict("admin operation effect receipt identity changed".into())) }
}

pub(super) async fn admin_effect_rollback<T>(tx: sqlx_core::transaction::Transaction<'_, sqlx_postgres::Postgres>) -> AdminEffectCommitV1<T> {
    match tx.rollback().await {
        Ok(()) => AdminEffectCommitV1::RejectedBeforeCommit,
        Err(_) => AdminEffectCommitV1::Indeterminate,
    }
}

//#region AdminOperations
impl AdminOperationDirectory for PostgresDirectory {
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        let (spaces, users, connections): (i64, i64, i64) =
            sqlx_core::query_as::query_as("SELECT (SELECT COUNT(*) FROM hub_space), (SELECT COUNT(*) FROM hub_user), (SELECT COUNT(*) FROM hub_sync_session WHERE disconnected_at IS NULL)").fetch_one(&self.pool).await.map_err(backend)?;
        Ok(AdminDirectoryOverviewCounts { spaces: u64::try_from(spaces).map_err(backend)?, users: u64::try_from(users).map_err(backend)?, connections: u64::try_from(connections).map_err(backend)? })
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        validate_admin_operation_audit(fact)?;
        let terminal = fact.phase != "accepted";
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let existing: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE request_id = $1 ORDER BY sequence LIMIT 2 FOR UPDATE",
        )
        .bind(&fact.request_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(backend)?;
        let existing = existing.into_iter().map(admin_operation_audit_from_row).collect::<DirectoryResult<Vec<_>>>()?;
        if !terminal {
            if let Some(established) = existing.first() {
                let outcome = if same_admin_operation_request(&established.fact, fact) { Ok(established.clone()) } else { Err(DirectoryError::Conflict("admin request id was reused for a different intent".into())) };
                tx.rollback().await.map_err(backend)?;
                return outcome;
            }
        } else {
            let accepted = existing.iter().find(|row| row.fact.phase == "accepted").ok_or_else(|| DirectoryError::Conflict("admin terminal fact requires accepted fact".into()))?;
            if accepted.fact.operation_id != fact.operation_id || !same_admin_operation_request(&accepted.fact, fact) {
                return Err(DirectoryError::Conflict("admin terminal fact changed operation identity".into()));
            }
            if let Some(established) = existing.iter().find(|row| row.fact.phase != "accepted") {
                let outcome = if established.fact.operation_id == fact.operation_id && established.fact.phase == fact.phase {
                    Ok(established.clone())
                } else {
                    Err(DirectoryError::Conflict("admin operation already terminated with a different outcome".into()))
                };
                tx.rollback().await.map_err(backend)?;
                return outcome;
            }
        }
        let inserted: Option<(i64,)> = sqlx_core::query_as::query_as(
            "INSERT INTO hub_admin_operation_audit (request_id, intent_digest, operation_id, occurred_at, phase, terminal, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17) ON CONFLICT (request_id, terminal) DO NOTHING RETURNING sequence",
        )
        .bind(&fact.request_id)
        .bind(&fact.intent_digest)
        .bind(&fact.operation_id)
        .bind(fact.occurred_at)
        .bind(&fact.phase)
        .bind(terminal)
        .bind(&fact.intent_kind)
        .bind(&fact.target_kind)
        .bind(&fact.target_id)
        .bind(&fact.principal_user_id)
        .bind(&fact.principal_session_id)
        .bind(i64::try_from(fact.principal_generation).map_err(backend)?)
        .bind(&fact.correlation_id)
        .bind(fact.event_seq_first.map(i64::try_from).transpose().map_err(backend)?)
        .bind(fact.event_seq_last.map(i64::try_from).transpose().map_err(backend)?)
        .bind(&fact.outcome_code)
        .bind(&fact.reason_code)
        .fetch_optional(&mut *tx)
        .await
        .map_err(backend)?;
        match inserted {
            Some((sequence,)) => {
                tx.commit().await.map_err(backend)?;
                Ok(AdminOperationAuditRecord { sequence: u64::try_from(sequence).map_err(backend)?, fact: fact.clone() })
            }
            None => {
                tx.rollback().await.map_err(backend)?;
                let established =
                    self.admin_operation_audit_for_request(&fact.request_id).await?.into_iter().find(|row| (row.fact.phase != "accepted") == terminal).ok_or_else(|| DirectoryError::Conflict("admin request race has no established receipt".into()))?;
                if same_admin_operation_request(&established.fact, fact) { Ok(established) } else { Err(DirectoryError::Conflict("admin request id race changed intent".into())) }
            }
        }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(request_id, "admin request id", AUTH_TEXT_MAX_BYTES)?;
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE request_id = $1 ORDER BY sequence LIMIT 2",
        )
        .bind(request_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES)?;
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE operation_id = $1 ORDER BY sequence LIMIT 2",
        )
        .bind(operation_id)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        validate_bounded_auth_text(operation_id, "admin effect operation id", AUTH_TEXT_MAX_BYTES)?;
        if intent_digest.len() != 64 || !intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict("admin effect intent digest must be 64 lowercase hex digits".into()));
        }
        let row: Option<(String, String, i64, String, Option<i64>, Option<i64>)> =
            sqlx_core::query_as::query_as("SELECT operation_id, intent_digest, committed_at, outcome_code, event_seq_first, event_seq_last FROM hub_admin_operation_effect_receipt WHERE operation_id = $1 AND intent_digest = $2")
                .bind(operation_id)
                .bind(intent_digest)
                .fetch_optional(&self.pool)
                .await
                .map_err(backend)?;
        row.map(|(operation_id, intent_digest, committed_at, outcome_code, first, last)| {
            let receipt =
                AdminOperationEffectReceiptV1 { operation_id, intent_digest, committed_at, outcome_code, event_seq_first: first.map(u64::try_from).transpose().map_err(backend)?, event_seq_last: last.map(u64::try_from).transpose().map_err(backend)? };
            validate_admin_operation_effect_receipt(&receipt)?;
            Ok(receipt)
        })
        .transpose()
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("admin audit limit must be 1..={ADMIN_PAGE_MAX}")));
        }
        let rows: Vec<AdminOperationAuditRow> = sqlx_core::query_as::query_as(
            "SELECT sequence, request_id, intent_digest, operation_id, occurred_at, phase, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code FROM hub_admin_operation_audit WHERE sequence > $1 ORDER BY sequence LIMIT $2",
        )
        .bind(i64::try_from(after_sequence).map_err(backend)?)
        .bind(i64::try_from(limit).map_err(backend)?)
        .fetch_all(&self.pool)
        .await
        .map_err(backend)?;
        rows.into_iter().map(admin_operation_audit_from_row).collect()
    }
}

impl ReceiptDirectory for PostgresDirectory {
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        validate_directory_command_claim(claim)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let existing: Option<DirectoryCommandReceiptRow> = sqlx_core::query_as::query_as(&format!("SELECT {DIRECTORY_COMMAND_RECEIPT_SELECT} FROM hub_directory_command_receipt WHERE actor_user_id = $1 AND request_id = $2 FOR UPDATE"))
            .bind(&claim.actor_user_id)
            .bind(&claim.request_id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(backend)?;
        if let Some(row) = existing {
            let record = directory_command_receipt_from_row(row)?;
            tx.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { DirectoryCommandClaimV1::Existing(record) } else { DirectoryCommandClaimV1::Conflict });
        }
        sqlx_core::query::query("INSERT INTO hub_directory_command_receipt (actor_user_id, request_id, command_sha256, result_kind, disposition, claimed_at) VALUES ($1, $2, $3, $4, 'pending', $5)")
            .bind(&claim.actor_user_id)
            .bind(&claim.request_id)
            .bind(&claim.command_sha256)
            .bind(directory_command_result_kind_str(claim.result_kind))
            .bind(claim.claimed_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        tx.commit().await.map_err(backend)?;
        Ok(DirectoryCommandClaimV1::Claimed(DirectoryCommandReceiptRecord {
            actor_user_id: claim.actor_user_id.clone(),
            request_id: claim.request_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            result_kind: claim.result_kind,
            disposition: DirectoryCommandDispositionV1::Pending,
            event_seq_first: None,
            event_seq_last: None,
            receipt_sha256: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        }))
    }

    async fn complete_directory_command_receipt(&self, completion: &DirectoryCommandReceiptCompletion) -> DirectoryResult<DirectoryCommandReceiptRecord> {
        let row: Option<DirectoryCommandReceiptRow> = sqlx_core::query_as::query_as(&format!(
            "UPDATE hub_directory_command_receipt SET disposition = 'completed', event_seq_first = $3, event_seq_last = $4, receipt_sha256 = $5, completed_at = $6 WHERE actor_user_id = $1 AND request_id = $2 AND disposition = 'pending' RETURNING {DIRECTORY_COMMAND_RECEIPT_SELECT}"
        ))
        .bind(&completion.actor_user_id)
        .bind(&completion.request_id)
        .bind(completion.event_seq_first.map(i64::try_from).transpose().map_err(backend)?)
        .bind(completion.event_seq_last.map(i64::try_from).transpose().map_err(backend)?)
        .bind(&completion.receipt_sha256)
        .bind(completion.completed_at)
        .fetch_optional(&self.pool)
        .await
        .map_err(backend)?;
        directory_command_receipt_from_row(row.ok_or_else(|| DirectoryError::Conflict("directory command receipt claim is not pending".into()))?)
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        let changed = sqlx_core::query::query("DELETE FROM hub_directory_command_receipt WHERE actor_user_id = $1 AND request_id = $2 AND command_sha256 = $3 AND disposition = 'pending'")
            .bind(actor_user_id)
            .bind(request_id)
            .bind(command_sha256)
            .execute(&self.pool)
            .await
            .map_err(backend)?
            .rows_affected();
        if changed != 1 {
            return Err(DirectoryError::Conflict("exact pending directory command receipt is not owned".into()));
        }
        Ok(())
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        validate_checkpoint_publication_claim(claim)?;
        let mut tx = self.pool.begin().await.map_err(backend)?;
        let existing: Option<CheckpointPublicationReceiptRowV1> = sqlx_core::query_as::query_as(
            "SELECT actor_user_id, correlation_id, command_sha256, disposition, checkpoint_id, claimed_at, completed_at FROM hub_checkpoint_publication_receipt WHERE actor_user_id = $1 AND correlation_id = $2 FOR UPDATE",
        )
        .bind(&claim.actor_user_id)
        .bind(&claim.correlation_id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(backend)?;
        if let Some(row) = existing {
            let record = checkpoint_publication_receipt_from_row(row)?;
            tx.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { CheckpointPublicationClaimV1::Existing(record) } else { CheckpointPublicationClaimV1::Conflict });
        }
        sqlx_core::query::query("INSERT INTO hub_checkpoint_publication_receipt(actor_user_id, correlation_id, command_sha256, disposition, checkpoint_id, claimed_at, completed_at) VALUES ($1,$2,$3,'pending',NULL,$4,NULL)")
            .bind(&claim.actor_user_id)
            .bind(&claim.correlation_id)
            .bind(&claim.command_sha256)
            .bind(claim.claimed_at)
            .execute(&mut *tx)
            .await
            .map_err(backend)?;
        tx.commit().await.map_err(backend)?;
        Ok(CheckpointPublicationClaimV1::Claimed(CheckpointPublicationReceiptRecordV1 {
            actor_user_id: claim.actor_user_id.clone(),
            correlation_id: claim.correlation_id.clone(),
            command_sha256: claim.command_sha256.clone(),
            disposition: CheckpointPublicationDispositionV1::Pending,
            checkpoint_id: None,
            claimed_at: claim.claimed_at,
            completed_at: None,
        }))
    }

    async fn release_checkpoint_publication(&self, actor_user_id: &str, correlation_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        sqlx_core::query::query("DELETE FROM hub_checkpoint_publication_receipt WHERE actor_user_id = $1 AND correlation_id = $2 AND command_sha256 = $3 AND disposition = 'pending'")
            .bind(actor_user_id)
            .bind(correlation_id)
            .bind(command_sha256)
            .execute(&self.pool)
            .await
            .map_err(backend)?;
        Ok(())
    }
}
//#endregion
