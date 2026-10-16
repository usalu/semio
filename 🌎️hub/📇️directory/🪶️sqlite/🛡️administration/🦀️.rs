//! 🛡️ Admin operation audit, directory command receipts and checkpoint publication claims.

use super::*;

pub(super) fn admin_effect_rollback<T>(tx: Transaction<'_>) -> AdminEffectCommitV1<T> {
    match tx.rollback() {
        Ok(()) => AdminEffectCommitV1::RejectedBeforeCommit,
        Err(_) => AdminEffectCommitV1::Indeterminate,
    }
}

pub(super) fn insert_admin_operation_effect_receipt(tx: &Connection, receipt: &AdminOperationEffectReceiptV1) -> DirectoryResult<()> {
    validate_admin_operation_effect_receipt(receipt)?;
    let changed = tx
        .execute(
            "INSERT OR IGNORE INTO hub_admin_operation_effect_receipt (operation_id, intent_digest, committed_at, outcome_code, event_seq_first, event_seq_last) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            rusqlite::params![
                receipt.operation_id,
                receipt.intent_digest,
                receipt.committed_at,
                receipt.outcome_code,
                receipt.event_seq_first.map(i64::try_from).transpose().map_err(backend)?,
                receipt.event_seq_last.map(i64::try_from).transpose().map_err(backend)?,
            ],
        )
        .map_err(backend)?;
    if changed == 1 {
        return Ok(());
    }
    let sql = format!("SELECT {ADMIN_OPERATION_EFFECT_RECEIPT_SELECT} FROM hub_admin_operation_effect_receipt WHERE operation_id = ?1");
    let established = tx.query_row(&sql, [&receipt.operation_id], admin_operation_effect_receipt_row).map_err(backend)?;
    if &established == receipt { Ok(()) } else { Err(DirectoryError::Conflict("admin operation effect receipt identity changed".into())) }
}

//#region AdminOperations
impl AdminOperationDirectory for SqliteDirectory {
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        let (spaces, users, connections): (i64, i64, i64) = self
            .lock()?
            .query_row("SELECT (SELECT COUNT(*) FROM hub_space), (SELECT COUNT(*) FROM hub_user), (SELECT COUNT(*) FROM hub_sync_session WHERE disconnected_at IS NULL)", [], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)))
            .map_err(backend)?;
        Ok(AdminDirectoryOverviewCounts { spaces: u64::try_from(spaces).map_err(backend)?, users: u64::try_from(users).map_err(backend)?, connections: u64::try_from(connections).map_err(backend)? })
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        validate_admin_operation_audit(fact)?;
        let terminal = i64::from(fact.phase != "accepted");
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let existing: Vec<AdminOperationAuditRecord> = {
            let sql = format!("SELECT {ADMIN_OPERATION_AUDIT_SELECT} FROM hub_admin_operation_audit WHERE request_id = ?1 ORDER BY sequence LIMIT 2");
            let mut statement = tx.prepare(&sql).map_err(backend)?;
            let rows = statement.query_map([&fact.request_id], admin_operation_audit_row).map_err(backend)?;
            rows.collect::<Result<Vec<_>, _>>().map_err(backend)?
        };
        if fact.phase == "accepted" {
            if let Some(established) = existing.first() {
                let outcome = if same_admin_operation_request(&established.fact, fact) { Ok(established.clone()) } else { Err(DirectoryError::Conflict("admin request id was reused for a different intent".into())) };
                tx.rollback().map_err(backend)?;
                return outcome;
            }
        } else {
            let accepted = existing.iter().find(|row| row.fact.phase == "accepted").ok_or_else(|| DirectoryError::Conflict("admin terminal fact requires accepted fact".into()))?;
            if accepted.fact.operation_id != fact.operation_id || !same_admin_operation_request(&accepted.fact, fact) {
                return Err(DirectoryError::Conflict("admin operation already terminated or changed identity".into()));
            }
            if let Some(established) = existing.iter().find(|row| row.fact.phase != "accepted") {
                let outcome = if established.fact.operation_id == fact.operation_id && established.fact.phase == fact.phase {
                    Ok(established.clone())
                } else {
                    Err(DirectoryError::Conflict("admin operation already terminated with a different outcome".into()))
                };
                tx.rollback().map_err(backend)?;
                return outcome;
            }
        }
        let changed = tx
            .execute(
            "INSERT OR IGNORE INTO hub_admin_operation_audit (request_id, intent_digest, operation_id, occurred_at, phase, terminal, intent_kind, target_kind, target_id, principal_user_id, principal_session_id, principal_generation, correlation_id, event_seq_first, event_seq_last, outcome_code, reason_code) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17)",
            rusqlite::params![fact.request_id, fact.intent_digest, fact.operation_id, fact.occurred_at, fact.phase, terminal, fact.intent_kind, fact.target_kind, fact.target_id, fact.principal_user_id, fact.principal_session_id, i64::try_from(fact.principal_generation).map_err(backend)?, fact.correlation_id, fact.event_seq_first.map(i64::try_from).transpose().map_err(backend)?, fact.event_seq_last.map(i64::try_from).transpose().map_err(backend)?, fact.outcome_code, fact.reason_code],
        )
        .map_err(backend)?;
        if changed == 0 {
            let sql = format!("SELECT {ADMIN_OPERATION_AUDIT_SELECT} FROM hub_admin_operation_audit WHERE request_id = ?1 AND terminal = ?2");
            let established = tx.query_row(&sql, rusqlite::params![fact.request_id, terminal], admin_operation_audit_row).map_err(backend)?;
            tx.rollback().map_err(backend)?;
            return if same_admin_operation_request(&established.fact, fact) { Ok(established) } else { Err(DirectoryError::Conflict("admin request id race changed intent".into())) };
        }
        let sequence = u64::try_from(tx.last_insert_rowid()).map_err(backend)?;
        tx.commit().map_err(backend)?;
        Ok(AdminOperationAuditRecord { sequence, fact: fact.clone() })
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(request_id, "admin request id", AUTH_TEXT_MAX_BYTES)?;
        let conn = self.lock()?;
        let sql = format!("SELECT {ADMIN_OPERATION_AUDIT_SELECT} FROM hub_admin_operation_audit WHERE request_id = ?1 ORDER BY sequence LIMIT 2");
        let mut statement = conn.prepare(&sql).map_err(backend)?;
        let rows = statement.query_map([request_id], admin_operation_audit_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES)?;
        let conn = self.lock()?;
        let sql = format!("SELECT {ADMIN_OPERATION_AUDIT_SELECT} FROM hub_admin_operation_audit WHERE operation_id = ?1 ORDER BY sequence LIMIT 2");
        let mut statement = conn.prepare(&sql).map_err(backend)?;
        let rows = statement.query_map([operation_id], admin_operation_audit_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        validate_bounded_auth_text(operation_id, "admin effect operation id", AUTH_TEXT_MAX_BYTES)?;
        if intent_digest.len() != 64 || !intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict("admin effect intent digest must be 64 lowercase hex digits".into()));
        }
        let sql = format!("SELECT {ADMIN_OPERATION_EFFECT_RECEIPT_SELECT} FROM hub_admin_operation_effect_receipt WHERE operation_id = ?1 AND intent_digest = ?2");
        self.lock()?.query_row(&sql, rusqlite::params![operation_id, intent_digest], admin_operation_effect_receipt_row).optional().map_err(backend)
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("admin audit limit must be 1..={ADMIN_PAGE_MAX}")));
        }
        let conn = self.lock()?;
        let sql = format!("SELECT {ADMIN_OPERATION_AUDIT_SELECT} FROM hub_admin_operation_audit WHERE sequence > ?1 ORDER BY sequence LIMIT ?2");
        let mut statement = conn.prepare(&sql).map_err(backend)?;
        let rows = statement.query_map(rusqlite::params![i64::try_from(after_sequence).map_err(backend)?, i64::try_from(limit).map_err(backend)?], admin_operation_audit_row).map_err(backend)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(backend)
    }
}

impl ReceiptDirectory for SqliteDirectory {
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        validate_directory_command_claim(claim)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let existing = tx
            .query_row(&format!("SELECT {DIRECTORY_COMMAND_RECEIPT_SELECT} FROM hub_directory_command_receipt WHERE actor_user_id = ?1 AND request_id = ?2"), rusqlite::params![claim.actor_user_id, claim.request_id], directory_command_receipt_row)
            .optional()
            .map_err(backend)?;
        if let Some(record) = existing {
            tx.rollback().map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { DirectoryCommandClaimV1::Existing(record) } else { DirectoryCommandClaimV1::Conflict });
        }
        tx.execute(
            "INSERT INTO hub_directory_command_receipt (actor_user_id, request_id, command_sha256, result_kind, disposition, event_seq_first, event_seq_last, receipt_sha256, claimed_at, completed_at) VALUES (?1, ?2, ?3, ?4, 'pending', NULL, NULL, NULL, ?5, NULL)",
            rusqlite::params![claim.actor_user_id, claim.request_id, claim.command_sha256, directory_command_result_kind_str(claim.result_kind), claim.claimed_at],
        )
        .map_err(backend)?;
        tx.commit().map_err(backend)?;
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
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let changed = tx
            .execute(
                "UPDATE hub_directory_command_receipt SET disposition = 'completed', event_seq_first = ?3, event_seq_last = ?4, receipt_sha256 = ?5, completed_at = ?6 WHERE actor_user_id = ?1 AND request_id = ?2 AND disposition = 'pending'",
                rusqlite::params![
                    completion.actor_user_id,
                    completion.request_id,
                    completion.event_seq_first.map(i64::try_from).transpose().map_err(backend)?,
                    completion.event_seq_last.map(i64::try_from).transpose().map_err(backend)?,
                    completion.receipt_sha256,
                    completion.completed_at
                ],
            )
            .map_err(backend)?;
        if changed == 0 {
            tx.rollback().map_err(backend)?;
            return Err(DirectoryError::Conflict("directory command receipt claim is not pending".into()));
        }
        let record = tx
            .query_row(
                &format!("SELECT {DIRECTORY_COMMAND_RECEIPT_SELECT} FROM hub_directory_command_receipt WHERE actor_user_id = ?1 AND request_id = ?2"),
                rusqlite::params![completion.actor_user_id, completion.request_id],
                directory_command_receipt_row,
            )
            .map_err(backend)?;
        tx.commit().map_err(backend)?;
        Ok(record)
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        let conn = self.lock()?;
        let changed =
            conn.execute("DELETE FROM hub_directory_command_receipt WHERE actor_user_id = ?1 AND request_id = ?2 AND command_sha256 = ?3 AND disposition = 'pending'", rusqlite::params![actor_user_id, request_id, command_sha256]).map_err(backend)?;
        if changed != 1 {
            return Err(DirectoryError::Conflict("exact pending directory command receipt is not owned".into()));
        }
        Ok(())
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        validate_checkpoint_publication_claim(claim)?;
        let mut conn = self.lock()?;
        let tx = conn.transaction().map_err(backend)?;
        let existing = tx
            .query_row(
                "SELECT actor_user_id, correlation_id, command_sha256, disposition, checkpoint_id, claimed_at, completed_at FROM hub_checkpoint_publication_receipt WHERE actor_user_id = ?1 AND correlation_id = ?2",
                rusqlite::params![claim.actor_user_id, claim.correlation_id],
                checkpoint_publication_receipt_row,
            )
            .optional()
            .map_err(backend)?;
        if let Some(record) = existing {
            tx.rollback().map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { CheckpointPublicationClaimV1::Existing(record) } else { CheckpointPublicationClaimV1::Conflict });
        }
        tx.execute(
            "INSERT INTO hub_checkpoint_publication_receipt(actor_user_id, correlation_id, command_sha256, disposition, checkpoint_id, claimed_at, completed_at) VALUES (?1, ?2, ?3, 'pending', NULL, ?4, NULL)",
            rusqlite::params![claim.actor_user_id, claim.correlation_id, claim.command_sha256, claim.claimed_at],
        )
        .map_err(backend)?;
        tx.commit().map_err(backend)?;
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
        self.lock()?
            .execute("DELETE FROM hub_checkpoint_publication_receipt WHERE actor_user_id = ?1 AND correlation_id = ?2 AND command_sha256 = ?3 AND disposition = 'pending'", rusqlite::params![actor_user_id, correlation_id, command_sha256])
            .map_err(backend)?;
        Ok(())
    }
}
//#endregion
