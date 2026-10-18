//! 🧬️ Db artifact commands, diffs, conflict records and durable-group receipts.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;
extern crate semio_framework_os_kernel as store;

use db_artifact_history::{ArtifactHistoryView, HistoryReplayReservation};
use db_durability::{DurabilityClass, Frontier};
use db_ids::{*, ActorId, ArtifactId, DbError};
use db_io_core::WalStorage;
use pack::ContentHash;
use protocol::MutationDiff as _;
use semio_framework_value::DslValue;
use std::{collections::VecDeque, pin::Pin, sync::Arc};

#[path = "🏷️ids-command-and-messages/🦀️.rs"]
mod ids_command_and_messages;
pub use ids_command_and_messages::{to_core_actor_id, to_core_document_id, ArtifactMessage, CommandBatch, SubmitOptions, MAX_RECENT_TOUCHES};
use ids_command_and_messages::wire_err;

#[path = "🗺️pathmap-diff-and-bridge/🦀️.rs"]
mod pathmap_diff_and_bridge;
pub use pathmap_diff_and_bridge::{
    decode_pathmap_json, diff_entries, encode_pathmap, encode_pathmap_json, entries_to_value, entries_touched, envelope_from_operation, inverse_entries, DB_PATHMAP_SCHEMA,
};

#[path = "⚔️conflict-records/🦀️.rs"]
mod conflict_records;
pub use conflict_records::{
    command_touch, durable_group_touch, grade_concurrent_write, remember_recent_touch, unseen_concurrent_writes, ConflictRecord, FOREIGN_HISTORY_TRANSITION_CODE,
    FOREIGN_HISTORY_TRANSITION_MESSAGE, MALFORMED_HISTORY_TRANSITION_CODE, MALFORMED_HISTORY_TRANSITION_MESSAGE, UNKNOWN_SUPERSEDE_TARGET_CODE, UNKNOWN_SUPERSEDE_TARGET_MESSAGE,
};

#[path = "🧾️receipts-and-checkpoints/🦀️.rs"]
mod receipts_and_checkpoints;
pub use receipts_and_checkpoints::{
    ArtifactCommittedDurableGroupDecisionV1, ArtifactCommittedDurableGroupRecoveryAdvanceV1, ArtifactDurableGroupJournalAppendV1, ArtifactDurableGroupRecoveredCheckpointV1,
    CheckpointPublicationSnapshot, CommandReceipt,
};
use receipts_and_checkpoints::ArtifactCommittedDurableGroupRecoveryStateV1;
pub(crate) use receipts_and_checkpoints::{
    ArtifactCommittedDurableGroupRecoveryRejectedTerminalV1, ArtifactCommittedDurableGroupRecoveryTerminalV1, ArtifactCommittedDurableGroupRecoveryV1,
};

#[path = "🛡️durable-group-recovery/🦀️.rs"]
mod durable_group_recovery;
pub use durable_group_recovery::{
    committed_durable_group_decision_from_transaction, ArtifactDurableGroupRecoveryDriverCoreV1, ArtifactDurableGroupRecoveryDriverV1, ArtifactDurableGroupRecoveryOwnerV1,
};

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
