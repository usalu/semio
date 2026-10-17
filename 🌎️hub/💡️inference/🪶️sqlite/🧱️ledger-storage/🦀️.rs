//! 🧱️ Ledger storage: the bounded SQL schema and the small row and error helpers.

use super::*;

/// 🧱️ The ledger's SQL, with every size and cursor CHECK taken from the declared bounds, so the store
/// admits exactly what the service admits and a bound changes in one place.
pub(super) fn ledger_schema() -> String {
    format!(
        "
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
 identity_json TEXT NOT NULL CHECK(length(identity_json)<={IDENTITY_JSON_MAX_BYTES}),
 expires_at INTEGER NOT NULL,
 state TEXT NOT NULL CHECK(state IN ('accepted','running','succeeded','failed','cancelled')),
 proposal_state TEXT NOT NULL CHECK(proposal_state IN ('none','offered','approved','stale','cancelled')),
 run_epoch INTEGER NOT NULL DEFAULT 0,
 lease_expires_at INTEGER NOT NULL DEFAULT 0,
 cancel_requested_at INTEGER,
 progress_cursor INTEGER NOT NULL DEFAULT 0 CHECK(progress_cursor BETWEEN 0 AND {PROGRESS_MAX_CURSOR}),
 input BLOB NOT NULL CHECK(length(input)<={INPUT_MAX_BYTES}),
 result BLOB NOT NULL CHECK(length(result)<={RESULT_MAX_BYTES}),
 proposal BLOB NOT NULL CHECK(length(proposal)<={PROPOSAL_MAX_BYTES}),
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
 cursor INTEGER NOT NULL CHECK(cursor BETWEEN 1 AND {PROGRESS_MAX_CURSOR}),
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
 command BLOB NOT NULL CHECK(length(command)<={COMMAND_MAX_BYTES}),
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
 original_command BLOB NOT NULL CHECK(length(original_command)<={COMMAND_MAX_BYTES}),
 undo_idempotency_key TEXT,
 undo_job_id TEXT,
 undo_proposal_hash TEXT,
 undo_mutation_id TEXT,
 undo_command_hash TEXT,
 undo_command BLOB NOT NULL DEFAULT X'' CHECK(length(undo_command)<={COMMAND_MAX_BYTES}),
 undo_frontier_head_ordinal INTEGER,
 undo_frontier_head_edit_id TEXT,
 undo_frontier_commit_seq INTEGER,
 undo_frontier_chain_sha256 TEXT,
 phase TEXT NOT NULL CHECK(phase IN ('available','prepared','committed')),
 CHECK((phase='available' AND undo_idempotency_key IS NULL) OR (phase<>'available' AND length(undo_idempotency_key)=32)),
 CHECK(phase<>'prepared' OR length(undo_command)>0),
 CHECK(phase<>'committed' OR (length(original_command)=0 AND length(undo_command)=0 AND length(undo_frontier_chain_sha256)=64))
);
"
    )
}

pub(super) fn storage(_: rusqlite::Error) -> InferenceErrorV1 {
    InferenceErrorV1::Storage
}

pub(super) fn sql_integer(value: u64) -> Result<i64, InferenceErrorV1> {
    if value > SAFE_INTEGER_MAX {
        return Err(InferenceErrorV1::Bounds);
    }
    i64::try_from(value).map_err(|_| InferenceErrorV1::Bounds)
}

pub(super) fn read_integer(row: &rusqlite::Row<'_>, index: usize) -> rusqlite::Result<u64> {
    let value: i64 = row.get(index)?;
    u64::try_from(value).ok().filter(|value| *value <= SAFE_INTEGER_MAX).ok_or(rusqlite::Error::IntegralValueOutOfRange(index, value))
}

pub(super) fn event(tx: &Transaction<'_>, job_id: &str, kind: &str, now: u64) -> Result<(), InferenceErrorV1> {
    tx.execute("INSERT INTO inference_job_event_v1(job_id,ordinal,kind,at_ms) SELECT ?1,COALESCE(MAX(ordinal),0)+1,?2,?3 FROM inference_job_event_v1 WHERE job_id=?1", params![job_id, kind, sql_integer(now)?]).map_err(storage)?;
    Ok(())
}

pub(super) fn identity(tx: &Transaction<'_>, job_id: &str) -> Result<InferenceIdentityV1, InferenceErrorV1> {
    let json: String = tx.query_row("SELECT identity_json FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| row.get(0)).optional().map_err(storage)?.ok_or(InferenceErrorV1::Denied)?;
    serde_json::from_str(&json).map_err(|_| InferenceErrorV1::Storage)
}

pub(super) fn state(tx: &Transaction<'_>, job_id: &str) -> Result<(String, String, u64), InferenceErrorV1> {
    tx.query_row("SELECT state,proposal_state,expires_at FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| Ok((row.get(0)?, row.get(1)?, read_integer(row, 2)?))).map_err(storage)
}

pub(super) fn run_lease(tx: &Transaction<'_>, job_id: &str) -> Result<(u64, u64, bool), InferenceErrorV1> {
    tx.query_row("SELECT run_epoch,lease_expires_at,cancel_requested_at IS NOT NULL FROM inference_job_v1 WHERE job_id=?1", [job_id], |row| Ok((read_integer(row, 0)?, read_integer(row, 1)?, row.get(2)?))).map_err(storage)
}

pub(super) fn terminate(tx: &Transaction<'_>, job_id: &str, kind: &str, now: u64) -> Result<(), InferenceErrorV1> {
    tx.execute("UPDATE inference_job_v1 SET state=?2,proposal_state='none',input=X'',result=X'',proposal=X'',terminal_at=?3 WHERE job_id=?1", params![job_id, kind, sql_integer(now)?]).map_err(storage)?;
    event(tx, job_id, kind, now)
}
