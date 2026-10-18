//! 🛠️ Db artifact engine: document state, query source, state pages and ledger tail.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as store;

use db_artifact_contract::{ArtifactDurableGroupJournalAppendV1, ArtifactDurableGroupRecoveryDriverCoreV1 as _, ArtifactDurableGroupRecoveryDriverV1, CheckpointPublicationSnapshot, CommandBatch, CommandReceipt, ConflictRecord, DB_PATHMAP_SCHEMA, FOREIGN_HISTORY_TRANSITION_CODE, FOREIGN_HISTORY_TRANSITION_MESSAGE, MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE, MAX_RECENT_TOUCHES, SubmitOptions, UNKNOWN_SUPERSEDE_TARGET_CODE, UNKNOWN_SUPERSEDE_TARGET_MESSAGE, command_touch, committed_durable_group_decision_from_transaction, diff_entries, durable_group_touch, encode_pathmap, entries_to_value, entries_touched, grade_concurrent_write, inverse_entries, remember_recent_touch, to_core_actor_id, to_core_document_id, unseen_concurrent_writes};
use db_artifact_history::{HistoryReplayFuture, HistoryReplayReservation};
use db_durability::{DurabilityClass, Frontier};
use db_ids::{*, ActorId, ArtifactId, DbError, DbLimits, check_len};
use db_io_core::WalStorage;
use db_observe::NullEmit;
use pack::ContentHash;
use protocol::MutationDiff as _;
use semio_framework_value::DslValue;
use std::{collections::HashMap, collections::HashSet, collections::VecDeque, sync::Arc};
#[cfg(test)]
use db_artifact_contract::decode_pathmap_json;
#[cfg(test)]
use db_artifact_history::{HistoryEnvelopeCursor, HistoryEnvelopeField, HistoryFrontierCursor, HistoryFrontierField, HistoryReplayPhase, HistoryReplayTransition, history_capacity_test_lock};
#[cfg(test)]
use std::pin::Pin;

#[path = "🧾️wal-batch-state-retirement-and-document-state/🦀️.rs"]
mod wal_batch_state_retirement_and_document_state;
pub use wal_batch_state_retirement_and_document_state::{
    admit_wal_bytes, artifact_state_retirement_maintenance_step, close_wal_record_batch,
    push_wal_record,
};
#[cfg(test)]
use wal_batch_state_retirement_and_document_state::{
    release_artifact_state_retirement, reserve_artifact_state_retirement,
    retire_artifact_state_owner, ArtifactStateRetirementCursor, ARTIFACT_STATE_RETIREMENT,
    ARTIFACT_STATE_RETIREMENT_OVERFLOW, ARTIFACT_STATE_RETIREMENT_PRESSURE_FAULT,
    ARTIFACT_STATE_RETIREMENT_QUARANTINE, ARTIFACT_STATE_RETIREMENT_RESERVATIONS,
    ARTIFACT_STATE_RETIREMENT_SLOTS,
};
use wal_batch_state_retirement_and_document_state::{DocumentState, PlannedEntries};

#[path = "⚙️config-index-backlog-and-open-rejection/🦀️.rs"]
mod config_index_backlog_and_open_rejection;
pub use config_index_backlog_and_open_rejection::{
    ArtifactEngine, ArtifactEngineConfig, ArtifactEngineOpenRejected,
};
use config_index_backlog_and_open_rejection::{
    ArtifactConsistencyResolver, ArtifactIndexBacklog, ArtifactIndexWatermarks,
    APPLIED_RECEIPTS_MAX, INDEX_BACKLOG_ENTRIES_MAX,
};

#[path = "🌱️engine-lifecycle/🦀️.rs"]
mod engine_lifecycle;

#[path = "🚦️engine-submit-pipeline/🦀️.rs"]
mod engine_submit_pipeline;

#[path = "↩️engine-undo-durable-group-query-and-preview/🦀️.rs"]
mod engine_undo_durable_group_query_and_preview;

#[path = "📖️state-query-source-and-pages/🦀️.rs"]
mod state_query_source_and_pages;
use state_query_source_and_pages::{
    StatePageDecodeCursor, StatePageEncodeCursor, StateQuerySource, MAX_STATE_PAGE_VALUE_BYTES,
};
pub use state_query_source_and_pages::MaterializeReport;

#[path = "📈️ledger-tail/🦀️.rs"]
mod ledger_tail;
pub use ledger_tail::{
    artifact_ledger_tail, ArtifactLedgerCommit, ArtifactLedgerPoint,
    ARTIFACT_LEDGER_TAIL_MAX_COMMITS,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
