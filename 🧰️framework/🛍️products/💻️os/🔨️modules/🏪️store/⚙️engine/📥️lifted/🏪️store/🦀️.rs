//! 📥️ Items of `store` that belong to `os-store-engine` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_one_item_publication::diagnostics::*;
use semio_framework_value_snapshot_read::SnapshotReadRegistryHandle;
use semio_framework_os_store_read::{HistoryReadPlan, history_read_admission};
use semio_framework_os_store_replay::ReplayConvergence;
use semio_framework_os_store_ledger::edit_retirement;
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_os_store_replay_operation::ArtifactReplayPreparationProgress;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, ArtifactId, HybridLogicalTimestamp, MutationId, SchemaId, UndoPolicy};
use semio_framework_os_spr::{Edit, Mutation, MutationMeta, OpBinary, OpText};
use semio_framework_os_spr::{ColdMutation, MutationDiff};
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
use semio_framework_os_store_ledger::{ArtifactStoreHistoryCommitReservation, CursorRevisionAccumulator, PendingCommandReport, StackMirrorDirt, retire_replayed_projection, retire_supersede_inputs, validate_composition_pins};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::{ArtifactStoreBatchAdmissionRejected, ArtifactStoreBatchPublication};
use semio_framework_os_store_one_item_publication::{ArtifactStoreOneItemAdvance, ArtifactStoreOneItemLiveAuthority, ArtifactStoreOneItemPreparationFactory};
use semio_framework_os_store_one_item_publication::{ArtifactStoreBatchItemAuthority, ArtifactStoreBatchSourceOf};
use semio_framework_os_store_one_item_batch::lifted_one_item_publication::ArtifactStoreBatchStage;
use semio_framework_os_store_operation_wire::{ArtifactOperationWire, ArtifactPreparedOperationCursor, ArtifactPreparedOperationProgress, ArtifactPreparedOperationError};
use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant, LaneItemReceipt};
use semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactStorePublication;
use semio_framework_os_store_replay::{EditReplay, EditReplayResult, IndexedAppliedReplayEdits, EffectiveSupersessions, PairMerge, ReplayEdits, ReplayMode, ReplayProgress, ReplayStep};
use semio_framework_os_store_replay::{PREFIX_SNAPSHOT_RING_CAPACITY, REPROJECTION_OWNER_SLOTS, fold_effective_edit, retire_shared_projection, same_effective_inputs};
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

#[path = "../../../🔁️replay/🎮️operation/🌱️derived/🧩️store/🦀️.rs"]
mod derived_operation_store;
#[path = "../../../🔁️replay/🎮️operation/🧬️native/🔗️history/📜️report/🦀️.rs"]
mod original_report_replay_store;
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
use semio_framework_os_store_replay_operation::{ArtifactDerivedOperation, ArtifactDerivedOperationMode, ArtifactReplayPreparationContext, ArtifactReplayPreparationMode, ArtifactReplayPreparationTask, ArtifactReplayPreparationRequest, ArtifactReplayPrepared};



impl<P, Mutation> ArtifactStore<P, Mutation>
where
    P: semio_framework_os_spr::StoreState,
    Mutation: semio_framework_os_spr::StoreMutationFor<P>,
{
    fn admit_derived_source(&self, registry: &SnapshotReadRegistryHandle, generation: u64, revision: [u8; 32]) -> Result<(), VcsError> {
        self.cursor.admit_derived_source(&self.owners, &self.wire, registry, generation, revision)
    }
    fn seal_derived_snapshot(&self, owner: Arc<P>) -> ArtifactDerivedSnapshot<P> {
        self.cursor.seal_derived_snapshot(&self.owners, &self.wire, owner)
    }
    fn applied_revision(&self) -> &CursorRevisionAccumulator {
        self.cursor.applied_revision(&self.wire)
    }
    fn applied_edit(&self, position: usize) -> Result<&Edit<Mutation>, VcsError> {
        self.cursor.applied_edit(&self.log, &self.wire, position)
    }
    fn step_history_read_plan(&self, plan: &mut HistoryReadPlan,order:Option<&semio_framework_os_vcs::HistoryPageStack<semio_framework_os_vcs::ArtifactHistoryKey>>,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<(bool,RetainedCloneProgress), VcsError> {
        self.live.step_history_read_plan(&self.log, &self.wire, &self.cursor, plan,order,grant,control,deadline)
    }
    fn live_base(&self, length: usize) -> (usize, Arc<P>) {
        self.live.live_base(&self.log, &self.cursor, length)
    }
    fn convergence_checkpoints(&self, horizon: usize, equal: fn(&P, &P) -> bool) -> Result<ReplayConvergence<P>, VcsError> {
        self.live.convergence_checkpoints(&self.log, &self.cursor, horizon, equal)
    }

    pub fn finished_history_preview_read_demands(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>) -> Result<semio_framework_value::RetirementDemand, ValueError> where P: Sync {
        if !source.finished || source.state.is_none() || !source.registry.ptr_eq(&self.owners.snapshot_read_leases) || source.generation != self.generation() || source.revision != self.content_revision() || !source.registry.authority_matches(source.generation, source.revision) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "finished checkpoint read requires its unchanged original Store frontier")); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: SnapshotRead::<P>::capture_copy_byte_demand(), depth: 1, ..Default::default() })
    }

    pub fn admit_finished_history_preview_read(&self, source: &ArtifactDerivedHistoryPreview<P, Mutation>, output: &mut Option<SnapshotRead<P>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> where P: Sync {
        if output.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "finished checkpoint read output is already occupied")); }
        let demand = self.finished_history_preview_read_demands(source)?;
        if grant.maximum_items == 0 || !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let owner = Arc::clone(source.state.as_ref().unwrap());
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let lease = match source.registry.try_issue_original(Arc::clone(&owner)) { Ok(lease) => lease, Err((error, alias)) => { drop(alias); return Err(error.into_value_error().with_retained_progress(progress)); } };
        *output = Some(SnapshotRead::new(owner, lease));
        Ok(RetainedCloneStep::Complete(progress))
    }

    pub fn retained_derived_read_demands(&self, source: &ArtifactDerivedSnapshot<P>) -> Result<semio_framework_value::RetirementDemand, ValueError>
    where P: Sync {
        if !source.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "projection read requires its original Store issuer")); }
        Ok(semio_framework_value::RetirementDemand { copy_bytes: SnapshotRead::<P>::capture_copy_byte_demand() + size_of::<ErasedSnapshotRead>() + size_of::<Option<ErasedSnapshotRead>>(), depth: 2, ..Default::default() })
    }

    pub fn admit_retained_derived_read(&self, source: &ArtifactDerivedSnapshot<P>, output: &mut Option<ErasedSnapshotRead>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError>
    where P: Sync {
        if output.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original projection read output is already occupied")); }
        let demand = self.retained_derived_read_demands(source)?;
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let owner = Arc::clone(&source.owner);
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let lease = match source.registry.try_issue(Arc::clone(&owner)) { Ok(lease) => lease, Err(alias) => { drop(alias); return Ok(RetainedCloneStep::Progress(progress)); } };
        *output = Some(ErasedSnapshotRead::new(owner, lease));
        Ok(RetainedCloneStep::Complete(progress))
    }

    pub fn derive_reference_provisional_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<Option<ArtifactDerivedSnapshot<P>>, VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let outcome = <Mutation as ColdMutation<P>>::diff(op, source.snapshot());
        let applicable = outcome.is_applicable(protocol::MergePolicy::default());
        let (diff, _) = outcome.into_parts();
        let next = applicable.then(|| semio_framework_os_spr::apply_diff_cold(&diff, source.snapshot()).ok()).flatten();
        <<Mutation as ColdMutation<P>>::ReferenceDiff as MutationDiff<P>>::retire_cold(diff);
        Ok(next.map(|owner| self.seal_derived_snapshot(Arc::new(owner))))
    }

    pub fn derive_reference_history_snapshot(&self, source: &ArtifactDerivedSnapshot<P>, op: &Mutation) -> Result<(Option<ArtifactDerivedSnapshot<P>>, Vec<protocol::MutationMessage>), VcsError> where P: Clone, Mutation: ColdMutation<P> {
        self.admit_derived_source(&source.registry, source.generation, source.revision)?;
        let (diff, mut messages) = <Mutation as ColdMutation<P>>::diff(op, source.snapshot()).into_parts();
        let applied = semio_framework_os_spr::apply_diff_cold(&diff, source.snapshot());
        <<Mutation as ColdMutation<P>>::ReferenceDiff as MutationDiff<P>>::retire_cold(diff);
        let next = match applied {
            Ok(owner) => Some(self.seal_derived_snapshot(Arc::new(owner))),
            Err(error) => {
                messages.push(protocol::MutationMessage::fatal(error.code, error.message).at(error.target));
                None
            }
        };
        Ok((next, messages))
    }

    pub fn begin_derived_history_preview(&self, target: MutationId, drafts: protocol::HistoryInputDrafts) -> Result<ArtifactDerivedHistoryPreview<P, Mutation>, VcsError> {
        self.wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedHistoryPreview { checkpoint:None,
            derived_operation:None,history_source:None,history_input:None,history_issuer:None,history_issuer_close:None,displaced_operation:None,operation_close:None,operation_identity:None,operation_adopted:false,preparation_receipt:Default::default(),plan: Some(HistoryReadPlan::new(Some(target), drafts, self.log.applied_edit_ids.len(), false)), plan_retirement:None, drafts: protocol::HistoryInputDrafts::new(), target: None, cursor: 0, operation: 0, state: None,
            progress: ReplayProgress::default(), registry: self.owners.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision(), marker: PhantomData, finished: false,
        })
    }

    pub fn begin_derived_report_replay(&self, drafts: protocol::HistoryInputDrafts, from: Option<MutationId>) -> Result<ArtifactDerivedReplay<P, Mutation>, VcsError> {
        self.wire.ensure_durable_group_idle()?;
        Ok(ArtifactDerivedReplay {replacement_wire:Default::default(),replacement_foreign:false,original_decode:None,original_selection:None,original_order:None,original_history_issuance:self.log.envelope.vcs.edits.retained_issuance(),target:Default::default(),target_operation:None,replacement:Default::default(),replacement_operation:None,replacement_phase:0,history_source:None,history_input:None,history_issuer:None,history_issuer_close:None,history_operation:None,failed_operation:None,preparation_receipt:Default::default(),replay: None, plan: Some(HistoryReadPlan::new(from, drafts, self.log.applied_edit_ids.len(), true)), plan_retirement:None, registry: self.owners.snapshot_read_leases.clone(), generation: self.generation(), revision: self.content_revision() })
    }

    pub fn begin_checkpoint_report_replay(&self,order:&semio_framework_os_vcs::HistoryPageStack<semio_framework_os_vcs::ArtifactHistoryKey>,supersessions:&mut semio_framework_os_store_replay::EffectiveSupersessions)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError>{let mut replay=self.begin_derived_report_replay(Default::default(),None)?;let identity=Some((order as*const _ as usize,order.len()));replay.original_order=identity;let plan=replay.plan.as_mut().unwrap();plan.original_order=identity;plan.supersessions=std::mem::take(supersessions);plan.original_supersessions=true;plan.first_input=0;Ok(replay)}

    pub fn begin_selected_report_replay(&self,selection:semio_framework_os_vcs::OriginalHistoryEditSelection)->Result<ArtifactDerivedReplay<P,Mutation>,VcsError>{let selected=selection.selected_len(self.log.applied_edit_ids.len()).ok_or(VcsError::HistoryReplaying)?;if self.log.envelope.vcs.edits.get_key(selection.key).is_none(){return Err(VcsError::HistoryReplaying)}if !selection.restoring&&self.cursor.revision_accumulator.applied.get(selection.position).and_then(|record|record.ledger_key)!=Some(selection.key){return Err(VcsError::HistoryReplaying)}let mut replay=self.begin_derived_report_replay(Default::default(),None)?;replay.original_selection=Some(selection);let plan=replay.plan.as_mut().unwrap();plan.original_selection=Some(selection);plan.first_input=selection.position.min(selected);Ok(replay)}

    pub fn retire_derived_history_preview(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_history_preview_with_control(owner,grant,None)}
    pub fn retire_derived_history_preview_controlled(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_history_preview_with_control(owner,grant,Some(control))}
    fn retire_derived_history_preview_with_control(&mut self,owner:&mut Option<ArtifactDerivedHistoryPreview<P,Mutation>>,grant:RetainedCloneGrant,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{
        let Some(preview) = owner.as_mut() else { return Ok(None) };
        if !preview.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history preview cannot retire through this Store")); }
        if let Some(operation)=preview.derived_operation.as_mut(){if !operation.is_closing(){let copy=2*size_of::<bool>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<copy{return Ok(None)}operation.cancel();preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(None)}let step=match control.as_mut(){Some(control)=>operation.close_step_controlled(grant,*control),None=>operation.close_step(grant)}?;preview.preparation_receipt=step.progress();if operation.terminal_is_empty(){preview.derived_operation=None;}return Ok(None);}
        if let Some(input)=preview.history_input.as_mut(){if input.terminal_is_empty(){let copy=size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>>();if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return Ok(None);}preview.history_input=None;preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};}else{preview.preparation_receipt=input.close_step(grant)?.progress();}return Ok(None);}
        if let Some(source)=preview.history_source.as_mut(){source.begin_close();preview.preparation_receipt=source.close_step(&mut self.log.envelope.vcs.edits,grant)?.progress();if source.terminal_is_empty(){preview.history_source=None;}return Ok(None);}
        if let Some(issuer)=preview.history_issuer_close.as_mut(){preview.preparation_receipt=issuer.step(grant)?.progress();if issuer.terminal_is_empty(){preview.history_issuer_close=None;}return Ok(None);}
        if preview.history_issuer.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}preview.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(preview.history_issuer.take().unwrap()));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        self.owners.admit_history_owner_retirement(owner,grant,|retirement,preview|*retirement.preview=Some(preview))
    }

    /// 📏️ Quotes the same original Report child selected by controlled cancellation.
    pub fn derived_report_replay_retirement_demands(&self,replay:&ArtifactDerivedReplay<P,Mutation>,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>where P:Sync{
        let header=|copy_bytes|semio_framework_value::RetirementDemand{copy_bytes,depth:1,..Default::default()};
        if !replay.replacement_wire.terminal_is_empty(){return replay.replacement_wire.close_demands(body)}
        if let Some(failed)=replay.failed_operation.as_ref(){return failed.next_demand(body)}
        if !replay.target.terminal_is_empty(){return replay.target.close_demands(body)}
        if replay.target_operation.is_some(){return Ok(header(size_of::<Option<(usize,usize,u8)>>()));}
        if replay.replacement_operation.is_some()&&replay.replay.as_ref().is_some_and(|raw|raw.original_preparation_ready()){
            if let Some(input)=replay.history_input.as_ref(){return if input.terminal_is_empty(){Ok(header(size_of::<Option<semio_framework_value::RetainedOwnedProjection<Mutation>>>()))}else{input.close_demand(body)}}
            if !replay.replacement.terminal_is_empty(){return replay.replacement.close_demands(body)}
            return Ok(header(size_of::<Option<(usize,usize,bool)>>()+size_of::<u8>()+size_of::<semio_framework_os_store_operation_wire::OriginalOperationWireComparison<Mutation>>()+size_of::<bool>()));
        }
        if let Some(raw)=replay.replay.as_ref(){return raw.retirement_demands(body)}
        if let Some(input)=replay.history_input.as_ref(){return if input.terminal_is_empty(){Ok(header(size_of::<Option<semio_framework_value::RetainedOwnedProjection<Mutation>>>()))}else{input.close_demand(body)}}
        if let Some(source)=replay.history_source.as_ref(){return source.close_demands(&self.log.envelope.vcs.edits,body)}
        if let Some(issuer)=replay.history_issuer_close.as_ref(){return issuer.demands(body)}
        if replay.history_issuer.is_some(){return Ok(header(size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>()))}
        Ok(semio_framework_value::RetirementDemand{capacity_bytes:semio_framework_os_store_read::history_read_admission::history_read_retirement_birth_bytes::<P,Mutation>(),depth:1,..Default::default()})
    }

    pub fn retire_derived_report_replay(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_report_replay_with_control(owner,grant,None)}
    pub fn retire_derived_report_replay_controlled(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{self.retire_derived_report_replay_with_control(owner,grant,Some(control))}
    fn retire_derived_report_replay_with_control(&mut self,owner:&mut Option<ArtifactDerivedReplay<P,Mutation>>,grant:RetainedCloneGrant,mut control:Option<&mut dyn semio_framework_value::OriginalRetirementControl>)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>where P:Sync{
        let Some(replay) = owner.as_mut() else { return Ok(None) };
        if !replay.registry.ptr_eq(&self.owners.snapshot_read_leases) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"foreign history replay cannot retire through this Store")); }
        if !replay.replacement_wire.terminal_is_empty(){let d=replay.replacement_wire.close_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}let control=control.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original canonical replacement closure requires its original decoder"))?;control.charge(d.capacity_bytes)?;replay.preparation_receipt=replay.replacement_wire.close_step(grant)?.progress();return Ok(None);}
        if let Some(failed)=replay.failed_operation.as_mut(){replay.preparation_receipt=match control.as_mut(){Some(control)=>failed.close_step_controlled(grant,*control),None=>failed.close_step(grant)}?.progress();if failed.terminal_is_empty(){replay.failed_operation=None;}return Ok(None);}
        if !replay.target.terminal_is_empty(){replay.preparation_receipt=replay.target.close_step(grant)?.progress();return Ok(None);}
        if replay.target_operation.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(usize,usize,u8)>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None)}replay.target_operation=None;replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        if replay.replacement_operation.is_some()&&replay.replay.as_ref().is_some_and(|raw|raw.original_preparation_ready()){let control=control.as_mut().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original replacement closure requires its original decoder custody"))?;replay.preparation_receipt=self.close_original_report_replacement(replay,grant,*control)?;return Ok(None);}
        if let Some(raw)=replay.replay.as_mut(){raw.cancel();replay.preparation_receipt=match control.as_mut(){Some(control)=>raw.close_original_step_controlled(grant,*control),None=>raw.close_original_step(grant)}?.progress();if raw.terminal_is_empty(){replay.replay=None;}return Ok(None);}
        if let Some(input)=replay.history_input.as_mut(){if input.terminal_is_empty(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mutation>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}replay.history_input=None;replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};}else{replay.preparation_receipt=input.close_step(grant)?.progress();}return Ok(None);}
        if let Some(source)=replay.history_source.as_mut(){source.begin_close();replay.preparation_receipt=source.close_step(&mut self.log.envelope.vcs.edits,grant)?.progress();if source.terminal_is_empty(){replay.history_source=None;replay.history_operation=None;}return Ok(None);}
        if let Some(issuer)=replay.history_issuer_close.as_mut(){replay.preparation_receipt=issuer.step(grant)?.progress();if issuer.terminal_is_empty(){replay.history_issuer_close=None;}return Ok(None);}
        if replay.history_issuer.is_some(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>(),depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(None);}replay.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(replay.history_issuer.take().unwrap()));replay.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};return Ok(None);}
        self.owners.admit_history_owner_retirement(owner,grant,|retirement,replay|*retirement.replay=Some(replay))
    }

    pub fn step_derived_history_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError>{self.step_derived_history_preview_inner(preview,None,grant,control,deadline)}

    pub fn step_derived_checkpoint_preview(&mut self,preview:&mut ArtifactDerivedHistoryPreview<P,Mutation>,checkpoint:&str,grant:RetainedCloneGrant,control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,deadline:&mut dyn FnMut()->bool)->Result<ReplayStep,VcsError>{self.step_derived_history_preview_inner(preview,Some(checkpoint),grant,control,deadline)}

    fn step_derived_history_preview_inner(&mut self, preview: &mut ArtifactDerivedHistoryPreview<P, Mutation>,checkpoint:Option<&str>, retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.admit_derived_source(&preview.registry, preview.generation, preview.revision)?;
        if preview.checkpoint.is_some(){let checkpoint=checkpoint.ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"checkpoint preview requires its explicit original selection")))?;preview.checkpoint.as_ref().unwrap().validate(checkpoint).map_err(VcsError::InverseRefused)?;if !self.step_checkpoint_selection(preview,checkpoint,retirement_grant,deadline)?{return Ok(ReplayStep::Pending(preview.progress));}}

        if preview.plan_retirement.is_some() {
            preview.preparation_receipt=preview.retire_completed_plan(retirement_grant).map_err(VcsError::InverseRefused)?.progress();
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if let Some(plan) = preview.plan.as_mut() {
            let(ready,p)=self.step_history_read_plan(plan,None,retirement_grant,control,deadline)?;preview.preparation_receipt=p;if !ready||p!=RetainedCloneProgress::default(){return Ok(ReplayStep::Pending(plan.progress));}
            let live_until=preview.retain_completed_plan();
            let (start, base) = self.live_base(live_until);
            preview.cursor = start;
            preview.state = Some(base);
            return Ok(ReplayStep::Pending(preview.progress));
        }
        if deadline()||retirement_grant.maximum_items==0{return Ok(ReplayStep::Pending(preview.progress));}
        preview.preparation_receipt=RetainedCloneProgress::default();
        if preview.operation_close.is_some(){preview.preparation_receipt=semio_framework_value::close_factory_ticket(&mut preview.operation_close,retirement_grant).map_err(VcsError::InverseRefused)?.progress();return Ok(ReplayStep::Pending(preview.progress));}
        if let Some(owner)=preview.derived_operation.as_mut(){
            if owner.is_closing(){let step=control.with_retirement(retirement_grant,|receiving|owner.close_step_controlled(retirement_grant,receiving)).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();if owner.terminal_is_empty(){drop(preview.derived_operation.take());}return Ok(ReplayStep::Pending(preview.progress));}
        }
        if preview.displaced_operation.is_some(){if let Some((owner,p))=self.retire_snapshot_alias(&mut preview.displaced_operation,retirement_grant).map_err(VcsError::InverseRefused)?{preview.operation_close=Some(owner);preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        if preview.operation_adopted{
            let return_source=preview.operation+1==self.history_preview_edit(preview)?.forwards.len()||preview.target==Some((preview.cursor,preview.operation+1));if return_source{if let Some(source)=preview.history_source.as_mut(){source.begin_close();let step=source.close_step(&mut self.log.envelope.vcs.edits,retirement_grant).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}}
            let release=preview.operation_identity.as_ref().map_or(0,|id|id.0.capacity());let copy=size_of::<Option<MutationId>>()+size_of::<usize>();if retirement_grant.maximum_copy_bytes<copy||retirement_grant.maximum_release_bytes<release{return Ok(ReplayStep::Pending(preview.progress));}drop(preview.operation_identity.take());preview.operation+=1;preview.operation_adopted=false;preview.progress.done=preview.progress.done.saturating_add(1);preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));
        }
        let at_end=if let Some(selection)=preview.checkpoint.as_ref(){selection.complete}else{let(position,op_index)=preview.target.ok_or_else(||VcsError::ValidationFailed("history preview lost its target".into()))?;preview.cursor==position&&preview.operation==op_index};
        if at_end{
            if let Some(factory)=preview.history_issuer_close.as_mut(){let step=factory.step(retirement_grant).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();if factory.terminal_is_empty(){preview.history_issuer_close=None;}return Ok(ReplayStep::Pending(preview.progress));}
            if preview.history_issuer.is_some(){let copy=size_of::<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>();if retirement_grant.maximum_items==0||retirement_grant.maximum_copy_bytes<copy||retirement_grant.maximum_depth==0{return Ok(ReplayStep::Pending(preview.progress));}preview.history_issuer_close=Some(semio_framework_value::FactoryAuthority::new(preview.history_issuer.take().unwrap()));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
            preview.finished=true;return Ok(ReplayStep::Finished(preview.progress));
        }
        if preview.operation==self.history_preview_edit(preview)?.forwards.len(){if let Some(source)=preview.history_source.as_mut(){source.begin_close();let step=source.close_step(&mut self.log.envelope.vcs.edits,retirement_grant).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}if retirement_grant.maximum_items==0||retirement_grant.maximum_copy_bytes<2*size_of::<usize>()||retirement_grant.maximum_depth==0{return Ok(ReplayStep::Pending(preview.progress));}preview.cursor+=1;preview.operation=0;if let Some(selection)=preview.checkpoint.as_mut(){selection.advance_edit();}preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
        if preview.history_issuer.is_none(){preview.preparation_receipt=self.admit_history_edit_read_issuer(&mut preview.history_issuer,retirement_grant).map_err(VcsError::InverseRefused)?.progress();return Ok(ReplayStep::Pending(preview.progress));}
        if preview.history_source.is_none(){let slot=self.history_preview_edit_slot(preview)?;let key=self.log.envelope.vcs.edits.retained_read_key(slot).ok_or(VcsError::HistoryReplaying)?;if let Some((source,p))=semio_framework_os_store_replay_operation::native::history::OriginalHistorySource::admit(key,self.log.envelope.vcs.edits.retained_read_issuer(key).map_err(VcsError::InverseRefused)?.unwrap_or(preview.history_issuer.as_ref().unwrap()),retirement_grant).map_err(VcsError::InverseRefused)?{preview.history_source=Some(source);preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        if preview.history_input.is_none()&&preview.derived_operation.is_none(){let index=preview.operation;if self.history_preview_edit(preview)?.forwards.len()>index{let step=preview.history_source.as_mut().unwrap().advance(&mut self.log.envelope.vcs.edits,&mut preview.history_input,index+1,|edit|&edit.forwards[index],retirement_grant).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();return Ok(ReplayStep::Pending(preview.progress));}}
        let edit=self.history_preview_edit(preview)?;
        if preview.operation==edit.forwards.len(){if let Some(source)=preview.history_source.as_mut(){source.begin_close();let step=source.close_step(&mut self.log.envelope.vcs.edits,retirement_grant).map_err(VcsError::InverseRefused)?;preview.preparation_receipt=step.progress();if source.terminal_is_empty(){preview.history_source=None;}return Ok(ReplayStep::Pending(preview.progress));}if retirement_grant.maximum_copy_bytes<2*size_of::<usize>(){return Ok(ReplayStep::Pending(preview.progress));}preview.cursor+=1;preview.operation=0;if let Some(selection)=preview.checkpoint.as_mut(){selection.advance_edit();}preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<usize>(),..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}
        let index=preview.operation;
        let id=if let Some(id)=edit.mutation_meta.get(index).and_then(|meta|meta.mutation_id.as_ref()){id}else{
            if preview.operation_identity.is_none(){let digits=if index==0{1}else{index.ilog10() as usize+1};let bytes=edit.id.len().saturating_add(1).saturating_add(digits);if bytes.saturating_add(size_of::<MutationId>())>4096||retirement_grant.maximum_capacity_bytes<bytes||retirement_grant.maximum_copy_bytes<bytes{return Ok(ReplayStep::Pending(preview.progress));}let mut id=String::new();id.try_reserve_exact(bytes).map_err(|_|VcsError::ValidationFailed("history operation identity reservation refused".into()))?;id.push_str(&edit.id);id.push('#');use std::fmt::Write;write!(&mut id,"{index}").unwrap();let capacity=id.capacity();preview.operation_identity=Some(MutationId(id));preview.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:capacity,..Default::default()};return Ok(ReplayStep::Pending(preview.progress));}preview.operation_identity.as_ref().unwrap()
        };
        let replacement=preview.drafts.get(id).or_else(||self.live.supersessions.get(id).map(|supersession|&supersession.replacement));
        if preview.derived_operation.is_none(){if replacement.is_some(){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original history replacement requires its retained semantic input issuer")));}let source=ArtifactDerivedSnapshot{owner:Arc::clone(preview.state.as_ref().ok_or(VcsError::HistoryReplaying)?),registry:preview.registry.clone(),generation:preview.generation,revision:preview.revision};if let Some(p)=self.begin_derived_operation(&mut preview.derived_operation,&source,&mut preview.history_input,index as u32,ArtifactDerivedOperationMode::HistoryPrefix,retirement_grant).map_err(VcsError::InverseRefused)?{preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        let owner=preview.derived_operation.as_mut().unwrap();
        if owner.is_finished(){if let Some(((next,_messages),p))=self.take_derived_operation(owner,retirement_grant).map_err(VcsError::InverseRefused)?{if let Some(next)=next{preview.displaced_operation=preview.state.replace(next.into_snapshot_owner());}preview.operation_adopted=true;preview.preparation_receipt=p;}return Ok(ReplayStep::Pending(preview.progress));}
        match self.step_derived_operation(owner,retirement_grant.maximum_copy_bytes,retirement_grant,control,deadline){Ok(step)=>{preview.preparation_receipt=step.progress();Ok(ReplayStep::Pending(preview.progress))},Err(error)=>{preview.preparation_receipt=error.retained_progress();Err(VcsError::InverseRefused(error))}}
    }

    pub fn step_derived_report_replay(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        self.step_derived_report_replay_in_order(owner,None,retirement_grant,control,deadline)
    }

    pub fn step_derived_report_replay_in_order(&mut self, owner: &mut ArtifactDerivedReplay<P, Mutation>, order:Option<&semio_framework_os_vcs::HistoryPageStack<semio_framework_os_vcs::ArtifactHistoryKey>>,retirement_grant:RetainedCloneGrant, control:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>, deadline: &mut dyn FnMut() -> bool) -> Result<ReplayStep, VcsError> {
        if owner.original_order!=order.map(|o|(o as*const _ as usize,o.len())){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"Report changed its original selected order source")))}
        self.admit_derived_source(&owner.registry, owner.generation, owner.revision)?;
        let recipient=control.retirement_recipient_identity().ok_or_else(||VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native Report requires its actual original decoder recipient")))?;let identity=(control.ledger_identity(),recipient);if owner.original_decode.is_some_and(|original|original!=identity){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"native Report changed its original decoder continuation")))}if owner.original_decode.is_none(){let d=semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<(u64,u64)>>(),depth:1,..Default::default()};if semio_framework_value::retained_clone::retained_clone_grant_admits(retirement_grant,d){owner.original_decode=Some(identity);owner.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};}return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())))}
        if owner.plan_retirement.is_some() {
            owner.preparation_receipt=owner.retire_completed_plan(retirement_grant).map_err(VcsError::InverseRefused)?.progress();
            return Ok(ReplayStep::Pending(owner.replay.as_ref().map_or_else(ReplayProgress::default,|replay|replay.progress())));
        }
        if let Some(plan) = owner.plan.as_mut() {
            let(ready,p)=self.step_history_read_plan(plan,order,retirement_grant,control,deadline)?;owner.preparation_receipt=p;if !ready||p!=RetainedCloneProgress::default(){return Ok(ReplayStep::Pending(plan.progress));}
            if self.owners.replay_preparation_factory.is_none()||self.owners.initial_snapshot_retirement_factory.is_none()||self.owners.mutation_retirement_factory.is_none(){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original report requires its installed semantic and retirement issuers")))}
            let capacity=semio_framework_os_spr::command::registered_replay_retirement_factory_allocation_bytes::<P,Mutation>();let copy=size_of::<EditReplay<P,Mutation>>()+4*size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>();let d=semio_framework_value::RetirementDemand{copy_bytes:copy,capacity_bytes:capacity,depth:1,..Default::default()};if !semio_framework_value::retained_clone::retained_clone_grant_admits(retirement_grant,d){return Ok(ReplayStep::Pending(plan.progress))}control.admit_turn_capacity(retirement_grant.maximum_capacity_bytes).map_err(VcsError::InverseRefused)?;control.charge(capacity).map_err(VcsError::InverseRefused)?;
            let generation=owner.generation;let revision=owner.revision;
            let plan=owner.retain_completed_plan();
            let from = plan.first_input;
            let (start, base) = self.live_base(from);
            let total = plan.operations.saturating_sub(plan.counts[from]);
            let mut replay = EditReplay::new_counted(ReplayMode::Report, base, std::mem::replace(&mut plan.order,semio_framework_os_vcs::HistoryPageStack::empty()), from, std::mem::take(&mut plan.native_schema), std::mem::take(&mut plan.supersessions), total)?;
            replay.cursor = start;
            replay.prefix_work_progress = true;
            replay.progress.total = replay.progress.total.saturating_add(plan.counts[from].saturating_sub(plan.counts[start]));
            replay.progress.done = plan.progress.done;
            replay.progress.total = replay.progress.total.saturating_add(plan.progress.done);
            replay = replay.with_base(generation, revision).with_drafts(std::mem::take(&mut plan.drafts));
            replay.operation_preparation_factory = (*self.owners.replay_preparation_factory).clone();
            if let (Some(snapshots), Some(mutations)) = ((*self.owners.initial_snapshot_retirement_factory).clone(), (*self.owners.mutation_retirement_factory).clone()) {
                replay = replay.with_retirement_factories(snapshots, mutations);
            } else if replay.operation_preparation_factory.is_some() {
                let _ = replay.retire_unscheduled();
                return Err(VcsError::ValidationFailed("interactive operation preparation requires exact replay retirement factories".into()));
            }
            owner.replay = Some(replay);owner.preparation_receipt=RetainedCloneProgress{copied_items:1,copied_bytes:copy,retained_capacity_bytes:capacity,..Default::default()};
            return Ok(ReplayStep::Pending(owner.replay.as_ref().expect("planned replay").progress()));
        }
        self.step_original_report_replay(owner,order,retirement_grant,control,deadline)
    }

    pub fn history_edit_read_issuer_demands(&self)->semio_framework_value::RetirementDemand {edit_retirement::history_edit_read_issuer_demands::<Mutation>()}

    pub fn admit_history_edit_read_issuer(&self,slot:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Edit<Mutation>>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        let original=self.owners.mutation_retirement_factory.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read requires the installed original mutation issuer"))?;
        edit_retirement::admit_history_edit_read_issuer(original,slot,grant)
    }

    pub fn projection_event<Previous, Policy>(&self, cause: ArtifactProjectionCause, previous: Option<Previous>, cache_mode: ArtifactProjectionCacheMode, policy: Policy) -> Result<ArtifactProjectionEvent<SnapshotRead<P>, Previous, Policy>, VcsError> {
        Ok(ArtifactProjectionEvent { stamp: self.cursor.projection_stamp(&self.log, &self.wire), cause, state: self.live.snapshot_read(&self.owners, &self.wire, &self.cursor)?, previous, cache_mode, policy })
    }

}

impl<P, Mutation> ArtifactStore<P, Mutation> where P: semio_framework_os_spr::StoreState, Mutation: semio_framework_os_spr::StoreMutationFor<P> {
    pub fn publish_prepared_diagnostics(&mut self, diagnostics:&mut ArtifactStoreDiagnosticStage, grant:RetainedCloneGrant)->Result<RetainedCloneProgress,VcsError>{
        semio_framework_os_store_engine_live::lifted_store::publish_prepared_diagnostics(&mut self.parts(), diagnostics, grant)
    }

}

#[path="📸️checkpoint-read/🦀️.rs"]
mod native_checkpoint_read;
pub use native_checkpoint_read::CheckpointSource;
