//! 📥️ Items of `store` that belong to `os-store-engine-live` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
use semio_framework_os_store_replay_operation::ArtifactReplayPreparationProgress;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{HybridLogicalTimestamp, MutationId};
use semio_framework_os_spr::{Edit, Mutation, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::sync::Arc;
use semio_framework_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ArtifactStoreDisplacedSnapshotRetirement, SnapshotRead};
use semio_framework_os_store_read::{ArtifactStoreCursorDisposerPhase, ArtifactStoreEnvelopeRetirement, ArtifactStoreMutationDagRetirement, ArtifactStoreRevisionAccumulatorRetirement, ArtifactStoreStringRetirement, ArtifactStoreStringVectorRetirement, ArtifactStoreSupersessionRetirement, HistoryReadPlan, SnapshotReadLeaseRefusal};
use semio_framework_os_store_ledger::edit_retirement::ArtifactStoreDecodedEditRetirement;
use semio_framework_os_spr::command::MutationMessageLedgerRetirement as ArtifactStoreMessageLedgerRetirement;
use semio_framework_value::factory::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::mint_edit_id;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_store_one_item_publication::diagnostics::ArtifactStoreDiagnosticStage;
use semio_framework_os_vcs::ArtifactHistoryKey;
use semio_framework_os_store_space_schema::{ArtifactEnvelope, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, HistoryLane, OpenToolTransaction, SupersedeInput};
use semio_framework_os_store_replay::ReplayTurnBudget;
use semio_framework_os_store_replay::original_ingress::OriginalReplayEdits;
use semio_framework_os_store_replay::{PendingLocalStep, PendingReprojection, ReprojectionAdvance, TailStep};
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_engine_core::lifted_document_materialize::now_iso;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use crate::lifted_document_materialize::stamp_primary_operation_identity;
use semio_framework_os_store_text_io::text_format::{common_address, mutation_ids};
use semio_framework_os_store_command::retire_operations;
use semio_framework_os_store_ledger::{edit_from_operation_envelope, fold_envelope_history, fold_event_log};
use semio_framework_os_store_ledger::{ARTIFACT_EDIT_MESSAGE_ID_BYTES, ArtifactStoreHistoryCommitReservation, ArtifactStorePendingReportRetirement, CursorRevisionAccumulator, PendingCommandReport, StackMirrorDirt, edit_timestamp, envelope_viewer_head, retire_scratch_edits, retire_scratch_operations, retire_supersede_inputs, stamp_edit_semantics};
#[cfg(test)]
use semio_framework_os_store_ledger::hot_path_census;
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchPublication;
use semio_framework_os_store_one_item_publication::ArtifactStoreOneItemPublicationPhase;
use semio_framework_os_store_operation_wire::{ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_framework_os_store_replay::{DRAFT_SUPERSESSION_ID, EditReplay, EditReplayResult, EffectiveSupersessions, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep, admit_replacement, draft_supersessions, effective_operation, fold_operation};
use semio_framework_os_store_replay::{ColdFoldProjection, IndexedAppliedReplayEdits, PREFIX_SNAPSHOT_RING_CAPACITY, PREFIX_SNAPSHOT_RING_FLOOR, PrefixSnapshot, ReplayConvergence, effective_operation_with_supersession, fold_effective_edit, fold_recorded, forward_prefix_digests, forward_prefix_genesis, forward_prefix_step, retire_shared_projection, same_effective_inputs, shared_prefix_len, superseded_positions};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
use semio_framework_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_HISTORY;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_EVERY_HISTORY;
use semio_framework_os_store_engine_core::lifted_store::MIRROR_ORACLE_STRIDE;
use semio_framework_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_os_store_initialization_runtime::lifted_store::bound_edit_messages;
use semio_framework_os_store_engine_core::lifted_store::StoreLog;
use semio_framework_os_store_engine_core::lifted_store::StoreOwners;
use semio_framework_os_store_engine_core::lifted_store::StoreWire;
use semio_framework_os_store_engine_core::lifted_store::StoreCursor;
use semio_framework_os_store_engine_core::lifted_store::ScratchOwner;
use semio_framework_os_store_engine_core::lifted_store::fold_history;

#[path = "🧱️live-fold-and-parts-types/🦀️.rs"]
mod live_fold_and_parts_types;
pub use live_fold_and_parts_types::{
    edit_actor_from_meta, ArtifactStoreDocumentRootCommitAuthority, StoreLive, StoreParts,
    StorePartsRef, StoreReplay,
};

#[path = "📸️snapshot-reads-and-retained-replacements/🦀️.rs"]
mod snapshot_reads_and_retained_replacements;

#[path = "🧮️time-travel-reads-publication-and-supersede-inputs/🦀️.rs"]
mod time_travel_reads_publication_and_supersede_inputs;

#[path = "🎞️event-sourced-history-reprojection/🦀️.rs"]
mod event_sourced_history_reprojection;

#[path = "📣️replay-invalidation-announcements-and-transactions/🦀️.rs"]
mod replay_invalidation_announcements_and_transactions;

#[path = "⏳️deferred-reprojection-and-supersede-reports/🦀️.rs"]
mod deferred_reprojection_and_supersede_reports;

#[path = "🛰️history-lane-commands-and-outbound/🦀️.rs"]
mod history_lane_commands_and_outbound;

#[path = "🟰️convergence-early-exit-and-applied-mutation/🦀️.rs"]
mod convergence_early_exit_and_applied_mutation;
pub use convergence_early_exit_and_applied_mutation::{
    document_backbone_batches, transition_retraction_closure, AppliedMutation,
};

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes

/// 📨️ Publishes the preflighted original diagnostic owner through the current Store component borrows.
pub fn publish_prepared_diagnostics<P, Mutation>(store: &mut StoreParts<'_, P, Mutation>, diagnostics: &mut ArtifactStoreDiagnosticStage, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
        if diagnostics.row_count()==0{return Ok(Default::default());}
        let Some((entry,messages,worst,receipt))=diagnostics.take(grant).map_err(|error|VcsError::ValidationFailed(error.into_message()))?else{return Err(VcsError::ValidationFailed("preflighted diagnostic transfer lost its exact grant".into()));};
        let previous=store.log.envelope.edit_messages.admit_prepared(entry.into_entry()).unwrap_or_else(|_|panic!("unchanged premeasured diagnostic ledger must admit its exact original"));
        if let Some(previous)=previous{let mut owner=diagnostics.displaced.take().expect("preborn exact displaced diagnostic owner");*owner=ArtifactStoreMessageLedgerRetirement::new(previous.edit_id,previous.messages);store.owners.displaced_retirements.push_reserved(owner);}
        store.replay.pending_report.install_prepared_messages(messages,worst);Ok(receipt)
    
}
