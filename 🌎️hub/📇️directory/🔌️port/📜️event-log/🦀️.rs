//! 📜️ The capability trait of the directory event log.

use super::*;

//#region EventLog
/// 📜️ The append-only directory event log: ordered appends and the reads that replay it.
pub trait EventLog: Send + Sync + 'static {
    /// ➕️ Persists `events` in one backend transaction: each gets a dense backend-assigned
    /// `seq` (contiguous with the current head, no gaps even under concurrent callers — callers are
    /// expected to already be serialized by `DirectoryService`'s write lock, but a backend MUST NOT
    /// rely on that alone for `seq` density since `rebuild_projections`/tests may call this
    /// directly), a minted uuid v7 `id`, and `recorded_at_ms`; then applies each event's projection
    /// (`//#region 🔖️Projections`) in the same transaction before committing.
    async fn append_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<Vec<DirectoryEvent>> {
        match self.append_decided_events(events).await? {
            DirectoryAppendOutcomeV1::Appended(events) => Ok(events),
            DirectoryAppendOutcomeV1::RejectedBeforeCommit(reason) => Err(reason.into_error()),
        }
    }
    /// 🔐️ Appends a decided batch or proves a semantic refusal by acknowledging rollback.
    /// Errors, including failed rollback or commit, never authorize release of an idempotency claim.
    async fn append_decided_events(&self, events: &[NewDirectoryEvent]) -> DirectoryResult<DirectoryAppendOutcomeV1>;
    /// 🧷️ Commits the exact event page and its private administrator effect proof atomically.
    async fn append_decided_events_with_admin_effect(&self, events: &[NewDirectoryEvent], effect: &NewAdminOperationEffectReceiptV1) -> AdminEffectCommitV1<Vec<DirectoryEvent>>;
    /// 📜️ Every event with `seq > since_seq`, ascending, capped at `limit` — backs both
    /// `GET /directory/events?since=` and `/directory/socket/v1`'s post-subscribe replay (contract C2).
    async fn events_since(&self, since_seq: u64, limit: usize) -> DirectoryResult<Vec<DirectoryEvent>>;
    /// 🔝️ The current log length (0 when empty) — `DirectoryStreamMessage::Heartbeat`'s
    /// `head_seq` and the admin overview's `headSeq` both read this.
    async fn head_seq(&self) -> DirectoryResult<u64>;
}
//#endregion EventLog
