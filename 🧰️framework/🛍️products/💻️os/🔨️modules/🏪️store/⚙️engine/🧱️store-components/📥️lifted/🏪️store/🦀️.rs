//! 📥️ Items of `store` that belong to `os-store-engine-core` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_os_vcs::io::binary::genesis::AdmittedArtifactGenesis;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation;
use replay_preparation::ArtifactReplayPreparationFactory;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId};
use semio_framework_os_spr::{Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead};
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_read::{ArtifactHistoryReadRetirement, ArtifactStoreCursorRetirement, ArtifactStoreDisplacedRetirements, ArtifactStoreEnvelopeRetirement, ArtifactStoreHistoryMetadataRetirement, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, DISPLACED_RELIEF_STEPS, HistoryReadPlan, SnapshotReadLeaseRefusal, SnapshotReadLeaseRegistryOwner};
use semio_framework_os_store_read::history_read_admission;
use history_read_admission::PreparedHistoryReadRetirement;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_store_text_io::operation_identity;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_alternative_id, mint_change_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id_with_pending_change;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint};
use semio_framework_os_store_space_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCursor, ArtifactEnvelope, ArtifactProjectionCause, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, OpenToolTransaction, StaleArtifactProjection};
use semio_framework_os_store_replay::PendingReprojection;
use semio_framework_os_store_envelope::owner::OwnerRef;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use semio_framework_os_store_owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use crate::lifted_document_materialize::{EMPTY_CHECKPOINT_MESSAGE, now_iso};
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use crate::lifted_document_materialize::uncommitted_edit_ids;
use semio_framework_os_store_text_io::text_format::print_document_pack;
use semio_framework_os_store_text_io::text_format::mutation_ids;
use semio_framework_os_store_ledger::{ArtifactEditMessageLedger, fold_envelope_history, fold_event_log};
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ID_BYTES, ARTIFACT_EDIT_MESSAGE_INDEX_CAPACITY, ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, ColdReplayProjection, StackMirrorDirt, edit_timestamp, envelope_viewer_head, operation_semantic_kind, retire_replayed_projection, retire_scratch_operations};
#[cfg(test)]
use semio_framework_os_store_ledger::hot_path_census;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOneItemPrepared, ArtifactStoreOneItemPublicationPhase};
use semio_framework_os_store_one_item_publication::lifted_publication::MemberStoreOneItemWirePreparationFactory;
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchSourceOf, ArtifactTypedBatchItemAuthority};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_one_item_publication::prepared_retirement::ArtifactStorePreparedRetirement;
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationSource};
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ArtifactStoreOneItemCheckpoint, ArtifactStoreOneItemFootprint, ArtifactStoreOneItemGrant};
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, ReplayProgress, apply_operation};
use semio_framework_os_store_replay::{REPROJECTION_OWNER_SLOTS, fold_effective_edit, mirror_id_suffix, order_ledger_as};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreOwnedDisposer;
use semio_framework_os_store_initialization_runtime::lifted_store::bound_edit_messages;

#[path = "📇️receipts-envelope-view-and-history-columns/🦀️.rs"]
mod receipts_envelope_view_and_history_columns;
pub use receipts_envelope_view_and_history_columns::{
    assign_history_checkpoint_lanes, branched_alternatives, build_history_columns,
    checkpoint_alternatives, has_main_only_descendant, is_checkpoint_main_only,
    ArtifactEnvelopeView, CommandReceipt, HistoryColumn, MIRROR_ORACLE_EVERY_HISTORY,
    MIRROR_ORACLE_HISTORY, MIRROR_ORACLE_STRIDE,
};

#[path = "🧩️store-component-structs/🦀️.rs"]
mod store_component_structs;
pub use store_component_structs::{StoreCursor, StoreLog, StoreMintToken, StoreOwners, StoreWire};

#[path = "📒️store-log-operations/🦀️.rs"]
mod store_log_operations;

#[path = "🏗️store-owners-retained-reads-and-retirements/🦀️.rs"]
mod store_owners_retained_reads_and_retirements;

#[path = "📬️store-owners-batch-fold-and-history/🦀️.rs"]
mod store_owners_batch_fold_and_history;

#[path = "🖱️store-cursor-operations/🦀️.rs"]
mod store_cursor_operations;

#[path = "🗑️store-wire-scratch-owner-and-fold-history/🦀️.rs"]
mod store_wire_scratch_owner_and_fold_history;
pub use store_wire_scratch_owner_and_fold_history::{fold_history, ScratchOwner};

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
