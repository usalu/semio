//! 📥️ Items of `store` that belong to `os-store-engine` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay::replay_preparation;
use replay_preparation::ArtifactReplayPreparationProgress;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{Edit, Mutation, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead, SnapshotRead, retire_document_envelope};
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreCursorDisposerPhase, ArtifactStoreDisplacedRetirements, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, SnapshotReadLeaseRegistryOwner};
use semio_framework_os_store_read::history_read_admission::PreparedHistoryReadRetirement;
use semio_framework_os_store_retirement::resident_backing;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_alternative_id, mint_change_id};
use semio_framework_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id;
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_vcs::{Alternative, ArtifactHistoryKey, ArtifactHistoryLedger, Change, Checkpoint};
use semio_framework_os_store_space_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactEnvelope, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, OpenToolTransaction, StaleArtifactProjection, SupersedeInput};
use semio_framework_os_store_replay::ReplayTurnBudget;
use semio_framework_os_store_replay::{PendingLocalStep, TailStep};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::ArtifactPackFiles;
use pack::snapshot::ArtifactPack;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::fresh_replica_clock;
use semio_framework_os_store_text_io::text_format::{ParsedDocumentText, parse_decoded_document_spr};
use semio_framework_os_store_text_io::text_format::{mutation_ids, validate_durable_history, validate_history_lanes};
use semio_framework_os_store_command::parse_command;
use semio_framework_os_store_command::retire_command;
use semio_framework_os_store_ledger::fold_envelope_history;
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, PendingCommandReport, StackMirrorDirt, retire_replayed_projection, retire_supersede_inputs, validate_composition_pins};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory};
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchSourceOf};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant, LaneItemReceipt};
use semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactStorePublication;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep};
use semio_framework_os_store_replay::{PREFIX_SNAPSHOT_RING_CAPACITY, PrefixSnapshot, REPROJECTION_OWNER_SLOTS, fold_effective_edit, retire_shared_projection, same_effective_inputs};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseHost;
use semio_framework_os_store_engine_owners::lifted_store::ArtifactStoreCloseView;
use semio_framework_os_store_engine_core::lifted_store::CommandReceipt;
use semio_framework_os_store_engine_core::lifted_store::ArtifactEnvelopeView;
use semio_framework_os_store_engine_core::lifted_store::HistoryColumn;
use semio_framework_os_store_initialization_runtime::lifted_store::artifact_initial_digest_of_pack;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
pub use semio_framework_os_store_engine_core::lifted_store::StoreLog;
pub use semio_framework_os_store_engine_core::lifted_store::StoreOwners;
use semio_framework_os_store_engine_core::lifted_store::StoreWire;
use semio_framework_os_store_engine_core::lifted_store::StoreCursor;
use semio_framework_os_store_engine_live::lifted_store::StoreLive;
use semio_framework_os_store_engine_live::lifted_store::StoreReplay;
use semio_framework_os_store_engine_live::lifted_store::StoreParts;
use semio_framework_os_store_engine_live::lifted_store::StorePartsRef;
use semio_framework_os_store_engine_live::lifted_store::ArtifactStoreDocumentRootCommitAuthority;
use semio_framework_os_store_engine_live::lifted_store::AppliedMutation;
use semio_framework_os_store_engine_live::lifted_store::transition_retraction_closure;
use semio_framework_os_store_engine_core::lifted_store::fold_history;

#[path = "🗒️draft-config-store-and-checkpoint-identity/🦀️.rs"]
mod draft_config_store_and_checkpoint_identity;
pub use draft_config_store_and_checkpoint_identity::{
    checkpoint_ancestors, checkpoint_identity, config_store_from_initialized_runtime_with_owners,
    merge_base, reconcile_alternative, ArtifactStore, ConfigStore, DraftStore,
};

#[path = "🌱️runtime-seed-construction-and-owned-close/🦀️.rs"]
mod runtime_seed_construction_and_owned_close;

#[path = "🚦️cursor-admission-dispatch-and-apply-batch/🦀️.rs"]
mod cursor_admission_dispatch_and_apply_batch;

#[path = "↩️supersede-undo-redo-and-backbone-pump/🦀️.rs"]
mod supersede_undo_redo_and_backbone_pump;

#[path = "🪞️delegating-accessors-and-reads/🦀️.rs"]
mod delegating_accessors_and_reads;

#[path = "📑️delegating-operations-and-publication/🦀️.rs"]
mod delegating_operations_and_publication;

#[path = "🧯️close-host-drop-and-pair-merge/🦀️.rs"]
mod close_host_drop_and_pair_merge;
pub use close_host_drop_and_pair_merge::{
    merge_by_id, preflight_merge_by_id, retract_history_transitions_from_spr, HistoryFinalization,
    SupersedeScope,
};

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
