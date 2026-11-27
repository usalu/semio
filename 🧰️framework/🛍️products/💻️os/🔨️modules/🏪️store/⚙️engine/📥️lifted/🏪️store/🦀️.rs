//! 📥️ Items of `store` that belong to `os-store-engine` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_os_store_one_item_publication::diagnostics::*;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_os_store_replay_operation::ArtifactReplayPreparationProgress;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_os_spr::{Edit, MutationMeta, OpBinary, OpText};
use semio_os_spr::{ColdMutation};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use semio_os_store_read::{ArtifactDerivedHistoryPreview, ArtifactDerivedReplay, ArtifactDerivedSnapshot, ErasedSnapshotRead, SnapshotRead};
use semio_os_store_engine_owners::DocumentStoreOwners;
use semio_os_store_read::{ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS, ArtifactStoreCursorDisposerPhase, ArtifactStoreDisplacedRetirements, ArtifactStoreStringVectorRetirement, SnapshotReadLeaseRegistryOwner};
use semio_os_store_read::history_read_admission::PreparedHistoryReadRetirement;
use semio_os_store_retirement::resident_backing;
use semio_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_os_vcs::io::binary::entity_identity::{mint_alternative_id};
use semio_os_vcs::io::binary::checkpoint_identity::content_addressed_checkpoint_id;
use semio_os_vcs::{Author, VcsError};
use semio_os_vcs::{ArtifactHistoryKey, ArtifactHistoryLedger};
use semio_os_store_space_schema::{AcceptedArtifactProjection, ArtifactBackboneRef, ArtifactCommand, ArtifactCursor, ArtifactEnvelope, ArtifactProjectionCacheMode, ArtifactProjectionCause, ArtifactProjectionEvent, ArtifactProjectionInvalidation, ArtifactProjectionResult, ArtifactProjectionStamp, ArtifactRevision, HistoryLane, OpenToolTransaction, StaleArtifactProjection, SupersedeInput};
use semio_os_store_replay::ReplayTurnBudget;
use semio_os_store_replay::{PendingLocalStep, TailStep};

use semio_os_store_text_io::ArtifactPackFiles;
use pack::snapshot::ArtifactPack;
use semio_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_os_store_initialization_runtime::lifted_document_materialize::fresh_replica_clock;
use semio_os_store_text_io::text_format::{ParsedDocumentText, parse_decoded_document_spr};
use semio_os_store_text_io::text_format::{validate_durable_history, validate_history_lanes};
use semio_os_store_command::parse_command;
use semio_os_store_command::retire_command;
use semio_os_store_ledger::fold_envelope_history;
use semio_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, PendingCommandReport, StackMirrorDirt, retire_replayed_projection, retire_supersede_inputs, validate_composition_pins};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory};
use semio_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchSourceOf};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant, LaneItemReceipt};
use semio_os_store_initialization_runtime::native_publication_producer::ArtifactStorePublication;
use semio_os_store_replay::{EditReplay, EditReplayResult, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep};
use semio_os_store_replay::{PREFIX_SNAPSHOT_RING_CAPACITY, REPROJECTION_OWNER_SLOTS, fold_effective_edit, retire_shared_projection, same_effective_inputs};
use semio_os_store_backbone as backbone;
use backbone::{Backbone, BackboneMessage, Backbones};
#[cfg(target_arch = "wasm32")]
use backbone::resolve_backbone;
use semio_os_store_backbone::backbone_retirement::ArtifactStoreBackboneRetirement;
use semio_os_store_engine_owners::lifted_store::ArtifactStoreCloseHost;
use semio_os_store_engine_owners::lifted_store::ArtifactStoreCloseView;
use semio_os_store_engine_core::lifted_store::CommandReceipt;
use semio_os_store_engine_core::lifted_store::ArtifactEnvelopeView;
use semio_os_store_engine_core::lifted_store::HistoryColumn;
use semio_os_store_initialization_runtime::lifted_store::artifact_initial_digest_of_pack;
use semio_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationOwnerCatalog;
use semio_os_store_initialization_runtime::lifted_store::ArtifactStoreInitializationRuntime;
pub use semio_os_store_engine_core::lifted_store::StoreLog;
pub use semio_os_store_engine_core::lifted_store::StoreOwners;
use semio_os_store_engine_core::lifted_store::StoreWire;
use semio_os_store_engine_core::lifted_store::StoreCursor;
use semio_os_store_engine_live::lifted_store::StoreLive;
use semio_os_store_engine_live::lifted_store::StoreReplay;
use semio_os_store_engine_live::lifted_store::StoreParts;
use semio_os_store_engine_live::lifted_store::StorePartsRef;
use semio_os_store_engine_live::lifted_store::ArtifactStoreDocumentRootCommitAuthority;
use semio_os_store_engine_live::lifted_store::AppliedMutation;
use semio_os_store_engine_core::lifted_store::fold_history;

#[path = "🗒️draft-config-store-and-checkpoint-identity/🦀️.rs"]
mod draft_config_store_and_checkpoint_identity;
pub use draft_config_store_and_checkpoint_identity::{config_store_from_initialized_runtime_with_owners, ArtifactStore, ConfigStore, DraftStore};

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
pub use close_host_drop_and_pair_merge::{HistoryFinalization, SupersedeScope};



impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_os_spr::StoreState,
    Mutation: semio_os_spr::StoreMutationFor<P>,
{
    /// 🔭️ Borrows the components a derived read steps over.
    #[inline] fn derived(&self) -> StoreDerived<'_, P, Mutation, &StoreLog<P, Mutation>> { StoreDerived { log: &self.log, owners: &self.owners, wire: &self.wire, cursor: &self.cursor, live: &self.live } }

    /// 🔭️ Borrows the components a derived step over the history ledger mutates.
    #[inline] fn derived_mut(&mut self) -> StoreDerived<'_, P, Mutation, &mut StoreLog<P, Mutation>> { StoreDerived { log: &mut self.log, owners: &self.owners, wire: &self.wire, cursor: &self.cursor, live: &self.live } }

    /// 🪞️ Delegates to [`StoreDerived::finished_history_preview_read_demands`].
    #[inline] pub fn finished_history_preview_read_demands(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Sync { self.derived().finished_history_preview_read_demands(source) }

    /// 🪞️ Delegates to [`StoreDerived::admit_finished_history_preview_read`].
    #[inline] pub fn admit_finished_history_preview_read(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>, output: &mut Option<SnapshotRead<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> where P: Sync { self.derived().admit_finished_history_preview_read(source, output, grant) }

    /// 🪞️ Delegates to [`StoreDerived::retained_derived_read_demands`].
    #[inline] pub fn retained_derived_read_demands(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Sync { self.derived().retained_derived_read_demands(source) }

    /// 🪞️ Delegates to [`StoreDerived::admit_retained_derived_read`].
    #[inline] pub fn admit_retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>, output: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Sync { self.derived().admit_retained_derived_read(source, output, grant) }

    /// 🪞️ Delegates to [`StoreDerived::derive_reference_provisional_snapshot`].
    #[inline] pub fn derive_reference_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> where P: Clone, Mutation: ColdMutation<P> { self.derived().derive_reference_provisional_snapshot(source, op) }

    /// 🪞️ Delegates to [`StoreDerived::derive_reference_history_snapshot`].
    #[inline] pub fn derive_reference_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> where P: Clone, Mutation: ColdMutation<P> { self.derived().derive_reference_history_snapshot(source, op) }

    /// 🪞️ Delegates to [`StoreDerived::begin_derived_history_preview`].
    #[inline] pub fn begin_derived_history_preview(&self, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> { self.derived().begin_derived_history_preview(target, drafts) }

    /// 🪞️ Delegates to [`StoreDerived::begin_derived_report_replay`].
    #[inline] pub fn begin_derived_report_replay(&self, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> { self.derived().begin_derived_report_replay(drafts, from) }

    /// 🪞️ Delegates to [`StoreDerived::derived_operation_birth_demand`].
    #[inline] pub fn derived_operation_birth_demand(&self,source:&ArtifactDerivedSnapshot<P>)->Result<semio_framework_value::RetirementDemand,ValueError> { self.derived().derived_operation_birth_demand(source) }

    /// 🪞️ Delegates to [`StoreDerived::begin_derived_operation`].
    #[inline] pub fn begin_derived_operation(&self,slot:&mut Option<semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mutation>>,source:&ArtifactDerivedSnapshot<P>,input:&mut Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>,operation_index:u32,mode:semio_os_store_replay_operation::ArtifactDerivedOperationMode,grant:RetainedCloneGrant)->Result<Option<RetainedCloneProgress>,ValueError> { self.derived().begin_derived_operation(slot,source,input,operation_index,mode,grant) }

    /// 🪞️ Delegates to [`StoreDerived::derived_operation_demands`].
    #[inline] pub fn derived_operation_demands(&self,owner:&semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mutation>,body:usize,maximum_work_units:u64)->Result<semio_framework_value::RetirementDemand,ValueError> { self.derived().derived_operation_demands(owner,body,maximum_work_units) }

    /// 🪞️ Delegates to [`StoreDerived::step_derived_operation`].
    #[inline] pub fn step_derived_operation(&self,owner:&mut semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mutation>,body:usize,grant:RetainedCloneGrant,receiving:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure> { self.derived().step_derived_operation(owner,body,grant,receiving,deadline) }

    /// 🪞️ Delegates to [`StoreDerived::take_derived_operation`].
    #[inline] pub fn take_derived_operation(&self,owner:&mut semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mutation>,grant:RetainedCloneGrant)->Result<Option<((Option<ArtifactDerivedSnapshot<P>>,Vec<protocol::MutationMessage>),RetainedCloneProgress)>,ValueError> { self.derived().take_derived_operation(owner,grant) }

    /// 🪞️ Delegates to [`StoreDerived::derived_checkpoint_preview_admission_demands`].
    #[inline] pub fn derived_checkpoint_preview_admission_demands()->semio_framework_value::RetirementDemand { StoreDerived::<P,Mutation,&StoreLog<P,Mutation>>::derived_checkpoint_preview_admission_demands() }

    /// 🪞️ Delegates to [`StoreDerived::begin_derived_checkpoint_preview`].
    #[inline] pub fn begin_derived_checkpoint_preview(&self,id:&str,grant:RetainedCloneGrant)->Result<Option<(ArtifactDerivedHistoryPreview<P,Mutation>,RetainedCloneProgress)>,ValueError> { self.derived().begin_derived_checkpoint_preview(id,grant) }

    /// 🪞️ Delegates to [`StoreDerived::derived_history_preview_retirement_demands`].
    #[inline] pub fn derived_history_preview_retirement_demands(&self,preview:&ArtifactDerivedHistoryPreview<P,Mutation>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> { self.derived().derived_history_preview_retirement_demands(preview,body) }

    /// 🪞️ Delegates to [`StoreDerived::history_preview_edit_slot`].
    #[inline] pub fn history_preview_edit_slot(&self,preview:&ArtifactDerivedHistoryPreview<P,Mutation>)->Result<semio_os_vcs::ArtifactHistoryKey,VcsError> { self.derived().history_preview_edit_slot(preview) }

    /// 🪞️ Delegates to [`StoreDerived::step_checkpoint_selection`].
    #[inline] pub fn step_checkpoint_selection(&self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,id:&str,grant:RetainedCloneGrant,deadline:&mut dyn FnMut()->bool)->Result<bool,VcsError> { self.derived().step_checkpoint_selection(preview,id,grant,deadline) }

    /// 🪞️ Delegates to [`StoreDerived::original_report_key`].
    #[inline] pub fn original_report_key(&self,selection:Option<semio_os_vcs::OriginalHistoryEditSelection>,position:usize)->Result<semio_os_vcs::ArtifactHistoryKey,VcsError> { self.derived().original_report_key(selection, position) }

    /// 🪞️ Delegates to [`StoreDerived::begin_checkpoint_report_replay`].
    #[inline] pub fn begin_checkpoint_report_replay(&self,order:&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>,supersessions:&mut semio_os_store_replay::EffectiveSupersessions)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError> { self.derived().begin_checkpoint_report_replay(order, supersessions) }

    /// 🪞️ Delegates to [`StoreDerived::begin_selected_report_replay`].
    #[inline] pub fn begin_selected_report_replay(&self,selection:semio_os_vcs::OriginalHistoryEditSelection)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError> { self.derived().begin_selected_report_replay(selection) }

    /// 🪞️ Delegates to [`StoreDerived::retire_derived_history_preview`].
    #[inline] pub fn retire_derived_history_preview(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync { self.derived_mut().retire_derived_history_preview(owner, effect) }

    /// 🪞️ Delegates to [`StoreDerived::retire_derived_history_preview_controlled`].
    #[inline] pub fn retire_derived_history_preview_controlled(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync { self.derived_mut().retire_derived_history_preview_controlled(owner, effect, control) }

    /// 🪞️ Delegates to [`StoreDerived::derived_report_replay_retirement_demands`].
    #[inline] /// 📏️ Quotes the same original Report child selected by controlled cancellation.
    pub fn derived_report_replay_retirement_demands(&self,replay:&ArtifactDerivedReplay<P,Mutation>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>where P:Sync { self.derived().derived_report_replay_retirement_demands(replay, body) }

    /// 🪞️ Delegates to [`StoreDerived::retire_derived_report_replay`].
    #[inline] pub fn retire_derived_report_replay(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync { self.derived_mut().retire_derived_report_replay(owner, effect) }

    /// 🪞️ Delegates to [`StoreDerived::retire_derived_report_replay_controlled`].
    #[inline] pub fn retire_derived_report_replay_controlled(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync { self.derived_mut().retire_derived_report_replay_controlled(owner, effect, control) }

    /// 🪞️ Delegates to [`StoreDerived::step_derived_history_preview`].
    #[inline] pub fn step_derived_history_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,maximum_work:u64,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError> { self.derived_mut().step_derived_history_preview(preview, grant, control, maximum_work, deadline) }

    /// 🪞️ Delegates to [`StoreDerived::step_derived_checkpoint_preview`].
    #[inline] pub fn step_derived_checkpoint_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,checkpoint:&str,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError> { self.derived_mut().step_derived_checkpoint_preview(preview, checkpoint, grant, control, deadline) }

    /// 🪞️ Delegates to [`StoreDerived::step_derived_report_replay`].
    #[inline] pub fn step_derived_report_replay(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> { self.derived_mut().step_derived_report_replay(owner, retirement_grant, control, maximum_work, deadline) }

    /// 🪞️ Delegates to [`StoreDerived::step_derived_report_replay_in_order`].
    /// 🎟️ Steps the Report replay's one-item units back to back while the caller's grant, the work claim and the wall deadline allow; the unit receipts and preparation work sum into one turn receipt on `owner`, whose capacity and release are netted by the replay's own ledger so the stock it charges is the peak it holds at once, not the sum of every operation it prepared and retired.
    pub fn step_derived_report_replay_batched(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        const UNIT_WORK: u64 = 1024;
        let (mut spent,mut prepared,mut work_left)=(RetainedCloneProgress::default(),0u64,maximum_work);
        loop{
            let remaining=RetainedCloneGrant{maximum_items:grant.maximum_items.saturating_sub(spent.copied_items),maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant};
            let step=self.step_derived_report_replay(owner,remaining,control,work_left,deadline);
            let own=owner.preparation_receipt;
            if matches!(step,Ok(ReplayStep::Pending(_)))&&own!=RetainedCloneProgress::default()&&remaining.maximum_items>own.copied_items&&work_left>=2*UNIT_WORK&&owner.replay.as_ref().is_some_and(|fold|!fold.finished){
                spent=spent.saturating_add(owner.take_preparation_receipt());prepared=prepared.saturating_add(owner.take_preparation_work().unwrap_or(0));work_left-=UNIT_WORK;continue;
            }
            let total=own.saturating_add(spent);owner.preparation_receipt=if step.is_ok(){let (capacity,released)=owner.capacity_ledger.net(total.retained_capacity_bytes,total.released_bytes);RetainedCloneProgress{retained_capacity_bytes:capacity,released_bytes:released,..total}}else{total};
            if prepared!=0{owner.preparation_work=Some(owner.preparation_work.unwrap_or(0).saturating_add(prepared));}
            return step;
        }
    }
    #[inline] pub fn step_derived_report_replay_in_order(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, order:Option<&semio_os_vcs::HistoryPageStack<semio_os_vcs::ArtifactHistoryKey>>,retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, maximum_work:u64, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> { self.derived_mut().step_derived_report_replay_in_order(owner, order, retirement_grant, control, maximum_work, deadline) }

    /// 🪞️ Delegates to [`StoreDerived::history_edit_read_issuer_demands`].
    #[inline] pub fn history_edit_read_issuer_demands(&self)->semio_framework_value::RetirementDemand { self.derived().history_edit_read_issuer_demands() }

    /// 🪞️ Delegates to [`StoreDerived::admit_history_edit_read_issuer`].
    #[inline] pub fn admit_history_edit_read_issuer(&self,slot:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> { self.derived().admit_history_edit_read_issuer(slot, grant) }

    pub fn projection_event<Previous, Policy>(&self, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<SnapshotRead<P>, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: self.cursor.projection_stamp(&self.log, &self.wire), cause, state: self.live.snapshot_read(&self.owners, &self.wire, &self.cursor)?, previous, cache_mode, policy })
    }

}

impl<P, Mutation> ArtifactStore<P, Mutation> where P: semio_os_spr::StoreState, Mutation: semio_os_spr::StoreMutationFor<P> {
    pub fn publish_prepared_diagnostics(&mut self, diagnostics:&mut ArtifactStoreDiagnosticStage, grant:RetainedCloneGrant)->Result<RetainedCloneProgress,VcsError>{
        semio_os_store_engine_live::lifted_store::publish_prepared_diagnostics(&mut self.parts(), diagnostics, grant)
    }

}

pub use semio_os_store_engine_derived::{CheckpointSource, StoreDerived};
