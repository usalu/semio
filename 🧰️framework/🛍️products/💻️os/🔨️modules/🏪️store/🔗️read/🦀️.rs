//! 🔗️ Opaque snapshot reads and the Store's owner catalogs: derived snapshots, replays and history previews, the
//! snapshot read lease registry, the per-owner retirement factories and displaced-owner reservations, and the
//! document and member store owner catalogs with their unscheduled completion. Engine-held views (close view and
//! disposers over a live `ArtifactStore`) stay with the engine; everything here is vocabulary the engine and every
//! facet above it read in one direction.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).

#[path = "📥️lifted/🏪️store/🦀️.rs"]
pub mod lifted_store;
#[path = "../🕰️history/📐️planning/♻️retirement/🦀️.rs"]
pub mod history_plan_retirement;
#[path = "../🕰️history/📐️planning/🎟️admission/🦀️.rs"]
pub mod history_read_admission;
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirementFactory;

use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreChangeRetirementFactory;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay_operation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress};
use semio_framework_os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::MutationId;
use semio_framework_os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_value_snapshot_read::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable};
use semio_framework_os_store_envelope::owner::OwnerRef;
use semio_framework_os_pack::pack_rt;
use semio_framework_os_store_message_ledger::ArtifactEditMessageLedgerRetirement;
use semio_framework_os_store_ledger::CursorRevisionAccumulator;
use semio_framework_pack_json::ArtifactCanonicalJsonTree;

use semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
use semio_framework_os_store_envelope::{ArtifactBackboneRef, ArtifactEnvelope, ArtifactEnvelopeOwners, MigrationProvenance};
use semio_framework_os_store_cursor::ArtifactCursor;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, ReplayProgress};
use semio_framework_os_store_ledger::lifted_replay::EffectiveSupersessions;
use semio_framework_os_store_command::{HistoryLane, OpenToolTransaction};
use semio_framework_os_store_projection::retire_shared_projection;

#[path = "📖️derived-snapshots-replays-and-history-read-plan/🦀️.rs"]
mod derived_snapshots_replays_and_history_read_plan;
pub use derived_snapshots_replays_and_history_read_plan::{
    retain_completed_history_plan, ArtifactDerivedHistoryPreview, ArtifactDerivedReplay,
    ArtifactDerivedSnapshot, ArtifactHistoryReadRetirement, HistoryReadPlan,
};

#[path = "🧮️cursor-ledger-and-vcs-retirements/🦀️.rs"]
mod cursor_ledger_and_vcs_retirements;
pub use cursor_ledger_and_vcs_retirements::{
    admit_original_snapshot_alias, artifact_history_ledger_birth, artifact_history_ledger_handoff,
    retire_document_envelope, retire_document_envelope_birth_bytes,
    ArtifactStoreAlternativeRetirementFactory, ArtifactStoreCheckpointRetirementFactory,
    ArtifactStoreCursorRetirement, ArtifactStoreHistoryLedgerRetirement,
    ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement,
    ArtifactStoreVcsRetirement, ArtifactStoreVcsRetirementPhase,
};

#[path = "🗑️envelope-genesis-and-displaced-snapshot-retirements/🦀️.rs"]
mod envelope_genesis_and_displaced_snapshot_retirements;
pub use envelope_genesis_and_displaced_snapshot_retirements::{
    ArtifactEnvelopeMetadata, ArtifactGenesisRetirement, ArtifactStoreCursorDisposerPhase,
    ArtifactStoreDisplacedRetirements, ArtifactStoreDisplacedSnapshotRetirement,
    ArtifactStoreEnvelopeRetirement, ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY,
    ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY, ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY,
    DISPLACED_RELIEF_STEPS,
};

pub use semio_framework_value_snapshot_read::*;

/// 🧳️ Admits a typed child without losing the exact original pending owner on refusal.

/// 🔗️ Prices the lease frame of a shared root the holder never owns uniquely: only the last alias pays the deep release.

/// 🔗️ Admits a shared root as a lease without losing the exact original alias on refusal.

pub use semio_framework_value_retirement_owner::{retirement_owner_demands,retirement_owner_close};
pub use semio_framework_os_store_replay::lifted_snapshot_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement};
pub use semio_framework_os_store_ledger::lifted_snapshot_read::{ArtifactStoreDisplacedOwnerReservation, ArtifactStoreEditMetadata, ArtifactStoreEditRetirement, ArtifactStoreEditRetirementState, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreHistoryMetadataRetirementState, ArtifactStoreOperationRowOwners, ArtifactStoreStringRetirement};

pub use semio_framework_value_snapshot_read::CapturedSnapshotSource;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
