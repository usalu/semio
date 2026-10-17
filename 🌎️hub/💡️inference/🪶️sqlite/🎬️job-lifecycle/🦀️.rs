//! 🎬️ Job lifecycle: open, accept, start, heartbeat, terminal transitions, reconcile, events and reads.

use super::*;

impl InferenceJobLedgerV1 {
    pub fn open(path: &Path) -> Result<Self, InferenceErrorV1> {
        let connection = Connection::open(path).map_err(storage)?;
        connection.busy_timeout(std::time::Duration::from_secs(2)).map_err(storage)?;
        connection.execute_batch(&ledger_schema()).map_err(storage)?;
        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn accept(&self, selected: &InferenceIdentityV1, input: &InferencePrivateBytesV1, now: u64) -> Result<InferenceLedgerJobRowV1, InferenceErrorV1> {
        let digest = selected.digest()?;
        if input.as_slice().is_empty() || input.as_slice().len() > INPUT_MAX_BYTES {
            return Err(InferenceErrorV1::Bounds);
        }
        if sha256(input.as_slice()) != selected.input_hash {
            return Err(InferenceErrorV1::Conflict);
        }
        let expires_at = now.checked_add(selected.request.lifetime_ms).filter(|time| *time <= SAFE_INTEGER_MAX).ok_or(InferenceErrorV1::Bounds)?;
        let json = serde_json::to_string(selected).map_err(|_| InferenceErrorV1::Invalid)?;
        if json.len() > IDENTITY_JSON_MAX_BYTES {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let existing = tx
            .query_row(
                "SELECT job_id,identity_digest,expires_at FROM inference_job_v1 WHERE user_id=?1 AND authorization_generation=?2 AND space_id=?3 AND document_id=?4 AND request_id=?5",
                params![selected.user_id, sql_integer(selected.authorization_generation)?, selected.space_id, selected.document_id, selected.request.request_id],
                |row| Ok(InferenceLedgerJobRowV1 { job_id: row.get(0)?, identity_digest: row.get(1)?, expires_at_ms: read_integer(row, 2)? }),
            )
            .optional()
            .map_err(storage)?;
        if let Some(existing) = existing {
            if existing.identity_digest != digest {
                return Err(InferenceErrorV1::Conflict);
            }
            return Ok(existing);
        }
        let count = tx.query_row("SELECT COUNT(*) FROM inference_job_v1", [], |row| read_integer(row, 0)).map_err(storage)?;
        if count >= JOB_CAPACITY as u64 {
            return Err(InferenceErrorV1::Capacity);
        }
        let job_id = sha256(format!("semio.hub.inference-job-id/v1\0{digest}").as_bytes())[..32].to_string();
        tx.execute(
            "INSERT INTO inference_job_v1(job_id,request_id,user_id,authorization_generation,space_id,document_id,identity_digest,identity_json,expires_at,state,proposal_state,input,result,proposal,terminal_at) VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,'accepted','none',?10,X'',X'',NULL)",
            params![job_id, selected.request.request_id, selected.user_id, sql_integer(selected.authorization_generation)?, selected.space_id, selected.document_id, digest, json, sql_integer(expires_at)?, input.as_slice()],
        )
        .map_err(storage)?;
        event(&tx, &job_id, "accepted", now)?;
        tx.commit().map_err(storage)?;
        Ok(InferenceLedgerJobRowV1 { job_id, identity_digest: digest, expires_at_ms: expires_at })
    }

    /// 🎟️ Claims the sole execution turn; a live lease is never stolen and a cancel request wins.
    pub fn start(&self, job_id: &str, current: &InferenceIdentityV1, now: u64) -> Result<Option<InferenceRunClaimV1>, InferenceErrorV1> {
        current.validate()?;
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let lease_expires_at = now.checked_add(CLAIM_LEASE_MAX_MS).filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(InferenceErrorV1::Bounds)?;
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let accepted = identity(&tx, job_id)?;
        if accepted.user_id != current.user_id || accepted.session_id != current.session_id || accepted.space_id != current.space_id || accepted.document_id != current.document_id {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, expires_at) = state(&tx, job_id)?;
        if phase != "accepted" && phase != "running" {
            return Ok(None);
        }
        if accepted != *current || now >= expires_at {
            terminate(&tx, job_id, "cancelled", now)?;
            tx.commit().map_err(storage)?;
            return Err(if now >= expires_at { InferenceErrorV1::Expired } else { InferenceErrorV1::Conflict });
        }
        let (run_epoch, lease, cancel_requested) = run_lease(&tx, job_id)?;
        if cancel_requested {
            terminate(&tx, job_id, "cancelled", now)?;
            tx.commit().map_err(storage)?;
            return Err(InferenceErrorV1::Cancelled);
        }
        if phase == "running" && now < lease {
            return Ok(None);
        }
        let claimed = run_epoch.checked_add(1).filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(InferenceErrorV1::Bounds)?;
        tx.execute("UPDATE inference_job_v1 SET state='running',run_epoch=?2,lease_expires_at=?3 WHERE job_id=?1", params![job_id, sql_integer(claimed)?, sql_integer(lease_expires_at)?]).map_err(storage)?;
        if phase == "accepted" {
            event(&tx, job_id, "running", now)?;
        }
        tx.commit().map_err(storage)?;
        Ok(Some(InferenceRunClaimV1 { run_epoch: claimed, lease_expires_at_ms: lease_expires_at }))
    }

    /// 🏁️ Publishes the private result and proposal for exactly the claiming epoch, once.
    pub fn succeed(&self, job_id: &str, current: &InferenceIdentityV1, run_epoch: u64, result: &InferencePrivateBytesV1, proposal: &InferencePrivateBytesV1, now: u64) -> Result<bool, InferenceErrorV1> {
        if result.as_slice().is_empty() || result.as_slice().len() > RESULT_MAX_BYTES || proposal.as_slice().is_empty() || proposal.as_slice().len() > PROPOSAL_MAX_BYTES {
            return Err(InferenceErrorV1::Bounds);
        }
        current.validate()?;
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let accepted = identity(&tx, job_id)?;
        if accepted.user_id != current.user_id || accepted.session_id != current.session_id || accepted.space_id != current.space_id || accepted.document_id != current.document_id {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, expires_at) = state(&tx, job_id)?;
        if phase != "accepted" && phase != "running" {
            return Ok(false);
        }
        if accepted != *current || now >= expires_at {
            terminate(&tx, job_id, "cancelled", now)?;
            tx.commit().map_err(storage)?;
            return Err(if now >= expires_at { InferenceErrorV1::Expired } else { InferenceErrorV1::Conflict });
        }
        let (current_epoch, lease, cancel_requested) = run_lease(&tx, job_id)?;
        if cancel_requested {
            terminate(&tx, job_id, "cancelled", now)?;
            tx.commit().map_err(storage)?;
            return Err(InferenceErrorV1::Cancelled);
        }
        if phase != "running" || current_epoch != run_epoch || run_epoch == 0 || now >= lease {
            return Ok(false);
        }
        tx.execute("UPDATE inference_job_v1 SET state='succeeded',proposal_state='offered',input=X'',result=?2,proposal=?3,terminal_at=?4 WHERE job_id=?1", params![job_id, result.as_slice(), proposal.as_slice(), sql_integer(now)?])
            .map_err(storage)?;
        event(&tx, job_id, "succeeded", now)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    /// 💓️ Renews one exact live claim while appending its bounded monotonic progress row.
    pub fn heartbeat(&self, job_id: &str, reader: &InferenceReaderV1<'_>, run_epoch: u64, completed: u64, total: u64, now: u64) -> Result<u64, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX || total == 0 || completed > total || total > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, expires_at) = state(&tx, job_id)?;
        let (current_epoch, lease, _) = run_lease(&tx, job_id)?;
        if phase != "running" || current_epoch != run_epoch || run_epoch == 0 || now >= lease || now >= expires_at {
            return Err(InferenceErrorV1::Conflict);
        }
        let lease_expires_at = now.checked_add(CLAIM_LEASE_MAX_MS).map(|value| value.min(expires_at)).filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(InferenceErrorV1::Bounds)?;
        let cursor: u64 = tx.query_row("SELECT progress_cursor FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| read_integer(row, 0)).map_err(storage)?;
        let next = cursor.checked_add(1).ok_or(InferenceErrorV1::Bounds)?;
        if next > PROGRESS_MAX_CURSOR {
            return Err(InferenceErrorV1::Bounds);
        }
        let previous: Option<u64> = tx.query_row("SELECT completed FROM inference_job_progress_v1 WHERE job_id=?1 AND cursor=?2", params![job_id, sql_integer(cursor)?], |row| read_integer(row, 0)).optional().map_err(storage)?;
        if previous.is_some_and(|value| completed < value) {
            return Err(InferenceErrorV1::Conflict);
        }
        tx.execute(
            "INSERT INTO inference_job_progress_v1(job_id,cursor,run_epoch,completed,total,at_ms) VALUES (?1,?2,?3,?4,?5,?6)",
            params![job_id, sql_integer(next)?, sql_integer(run_epoch)?, sql_integer(completed)?, sql_integer(total)?, sql_integer(now)?],
        )
        .map_err(storage)?;
        tx.execute("UPDATE inference_job_v1 SET progress_cursor=?2,lease_expires_at=?3 WHERE job_id=?1 AND run_epoch=?4", params![job_id, sql_integer(next)?, sql_integer(lease_expires_at)?, sql_integer(run_epoch)?]).map_err(storage)?;
        tx.commit().map_err(storage)?;
        Ok(next)
    }

    /// 📈️ Preserves the public progress operation as the claim-renewing heartbeat boundary.
    pub fn progress(&self, job_id: &str, reader: &InferenceReaderV1<'_>, run_epoch: u64, completed: u64, total: u64, now: u64) -> Result<u64, InferenceErrorV1> {
        self.heartbeat(job_id, reader, run_epoch, completed, total, now)
    }

    /// 🔄️ Renews an exact live epoch after its bounded progress page is already full.
    pub fn renew_claim(&self, job_id: &str, reader: &InferenceReaderV1<'_>, run_epoch: u64, now: u64) -> Result<u64, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX || run_epoch == 0 {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, expires_at) = state(&tx, job_id)?;
        let (current_epoch, lease, cancel_requested) = run_lease(&tx, job_id)?;
        if phase != "running" || current_epoch != run_epoch || now >= lease || now >= expires_at {
            return Err(InferenceErrorV1::Conflict);
        }
        if cancel_requested {
            return Err(InferenceErrorV1::Cancelled);
        }
        let lease_expires_at = now.checked_add(CLAIM_LEASE_MAX_MS).map(|value| value.min(expires_at)).filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(InferenceErrorV1::Bounds)?;
        tx.execute("UPDATE inference_job_v1 SET lease_expires_at=?2 WHERE job_id=?1 AND run_epoch=?3", params![job_id, sql_integer(lease_expires_at)?, sql_integer(run_epoch)?]).map_err(storage)?;
        tx.commit().map_err(storage)?;
        Ok(lease_expires_at)
    }

    /// 🛑️ Cancels only the still-current execution epoch and publishes its durable request first.
    pub fn cancel_run(&self, job_id: &str, reader: &InferenceReaderV1<'_>, run_epoch: u64, now: u64) -> Result<bool, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX || run_epoch == 0 {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, _) = state(&tx, job_id)?;
        let (current_epoch, _, cancel_requested) = run_lease(&tx, job_id)?;
        if phase != "running" || current_epoch != run_epoch {
            return Ok(false);
        }
        if !cancel_requested {
            tx.execute("UPDATE inference_job_v1 SET cancel_requested_at=?2 WHERE job_id=?1", params![job_id, sql_integer(now)?]).map_err(storage)?;
            event(&tx, job_id, "cancel-requested", now)?;
        }
        terminate(&tx, job_id, "cancelled", now)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    /// 🧯️ Fails only the still-current execution epoch; a late worker cannot retire its successor.
    pub fn fail_run(&self, job_id: &str, reader: &InferenceReaderV1<'_>, run_epoch: u64, now: u64) -> Result<bool, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX || run_epoch == 0 {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, _) = state(&tx, job_id)?;
        let (current_epoch, _, _) = run_lease(&tx, job_id)?;
        if phase != "running" || current_epoch != run_epoch {
            return Ok(false);
        }
        terminate(&tx, job_id, "failed", now)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    /// 🛑️ Records a durable cancel request the executor observes at its next bounded checkpoint.
    pub fn request_cancel(&self, job_id: &str, reader: &InferenceReaderV1<'_>, now: u64) -> Result<bool, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        {
            let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
            let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
            if !reader.matches(&identity(&tx, job_id)?) {
                return Err(InferenceErrorV1::Denied);
            }
            let (phase, _, _) = state(&tx, job_id)?;
            let (_, _, cancel_requested) = run_lease(&tx, job_id)?;
            if !cancel_requested && matches!(phase.as_str(), "accepted" | "running" | "succeeded") {
                tx.execute("UPDATE inference_job_v1 SET cancel_requested_at=?2 WHERE job_id=?1", params![job_id, sql_integer(now)?]).map_err(storage)?;
                event(&tx, job_id, "cancel-requested", now)?;
            }
            tx.commit().map_err(storage)?;
        }
        self.cancel(job_id, reader, now)
    }

    /// 🪪️ Returns the immutable accepted identity to its original owner alone.
    pub fn identity_of(&self, job_id: &str, reader: &InferenceReaderV1<'_>) -> Result<InferenceIdentityV1, InferenceErrorV1> {
        if !hex(job_id, 32) {
            return Err(InferenceErrorV1::Invalid);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let accepted = identity(&tx, job_id)?;
        if !reader.matches(&accepted) {
            return Err(InferenceErrorV1::Denied);
        }
        tx.commit().map_err(storage)?;
        Ok(accepted)
    }

    /// 🧭️ Resolves one existing submit identity for its exact original reader without applying expiry or creating work.
    pub fn reconcile_request(&self, request_id: &str, reader: &InferenceReaderV1<'_>, now: u64) -> Result<Option<InferenceJobReconcileJobV1>, InferenceErrorV1> {
        if !hex(request_id, 32) || now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        let row: Option<(String, String, String, u64, String, String, bool, Vec<u8>)> = tx
            .query_row(
                "SELECT job_id,identity_digest,identity_json,expires_at,state,proposal_state,cancel_requested_at IS NOT NULL,proposal FROM inference_job_v1 WHERE user_id=?1 AND authorization_generation=?2 AND space_id=?3 AND document_id=?4 AND request_id=?5",
                params![reader.user_id, sql_integer(reader.authorization_generation)?, reader.space_id, reader.document_id, request_id],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, read_integer(row, 3)?, row.get(4)?, row.get(5)?, row.get(6)?, row.get(7)?)),
            )
            .optional()
            .map_err(storage)?;
        let Some((job_id, identity_digest, identity_json, expires_at_ms, state_value, proposal_state_value, cancel_requested, proposal)) = row else {
            tx.commit().map_err(storage)?;
            return Ok(None);
        };
        let accepted: InferenceIdentityV1 = serde_json::from_str(&identity_json).map_err(|_| InferenceErrorV1::Storage)?;
        if !reader.matches(&accepted) {
            return Err(InferenceErrorV1::Denied);
        }
        if accepted.request.request_id != request_id || accepted.digest()? != identity_digest {
            return Err(InferenceErrorV1::Storage);
        }
        let state: InferenceJobStateV1 = serde_json::from_value(serde_json::Value::String(state_value)).map_err(|_| InferenceErrorV1::Storage)?;
        let proposal_state: InferenceProposalStateV1 = serde_json::from_value(serde_json::Value::String(proposal_state_value)).map_err(|_| InferenceErrorV1::Storage)?;
        let mut events = Vec::with_capacity(EVENT_PAGE_MAX_ITEMS);
        {
            let mut query = tx.prepare("SELECT ordinal,kind,at_ms FROM inference_job_event_v1 WHERE job_id=?1 ORDER BY ordinal LIMIT ?2").map_err(storage)?;
            let mut found = query.query(params![job_id, EVENT_PAGE_MAX_ITEMS as i64]).map_err(storage)?;
            while let Some(row) = found.next().map_err(storage)? {
                events.push(InferenceEventV1 { ordinal: read_integer(row, 0).map_err(storage)?, kind: row.get(1).map_err(storage)?, at_ms: read_integer(row, 2).map_err(storage)? });
            }
        }
        let mut progress = Vec::with_capacity(PROGRESS_MAX_CURSOR as usize);
        {
            let mut query = tx.prepare("SELECT cursor,run_epoch,completed,total,at_ms FROM inference_job_progress_v1 WHERE job_id=?1 ORDER BY cursor LIMIT ?2").map_err(storage)?;
            let mut found = query.query(params![job_id, PROGRESS_MAX_CURSOR as i64]).map_err(storage)?;
            while let Some(row) = found.next().map_err(storage)? {
                progress.push(InferenceProgressV1 {
                    cursor: read_integer(row, 0).map_err(storage)?,
                    run_epoch: read_integer(row, 1).map_err(storage)?,
                    completed: read_integer(row, 2).map_err(storage)?,
                    total: read_integer(row, 3).map_err(storage)?,
                    at_ms: read_integer(row, 4).map_err(storage)?,
                });
            }
        }
        let next_cursor = progress.last().map_or(0, |row| row.cursor);
        let mut proposal_hash = (!proposal.is_empty()).then(|| sha256(&proposal));
        let approval = if proposal_state == InferenceProposalStateV1::Approved {
            let approval: (String, String, String, String, String, u64, String, u64, String, String, String, u64, String, String, String) = tx
                .query_row(
                    "SELECT o.mutation_id,o.command_hash,o.proposal_hash,o.phase,u.target_id,u.after_head_ordinal,u.after_head_edit_id,u.after_commit_seq,u.after_chain_sha256,u.phase,u.user_id,u.authorization_generation,u.session_id,u.space_id,u.document_id FROM inference_approval_outbox_v1 o JOIN inference_approval_undo_v1 u ON u.job_id=o.job_id WHERE o.job_id=?1",
                    [&job_id],
                    |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, read_integer(row, 5)?, row.get(6)?, read_integer(row, 7)?, row.get(8)?, row.get(9)?, row.get(10)?, read_integer(row, 11)?, row.get(12)?, row.get(13)?, row.get(14)?)),
                )
                .optional()
                .map_err(storage)?
                .ok_or(InferenceErrorV1::Storage)?;
            if approval.3 != "committed"
                || !hex(&approval.0, 32)
                || !hex(&approval.1, 64)
                || !hex(&approval.2, 64)
                || !hex(&approval.4, 32)
                || !hex(&approval.8, 64)
                || approval.10 != reader.user_id
                || approval.11 != reader.authorization_generation
                || approval.12 != reader.session_id
                || approval.13 != reader.space_id
                || approval.14 != reader.document_id
            {
                return Err(InferenceErrorV1::Storage);
            }
            proposal_hash = Some(approval.2.clone());
            let expected_current =
                directory::os_directory::EditedArtifactFrontierV1 { document_id: reader.document_id.to_owned(), head_edit_ordinal: approval.5, head_edit_id: approval.6, last_commit_seq: approval.7, chain_sha256: approval.8 };
            if !expected_current.validate() {
                return Err(InferenceErrorV1::Storage);
            }
            let (state, receipt) = match approval.9.as_str() {
                "available" => (
                    InferenceJobReconcileApprovalStateV1::Available,
                    Some(InferenceApprovalReceiptV1 {
                        schema: "semio.hub.inference-approval-receipt/v1",
                        job_id: job_id.clone(),
                        mutation_id: approval.0,
                        command_hash: approval.1,
                        proposal_hash: approval.2,
                        applied: true,
                        undo: GisMapApprovalUndoHandleV1 { target_id: approval.4, expected_current },
                    }),
                ),
                "prepared" => (InferenceJobReconcileApprovalStateV1::UndoPrepared, None),
                "committed" => (InferenceJobReconcileApprovalStateV1::Undone, None),
                _ => return Err(InferenceErrorV1::Storage),
            };
            Some(InferenceJobReconcileApprovalV1 { state, receipt })
        } else {
            None
        };
        let receipt = InferenceJobReceiptV1 { schema: "semio.hub.inference-job-receipt/v1", job_id: job_id.clone(), state, proposal_state, proposal_hash: proposal_hash.clone(), cursor: next_cursor, expires_at_ms };
        let page = InferenceJobReconcilePageV1 { job_id, state, proposal_state, cancel_requested, expired: now >= expires_at_ms, proposal_hash, events, progress, next_cursor };
        tx.commit().map_err(storage)?;
        Ok(Some(InferenceJobReconcileJobV1 { receipt, page, approval }))
    }

    /// 📃️ Returns the owner-private ordered event/progress page after an exact cursor.
    pub fn events(&self, job_id: &str, reader: &InferenceReaderV1<'_>, after: u64, now: u64) -> Result<InferenceLedgerEventPageV1, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX || after > PROGRESS_MAX_CURSOR {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, proposal_phase, expires_at) = state(&tx, job_id)?;
        if now >= expires_at {
            return Err(InferenceErrorV1::Expired);
        }
        let (_, _, cancel_requested) = run_lease(&tx, job_id)?;
        let mut events = Vec::with_capacity(EVENT_PAGE_MAX_ITEMS);
        {
            let mut query = tx.prepare("SELECT ordinal,kind,at_ms FROM inference_job_event_v1 WHERE job_id=?1 ORDER BY ordinal LIMIT ?2").map_err(storage)?;
            let mut found = query.query(params![job_id, EVENT_PAGE_MAX_ITEMS as i64]).map_err(storage)?;
            while let Some(row) = found.next().map_err(storage)? {
                events.push(InferenceEventRowV1 { ordinal: read_integer(row, 0).map_err(storage)?, kind: row.get(1).map_err(storage)?, at_ms: read_integer(row, 2).map_err(storage)? });
            }
        }
        let mut progress = Vec::with_capacity(EVENT_PAGE_MAX_ITEMS);
        {
            let mut query = tx.prepare("SELECT cursor,run_epoch,completed,total,at_ms FROM inference_job_progress_v1 WHERE job_id=?1 AND cursor>?2 ORDER BY cursor LIMIT ?3").map_err(storage)?;
            let mut found = query.query(params![job_id, sql_integer(after)?, EVENT_PAGE_MAX_ITEMS as i64]).map_err(storage)?;
            while let Some(row) = found.next().map_err(storage)? {
                progress.push(InferenceProgressRowV1 {
                    cursor: read_integer(row, 0).map_err(storage)?,
                    run_epoch: read_integer(row, 1).map_err(storage)?,
                    completed: read_integer(row, 2).map_err(storage)?,
                    total: read_integer(row, 3).map_err(storage)?,
                    at_ms: read_integer(row, 4).map_err(storage)?,
                });
            }
        }
        let proposal: Vec<u8> = tx.query_row("SELECT proposal FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| row.get(0)).map_err(storage)?;
        let page = InferenceLedgerEventPageV1 {
            state: serde_json::from_value(serde_json::Value::String(phase)).map_err(|_| InferenceErrorV1::Storage)?,
            proposal_state: serde_json::from_value(serde_json::Value::String(proposal_phase)).map_err(|_| InferenceErrorV1::Storage)?,
            cancel_requested,
            next_cursor: progress.last().map_or(after, |row| row.cursor),
            proposal_hash: (!proposal.is_empty()).then(|| sha256(&proposal)),
            events,
            progress,
        };
        tx.commit().map_err(storage)?;
        Ok(page)
    }

    pub fn cancel(&self, job_id: &str, reader: &InferenceReaderV1<'_>, now: u64) -> Result<bool, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, proposal, _) = state(&tx, job_id)?;
        if phase == "accepted" || phase == "running" {
            terminate(&tx, job_id, "cancelled", now)?;
        } else if phase == "succeeded" && proposal == "offered" {
            let pending: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM inference_approval_outbox_v1 WHERE job_id=?1 AND phase='prepared')", [job_id], |row| row.get(0)).map_err(storage)?;
            if pending {
                return Err(InferenceErrorV1::Conflict);
            }
            tx.execute("UPDATE inference_job_v1 SET proposal_state='cancelled',result=X'',proposal=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
            event(&tx, job_id, "proposal-cancelled", now)?;
        } else {
            return Ok(false);
        }
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    pub fn fail(&self, job_id: &str, reader: &InferenceReaderV1<'_>, now: u64) -> Result<bool, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, _, _) = state(&tx, job_id)?;
        if phase != "accepted" && phase != "running" {
            return Ok(false);
        }
        terminate(&tx, job_id, "failed", now)?;
        tx.commit().map_err(storage)?;
        Ok(true)
    }

    pub fn read(&self, job_id: &str, reader: &InferenceReaderV1<'_>, now: u64) -> Result<InferenceJobViewV1, InferenceErrorV1> {
        if now > SAFE_INTEGER_MAX {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut connection = self.connection.lock().map_err(|_| InferenceErrorV1::Storage)?;
        let tx = connection.transaction_with_behavior(TransactionBehavior::Immediate).map_err(storage)?;
        if !reader.matches(&identity(&tx, job_id)?) {
            return Err(InferenceErrorV1::Denied);
        }
        let (phase, proposal_phase, expires_at) = state(&tx, job_id)?;
        if now >= expires_at {
            if phase == "accepted" || phase == "running" {
                terminate(&tx, job_id, "cancelled", now)?;
            } else if phase == "succeeded" && proposal_phase == "offered" {
                let pending: bool = tx.query_row("SELECT EXISTS(SELECT 1 FROM inference_approval_outbox_v1 WHERE job_id=?1 AND phase='prepared')", [job_id], |row| row.get(0)).map_err(storage)?;
                if !pending {
                    tx.execute("UPDATE inference_job_v1 SET proposal_state='stale',result=X'',proposal=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
                    event(&tx, job_id, "proposal-stale", now)?;
                }
            } else if phase == "succeeded" {
                tx.execute("UPDATE inference_job_v1 SET result=X'',proposal=X'' WHERE job_id=?1", [job_id]).map_err(storage)?;
            }
            tx.commit().map_err(storage)?;
            return Err(InferenceErrorV1::Expired);
        }
        let (digest, result, proposal): (String, Vec<u8>, Vec<u8>) = tx.query_row("SELECT identity_digest,result,proposal FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?))).map_err(storage)?;
        let result = InferencePrivateBytesV1::new(result, RESULT_MAX_BYTES)?;
        let proposal = InferencePrivateBytesV1::new(proposal, PROPOSAL_MAX_BYTES)?;
        let view = InferenceJobViewV1 {
            receipt: InferenceLedgerJobRowV1 { job_id: job_id.to_string(), identity_digest: digest, expires_at_ms: expires_at },
            state: serde_json::from_value(serde_json::Value::String(phase)).map_err(|_| InferenceErrorV1::Storage)?,
            proposal_state: serde_json::from_value(serde_json::Value::String(proposal_phase)).map_err(|_| InferenceErrorV1::Storage)?,
            result,
            proposal,
        };
        tx.commit().map_err(storage)?;
        Ok(view)
    }
}
