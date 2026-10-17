//! 🗺️ GIS map approval undo: the undo target, admission and recovery records and their ledger transitions.

use super::*;

/// ↩️ Exact private inverse authority retained from the original committed approval.
pub struct GisMapApprovalUndoTargetV1 {
    pub target_id: String,
    pub original_job_id: String,
    pub original_mutation_id: String,
    pub original_command_hash: String,
    pub committed_witness_digest: String,
    pub user_id: String,
    pub session_id: String,
    pub authorization_generation: u64,
    pub scope: directory::os_directory::DocumentScope,
    pub after_frontier: directory::os_directory::EditedArtifactFrontierV1,
    pub descriptor_digest: String,
    pub after_base_digest: String,
    pub original_command: InferencePrivateBytesV1,
}

/// 🎫️ Durable undo idempotency admission or exact terminal replay.
pub enum GisMapApprovalUndoAdmissionV1 {
    Prepared,
    Replayed(GisMapApprovalUndoReceiptV1),
}

/// 🧊 One exact retained undo identity needed to resume a committed Store journal after restart.
pub(crate) struct GisMapApprovalUndoRecoveryV1 {
    pub target: GisMapApprovalUndoTargetV1,
    pub idempotency_key: String,
    pub operation_id: String,
    pub proposal_hash: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub command: InferencePrivateBytesV1,
    pub ledger_applied: bool,
}

impl InferenceJobLedgerV1 {
    /// 🔎️ Reads the exact private undo authority only for its original authenticated owner.
    pub fn gis_map_approval_undo_target(&self, target_id: &str, reader: &InferenceReaderV1<'_>) -> Result<GisMapApprovalUndoTargetV1, InferenceErrorV1> {
        if !hex(target_id, 32) {
            return Err(InferenceErrorV1::Bounds);
        }
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let row = connection
            .query_row(
                "SELECT job_id,original_mutation_id,original_command_hash,committed_witness_digest,user_id,session_id,authorization_generation,space_id,document_id,after_head_ordinal,after_head_edit_id,after_commit_seq,after_chain_sha256,descriptor_digest,after_base_digest,original_command FROM inference_approval_undo_v1 WHERE target_id=?1 AND phase IN ('available','prepared')",
                [target_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?, row.get::<_, String>(3)?, row.get::<_, String>(4)?, row.get::<_, String>(5)?, read_integer(row, 6)?, row.get::<_, String>(7)?,
                        row.get::<_, String>(8)?, read_integer(row, 9)?, row.get::<_, String>(10)?, read_integer(row, 11)?, row.get::<_, String>(12)?, row.get::<_, String>(13)?, row.get::<_, String>(14)?, row.get::<_, Vec<u8>>(15)?,
                    ))
                },
            )
            .optional()
            .map_err(storage)?
            .ok_or(InferenceErrorV1::Denied)?;
        if reader.user_id != row.4 || reader.session_id != row.5 || reader.authorization_generation != row.6 || reader.space_id != row.7 || reader.document_id != row.8 {
            return Err(InferenceErrorV1::Denied);
        }
        let after_frontier = directory::os_directory::EditedArtifactFrontierV1 { document_id: row.8.clone(), head_edit_ordinal: row.9, head_edit_id: row.10, last_commit_seq: row.11, chain_sha256: row.12 };
        if !after_frontier.validate() || !hex(&row.13, 64) || !hex(&row.14, 64) || sha256(&row.15) != row.2 {
            return Err(InferenceErrorV1::Conflict);
        }
        Ok(GisMapApprovalUndoTargetV1 {
            target_id: target_id.to_owned(),
            original_job_id: row.0,
            original_mutation_id: row.1,
            original_command_hash: row.2,
            committed_witness_digest: row.3,
            user_id: row.4,
            session_id: row.5,
            authorization_generation: row.6,
            scope: directory::os_directory::DocumentScope::new(row.7, row.8),
            after_frontier,
            descriptor_digest: row.13,
            after_base_digest: row.14,
            original_command: InferencePrivateBytesV1::new(row.15, COMMAND_MAX_BYTES)?,
        })
    }

    /// 🪪️ Resolves only the original owner's private job identity for live authorization recheck.
    pub fn gis_map_approval_undo_job_id(&self, target_id: &str, reader: &InferenceReaderV1<'_>) -> Result<String, InferenceErrorV1> {
        if !hex(target_id, 32) {
            return Err(InferenceErrorV1::Bounds);
        }
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let row: (String, String, String, u64, String, String) = connection
            .query_row("SELECT job_id,user_id,session_id,authorization_generation,space_id,document_id FROM inference_approval_undo_v1 WHERE target_id=?1", [target_id], |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, read_integer(row, 3)?, row.get(4)?, row.get(5)?))
            })
            .optional()
            .map_err(storage)?
            .ok_or(InferenceErrorV1::Denied)?;
        if reader.user_id != row.1 || reader.session_id != row.2 || reader.authorization_generation != row.3 || reader.space_id != row.4 || reader.document_id != row.5 {
            return Err(InferenceErrorV1::Denied);
        }
        Ok(row.0)
    }

    /// 🔁️ Replays only the exact committed target/idempotency tuple to its original owner.
    pub fn replayed_gis_map_approval_undo(&self, target_id: &str, idempotency_key: &str, reader: &InferenceReaderV1<'_>) -> Result<Option<GisMapApprovalUndoReceiptV1>, InferenceErrorV1> {
        if !hex(target_id, 32) || !hex(idempotency_key, 32) {
            return Err(InferenceErrorV1::Bounds);
        }
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let row: (String, String, String, u64, String, String, String, Option<String>, Option<String>, Option<String>, Option<u64>, Option<String>, Option<u64>, Option<String>) = connection
            .query_row(
                "SELECT phase,user_id,session_id,authorization_generation,space_id,document_id,job_id,undo_idempotency_key,undo_mutation_id,undo_command_hash,undo_frontier_head_ordinal,undo_frontier_head_edit_id,undo_frontier_commit_seq,undo_frontier_chain_sha256 FROM inference_approval_undo_v1 WHERE target_id=?1",
                [target_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, read_integer(row, 3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?, row.get(8)?, row.get(9)?, row.get::<_, Option<i64>>(10)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)), row.get(11)?, row.get::<_, Option<i64>>(12)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)), row.get(13)?)),
            )
            .optional()
            .map_err(storage)?
            .ok_or(InferenceErrorV1::Denied)?;
        if reader.user_id != row.1 || reader.session_id != row.2 || reader.authorization_generation != row.3 || reader.space_id != row.4 || reader.document_id != row.5 {
            return Err(InferenceErrorV1::Denied);
        }
        match row.0.as_str() {
            "available" => Ok(None),
            "prepared" if row.7.as_deref() == Some(idempotency_key) => Ok(None),
            "committed" if row.7.as_deref() == Some(idempotency_key) => {
                let frontier = directory::os_directory::EditedArtifactFrontierV1 {
                    document_id: row.5,
                    head_edit_ordinal: row.10.ok_or(InferenceErrorV1::Storage)?,
                    head_edit_id: row.11.ok_or(InferenceErrorV1::Storage)?,
                    last_commit_seq: row.12.ok_or(InferenceErrorV1::Storage)?,
                    chain_sha256: row.13.ok_or(InferenceErrorV1::Storage)?,
                };
                if !frontier.validate() {
                    return Err(InferenceErrorV1::Storage);
                }
                Ok(Some(GisMapApprovalUndoReceiptV1 {
                    schema: "semio.hub.gis-map-approval-undo-receipt/v1".into(),
                    target_id: target_id.to_owned(),
                    original_job_id: row.6,
                    mutation_id: row.8.ok_or(InferenceErrorV1::Storage)?,
                    command_hash: row.9.ok_or(InferenceErrorV1::Storage)?,
                    applied: true,
                    replayed: true,
                    frontier,
                }))
            }
            _ => Err(InferenceErrorV1::Conflict),
        }
    }

    /// 🎫️ Persists one exact server-derived inverse identity before any Store/WAL effect.
    #[allow(clippy::too_many_arguments)]
    pub fn prepare_gis_map_approval_undo(
        &self,
        target: &GisMapApprovalUndoTargetV1,
        idempotency_key: &str,
        operation_id: &str,
        proposal_hash: &str,
        mutation_id: &str,
        command_hash: &str,
        command: &InferencePrivateBytesV1,
    ) -> Result<GisMapApprovalUndoAdmissionV1, InferenceErrorV1> {
        if !hex(idempotency_key, 32) || !hex(operation_id, 32) || !hex(proposal_hash, 64) || !hex(mutation_id, 32) || !hex(command_hash, 64) || command.as_slice().is_empty() || sha256(command.as_slice()) != command_hash {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let row: (String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, Option<u64>, Option<String>, Option<u64>, Option<String>, Vec<u8>) = tx
            .query_row(
                "SELECT phase,undo_idempotency_key,undo_job_id,undo_proposal_hash,undo_mutation_id,undo_command_hash,undo_frontier_head_ordinal,undo_frontier_head_edit_id,undo_frontier_commit_seq,undo_frontier_chain_sha256,undo_command FROM inference_approval_undo_v1 WHERE target_id=?1 AND job_id=?2 AND committed_witness_digest=?3",
                params![target.target_id, target.original_job_id, target.committed_witness_digest],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?, row.get::<_, Option<i64>>(6)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)), row.get(7)?, row.get::<_, Option<i64>>(8)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)), row.get(9)?, row.get(10)?)),
            )
            .optional()
            .map_err(storage)?
            .ok_or(InferenceErrorV1::Denied)?;
        let exact = row.1.as_deref() == Some(idempotency_key) && row.2.as_deref() == Some(operation_id) && row.3.as_deref() == Some(proposal_hash) && row.4.as_deref() == Some(mutation_id) && row.5.as_deref() == Some(command_hash);
        match row.0.as_str() {
            "available" => {
                let updated = tx
                    .execute(
                        "UPDATE inference_approval_undo_v1 SET undo_idempotency_key=?2,undo_job_id=?3,undo_proposal_hash=?4,undo_mutation_id=?5,undo_command_hash=?6,undo_command=?7,phase='prepared' WHERE target_id=?1 AND phase='available'",
                        params![target.target_id, idempotency_key, operation_id, proposal_hash, mutation_id, command_hash, command.as_slice()],
                    )
                    .map_err(storage)?;
                if updated != 1 {
                    return Err(InferenceErrorV1::Conflict);
                }
                tx.commit().map_err(storage)?;
                Ok(GisMapApprovalUndoAdmissionV1::Prepared)
            }
            "prepared" if exact && row.10 == command.as_slice() => {
                tx.commit().map_err(storage)?;
                Ok(GisMapApprovalUndoAdmissionV1::Prepared)
            }
            "committed" if exact => {
                let frontier = directory::os_directory::EditedArtifactFrontierV1 {
                    document_id: target.scope.document_id.clone(),
                    head_edit_ordinal: row.6.ok_or(InferenceErrorV1::Storage)?,
                    head_edit_id: row.7.ok_or(InferenceErrorV1::Storage)?,
                    last_commit_seq: row.8.ok_or(InferenceErrorV1::Storage)?,
                    chain_sha256: row.9.ok_or(InferenceErrorV1::Storage)?,
                };
                if !frontier.validate() {
                    return Err(InferenceErrorV1::Storage);
                }
                tx.commit().map_err(storage)?;
                Ok(GisMapApprovalUndoAdmissionV1::Replayed(GisMapApprovalUndoReceiptV1 {
                    schema: "semio.hub.gis-map-approval-undo-receipt/v1".into(),
                    target_id: target.target_id.clone(),
                    original_job_id: target.original_job_id.clone(),
                    mutation_id: mutation_id.to_owned(),
                    command_hash: command_hash.to_owned(),
                    applied: true,
                    replayed: true,
                    frontier,
                }))
            }
            _ => Err(InferenceErrorV1::Conflict),
        }
    }

    /// 🧾️ Finalizes an undo only from its exact second committed-WAL witness.
    pub fn reconcile_committed_gis_map_approval_undo(
        &self,
        target_id: &str,
        witness: &semio_hub_inference::wal::CommittedInferenceWalWitnessV1,
        document_generation: u64,
        frontier: &directory::os_directory::EditedArtifactFrontierV1,
    ) -> Result<bool, InferenceErrorV1> {
        if !hex(target_id, 32) || !frontier.validate() {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let row: (String, String, String, u64, String, u64, String, String, String, String, String, Option<u64>, Option<String>, Option<u64>, Option<String>) = tx
            .query_row(
                "SELECT phase,space_id,document_id,after_head_ordinal,after_head_edit_id,after_commit_seq,undo_idempotency_key,undo_job_id,undo_proposal_hash,undo_mutation_id,undo_command_hash,undo_frontier_head_ordinal,undo_frontier_head_edit_id,undo_frontier_commit_seq,undo_frontier_chain_sha256 FROM inference_approval_undo_v1 WHERE target_id=?1",
                [target_id],
                |row| {
                    Ok((
                        row.get(0)?,
                        row.get(1)?,
                        row.get(2)?,
                        read_integer(row, 3)?,
                        row.get(4)?,
                        read_integer(row, 5)?,
                        row.get(6)?,
                        row.get(7)?,
                        row.get(8)?,
                        row.get(9)?,
                        row.get(10)?,
                        row.get::<_, Option<i64>>(11)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)),
                        row.get(12)?,
                        row.get::<_, Option<i64>>(13)?.map(|value| u64::try_from(value).unwrap_or(u64::MAX)),
                        row.get(14)?,
                    ))
                },
            )
            .optional()
            .map_err(storage)?
            .ok_or(InferenceErrorV1::Denied)?;
        let scope = directory::os_directory::DocumentScope::new(row.1, row.2);
        if !witness.matches(&scope, document_generation, &row.7, &row.8, &row.9, &row.10)
            || frontier.document_id != scope.document_id
            || frontier.head_edit_id != row.9
            || frontier.head_edit_ordinal != row.3.checked_add(1).ok_or(InferenceErrorV1::Bounds)?
            || frontier.last_commit_seq != row.5.checked_add(1).ok_or(InferenceErrorV1::Bounds)?
        {
            return Err(InferenceErrorV1::Conflict);
        }
        if row.0 == "committed" {
            let exact_terminal = row.11 == Some(frontier.head_edit_ordinal) && row.12.as_deref() == Some(frontier.head_edit_id.as_str()) && row.13 == Some(frontier.last_commit_seq) && row.14.as_deref() == Some(frontier.chain_sha256.as_str());
            return if exact_terminal { Ok(false) } else { Err(InferenceErrorV1::Conflict) };
        }
        if row.0 != "prepared" {
            return Err(InferenceErrorV1::Conflict);
        }
        tx.execute(
            "UPDATE inference_approval_undo_v1 SET original_command=X'',undo_command=X'',undo_frontier_head_ordinal=?2,undo_frontier_head_edit_id=?3,undo_frontier_commit_seq=?4,undo_frontier_chain_sha256=?5,phase='committed' WHERE target_id=?1 AND phase='prepared'",
            params![target_id, sql_integer(frontier.head_edit_ordinal)?, frontier.head_edit_id, sql_integer(frontier.last_commit_seq)?, frontier.chain_sha256],
        )
        .map_err(storage)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    /// 🧰 Restores only the exact prepared or committed undo identity addressed by its WAL mutation.
    pub fn gis_map_approval_undo_recovery_by_mutation(&self, mutation_id: &str) -> Result<Option<GisMapApprovalUndoRecoveryV1>, InferenceErrorV1> {
        if !hex(mutation_id, 32) {
            return Err(InferenceErrorV1::Bounds);
        }
        let connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let row = connection
            .query_row(
                "SELECT target_id,job_id,original_mutation_id,original_command_hash,committed_witness_digest,user_id,session_id,authorization_generation,space_id,document_id,after_head_ordinal,after_head_edit_id,after_commit_seq,after_chain_sha256,descriptor_digest,after_base_digest,original_command,undo_idempotency_key,undo_job_id,undo_proposal_hash,undo_mutation_id,undo_command_hash,undo_command,phase FROM inference_approval_undo_v1 WHERE undo_mutation_id=?1 AND phase IN ('prepared','committed')",
                [mutation_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, String>(4)?,
                        row.get::<_, String>(5)?,
                        row.get::<_, String>(6)?,
                        read_integer(row, 7)?,
                        row.get::<_, String>(8)?,
                        row.get::<_, String>(9)?,
                        read_integer(row, 10)?,
                        row.get::<_, String>(11)?,
                        read_integer(row, 12)?,
                        row.get::<_, String>(13)?,
                        row.get::<_, String>(14)?,
                        row.get::<_, String>(15)?,
                        row.get::<_, Vec<u8>>(16)?,
                        row.get::<_, String>(17)?,
                        row.get::<_, String>(18)?,
                        row.get::<_, String>(19)?,
                        row.get::<_, String>(20)?,
                        row.get::<_, String>(21)?,
                        row.get::<_, Vec<u8>>(22)?,
                        row.get::<_, String>(23)?,
                    ))
                },
            )
            .optional()
            .map_err(storage)?;
        let Some(row) = row else { return Ok(None) };
        let after_frontier = directory::os_directory::EditedArtifactFrontierV1 { document_id: row.9.clone(), head_edit_ordinal: row.10, head_edit_id: row.11.clone(), last_commit_seq: row.12, chain_sha256: row.13.clone() };
        let prepared = row.23 == "prepared";
        if !after_frontier.validate()
            || !hex(&row.0, 32)
            || !hex(&row.2, 32)
            || !hex(&row.3, 64)
            || !hex(&row.4, 64)
            || !hex(&row.14, 64)
            || !hex(&row.15, 64)
            || !hex(&row.17, 32)
            || !hex(&row.18, 32)
            || !hex(&row.19, 64)
            || row.20 != mutation_id
            || !hex(&row.21, 64)
            || prepared && (row.16.is_empty() || sha256(&row.16) != row.3 || row.22.is_empty() || sha256(&row.22) != row.21)
            || !prepared && (!row.16.is_empty() || !row.22.is_empty())
        {
            return Err(InferenceErrorV1::Conflict);
        }
        Ok(Some(GisMapApprovalUndoRecoveryV1 {
            target: GisMapApprovalUndoTargetV1 {
                target_id: row.0,
                original_job_id: row.1,
                original_mutation_id: row.2,
                original_command_hash: row.3,
                committed_witness_digest: row.4,
                user_id: row.5,
                session_id: row.6,
                authorization_generation: row.7,
                scope: directory::os_directory::DocumentScope::new(row.8, row.9),
                after_frontier,
                descriptor_digest: row.14,
                after_base_digest: row.15,
                original_command: InferencePrivateBytesV1::new(row.16, COMMAND_MAX_BYTES)?,
            },
            idempotency_key: row.17,
            operation_id: row.18,
            proposal_hash: row.19,
            mutation_id: row.20,
            command_hash: row.21,
            command: InferencePrivateBytesV1::new(row.22, COMMAND_MAX_BYTES)?,
            ledger_applied: !prepared,
        }))
    }
}
