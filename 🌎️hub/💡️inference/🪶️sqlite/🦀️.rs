//! 🪶️ Bounded SQLite private-job ledger with durable idempotency and first-terminal-wins.

use super::{InferenceErrorV1, InferencePrivateBytesV1, schema::*, sha256};
use rusqlite::{Connection, OptionalExtension, Transaction, TransactionBehavior, params};
use std::path::Path;
use std::sync::Mutex;

const SCHEMA: &str = "
PRAGMA foreign_keys=ON;
PRAGMA secure_delete=ON;
CREATE TABLE IF NOT EXISTS inference_job_v1 (
 job_id TEXT PRIMARY KEY CHECK(length(job_id)=32),
 request_id TEXT NOT NULL CHECK(length(request_id)=32),
 user_id TEXT NOT NULL,
 authorization_generation INTEGER NOT NULL,
 space_id TEXT NOT NULL,
 document_id TEXT NOT NULL,
 identity_digest TEXT NOT NULL CHECK(length(identity_digest)=64),
 identity_json TEXT NOT NULL CHECK(length(identity_json)<=8192),
 expires_at INTEGER NOT NULL,
 state TEXT NOT NULL CHECK(state IN ('accepted','running','succeeded','failed','cancelled')),
 proposal_state TEXT NOT NULL CHECK(proposal_state IN ('none','offered','approved','stale','cancelled')),
 run_epoch INTEGER NOT NULL DEFAULT 0,
 lease_expires_at INTEGER NOT NULL DEFAULT 0,
 cancel_requested_at INTEGER,
 progress_cursor INTEGER NOT NULL DEFAULT 0 CHECK(progress_cursor BETWEEN 0 AND 16),
 input BLOB NOT NULL CHECK(length(input)<=65536),
 result BLOB NOT NULL CHECK(length(result)<=16384),
 proposal BLOB NOT NULL CHECK(length(proposal)<=4096),
 terminal_at INTEGER,
 CHECK((state IN ('accepted','running') AND terminal_at IS NULL) OR (state IN ('succeeded','failed','cancelled') AND terminal_at IS NOT NULL)),
 CHECK(state='succeeded' OR (length(result)=0 AND length(proposal)=0)),
 CHECK(proposal_state='offered' OR length(proposal)=0),
 CHECK(state<>'running' OR run_epoch>=1),
 UNIQUE(user_id,authorization_generation,space_id,document_id,request_id)
);
CREATE TABLE IF NOT EXISTS inference_job_event_v1 (
 job_id TEXT NOT NULL REFERENCES inference_job_v1(job_id),
 ordinal INTEGER NOT NULL CHECK(ordinal BETWEEN 1 AND 6),
 kind TEXT NOT NULL CHECK(kind IN ('accepted','running','succeeded','failed','cancelled','cancel-requested','proposal-cancelled','proposal-stale','approval-prepared','approved')),
 at_ms INTEGER NOT NULL,
 PRIMARY KEY(job_id,ordinal),
 UNIQUE(job_id,kind)
);
CREATE TABLE IF NOT EXISTS inference_job_progress_v1 (
 job_id TEXT NOT NULL REFERENCES inference_job_v1(job_id),
 cursor INTEGER NOT NULL CHECK(cursor BETWEEN 1 AND 16),
 run_epoch INTEGER NOT NULL,
 completed INTEGER NOT NULL,
 total INTEGER NOT NULL,
 at_ms INTEGER NOT NULL,
 PRIMARY KEY(job_id,cursor),
 CHECK(completed<=total)
);
CREATE TRIGGER IF NOT EXISTS inference_event_no_update BEFORE UPDATE ON inference_job_event_v1 BEGIN SELECT RAISE(ABORT,'immutable inference event'); END;
CREATE TRIGGER IF NOT EXISTS inference_event_no_delete BEFORE DELETE ON inference_job_event_v1 BEGIN SELECT RAISE(ABORT,'immutable inference event'); END;
CREATE TRIGGER IF NOT EXISTS inference_progress_no_update BEFORE UPDATE ON inference_job_progress_v1 BEGIN SELECT RAISE(ABORT,'immutable inference progress'); END;
CREATE TRIGGER IF NOT EXISTS inference_progress_no_delete BEFORE DELETE ON inference_job_progress_v1 BEGIN SELECT RAISE(ABORT,'immutable inference progress'); END;
CREATE TRIGGER IF NOT EXISTS inference_identity_no_update BEFORE UPDATE OF job_id,request_id,user_id,authorization_generation,space_id,document_id,identity_digest,identity_json,expires_at ON inference_job_v1 BEGIN SELECT RAISE(ABORT,'immutable inference identity'); END;
CREATE TABLE IF NOT EXISTS inference_approval_outbox_v1 (
 job_id TEXT PRIMARY KEY REFERENCES inference_job_v1(job_id),
 mutation_id TEXT NOT NULL UNIQUE CHECK(length(mutation_id)=32),
 command_hash TEXT NOT NULL CHECK(length(command_hash)=64),
 proposal_hash TEXT NOT NULL CHECK(length(proposal_hash)=64),
 command BLOB NOT NULL CHECK(length(command)<=8192),
 prepared_at INTEGER NOT NULL,
 phase TEXT NOT NULL CHECK(phase IN ('prepared','committed','abandoned')),
 CHECK(phase='prepared' OR length(command)=0)
);
CREATE TABLE IF NOT EXISTS inference_approval_undo_v1 (
 target_id TEXT PRIMARY KEY CHECK(length(target_id)=32),
 job_id TEXT NOT NULL UNIQUE REFERENCES inference_job_v1(job_id),
 original_mutation_id TEXT NOT NULL CHECK(length(original_mutation_id)=32),
 original_command_hash TEXT NOT NULL CHECK(length(original_command_hash)=64),
 committed_witness_digest TEXT NOT NULL UNIQUE CHECK(length(committed_witness_digest)=64),
 user_id TEXT NOT NULL,
 session_id TEXT NOT NULL,
 authorization_generation INTEGER NOT NULL,
 space_id TEXT NOT NULL,
 document_id TEXT NOT NULL,
 after_head_ordinal INTEGER NOT NULL,
 after_head_edit_id TEXT NOT NULL,
 after_commit_seq INTEGER NOT NULL,
 after_chain_sha256 TEXT NOT NULL CHECK(length(after_chain_sha256)=64),
 descriptor_digest TEXT NOT NULL CHECK(length(descriptor_digest)=64),
 after_base_digest TEXT NOT NULL CHECK(length(after_base_digest)=64),
 original_command BLOB NOT NULL CHECK(length(original_command)<=8192),
 undo_idempotency_key TEXT,
 undo_job_id TEXT,
 undo_proposal_hash TEXT,
 undo_mutation_id TEXT,
 undo_command_hash TEXT,
 undo_command BLOB NOT NULL DEFAULT X'' CHECK(length(undo_command)<=8192),
 undo_frontier_head_ordinal INTEGER,
 undo_frontier_head_edit_id TEXT,
 undo_frontier_commit_seq INTEGER,
 undo_frontier_chain_sha256 TEXT,
 phase TEXT NOT NULL CHECK(phase IN ('available','prepared','committed')),
 CHECK((phase='available' AND undo_idempotency_key IS NULL) OR (phase<>'available' AND length(undo_idempotency_key)=32)),
 CHECK(phase<>'prepared' OR length(undo_command)>0),
 CHECK(phase<>'committed' OR (length(original_command)=0 AND length(undo_command)=0 AND length(undo_frontier_chain_sha256)=64))
);
";

pub struct InferenceJobLedgerV1 {
    connection: Mutex<Connection>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceJobReceiptV1 {
    pub job_id: String,
    pub identity_digest: String,
    pub expires_at_ms: u64,
}

pub struct InferenceJobViewV1 {
    pub receipt: InferenceJobReceiptV1,
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub result: InferencePrivateBytesV1,
    pub proposal: InferencePrivateBytesV1,
}

pub struct InferenceApprovalOutboxV1 {
    pub job_id: String,
    pub mutation_id: String,
    pub command_hash: String,
    pub proposal_hash: String,
    pub prepared_at_ms: u64,
    pub command: InferencePrivateBytesV1,
}

pub struct InferenceApprovalPageV1 {
    pub rows: Vec<InferenceApprovalOutboxV1>,
    pub next_cursor: Option<String>,
}

/// ↩️ Exact private inverse authority retained from the original committed approval.
pub(crate) struct GisMapApprovalUndoTargetV1 {
    pub target_id: String,
    pub original_job_id: String,
    pub original_mutation_id: String,
    pub original_command_hash: String,
    pub committed_witness_digest: String,
    pub user_id: String,
    pub session_id: String,
    pub authorization_generation: u64,
    pub scope: directory::os_directory::DocumentScope,
    pub after_frontier: directory::os_directory::CheckpointPublicationFrontierV1,
    pub descriptor_digest: String,
    pub after_base_digest: String,
    pub original_command: InferencePrivateBytesV1,
}

/// 🧾️ Approval reconciliation returns the durable undo handle from the same transaction.
pub(crate) struct InferenceApprovalReconciliationV1 {
    pub applied: bool,
    pub undo: GisMapApprovalUndoHandleV1,
}

/// 🎫️ Durable undo idempotency admission or exact terminal replay.
pub(crate) enum GisMapApprovalUndoAdmissionV1 {
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

/// 🎟️ One exclusive execution turn: only this epoch may append progress or a terminal outcome.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InferenceRunClaimV1 {
    pub run_epoch: u64,
    pub lease_expires_at_ms: u64,
}

/// 📈️ One appended owner-private progress row of the bounded monotonic cursor.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceProgressRowV1 {
    pub cursor: u64,
    pub run_epoch: u64,
    pub completed: u64,
    pub total: u64,
    pub at_ms: u64,
}

/// 🗓️ One appended lifecycle event of the ordered private job stream.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InferenceEventRowV1 {
    pub ordinal: u64,
    pub kind: String,
    pub at_ms: u64,
}

/// 📃️ Owner-private bounded page: ordered events, progress rows after a cursor, and current state.
pub struct InferenceEventPageV1 {
    pub state: InferenceJobStateV1,
    pub proposal_state: InferenceProposalStateV1,
    pub cancel_requested: bool,
    pub events: Vec<InferenceEventRowV1>,
    pub progress: Vec<InferenceProgressRowV1>,
    pub next_cursor: u64,
    pub proposal_hash: Option<String>,
}

pub struct InferenceReaderV1<'a> {
    pub user_id: &'a str,
    pub session_id: &'a str,
    pub authorization_generation: u64,
    pub space_id: &'a str,
    pub document_id: &'a str,
}

impl InferenceReaderV1<'_> {
    fn matches(&self, identity: &InferenceIdentityV1) -> bool {
        self.user_id == identity.user_id && self.session_id == identity.session_id && self.authorization_generation == identity.authorization_generation && self.space_id == identity.space_id && self.document_id == identity.document_id
    }
}

fn storage(_: rusqlite::Error) -> InferenceErrorV1 {
    InferenceErrorV1::Storage
}

fn sql_integer(value: u64) -> Result<i64, InferenceErrorV1> {
    if value > SAFE_INTEGER_MAX {
        return Err(InferenceErrorV1::Bounds);
    }
    i64::try_from(value).map_err(|_| InferenceErrorV1::Bounds)
}

fn read_integer(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<u64> {
    let value: i64 = row.get(index)?;
    u64::try_from(value).ok().filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(rusqlite::Error::IntegralValueOutOfRange(index, value))
}

fn event(tx: &Transaction<'_>, job_id: &str, kind: &str, now: u64) -> Result<(), InferenceErrorV1> {
    tx.execute("INSERT INTO inference_job_event_v1(job_id,ordinal,kind,at_ms) SELECT ?1,COALESCE(MAX(ordinal),0)+1,?2,?3 FROM inference_job_event_v1 WHERE job_id=?1", params![job_id, kind, sql_integer(now)?]).map_err(storage)?;
    Ok(())
}

fn identity(tx: &Transaction<'_>, job_id: &str) -> Result<InferenceIdentityV1, InferenceErrorV1> {
    let json: String = tx.query_row("SELECT identity_json FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| row.get(0)).optional().map_err(storage)?.ok_or(InferenceErrorV1::Denied)?;
    serde_json::from_str(&json).map_err(|_| InferenceErrorV1::Storage)
}

fn state(tx: &Transaction<'_>, job_id: &str) -> Result<(String, String, u64), InferenceErrorV1> {
    tx.query_row("SELECT state,proposal_state,expires_at FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, read_integer(row, 2)?))).map_err(storage)
}

fn run_lease(tx: &Transaction<'_>, job_id: &str) -> Result<(u64, u64, bool), InferenceErrorV1> {
    tx.query_row("SELECT run_epoch,lease_expires_at,cancel_requested_at IS NOT NULL FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| Ok((read_integer(row, 0)?, read_integer(row, 1)?, row.get(2)?))).map_err(storage)
}

fn terminate(tx: &Transaction<'_>, job_id: &str, kind: &str, now: u64) -> Result<(), InferenceErrorV1> {
    tx.execute("UPDATE inference_job_v1 SET state=?2,proposal_state='none',input=X'',result=X'',proposal=X'',terminal_at=?3 WHERE job_id=?1", params![job_id, kind, sql_integer(now)?]).map_err(storage)?;
    event(tx, job_id, kind, now)
}

impl InferenceJobLedgerV1 {
    pub fn open(path: &Path) -> Result<Self, InferenceErrorV1> {
        let connection = Connection::open(path).map_err(storage)?;
        connection.busy_timeout(std::time::Duration::from_secs(2)).map_err(storage)?;
        connection.execute_batch(SCHEMA).map_err(storage)?;
        Ok(Self { connection: Mutex::new(connection) })
    }

    pub fn accept(&self, selected: &InferenceIdentityV1, input: &InferencePrivateBytesV1, now: u64) -> Result<InferenceJobReceiptV1, InferenceErrorV1> {
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
                |row| Ok(InferenceJobReceiptV1 { job_id: row.get(0)?, identity_digest: row.get(1)?, expires_at_ms: read_integer(row, 2)? }),
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
        Ok(InferenceJobReceiptV1 { job_id, identity_digest: digest, expires_at_ms: expires_at })
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

    /// 📃️ Returns the owner-private ordered event/progress page after an exact cursor.
    pub fn events(&self, job_id: &str, reader: &InferenceReaderV1<'_>, after: u64, now: u64) -> Result<InferenceEventPageV1, InferenceErrorV1> {
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
        let page = InferenceEventPageV1 {
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

    pub fn prepare_approval(&self, job_id: &str, current: &InferenceIdentityV1, proposal_hash: &str, command: &InferencePrivateBytesV1, now: u64) -> Result<InferenceApprovalOutboxV1, InferenceErrorV1> {
        current.validate()?;
        if command.as_slice().is_empty() || command.as_slice().len() > 8192 || now > SAFE_INTEGER_MAX || !hex(proposal_hash, 64) {
            return Err(InferenceErrorV1::Bounds);
        }
        let mutation_id = sha256(format!("semio.hub.inference-approval-mutation/v1\0{job_id}\0{proposal_hash}").as_bytes())[..32].to_string();
        let document_key = format!("v1:{}:{}:{}{}", current.space_id.len(), current.document_id.len(), current.space_id, current.document_id);
        let actor = format!("user:{}#session:{}", current.user_id, current.session_id);
        let decoded = super::command::CanonicalInferenceCommandV1::decode(command.as_slice())?;
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
                return Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), 8192)? });
            }
            if outbox_phase != "abandoned" {
                return Err(InferenceErrorV1::Conflict);
            }
            tx.execute("UPDATE inference_approval_outbox_v1 SET command=?2,prepared_at=?3,phase='prepared' WHERE job_id=?1 AND phase='abandoned'", params![job_id, command.as_slice(), sql_integer(now)?]).map_err(storage)?;
            tx.commit().map_err(storage)?;
            return Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms: now, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), 8192)? });
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
        Ok(InferenceApprovalOutboxV1 { job_id: job_id.to_string(), mutation_id, command_hash, proposal_hash: proposal_hash.to_string(), prepared_at_ms: now, command: InferencePrivateBytesV1::new(command.as_slice().to_vec(), 8192)? })
    }

    pub(crate) fn abandon_prepared_approval(&self, reader: &InferenceReaderV1<'_>, job_id: &str, mutation_id: &str, command_hash: &str, proposal_hash: &str) -> Result<bool, InferenceErrorV1> {
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

    pub fn pending_approvals(&self, after: Option<&str>, control: &super::InferenceOperationControlV1) -> Result<InferenceApprovalPageV1, InferenceErrorV1> {
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
                command: InferencePrivateBytesV1::new(row.get(5).map_err(storage)?, 8192)?,
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
        row.map(|(job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command)| Ok(InferenceApprovalOutboxV1 { job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command: InferencePrivateBytesV1::new(command, 8192)? }))
            .transpose()
    }

    pub(crate) fn approval_recovery_by_mutation(&self, mutation_id: &str) -> Result<Option<(InferenceApprovalOutboxV1, InferenceIdentityV1, bool)>, InferenceErrorV1> {
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
        let outbox = InferenceApprovalOutboxV1 { job_id, mutation_id, command_hash, proposal_hash, prepared_at_ms, command: InferencePrivateBytesV1::new(command, 8192)? };
        Ok(Some((outbox, accepted, phase == "committed")))
    }

    pub(crate) fn reconcile_committed_approval(
        &self,
        job_id: &str,
        witness: &super::wal::CommittedInferenceWalWitnessV1,
        document_generation: u64,
        after_frontier: &directory::os_directory::CheckpointPublicationFrontierV1,
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
        if !witness.matches(&scope, document_generation, job_id, &accepted_proposal, &accepted_mutation, &accepted_command)
            || after_frontier.document_id != scope.document_id
            || after_frontier.head_edit_id != accepted_mutation
            || after_frontier.head_edit_ordinal != accepted.head_ordinal.checked_add(1).ok_or(InferenceErrorV1::Bounds)?
            || after_frontier.last_commit_seq != accepted.last_commit_seq.checked_add(1).ok_or(InferenceErrorV1::Bounds)?
        {
            return Err(InferenceErrorV1::Conflict);
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
                return Err(InferenceErrorV1::Conflict);
            }
            tx.commit().map_err(storage)?;
            return Ok(InferenceApprovalReconciliationV1 { applied: false, undo });
        }
        if phase != "prepared" {
            return Err(InferenceErrorV1::Conflict);
        }
        let (job_phase, proposal_phase, _) = state(&tx, job_id)?;
        if job_phase != "succeeded" || proposal_phase != "offered" {
            return Err(InferenceErrorV1::Conflict);
        }
        if command.is_empty() || command.len() > 8192 || sha256(&command) != accepted_command {
            return Err(InferenceErrorV1::Conflict);
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

    /// 🔎️ Reads the exact private undo authority only for its original authenticated owner.
    pub(crate) fn gis_map_approval_undo_target(&self, target_id: &str, reader: &InferenceReaderV1<'_>) -> Result<GisMapApprovalUndoTargetV1, InferenceErrorV1> {
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
        let after_frontier = directory::os_directory::CheckpointPublicationFrontierV1 { document_id: row.8.clone(), head_edit_ordinal: row.9, head_edit_id: row.10, last_commit_seq: row.11, chain_sha256: row.12 };
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
            original_command: InferencePrivateBytesV1::new(row.15, 8192)?,
        })
    }

    /// 🪪️ Resolves only the original owner's private job identity for live authorization recheck.
    pub(crate) fn gis_map_approval_undo_job_id(&self, target_id: &str, reader: &InferenceReaderV1<'_>) -> Result<String, InferenceErrorV1> {
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
    pub(crate) fn replayed_gis_map_approval_undo(&self, target_id: &str, idempotency_key: &str, reader: &InferenceReaderV1<'_>) -> Result<Option<GisMapApprovalUndoReceiptV1>, InferenceErrorV1> {
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
                let frontier = directory::os_directory::CheckpointPublicationFrontierV1 {
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
    pub(crate) fn prepare_gis_map_approval_undo(
        &self,
        target: &GisMapApprovalUndoTargetV1,
        idempotency_key: &str,
        operation_id: &str,
        proposal_hash: &str,
        mutation_id: &str,
        command_hash: &str,
        command: &InferencePrivateBytesV1,
    ) -> Result<GisMapApprovalUndoAdmissionV1, InferenceErrorV1> {
        if !hex(idempotency_key, 32)
            || !hex(operation_id, 32)
            || !hex(proposal_hash, 64)
            || !hex(mutation_id, 32)
            || !hex(command_hash, 64)
            || command.as_slice().is_empty()
            || sha256(command.as_slice()) != command_hash
        {
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
                let updated = tx.execute(
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
                let frontier = directory::os_directory::CheckpointPublicationFrontierV1 {
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
    pub(crate) fn reconcile_committed_gis_map_approval_undo(
        &self,
        target_id: &str,
        witness: &super::wal::CommittedInferenceWalWitnessV1,
        document_generation: u64,
        frontier: &directory::os_directory::CheckpointPublicationFrontierV1,
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
            let exact_terminal = row.11 == Some(frontier.head_edit_ordinal)
                && row.12.as_deref() == Some(frontier.head_edit_id.as_str())
                && row.13 == Some(frontier.last_commit_seq)
                && row.14.as_deref() == Some(frontier.chain_sha256.as_str());
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
    pub(crate) fn gis_map_approval_undo_recovery_by_mutation(&self, mutation_id: &str) -> Result<Option<GisMapApprovalUndoRecoveryV1>, InferenceErrorV1> {
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
        let after_frontier = directory::os_directory::CheckpointPublicationFrontierV1 {
            document_id: row.9.clone(),
            head_edit_ordinal: row.10,
            head_edit_id: row.11.clone(),
            last_commit_seq: row.12,
            chain_sha256: row.13.clone(),
        };
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
                original_command: InferencePrivateBytesV1::new(row.16, 8192)?,
            },
            idempotency_key: row.17,
            operation_id: row.18,
            proposal_hash: row.19,
            mutation_id: row.20,
            command_hash: row.21,
            command: InferencePrivateBytesV1::new(row.22, 8192)?,
            ledger_applied: !prepared,
        }))
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
            receipt: InferenceJobReceiptV1 { job_id: job_id.to_string(), identity_digest: digest, expires_at_ms: expires_at },
            state: serde_json::from_value(serde_json::Value::String(phase)).map_err(|_| InferenceErrorV1::Storage)?,
            proposal_state: serde_json::from_value(serde_json::Value::String(proposal_phase)).map_err(|_| InferenceErrorV1::Storage)?,
            result,
            proposal,
        };
        tx.commit().map_err(storage)?;
        Ok(view)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Barrier};

    fn fixture() -> serde_json::Value {
        serde_json::from_str(include_str!("../../🧪️fixtures/🗺️gis-inference-job-v1/🔣️.json")).unwrap()
    }

    fn selected(fixture: &serde_json::Value) -> InferenceIdentityV1 {
        serde_json::from_value(fixture["identity"].clone()).unwrap()
    }

    fn memory() -> InferenceJobLedgerV1 {
        let connection = Connection::open_in_memory().unwrap();
        connection.execute_batch(SCHEMA).unwrap();
        InferenceJobLedgerV1 { connection: Mutex::new(connection) }
    }

    fn reader(identity: &InferenceIdentityV1) -> InferenceReaderV1<'_> {
        InferenceReaderV1 { user_id: &identity.user_id, session_id: &identity.session_id, authorization_generation: identity.authorization_generation, space_id: &identity.space_id, document_id: &identity.document_id }
    }

    #[test]
    fn gis_inference_sqlite_ledger_executes_neutral_traces_with_private_first_terminal_wins() {
        let fixture = fixture();
        let selected = selected(&fixture);
        assert_eq!(selected.digest().unwrap(), fixture["identityDigest"].as_str().unwrap());
        let identifiers: serde_json::Value = serde_json::from_str(include_str!("../../🧪️fixtures/🖥️inference-server-identity-v1/🔣️.json")).unwrap();
        for row in identifiers["cases"].as_array().unwrap() {
            for field in identifiers["fields"].as_array().unwrap() {
                let mut candidate = fixture["identity"].clone();
                candidate["headOrdinal"] = 1.into();
                candidate["headEditId"] = "0".repeat(32).into();
                candidate[field.as_str().unwrap()] = row["value"].clone();
                let value: InferenceIdentityV1 = serde_json::from_value(candidate).unwrap();
                assert_eq!(value.validate().is_ok(), row["accepted"].as_bool().unwrap(), "{}/{}", row["name"], field);
            }
        }
        let maximum_id = "a".repeat(identifiers["maximumBytes"].as_u64().unwrap() as usize);
        assert!(format!("v1:{}:{}:{}{}", maximum_id.len(), maximum_id.len(), maximum_id, maximum_id).len() <= 256);
        assert!(format!("user:{maximum_id}#session:{maximum_id}").len() <= 256);
        for hostile in fixture["hostileIdentities"].as_array().unwrap() {
            let mut candidate = fixture["identity"].clone();
            let path = hostile["path"].as_array().unwrap();
            let mut at = &mut candidate;
            for segment in &path[..path.len() - 1] {
                at = &mut at[segment.as_str().unwrap()];
            }
            at[path.last().unwrap().as_str().unwrap()] = hostile["value"].clone();
            if let Ok(identity) = serde_json::from_value::<InferenceIdentityV1>(candidate) {
                assert!(identity.validate().is_err(), "{}", hostile["name"]);
            }
        }
        let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
        let result = InferencePrivateBytesV1::new(serde_json::to_vec(&fixture["expectedInference"]).unwrap(), RESULT_MAX_BYTES).unwrap();
        let proposal = InferencePrivateBytesV1::new(b"bounded-ledger-payload-not-an-executable-proposal".to_vec(), PROPOSAL_MAX_BYTES).unwrap();
        for trace in fixture["traces"].as_array().unwrap() {
            let ledger = memory();
            let receipt = ledger.accept(&selected, &input, 1000).unwrap();
            let mut epoch = 0;
            for (step, operation) in trace["operations"].as_array().unwrap().iter().enumerate() {
                let at = 1001 + step as u64;
                match operation.as_str().unwrap() {
                    "start" => {
                        if let Some(claim) = ledger.start(&receipt.job_id, &selected, at).unwrap() {
                            epoch = claim.run_epoch;
                        }
                    }
                    "succeed" => {
                        ledger.succeed(&receipt.job_id, &selected, epoch, &result, &proposal, at).unwrap();
                    }
                    "cancel" => {
                        ledger.cancel(&receipt.job_id, &reader(&selected), at).unwrap();
                    }
                    _ => panic!("unknown literal trace operation"),
                }
            }
            let view = ledger.read(&receipt.job_id, &reader(&selected), 1010).unwrap();
            assert_eq!(serde_json::to_value(view.state).unwrap(), trace["state"], "{}", trace["name"]);
            assert_eq!(serde_json::to_value(view.proposal_state).unwrap(), trace["proposalState"], "{}", trace["name"]);
            assert_eq!(!view.result.as_slice().is_empty(), trace["hasResult"].as_bool().unwrap());
            let count = ledger.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1", [], |row| read_integer(row, 0)).unwrap();
            assert_eq!(count, trace["eventCount"].as_u64().unwrap());
            let mut foreign = reader(&selected);
            foreign.user_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
            assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1004), Err(InferenceErrorV1::Denied)));
            assert_eq!(ledger.cancel(&receipt.job_id, &foreign, 1004), Err(InferenceErrorV1::Denied));
            foreign = reader(&selected);
            foreign.space_id = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
            assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1010), Err(InferenceErrorV1::Denied)));
            foreign = reader(&selected);
            foreign.authorization_generation += 1;
            assert!(matches!(ledger.read(&receipt.job_id, &foreign, 1010), Err(InferenceErrorV1::Denied)));
            assert!(ledger.connection.lock().unwrap().execute("UPDATE inference_job_event_v1 SET kind='failed'", []).is_err());
        }
    }

    #[test]
    fn gis_inference_sqlite_request_identity_capacity_expiry_and_progress_are_bounded() {
        let fixture = fixture();
        let selected = selected(&fixture);
        let integers = Connection::open_in_memory().unwrap();
        for case in fixture["sqliteIntegers"].as_array().unwrap() {
            let decimal = case["decimal"].as_str().unwrap();
            let expected = case["accepted"].as_bool().unwrap();
            let parsed = decimal.parse::<u64>();
            assert_eq!(parsed.ok().and_then(|value| sql_integer(value).ok()).is_some(), expected, "write {decimal}");
            if let Ok(signed) = decimal.parse::<i64>() {
                let read = integers.query_row("SELECT ?1", [signed], |row| read_integer(row, 0));
                assert_eq!(read.is_ok(), expected, "read {decimal}");
                if expected {
                    assert_eq!(read.unwrap().to_string(), decimal);
                }
            }
        }
        assert!(InferenceRequestV1::decode(&serde_json::to_vec(&selected.request).unwrap()).is_ok());
        for hostile in fixture["hostileRequests"].as_array().unwrap() {
            let mut candidate = fixture["identity"]["request"].clone();
            candidate[hostile["field"].as_str().unwrap()] = hostile["value"].clone();
            assert!(InferenceRequestV1::decode(&serde_json::to_vec(&candidate).unwrap()).is_err(), "{}", hostile["name"]);
        }
        let duplicate = serde_json::to_string(&selected.request).unwrap().replacen("{", "{\"version\":1,", 1);
        assert_eq!(InferenceRequestV1::decode(duplicate.as_bytes()), Err(InferenceErrorV1::Invalid));
        assert_eq!(InferenceRequestV1::decode(&vec![b' '; REQUEST_MAX_BYTES + 1]), Err(InferenceErrorV1::Bounds));
        let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
        let ledger = memory();
        assert_eq!(ledger.accept(&selected, &input, SAFE_INTEGER_MAX), Err(InferenceErrorV1::Bounds));
        assert_eq!(ledger.accept(&selected, &input, u64::MAX), Err(InferenceErrorV1::Bounds));
        let receipt = ledger.accept(&selected, &input, 1000).unwrap();
        assert_eq!(ledger.accept(&selected, &input, 2000).unwrap(), receipt);
        let mut changed = selected.clone();
        changed.binding.catalog_generation_id = "7".repeat(64);
        assert_eq!(ledger.accept(&changed, &input, 1001), Err(InferenceErrorV1::Conflict));
        assert_eq!(ledger.start(&receipt.job_id, &changed, 1001), Err(InferenceErrorV1::Conflict));
        assert_eq!(ledger.read(&receipt.job_id, &reader(&selected), 1002).unwrap().state, InferenceJobStateV1::Cancelled);
        for index in 1..JOB_CAPACITY {
            let mut next = selected.clone();
            next.request.request_id = format!("{index:032x}");
            ledger.accept(&next, &input, 1000).unwrap();
        }
        let mut extra = selected.clone();
        extra.request.request_id = "ffffffffffffffffffffffffffffffff".into();
        assert_eq!(ledger.accept(&extra, &input, 1000), Err(InferenceErrorV1::Capacity));
        let short = memory();
        let receipt = short.accept(&selected, &input, 1000).unwrap();
        assert_eq!(short.start(&receipt.job_id, &selected, receipt.expires_at_ms), Err(InferenceErrorV1::Expired));
        assert_eq!(short.start(&receipt.job_id, &selected, receipt.expires_at_ms), Ok(None));
        let failed = memory();
        let receipt = failed.accept(&selected, &input, 1000).unwrap();
        assert!(failed.fail(&receipt.job_id, &reader(&selected), 1001).unwrap());
        assert!(!failed.cancel(&receipt.job_id, &reader(&selected), 1002).unwrap());
        assert_eq!(failed.read(&receipt.job_id, &reader(&selected), 1003).unwrap().state, InferenceJobStateV1::Failed);
        assert!(matches!(InferencePrivateBytesV1::new(vec![0; INPUT_MAX_BYTES + 1], INPUT_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
        assert!(matches!(InferencePrivateBytesV1::new(vec![0; RESULT_MAX_BYTES + 1], RESULT_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
        assert!(matches!(InferencePrivateBytesV1::new(vec![0; PROPOSAL_MAX_BYTES + 1], PROPOSAL_MAX_BYTES), Err(InferenceErrorV1::Bounds)));
        let control = crate::inference::InferenceOperationControlV1::new(1000, 2).unwrap();
        control.checkpoint(2).unwrap();
        control.checkpoint(1).unwrap();
        assert_eq!(control.progress(), (2, 2));
        assert_eq!(control.checkpoint(3), Err(InferenceErrorV1::Bounds));
        control.cancel();
        assert_eq!(control.checkpoint(2), Err(InferenceErrorV1::Cancelled));
    }

    #[test]
    fn gis_inference_sqlite_concurrent_connections_have_one_durable_request_winner() {
        let fixture = fixture();
        let selected = selected(&fixture);
        let path = std::env::temp_dir().join(format!("semio-gis-ledger-{}.sqlite", directory::os_identity::time_ordered_id()));
        let first = InferenceJobLedgerV1::open(&path).unwrap();
        let second = InferenceJobLedgerV1::open(&path).unwrap();
        let barrier = Arc::new(Barrier::new(2));
        let input = fixture["input"].as_str().unwrap().as_bytes().to_vec();
        let receipts = std::thread::scope(|scope| {
            let submit = |ledger: InferenceJobLedgerV1, barrier: Arc<Barrier>| {
                let selected = &selected;
                let input = &input;
                scope.spawn(move || {
                    barrier.wait();
                    ledger.accept(selected, &InferencePrivateBytesV1::new(input.clone(), INPUT_MAX_BYTES).unwrap(), 1000).unwrap()
                })
            };
            let a = submit(first, barrier.clone());
            let b = submit(second, barrier);
            (a.join().unwrap(), b.join().unwrap())
        });
        assert_eq!(receipts.0, receipts.1);
        let reopened = InferenceJobLedgerV1::open(&path).unwrap();
        let count = reopened.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1 WHERE kind='accepted'", [], |row| read_integer(row, 0)).unwrap();
        assert_eq!(count, 1);
        assert_eq!(reopened.read(&receipts.0.job_id, &reader(&selected), 1001).unwrap().state, InferenceJobStateV1::Accepted);
        drop(reopened);
        std::fs::remove_file(&path).unwrap();
    }

    #[tokio::test]
    async fn gis_inference_sqlite_prepared_approval_survives_restart_and_reconciles_exactly_once() {
        let fixture = fixture();
        let selected = selected(&fixture);
        let input = InferencePrivateBytesV1::new(fixture["input"].as_str().unwrap().as_bytes().to_vec(), INPUT_MAX_BYTES).unwrap();
        let result = InferencePrivateBytesV1::new(serde_json::to_vec(&fixture["expectedInference"]).unwrap(), RESULT_MAX_BYTES).unwrap();
        let outbox = &fixture["outbox"];
        let proposal = InferencePrivateBytesV1::new(outbox["proposal"].as_str().unwrap().as_bytes().to_vec(), PROPOSAL_MAX_BYTES).unwrap();
        let command_hex = outbox["commandHex"].as_str().unwrap();
        let command = InferencePrivateBytesV1::new((0..command_hex.len()).step_by(2).map(|index| u8::from_str_radix(&command_hex[index..index + 2], 16).unwrap()).collect(), 8192).unwrap();
        let path = std::env::temp_dir().join(format!("semio-gis-outbox-{}.sqlite", directory::os_identity::time_ordered_id()));
        let ledger = InferenceJobLedgerV1::open(&path).unwrap();
        let receipt = ledger.accept(&selected, &input, 1000).unwrap();
        let claim = ledger.start(&receipt.job_id, &selected, 1001).unwrap().expect("owned run claim");
        ledger.succeed(&receipt.job_id, &selected, claim.run_epoch, &result, &proposal, 1002).unwrap();
        let hash = sha256(proposal.as_slice());
        let mut trailing = command.as_slice().to_vec();
        trailing.push(0);
        let trailing = InferencePrivateBytesV1::new(trailing, 8192).unwrap();
        assert!(matches!(ledger.prepare_approval(&receipt.job_id, &selected, &hash, &trailing, 1003), Err(InferenceErrorV1::Invalid)));
        let empty = ledger.pending_approvals(None, &super::super::InferenceOperationControlV1::new(1000, 5).unwrap()).unwrap();
        assert!(empty.rows.is_empty(), "prefix-valid trailing bytes never create a prepared outbox row");
        let prepared = ledger.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1003).unwrap();
        assert_eq!(prepared.job_id, outbox["jobId"].as_str().unwrap());
        assert_eq!(prepared.mutation_id, outbox["mutationId"].as_str().unwrap());
        assert_eq!(prepared.command_hash, outbox["commandHash"].as_str().unwrap());
        assert_eq!(prepared.proposal_hash, outbox["proposalHash"].as_str().unwrap());
        let repeated = ledger.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1004).unwrap();
        assert_eq!(prepared.mutation_id, repeated.mutation_id);
        assert_eq!(prepared.prepared_at_ms, repeated.prepared_at_ms);
        assert_eq!(ledger.cancel(&receipt.job_id, &reader(&selected), 1005), Err(InferenceErrorV1::Conflict));
        let foreign = InferenceReaderV1 { authorization_generation: selected.authorization_generation + 1, ..reader(&selected) };
        assert_eq!(ledger.abandon_prepared_approval(&foreign, &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash), Err(InferenceErrorV1::Denied),);
        assert_eq!(ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &"0".repeat(64), &prepared.proposal_hash), Err(InferenceErrorV1::Conflict),);
        assert!(ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).unwrap());
        assert!(!ledger.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash).unwrap());
        assert!(ledger.pending_approvals(None, &super::super::InferenceOperationControlV1::new(1000, 5).unwrap()).unwrap().rows.is_empty());
        drop(ledger);
        let reopened = InferenceJobLedgerV1::open(&path).unwrap();
        let control = super::super::InferenceOperationControlV1::new(1000, 5).unwrap();
        assert!(reopened.pending_approvals(None, &control).unwrap().rows.is_empty(), "an abandoned request survives restart without blocking the document");
        let revived = reopened.prepare_approval(&receipt.job_id, &selected, &hash, &command, 1006).unwrap();
        assert_eq!((revived.mutation_id.as_str(), revived.command_hash.as_str(), revived.proposal_hash.as_str()), (prepared.mutation_id.as_str(), prepared.command_hash.as_str(), prepared.proposal_hash.as_str()));
        assert_eq!(revived.prepared_at_ms, 1006);
        let pending = reopened.pending_approvals(None, &control).unwrap();
        assert_eq!(pending.rows.len() as u64, outbox["preparedCount"].as_u64().unwrap());
        assert_eq!(pending.rows[0].mutation_id, prepared.mutation_id);
        assert_eq!(pending.rows[0].command.as_slice(), command.as_slice());
        assert!(pending.next_cursor.is_none());
        let prepared_events = reopened.events(&receipt.job_id, &reader(&selected), 0, 1007).unwrap().events.into_iter().filter(|event| event.kind == "approval-prepared").count();
        assert_eq!(prepared_events, 1, "reviving the exact abandoned row never fabricates a second prepared event");
        assert!(matches!(reopened.read(&receipt.job_id, &reader(&selected), receipt.expires_at_ms), Err(InferenceErrorV1::Expired)));
        #[cfg(feature = "native-artifact-execution")]
        {
            let (witness, fence) = super::super::wal::tests::committed_fixture_witness().await;
            let frontier = directory::os_directory::CheckpointPublicationFrontierV1 {
                document_id: selected.document_id.clone(),
                head_edit_ordinal: selected.head_ordinal + 1,
                head_edit_id: prepared.mutation_id.clone(),
                last_commit_seq: selected.last_commit_seq + 1,
                chain_sha256: "44".repeat(32),
            };
            let descriptor_digest = selected.descriptor_digest.clone();
            let after_base_digest = "11".repeat(32);
            assert_eq!(reopened.reconcile_committed_approval(&receipt.job_id, &witness, 18, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).map(|value| value.applied), Err(InferenceErrorV1::Conflict));
            let reconciled = reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).unwrap();
            assert!(reconciled.applied);
            let undo_target = reopened.gis_map_approval_undo_target(&reconciled.undo.target_id, &reader(&selected)).expect("original owner reads the retained command");
            assert_eq!(undo_target.original_command.as_slice(), command.as_slice());
            assert_eq!(undo_target.after_frontier, frontier);
            let foreign_target = InferenceReaderV1 { session_id: "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee", ..reader(&selected) };
            assert!(matches!(reopened.gis_map_approval_undo_target(&reconciled.undo.target_id, &foreign_target), Err(InferenceErrorV1::Denied)));
            assert!(!reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).unwrap().applied);
            let undo_command = InferencePrivateBytesV1::new(vec![9], 8192).unwrap();
            let undo_command_hash = sha256(undo_command.as_slice());
            assert!(matches!(
                reopened.prepare_gis_map_approval_undo(&undo_target, &"55".repeat(16), &"66".repeat(16), &"77".repeat(32), &"88".repeat(16), &undo_command_hash, &undo_command),
                Ok(GisMapApprovalUndoAdmissionV1::Prepared)
            ));
            assert!(
                matches!(reopened.prepare_gis_map_approval_undo(&undo_target, &"aa".repeat(16), &"66".repeat(16), &"77".repeat(32), &"88".repeat(16), &undo_command_hash, &undo_command), Err(InferenceErrorV1::Conflict)),
                "a distinct retry identity cannot take over the durable target",
            );
            fence.invalidate();
            assert_eq!(reopened.reconcile_committed_approval(&receipt.job_id, &witness, 17, &frontier, &descriptor_digest, &after_base_digest, receipt.expires_at_ms).map(|value| value.applied), Err(InferenceErrorV1::Conflict));
            assert!(reopened.pending_approvals(None, &control).unwrap().rows.is_empty());
            assert_eq!(
                reopened.abandon_prepared_approval(&reader(&selected), &receipt.job_id, &prepared.mutation_id, &prepared.command_hash, &prepared.proposal_hash),
                Err(InferenceErrorV1::Conflict),
                "a committed decision cannot be demoted back to abandoned",
            );
            let count = reopened.connection.lock().unwrap().query_row("SELECT COUNT(*) FROM inference_job_event_v1 WHERE kind='approved'", [], |row| read_integer(row, 0)).unwrap();
            assert_eq!(count, outbox["reconciledCount"].as_u64().unwrap());
        }
        drop(reopened);
        std::fs::remove_file(path).unwrap();
    }
}
