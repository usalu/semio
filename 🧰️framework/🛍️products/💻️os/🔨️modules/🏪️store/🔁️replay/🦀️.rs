//! 🔁️ Replay of the edit history: effective forwards and supersessions, the bounded edit replay with its prefix snapshot
//! ring, and pair merges. The engine, history reads and time travel replay through it; it never names the engine.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use semio_framework_os_spr::{ColdDiffAlgebra};
#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
use semio_framework_os_store_replay_operation as replay_preparation;
#[path = "🛑️cancel/🦀️.rs"]
pub mod raw_replay_retirement;
#[path = "📥️lifted/🔗️read/🦀️.rs"]
pub mod lifted_snapshot_read;
#[path = "../📥️ingress/📚️original/🦀️.rs"]
pub mod original_ingress;
#[cfg(any(test, feature = "hot-path-census"))] use semio_framework_os_store_ledger::hot_path_census;

use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
use crate::replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress, ArtifactReplayPreparationRetirement, ArtifactReplayPreparationTask};
use semio_framework_os_spr::command::ArtifactReplayPrepared;
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_os_spr::{HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_vcs::{ArtifactHistoryLedger, Identified, Patchable};
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_message_ledger::ARTIFACT_EDIT_MESSAGE_ENTRY_BYTES;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_os_store_projection::retire_replayed_projection;
use semio_framework_os_store_command::retire_scratch_operations;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;

#[path = "🧮️effective-operations-and-folds/🦀️.rs"]
mod effective_operations_and_folds;
pub use effective_operations_and_folds::{
    admit_replacement, apply_outcome, effective_forwards, effective_operation,
    effective_operation_with_supersession, fold_effective_edit, fold_operation, fold_operation_into, fold_recorded,
    ColdFoldProjection, EffectiveOperation, PairMerge,
};

#[path = "🔢️prefix-digests-replay-progress-and-edit-replay/🦀️.rs"]
mod prefix_digests_replay_progress_and_edit_replay;
pub use prefix_digests_replay_progress_and_edit_replay::{
    draft_supersessions, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step,
    mirror_id_suffix, order_ledger_as, same_effective_inputs, shared_prefix_len,
    superseded_positions, EditReplay, EditReplayResult, IndexedAppliedReplayEdits, SelectedReplayEdits, PrefixSnapshot,
    ReplayConvergence, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep, DRAFT_SUPERSESSION_ID,
    PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, REPROJECTION_OWNER_SLOTS,
};

#[path = "🧱️replay-owned-state-and-result-types/🦀️.rs"]
mod replay_owned_state_and_result_types;
pub use replay_owned_state_and_result_types::{ReplayOwnedState, ReplayResultOwnedState};

#[path = "⏩️replay-owned-state-operations/🦀️.rs"]
mod replay_owned_state_operations;

pub use protocol::apply_operation;

pub use semio_framework_os_store_projection::retire_shared_projection;

#[path="⏳pending/🦀️.rs"]
pub(crate) mod pending;
pub use pending::{PendingLocalStep,PendingReprojection,ReprojectionAdvance,TailStep};

#[path="⏱️budget/🦀️.rs"]
mod budget;
pub use budget::ReplayTurnBudget;
pub use semio_framework_os_store_ledger::lifted_replay::{EffectiveSupersessions, edit_supersession_digest, supersession_digest};

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

#[path="🧬️supersession/🦀️.rs"]
pub mod native_supersession;
