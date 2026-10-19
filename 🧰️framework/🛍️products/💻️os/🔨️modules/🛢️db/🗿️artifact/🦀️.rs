//! 🗄️ `db_artifact` — the document authority actor and its command pipeline: admit → dedupe →
//! base-resolve → authz → deps → validate → conflict → execute → WAL append → durability →
//! publish → project → preview-reconcile → receipt. Composes `db_state` (materialized
//! overlay), `db_wal` (durability), `db_storage` (the pluggable substrate), and `protocol`
//! (`MutationEnvelope`/`MutationDiff`) into `ArtifactEngine`, the crate's central type, plus a
//! thin `db_actor`-mailbox wrapper (`ArtifactAuthority`) around it. Frozen contract:
//! `.🧬semio/🦑️repo/🎫️tickets/26/07/27/INTRODUCE-DB-PROTOCOL-COMMAND-LAYER-AND-VCS-SLIMMING/contract.md`
//! (`## db crate family`, `db_artifact` row).
//!
//! 🎯️ Design choice (compatibility surface — re-checked against `db_engine`'s live state, which
//! was itself being revised concurrently while this file was authored): `db_engine`'s
//! `document_engine_config` now builds `ArtifactEngineConfig{limits, preview_ttl_ms,
//! ..ArtifactEngineConfig::default()}` (a real `db_security::SecurityGate` baked directly into the
//! default, defaulted `..` spread for the rest),
//! `CommandReceipt.conflicts: Vec<db_conflict::ConflictRecord>` (the real type), and
//! `ArtifactAuthority::run_query(query, consistency)` (two arguments, `db_query::
//! Consistency`-aware). This revision matches that shape exactly: `security`/`emit` are real
//! `ArtifactEngineConfig` fields, `submit`'s conflict step is a genuine
//! `db_conflict::ConflictDetector` fed by retained recent-commit `TouchedSet` history (not a local
//! last-writer stand-in), and `query`/`RunQuery`/`run_query` take a `db_query::
//! Consistency` and resolve it via `db_query::resolve_consistency` + `db_index::
//! IndexConsistencyResolver`. `submit` authorizes through `ArtifactEngineConfig::security` (the real
//! `db_security::SecurityGate`) only. Because `ArtifactEngineConfig`'s new fields are absorbed via `..Default::default()` at every call site
//! observed, `db_projection` registration is also wired in as a `projections` factory field (see
//! `🔖️Engine`'s doc for why a factory, not a stored engine).
//!
//! 🎯️ Design choice (diff convention, unchanged): `protocol::MutationEnvelope`'s `diff`/`inverse`
//! payloads are schema-erased `dsl::DslValue` pathmaps encoded with `store::pack_rt::encode_wire_value`
//! — `db_artifact` has no compile-time knowledge of
//! any concrete document schema, so it adopts one generic convention for BOTH: a JSON *object*
//! whose keys are `db_state`-style `/`-segmented paths and whose values are either the new JSON
//! value to set at that path, or JSON `null` as an explicit tombstone (path deleted). This is a
//! real, documented limitation, not a workaround: a legitimate application-level `null` value is
//! indistinguishable from a delete under this convention. `envelope_from_operation_cold`
//! explicitly selects `protocol::ColdMutation` reference arithmetic and `protocol::Mutation`
//! metadata (`mutation_id`/`dependencies`/`author_id`/`timestamp`, the diff applied once through
//! `protocol::apply_diff_cold`) to build an envelope in this same convention from a typed operation, persisting only the changed fields — the one place in the `db` family
//! below `db_artifact` allowed to interpret operation semantics at all (per the contract's hard
//! dependency rule).
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use crate::os_spr::{ColdMutation,ColdMutationDiff,ColdDiffAlgebra};
use db_artifact_contract::{ArtifactDurableGroupJournalAppendV1, ArtifactDurableGroupRecoveryOwnerV1, ArtifactMessage, CheckpointPublicationSnapshot, CommandBatch, CommandReceipt, SubmitOptions};
use db_artifact_engine::{ArtifactEngine, ArtifactEngineOpenRejected};
use db_artifact_history::{ArtifactHistoryView, HistoryReplayFuture, HistoryReplayReservation};
use db_durability::Frontier;
use db_ids::{*, ArtifactId, DbError, GenerationId};
use db_policy::{MailboxCapacities, Priority, worker_submit_retry_attempt};
use protocol::MutationDiff as _;
use std::{future::Future, pin::Pin, sync::Arc};
#[cfg(test)]
use db_artifact_contract::{ArtifactCommittedDurableGroupRecoveryAdvanceV1, ArtifactDurableGroupRecoveryDriverCoreV1 as _, DB_PATHMAP_SCHEMA, MALFORMED_HISTORY_TRANSITION_CODE, UNKNOWN_SUPERSEDE_TARGET_CODE, committed_durable_group_decision_from_transaction, decode_pathmap_json, encode_pathmap, to_core_document_id};
#[cfg(test)]
use db_artifact_engine::{ArtifactEngineConfig, admit_wal_bytes, close_wal_record_batch, push_wal_record};
#[cfg(test)]
use db_artifact_history::history_capacity_test_lock;
#[cfg(test)]
use db_durability::DurabilityClass;
#[cfg(test)]
use db_ids::ActorId;
#[cfg(test)]
use semio_framework_value::DslValue;

#[path = "🧾️authority-and-journal-sink/🦀️.rs"]
mod authority_and_journal_sink;
#[cfg(not(target_arch="wasm32"))]
pub use authority_and_journal_sink::ArtifactAuthority;
#[cfg(not(target_arch="wasm32"))]
use authority_and_journal_sink::{
    ArtifactBuildFuture, ArtifactDurableGroupJournalSinkV1, ArtifactTurn,
};

#[path = "🔁️close-retry-and-runner-handoff/🦀️.rs"]
mod close_retry_and_runner_handoff;
#[cfg(not(target_arch="wasm32"))]
use close_retry_and_runner_handoff::{
    ArtifactCloseRetry, ArtifactRunnerDriver, ArtifactRunnerHandoff, ARTIFACT_RUNNER_RETRY_LIMIT,
    ARTIFACT_RUNNER_RETRY_MS,
};
pub use close_retry_and_runner_handoff::{
    ArtifactCloseRetryProgress, ArtifactCloseRetryState, ARTIFACT_CLOSE_RETRY_LIMIT,
};
#[cfg(not(target_arch="wasm32"))]
#[cfg(test)]
use close_retry_and_runner_handoff::ARTIFACT_CLOSE_RETRY_CAP_MS;

#[path = "🧹️runner-retirement-and-poll/🦀️.rs"]
mod runner_retirement_and_poll;
#[cfg(not(target_arch="wasm32"))]
#[cfg(test)]
use runner_retirement_and_poll::{
    artifact_runner_retirement_step, ARTIFACT_RUNNER_RETIREMENTS,
    ARTIFACT_RUNNER_RETIREMENT_GENERATIONS,
};
#[cfg(not(target_arch="wasm32"))]
pub use runner_retirement_and_poll::{
    artifact_authority_live_pool_uses, artifact_close_retry_cancel, artifact_close_retry_progress,
    artifact_close_retry_readmit, artifact_runner_handoff_pool_use_live_slots,
    artifact_runner_retirement_live_slots, ArtifactRunnerTerminalJob,
};
#[cfg(not(target_arch="wasm32"))]
use runner_retirement_and_poll::{
    ArtifactRunner, ArtifactRunnerClosePoll, ArtifactRunnerPoll,
    ArtifactRunnerRetirementReservation, ArtifactRunnerWake, ARTIFACT_AUTHORITY_POOL_USES,
    ARTIFACT_RUNNER_HANDOFF_POOL_USES, ARTIFACT_RUNNER_TURN_MICROS,
};

#[path = "🏃️runner-turns/🦀️.rs"]
mod runner_turns;

#[path = "🎭️authority-impl-and-terminal-job/🦀️.rs"]
mod authority_impl_and_terminal_job;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "🧪️tests/⚔️concurrent-write/🦀️.rs"]
mod concurrent_write_tests;
//#endregion 🧪️Tests
