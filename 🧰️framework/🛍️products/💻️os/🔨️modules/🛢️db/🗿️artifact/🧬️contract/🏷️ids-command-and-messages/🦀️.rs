//! 🏷️ Ids, command and messages: the core document and actor id bridges, the wire and dsl error mappers, the command batch and submit options, the recent touch limit and the artifact message. See [`super`].

use super::*;

//#region 🔖️Ids
/// 🌉️ `protocol::ArtifactId` → `ArtifactId`, the lossless single-`String` bridge
/// `db_core`'s module doc promises (see `ArtifactId`'s own doc for the rationale: this
/// crate is the first one in the family that depends on both `db_core` and `protocol` and so is
/// where the bridge is actually exercised).
pub async fn to_core_document_id(id: &protocol::ArtifactId) -> ArtifactId {
    ArtifactId(id.0.clone())
}

/// 🌉️ `protocol::ActorId` → `ActorId`, same bridge as `to_core_document_id`.
pub async fn to_core_actor_id(id: &protocol::ActorId) -> ActorId {
    ActorId::from(id.0.as_str())
}

// 🔒️ Used as a bare fn-pointer error mapper (`.map_err(json_err)`) throughout this file —
// `Result::map_err`'s `FnOnce(E) -> F2` bound always calls the mapper with an owned `E`, so a
// by-reference signature would not type-check at any of those call sites despite the function
// body itself only borrowing `err` to format it.
// 🚫️async: E4 fn-pointer slot
pub(super) fn wire_err(err: store::PackRefusal) -> DbError {
    DbError::from(err)
}

// 🚫️async: E4 fn-pointer slot
fn dsl_err(err: String) -> DbError {
    DbError::InvalidArgument(format!("db_artifact dsl error: {err}"))
}

//#region 🔖️Command
/// 📦️ One atomically-submitted group of causally-related operations against a single
/// document — the unit `ArtifactEngine::submit` accepts. Every envelope must target the same
/// `document_id` (checked at construction, and again against the engine's own document at submit
/// time); the batch's `command_id` (for dedupe/the returned `CommandReceipt`) is its LAST
/// envelope's `mutation_id`, since `MutationEnvelope` has no separate batch-level id of its own.
pub struct CommandBatch {
    pub envelopes: Vec<protocol::MutationEnvelope>,
}

impl CommandBatch {
    /// 🏗️ Builds a batch, rejecting an empty one or one whose envelopes disagree on
    /// `document_id`.
    pub async fn new(envelopes: Vec<protocol::MutationEnvelope>) -> Result<CommandBatch, DbError> {
        let first = envelopes.first().ok_or_else(|| DbError::InvalidArgument("command batch must contain at least one operation".to_string()))?;
        let document_id = first.document_id.clone();
        if envelopes.iter().any(|envelope| envelope.document_id != document_id) {
            return Err(DbError::InvalidArgument("every envelope in a command batch must target the same document".to_string()));
        }
        Ok(CommandBatch { envelopes })
    }
}

/// 🎚️ Per-submit durability override — see `DurabilityClass`'s doc for the
/// strength ordering `ArtifactWal::submit` honors. `policy` is the authority-local `protocol::
/// MergePolicy` `submit`'s outcome step judges this batch's worst graded conflict/message level
/// against (contract §C9) — never carried on the wire, never part of shared history (see
/// `protocol::MergePolicy`'s own doc).
#[derive(Clone, Copy, Debug)]
pub struct SubmitOptions {
    pub durability: DurabilityClass,
    pub policy: protocol::MergePolicy,
}

impl Default for SubmitOptions {
    fn default() -> Self {
        SubmitOptions { durability: DurabilityClass::Memory, policy: protocol::MergePolicy::default() }
    }
}
//#endregion 🔖️Command

//#region 🔖️Engine
pub const MAX_RECENT_TOUCHES: usize = 256;
//#endregion 🔖️Engine

//#region 🔖️Actor
/// 📨️ A `Send` message crossing `ArtifactAuthority`'s bounded mailbox.
pub enum ArtifactMessage {
    AppendDurableGroupDecision {
        record: store::durable_group::DurableOwnedGroupJournalRecordV1,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        now_ms: u64,
        reply: db_actor::ReplySender<Result<ArtifactDurableGroupJournalAppendV1, DbError>>,
    },
    RecoverDurableGroupDecisions {
        driver: ArtifactDurableGroupRecoveryDriverV1,
        reply: db_actor::ReplySender<Result<(), DbError>>,
    },
    Submit {
        batch: CommandBatch,
        options: SubmitOptions,
        now_ms: u64,
        reply: db_actor::ReplySender<Result<CommandReceipt, DbError>>,
    },
    Query {
        path: String,
        reply: db_actor::ReplySender<Result<Option<db_query::QueryBytes>, DbError>>,
    },
    Frontier {
        reply: db_actor::ReplySender<Frontier>,
    },
    CheckpointPublicationSnapshot {
        reply: db_actor::ReplySender<CheckpointPublicationSnapshot>,
    },
    /// 🔎️ Additive this revision — `db_engine`'s current `ArtifactHandle::query` goes
    /// through `Query { path, .. }` above and never constructs this variant, so adding it is safe.
    RunQuery {
        query: db_query::Query,
        consistency: db_query::Consistency,
        reply: db_actor::ReplySender<Result<db_query::QueryResult, DbError>>,
    },
    SnapshotNow {
        now_ms: u64,
        reply: db_actor::ReplySender<Result<u64, DbError>>,
    },
    History {
        operation_generation: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        reservation: HistoryReplayReservation,
        reply: db_actor::ReplySender<Result<ArtifactHistoryView, DbError>>,
    },
    Compact {
        holder: db_io_core::DbIoText,
        consolidate_snapshots: bool,
        budget: db_compact::CompactionBudget,
        now_ms: u64,
        cancelled: Arc<std::sync::atomic::AtomicBool>,
        reply: db_actor::ReplySender<Result<db_compact::CompactionReport, DbError>>,
    },
}
//#endregion 🔖️Actor
//#endregion 🔖️Ids
