//! 🛡️ Admin operation audit, directory command receipts and checkpoint publication claims.

use super::*;

//#region AdminOperations
impl AdminOperationDirectory for Neo4jDirectory {
    async fn admin_overview_counts(&self) -> DirectoryResult<AdminDirectoryOverviewCounts> {
        let mut result = self
            .graph
            .execute(query(
                "MATCH (s:Space) WITH count(s) AS spaces
                 OPTIONAL MATCH (u:User) WITH spaces, count(u) AS users
                 OPTIONAL MATCH (session:SyncSession) WHERE session.disconnectedAt IS NULL
                 RETURN spaces, users, count(session) AS connections",
            ))
            .await
            .map_err(backend)?;
        let row = result.next().await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("administrator overview count query returned no row".into()))?;
        let spaces: i64 = row.get("spaces").map_err(backend)?;
        let users: i64 = row.get("users").map_err(backend)?;
        let connections: i64 = row.get("connections").map_err(backend)?;
        Ok(AdminDirectoryOverviewCounts { spaces: u64::try_from(spaces).map_err(backend)?, users: u64::try_from(users).map_err(backend)?, connections: u64::try_from(connections).map_err(backend)? })
    }

    async fn append_admin_operation_audit(&self, fact: &NewAdminOperationAuditRecord) -> DirectoryResult<AdminOperationAuditRecord> {
        validate_admin_operation_audit(fact)?;
        let terminal = fact.phase != "accepted";
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing_result = txn.execute(query("MATCH (a:AdminOperationAudit {requestId: $request_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("request_id", fact.request_id.clone())).await.map_err(backend)?;
        let mut existing = Vec::new();
        while let Some(row) = existing_result.next(txn.handle()).await.map_err(backend)? {
            existing.push(admin_operation_audit_from_node(&row)?);
        }
        drop(existing_result);
        if !terminal {
            if let Some(established) = existing.first() {
                let outcome = if same_admin_operation_request(&established.fact, fact) { Ok(established.clone()) } else { Err(DirectoryError::Conflict("admin request id was reused for a different intent".into())) };
                txn.rollback().await.map_err(backend)?;
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
                txn.rollback().await.map_err(backend)?;
                return outcome;
            }
        }
        let mut sequence_result = txn.execute(query("MERGE (c:AdminOperationAuditCounter {id: 'singleton'}) ON CREATE SET c.sequence = 0 SET c.sequence = c.sequence + 1 RETURN c.sequence AS sequence")).await.map_err(backend)?;
        let sequence: i64 = sequence_result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("admin operation audit counter returned no row".into()))?.get("sequence").map_err(backend)?;
        drop(sequence_result);
        let mut merged = txn
            .execute(
            query("MERGE (a:AdminOperationAudit {requestTerminalKey: $request_terminal_key}) ON CREATE SET a.sequence = $sequence, a.requestId = $request_id, a.intentDigest = $intent_digest, a.operationId = $operation_id, a.occurredAt = $occurred_at, a.phase = $phase, a.intentKind = $intent_kind, a.targetKind = $target_kind, a.targetId = $target_id, a.principalUserId = $principal_user_id, a.principalSessionId = $principal_session_id, a.principalGeneration = $principal_generation, a.correlationId = $correlation_id, a.eventSeqFirst = $event_seq_first, a.eventSeqLast = $event_seq_last, a.outcomeCode = $outcome_code, a.reasonCode = $reason_code RETURN a AS a")
                .param("sequence", sequence)
                .param("request_id", fact.request_id.clone())
                .param("intent_digest", fact.intent_digest.clone())
                .param("request_terminal_key", format!("{}:{}", fact.request_id, u8::from(terminal)))
                .param("operation_id", fact.operation_id.clone())
                .param("occurred_at", fact.occurred_at)
                .param("phase", fact.phase.clone())
                .param("intent_kind", fact.intent_kind.clone())
                .param("target_kind", fact.target_kind.clone())
                .param("target_id", fact.target_id.clone())
                .param("principal_user_id", fact.principal_user_id.clone())
                .param("principal_session_id", fact.principal_session_id.clone())
                .param("principal_generation", i64::try_from(fact.principal_generation).map_err(backend)?)
                .param("correlation_id", fact.correlation_id.clone())
                .param("event_seq_first", fact.event_seq_first.map(i64::try_from).transpose().map_err(backend)?.unwrap_or_default())
                .param("event_seq_last", fact.event_seq_last.map(i64::try_from).transpose().map_err(backend)?.unwrap_or_default())
                .param("outcome_code", fact.outcome_code.clone())
                .param("reason_code", fact.reason_code.clone().unwrap_or_default()),
        )
        .await
        .map_err(backend)?;
        let established = merged.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("admin operation audit merge returned no row".into()))?;
        let established = admin_operation_audit_from_node(&established)?;
        drop(merged);
        txn.commit().await.map_err(backend)?;
        if same_admin_operation_request(&established.fact, fact) { Ok(established) } else { Err(DirectoryError::Conflict("admin request id race changed intent".into())) }
    }

    async fn admin_operation_audit_for_request(&self, request_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(request_id, "admin request id", AUTH_TEXT_MAX_BYTES)?;
        let mut result = self.graph.execute(query("MATCH (a:AdminOperationAudit {requestId: $request_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("request_id", request_id)).await.map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }

    async fn admin_operation_audit_for_operation(&self, operation_id: &str) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        validate_bounded_auth_text(operation_id, "admin operation id", AUTH_TEXT_MAX_BYTES)?;
        let mut result = self.graph.execute(query("MATCH (a:AdminOperationAudit {operationId: $operation_id}) RETURN a AS a ORDER BY a.sequence LIMIT 2").param("operation_id", operation_id)).await.map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }

    async fn admin_operation_effect_receipt(&self, operation_id: &str, intent_digest: &str) -> DirectoryResult<Option<AdminOperationEffectReceiptV1>> {
        validate_bounded_auth_text(operation_id, "admin effect operation id", AUTH_TEXT_MAX_BYTES)?;
        if intent_digest.len() != 64 || !intent_digest.bytes().all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase()) {
            return Err(DirectoryError::Conflict("admin effect intent digest must be 64 lowercase hex digits".into()));
        }
        let mut result = self
            .graph
            .execute(query("MATCH (r:AdminOperationEffectReceipt {operationId: $operation_id, intentDigest: $intent_digest}) RETURN r AS r LIMIT 1").param("operation_id", operation_id).param("intent_digest", intent_digest))
            .await
            .map_err(backend)?;
        let Some(row) = result.next().await.map_err(backend)? else { return Ok(None) };
        let node: neo4rs::Node = row.get("r").map_err(backend)?;
        let first = node.get::<i64>("eventSeqFirst").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
        let last = node.get::<i64>("eventSeqLast").ok().filter(|value| *value > 0).map(u64::try_from).transpose().map_err(backend)?;
        let receipt = AdminOperationEffectReceiptV1 {
            operation_id: node.get("operationId").map_err(backend)?,
            intent_digest: node.get("intentDigest").map_err(backend)?,
            committed_at: node.get("committedAt").map_err(backend)?,
            outcome_code: node.get("outcomeCode").map_err(backend)?,
            event_seq_first: first,
            event_seq_last: last,
        };
        validate_admin_operation_effect_receipt(&receipt)?;
        Ok(Some(receipt))
    }

    async fn list_admin_operation_audit(&self, after_sequence: u64, limit: usize) -> DirectoryResult<Vec<AdminOperationAuditRecord>> {
        if limit == 0 || limit > ADMIN_PAGE_MAX {
            return Err(DirectoryError::Conflict(format!("admin audit limit must be 1..={ADMIN_PAGE_MAX}")));
        }
        let mut result = self
            .graph
            .execute(
                query("MATCH (a:AdminOperationAudit) WHERE a.sequence > $after RETURN a AS a ORDER BY a.sequence LIMIT $limit").param("after", i64::try_from(after_sequence).map_err(backend)?).param("limit", i64::try_from(limit).map_err(backend)?),
            )
            .await
            .map_err(backend)?;
        let mut records = Vec::new();
        while let Some(row) = result.next().await.map_err(backend)? {
            records.push(admin_operation_audit_from_node(&row)?);
        }
        Ok(records)
    }
}

impl ReceiptDirectory for Neo4jDirectory {
    async fn claim_or_read_directory_command_receipt(&self, claim: &NewDirectoryCommandReceipt) -> DirectoryResult<DirectoryCommandClaimV1> {
        validate_directory_command_claim(claim)?;
        let key = directory_command_receipt_key(&claim.actor_user_id, &claim.request_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing_result = txn.execute(query("MATCH (r:DirectoryCommandReceipt {key: $key}) RETURN r AS r LIMIT 1").param("key", key.clone())).await.map_err(backend)?;
        let existing = match existing_result.next(txn.handle()).await.map_err(backend)? {
            Some(row) => Some(directory_command_receipt_from_node(&row)?),
            None => None,
        };
        drop(existing_result);
        if let Some(record) = existing {
            txn.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { DirectoryCommandClaimV1::Existing(record) } else { DirectoryCommandClaimV1::Conflict });
        }
        txn.run(
            query("CREATE (:DirectoryCommandReceipt {key: $key, actorUserId: $actor_user_id, requestId: $request_id, commandSha256: $command_sha256, resultKind: $result_kind, disposition: 'pending', eventSeqFirst: 0, eventSeqLast: 0, receiptSha256: '', claimedAt: $claimed_at, completedAt: 0})")
                .param("key", key)
                .param("actor_user_id", claim.actor_user_id.clone())
                .param("request_id", claim.request_id.clone())
                .param("command_sha256", claim.command_sha256.clone())
                .param("result_kind", directory_command_result_kind_str(claim.result_kind))
                .param("claimed_at", claim.claimed_at),
        )
        .await
        .map_err(backend)?;
        txn.commit().await.map_err(backend)?;
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
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut updated = txn
            .execute(
                query("MATCH (r:DirectoryCommandReceipt {key: $key, disposition: 'pending'}) SET r.disposition = 'completed', r.eventSeqFirst = $event_seq_first, r.eventSeqLast = $event_seq_last, r.receiptSha256 = $receipt_sha256, r.completedAt = $completed_at RETURN r AS r")
                    .param("key", directory_command_receipt_key(&completion.actor_user_id, &completion.request_id))
                    .param("event_seq_first", i64::try_from(completion.event_seq_first.unwrap_or(0)).map_err(backend)?)
                    .param("event_seq_last", i64::try_from(completion.event_seq_last.unwrap_or(0)).map_err(backend)?)
                    .param("receipt_sha256", completion.receipt_sha256.clone())
                    .param("completed_at", completion.completed_at),
            )
            .await
            .map_err(backend)?;
        let row = updated.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Conflict("directory command receipt claim is not pending".into()))?;
        let record = directory_command_receipt_from_node(&row)?;
        drop(updated);
        txn.commit().await.map_err(backend)?;
        Ok(record)
    }

    async fn release_directory_command_receipt(&self, actor_user_id: &str, request_id: &str, command_sha256: &str) -> DirectoryResult<()> {
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut result = txn
            .execute(
                query("MATCH (r:DirectoryCommandReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) DELETE r RETURN count(*) AS changed")
                    .param("key", directory_command_receipt_key(actor_user_id, request_id))
                    .param("command_sha256", command_sha256),
            )
            .await
            .map_err(backend)?;
        let changed: i64 = result.next(txn.handle()).await.map_err(backend)?.ok_or_else(|| DirectoryError::Backend("receipt release returned no count".into()))?.get("changed").map_err(backend)?;
        drop(result);
        if changed != 1 {
            txn.rollback().await.map_err(backend)?;
            return Err(DirectoryError::Conflict("exact pending directory command receipt is not owned".into()));
        }
        txn.commit().await.map_err(backend)?;
        Ok(())
    }

    async fn claim_or_read_checkpoint_publication(&self, claim: &NewCheckpointPublicationClaimV1) -> DirectoryResult<CheckpointPublicationClaimV1> {
        validate_checkpoint_publication_claim(claim)?;
        let key = checkpoint_publication_receipt_key(&claim.actor_user_id, &claim.correlation_id);
        let mut txn = self.graph.start_txn().await.map_err(backend)?;
        let mut existing = txn.execute(query("MATCH (r:CheckpointPublicationReceipt {key: $key}) RETURN r AS r LIMIT 1").param("key", key.clone())).await.map_err(backend)?;
        let record = existing.next(txn.handle()).await.map_err(backend)?.map(|row| checkpoint_publication_receipt_from_node(&row)).transpose()?;
        drop(existing);
        if let Some(record) = record {
            txn.rollback().await.map_err(backend)?;
            return Ok(if record.command_sha256 == claim.command_sha256 { CheckpointPublicationClaimV1::Existing(record) } else { CheckpointPublicationClaimV1::Conflict });
        }
        txn.run(
            query("CREATE (:CheckpointPublicationReceipt {key: $key, actorUserId: $actor_user_id, correlationId: $correlation_id, commandSha256: $command_sha256, disposition: 'pending', checkpointId: '', claimedAt: $claimed_at, completedAt: 0})")
                .param("key", key)
                .param("actor_user_id", claim.actor_user_id.clone())
                .param("correlation_id", claim.correlation_id.clone())
                .param("command_sha256", claim.command_sha256.clone())
                .param("claimed_at", claim.claimed_at),
        )
        .await
        .map_err(backend)?;
        txn.commit().await.map_err(backend)?;
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
        self.graph
            .run(
                query("MATCH (r:CheckpointPublicationReceipt {key: $key, commandSha256: $command_sha256, disposition: 'pending'}) DELETE r")
                    .param("key", checkpoint_publication_receipt_key(actor_user_id, correlation_id))
                    .param("command_sha256", command_sha256.to_string()),
            )
            .await
            .map_err(backend)?;
        Ok(())
    }
}
//#endregion
