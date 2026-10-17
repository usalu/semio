//! ✅️ Approval outbox: the reconciliation vocabulary, prepared approvals, recovery by mutation and reconciliation of committed approvals.

use super::*;

/// 🧾️ Approval reconciliation returns the durable undo handle from the same transaction.
pub(crate) struct InferenceApprovalReconciliationV1 {
    pub applied: bool,
    pub undo: GisMapApprovalUndoHandleV1,
}

/// 🔎️ Which guard of one approval reconciliation answered `Conflict`. Five distinct refusals share
/// the one taxonomy value the caller sees, so a law that reads the error cannot name the guard. The
/// returned error is identical in production; only a test build records which guard produced it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ApprovalReconciliationConflictV1 {
    WitnessDiffers,
    FrontierDocumentDiffers,
    FrontierHeadEditIdDiffers,
    FrontierHeadOrdinalDiffers,
    FrontierCommitSeqDiffers,
    CommittedUndoRowMissing,
    OutboxPhaseNotPrepared,
    JobOrProposalPhaseDiffers,
    OutboxCommandDiffers,
}

#[cfg(test)]
static LAST_APPROVAL_RECONCILIATION_CONFLICT: Mutex<Option<ApprovalReconciliationConflictV1>> = Mutex::new(None);

/// 🧾️ Answers the unchanged `Conflict` after recording which guard produced it.
pub(crate) fn approval_reconciliation_conflict(guard: ApprovalReconciliationConflictV1) -> InferenceErrorV1 {
    #[cfg(test)]
    {
        *LAST_APPROVAL_RECONCILIATION_CONFLICT.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Some(guard);
    }
    let _ = guard;
    InferenceErrorV1::Conflict
}

/// 🔬️ The last recorded reconciliation guard of this process, for laws that only see the taxonomy.
#[cfg(test)]
pub(crate) fn last_approval_reconciliation_conflict() -> Option<ApprovalReconciliationConflictV1> {
    *LAST_APPROVAL_RECONCILIATION_CONFLICT.lock().unwrap_or_else(std::sync::PoisonError::into_inner)
}

impl InferenceJobLedgerV1 {
    pub fn prepare_approval(&self, job_id: &str, current: &InferenceIdentityV1, proposal_hash: &str, command: &InferencePrivateBytesV1, now: u64) -> Result<InferenceApprovalOutboxV1, InferenceErrorV1> {
        current.validate()?;
        if command.as_slice().is_empty() || command.as_slice().len() > COMMAND_MAX_BYTES || now > SAFE_INTEGER_MAX || !hex(proposal_hash, 64) {
            return Err(InferenceErrorV1::Bounds);
        }
        let mutation_id = sha256(format!("semio.hub.inference-approval-mutation/v1\0{job_id}\0{proposal_hash}").as_bytes())[..32].to_string();
        let document_key = format!("v1:{}:{}:{}{}", current.space_id.len(), current.document_id.len(), current.space_id, current.document_id);
        let actor = format!("user:{}#session:{}", current.user_id, current.session_id);
        let decoded = semio_hub_inference::command::CanonicalInferenceCommandV1::decode(command.as_slice())?;
        if !decoded.matches_identity(&mutation_id, &document_key, &actor) {
            return Err(InferenceErrorV1::Conflict);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let accepted = identity(&tx, job_id)?;
        if accepted.user_id != current.user_id || accepted.session_id != current.session_id || accepted.space_id != current.space_id || accepted.document_id != current.document_id {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, proposal_phase, expires_at) = state(&tx, job_id)?;
        if phase != "succeeded" || proposal_phase != "offered" {
            return Err(InferenceErrorV1::Conflict);
        }
        let existing: Option<(String, String, String, u64, String)> = tx
            .query_row("SELECT mutation_id,command_hash,proposal_hash,prepared_at,phase FROM inference_approval_outbox_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, read_integer(row, 3)?, row.get(4)?)))
            .optional()
            .map_err(storage)?;
        let command_hash = sha256(command.as_slice());
        if let Some((mutation_id, previous_command_hash, previous_proposal_hash, prepared_at_ms, outbox_phase)) = existing {
            if previous_command_hash != command_hash || previous_proposal_hash != proposal_hash || accepted != *current {
                return Err(InferenceErrorV1::Conflict);
            }
            if now >= expires_at {
                return Err(InferenceErrorV1::Expired);
            }
            if outbox_phase == "prepared" {
                return Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), COMMAND_MAX_BYTES)? });
            }
            if outbox_phase != "abandoned" {
                return Err(InferenceErrorV1::Conflict);
            }
            tx.execute("UPDATE inference_approval_outbox_v1 SET command=?2,prepared_at=?3,phase='prepared' WHERE job_id=?1 AND phase='abandoned'", params![job_id, command.as_slice(), sql_integer(now)?]).map_err(storage)?;
            tx.commit().map_err(storage)?;
            return Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms: now, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), COMMAND_MAX_BYTES)? });
        }
        if accepted != *current || now >= expires_at {
            tx.execute("UPDATE inference_job_v1 SET proposal_state='stale',result=X'',proposal=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
            event(&tx, job_id, "proposal-stale", now)?;
            tx.commit().map_err(storage)?;
            return Err(if now >= expires_at { InferenceErrorV1::Expired } else { InferenceErrorV1::Conflict });
        }
        let proposal = InferencePrivateBytesV1::new(tx.query_row("SELECT proposal FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| row.get(0)).map_err(storage)?, PROPOSAL_MAX_BYTES)?;
        if sha256(proposal.as_slice()) != proposal_hash {
            return Err(InferenceErrorV1::Conflict);
        }
        tx.execute("INSERT INTO inference_approval_outbox_v1 VALUES (?1,?2,?3,?4,?5,?6,'prepared')", params![job_id, mutation_id, command_hash, proposal_hash, command.as_slice(), sql_integer(now)?]).map_err(storage)?;
        event(&tx, job_id, "approval-prepared", now)?;
        tx.commit().map_err(storage)?;
        Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms: now, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), COMMAND_MAX_BYTES)? })
    }

    pub fn abandon_prepared_approval(&self, reader: &InferenceReaderV1<'_>, job_id: &str, mutation_id: &str, command_hash: &str, proposal_hash: &str) -> Result<bool, InferenceErrorV1> {
        if !hex(job_id, 32) || !hex(mutation_id, 32) || !hex(command_hash, 64) || !hex(proposal_hash, 64) {
            return Err(InferenceErrorV1::Invalid);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let row: Option<(String, String, String, String)> =
            tx.query_row("SELECT mutation_id,command_hash,proposal_hash,phase FROM inference_approval_outbox_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))).optional().map_err(storage)?;
        let Some((stored_mutation, stored_command, stored_proposal, phase)) = row else { return Err(InferenceErrorV1::Denied) };
        if stored_mutation != mutation_id || stored_command != command_hash || stored_proposal != proposal_hash {
            return Err(InferenceErrorV1::Conflict);
        }
        if phase == "abandoned" {
            tx.commit().map_err(storage)?;
            return Ok(false);
        }
        if phase != "prepared" {
            return Err(InferenceErrorV1::Conflict);
        }
        tx.execute("UPDATE inference_approval_outbox_v1 SET command=X'',phase='abandoned' WHERE job_id=?1 AND phase='prepared'", [job_id]).map_err(storage)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    pub fn pending_approvals(&self, after: Option<&str>, control: &semio_hub_inference::InferenceOperationControlV1) -> Result<InferenceApprovalPageV1, InferenceErrorV1> {
        if after.is_some_and(|cursor| !hex(cursor, 32)) {
            return Err(InferenceErrorV1::Invalid);
        }
        control.checkpoint(0)?;
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let mut query = connection.prepare("SELECT job_id,mutation_id,command_hash,proposal_hash,prepared_at,command FROM inference_approval_outbox_v1 WHERE phase='prepared' AND (?1 IS NULL OR job_id>?1) ORDER BY job_id LIMIT 5").map_err(storage)?;
        let mut found = query.query([after]).map_err(storage)?;
        let mut rows = Vec::with_capacity(4);
        let mut next_cursor = None;
        while let Some(row) = found.next().map_err(storage)? {
            control.checkpoint(rows.len() as u64 + 1)?;
            if rows.len() == 4 {
                next_cursor = rows.last().map(|row: &InferenceApprovalOutboxV1| row.job_id.clone());
                break;
            }
            rows.push(InferenceApprovalOutboxV1 {
                job_id: row.get(0).map_err(storage)?,
                mutation_id: row.get(1).map_err(storage)?,
                command_hash: row.get(2).map_err(storage)?,
                proposal_hash: row.get(3).map_err(storage)?,
                prepared_at_ms: read_integer(row, 4).map_err(storage)?,
                command: InferencePrivateBytesV1::new(row.get(5).map_err(storage)?, COMMAND_MAX_BYTES)?,
            });
        }
        Ok(InferenceApprovalPageV1 { rows, next_cursor })
    }

    pub(crate) fn prepared_approval_by_mutation(&self, mutation_id: &str) -> Result<Option<InferenceApprovalOutboxV1>, InferenceErrorV1> {
        if !hex(mutation_id, 32) {
            return Err(InferenceErrorV1::Invalid);
        }
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let row: Option<(String, String, String, String, u64, Vec<u8>)> = connection
            .query_row("SELECT job_id,mutation_id,command_hash,proposal_hash,prepared_at,command FROM inference_approval_outbox_v1 WHERE mutation_id=?1 AND phase='prepared'", [mutation_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, read_integer(row, 4)?, row.get(5)?))
            })
            .optional()
            .map_err(storage)?;
        row.map(|(job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command)| Ok(InferenceApprovalOutboxV1 { job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command: InferencePrivateBytesV1::new(command, COMMAND_MAX_BYTES)? }))
            .transpose()
    }

    pub fn approval_recovery_by_mutation(&self, mutation_id: &str) -> Result<Option<(InferenceApprovalOutboxV1, InferenceIdentityV1, bool)>, InferenceErrorV1> {
        if !hex(mutation_id, 32) {
            return Err(InferenceErrorV1::Invalid);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let row: Option<(String, String, String, String, u64, Vec<u8>, String)> = tx
            .query_row("SELECT job_id,mutation_id,command_hash,proposal_hash,prepared_at,command,phase FROM inference_approval_outbox_v1 WHERE mutation_id=?1 AND phase IN ('prepared','committed')", [mutation_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, read_integer(row, 4)?, row.get(5)?, row.get(6)?))
            })
            .optional()
            .map_err(storage)?;
        let Some((job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command, phase)) = row else {
            tx.commit().map_err(storage)?;
            return Ok(None);
        };
        let accepted = identity(&tx, &job_id)?;
        tx.commit().map_err(storage)?;
        let outbox = InferenceApprovalOutboxV1 { job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command: InferencePrivateBytesV1::new(command, COMMAND_MAX_BYTES)? };
        Ok(Some((outbox, accepted, phase == "committed")))
    }

    pub fn reconcile_committed_approval(
        &self,
        job_id: &str,
        witness: &semio_hub_inference::wal::CommittedInferenceWalWitnessV1,
        document_generation: u64,
        after_frontier: &directory::os_directory::EditedArtifactFrontierV1,
        descriptor_digest: &str,
        after_base_digest: &str,
        now: u64,
    ) -> Result<InferenceApprovalReconciliationV1, InferenceErrorV1> {
        if !hex(job_id, 32) || !hex(descriptor_digest, 64) || !hex(after_base_digest, 64) || document_generation > SAFE_INTEGER_MAX || now > SAFE_INTEGER_MAX || !after_frontier.validate() {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let existing: Option<(String, String, String, Vec<u8>, String)> = tx
            .query_row("SELECT mutation_id,command_hash,proposal_hash,command,phase FROM inference_approval_outbox_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?)))
            .optional()
            .map_err(storage)?;
        let (accepted_mutation, accepted_command, accepted_proposal, command, phase) = existing.ok_or(InferenceErrorV1::Denied)?;
        let accepted = identity(&tx, job_id)?;
        let scope = directory::os_directory::DocumentScope::new(accepted.space_id, accepted.document_id);
        if !witness.matches(&scope, document_generation, job_id, &accepted_proposal, &accepted_mutation, &accepted_command) {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::WitnessDiffers));
        }
        if after_frontier.document_id != scope.document_id {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::FrontierDocumentDiffers));
        }
        if after_frontier.head_edit_id != accepted_mutation {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::FrontierHeadEditIdDiffers));
        }
        if after_frontier.head_edit_ordinal != accepted.head_ordinal.checked_add(1).ok_or(InferenceErrorV1::Bounds)? {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::FrontierHeadOrdinalDiffers));
        }
        if after_frontier.last_commit_seq != accepted.last_commit_seq.checked_add(1).ok_or(InferenceErrorV1::Bounds)? {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::FrontierCommitSeqDiffers));
        }
        let witness_digest = witness.approval_undo_witness_digest();
        let target_id = sha256(format!("semio.hub.gis-map-approval-undo-target/v1\0{witness_digest}").as_bytes())[..32].to_owned();
        let undo = GisMapApprovalUndoHandleV1 { target_id: target_id.clone(), expected_current: after_frontier.clone() };
        if phase == "committed" {
            let retained: bool = tx
                .query_row(
                    "SELECT EXISTS(SELECT 1 FROM inference_approval_undo_v1 WHERE target_id=?1 AND job_id=?2 AND original_mutation_id=?3 AND original_command_hash=?4 AND committed_witness_digest=?5 AND user_id=?6 AND session_id=?7 AND authorization_generation=?8 AND space_id=?9 AND document_id=?10 AND after_head_ordinal=?11 AND after_head_edit_id=?12 AND after_commit_seq=?13 AND after_chain_sha256=?14 AND descriptor_digest=?15 AND after_base_digest=?16)",
                    params![target_id, job_id, accepted_mutation, accepted_command, witness_digest, accepted.user_id, accepted.session_id, sql_integer(accepted.authorization_generation)?, scope.space_id, scope.document_id, sql_integer(after_frontier.head_edit_ordinal)?, after_frontier.head_edit_id, sql_integer(after_frontier.last_commit_seq)?, after_frontier.chain_sha256, descriptor_digest, after_base_digest],
                    |row| row.get(0),
                )
                .map_err(storage)?;
            if !retained {
                return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::CommittedUndoRowMissing));
            }
            tx.commit().map_err(storage)?;
            return Ok(InferenceApprovalReconciliationV1 { applied: false, undo });
        }
        if phase != "prepared" {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::OutboxPhaseNotPrepared));
        }
        let (job_phase, proposal_phase, _) = state(&tx, job_id)?;
        if job_phase != "succeeded" || proposal_phase != "offered" {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::JobOrProposalPhaseDiffers));
        }
        if command.is_empty() || command.len() > COMMAND_MAX_BYTES || sha256(&command) != accepted_command {
            return Err(approval_reconciliation_conflict(ApprovalReconciliationConflictV1::OutboxCommandDiffers));
        }
        tx.execute(
            "INSERT INTO inference_approval_undo_v1(target_id,job_id,original_mutation_id,original_command_hash,committed_witness_digest,user_id,session_id,authorization_generation,space_id,document_id,after_head_ordinal,after_head_edit_id,after_commit_seq,after_chain_sha256,descriptor_digest,after_base_digest,original_command,phase) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13,?14,?15,?16,?17,'available')",
            params![target_id, job_id, accepted_mutation, accepted_command, witness_digest, accepted.user_id, accepted.session_id, sql_integer(accepted.authorization_generation)?, scope.space_id, scope.document_id, sql_integer(after_frontier.head_edit_ordinal)?, after_frontier.head_edit_id, sql_integer(after_frontier.last_commit_seq)?, after_frontier.chain_sha256, descriptor_digest, after_base_digest, command],
        )
        .map_err(storage)?;
        tx.execute("UPDATE inference_approval_outbox_v1 SET phase='committed',command=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
        tx.execute("UPDATE inference_job_v1 SET proposal_state='approved',proposal=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
        event(&tx, job_id, "approved", now)?;
        tx.commit().map_err(storage)?;
        Ok(InferenceApprovalReconciliationV1 { applied: true, undo })
    }
}
