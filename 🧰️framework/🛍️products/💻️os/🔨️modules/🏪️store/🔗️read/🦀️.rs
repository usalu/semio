//! 🔗️ Opaque snapshot reads and the Store's owner catalogs: derived snapshots, replays and history previews, the
//! snapshot read lease registry, the per-owner retirement factories and displaced-owner reservations, and the
//! document and member store owner catalogs with their unscheduled completion. Engine-held views (close view and
//! disposers over a live `ArtifactStore`) stay with the engine; everything here is vocabulary the engine and every
//! facet above it read in one direction.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use crate::MemberSnapshotOpenOperation;
use super::history_read_admission;
use super::ArtifactStoreDecodedEditRetirementFactory;
use super::ArtifactStoreDecodedEditRetirement;
use super::ArtifactStoreChangeRetirementFactory;
use crate::os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisCodec};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement, SnapshotRetirementFactory};
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use super::replay_preparation::{ArtifactReplayPreparation, ArtifactReplayPreparationFactory, ArtifactReplayPreparationProgress};
use crate::os_spr::command::ArtifactMessageSource;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use crate::os_spr::MutationId;
use crate::os_spr::{DiffRegions, Edit, Mutation, MutationDiff, MutationMeta, OpBinary, OpText};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::marker::PhantomData;
use std::sync::Arc;
use super::snapshot_read_registry_handle::SnapshotReadRegistryHandle;
use super::snapshot_read_registry_retirement::{SnapshotReadRegistryAliasRetirement,snapshot_registry_alias_demands,snapshot_registry_alias_close_step};
use crate::os_spr::conflict::ProtocolConflictRetirement as ArtifactStoreConflictRetirement;
use semio_framework_value::retirement::frame::admit_retirement_frame as admit_artifact_retirement;
use semio_framework_value::retirement::shared::FactorySharedRetirement as ReturnedSnapshotReadRetirement;
use crate::os_vcs::{Author, VcsError};
use crate::os_vcs::{
    Alternative, ArtifactHistoryLedger, ArtifactVcs, Change, Checkpoint, Identified, Patchable,
};
use super::artifact_child_owner_schema::OwnerRef;
use crate::os_pack::pack_rt;
use super::ledger::{ArtifactEditMessageLedgerRetirement, CursorRevisionAccumulator};
use super::canonical_edit::{
    ArtifactCanonicalJson, ArtifactCanonicalJsonTree,
};
use super::prepared_operation_wire::{ArtifactOperationWire, ArtifactOperationText};
use super::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
use super::{ArtifactBackboneRef, ArtifactCursor, ArtifactEnvelope, ArtifactEnvelopeOwners, ArtifactStoreOneItemPreparationFactory, ArtifactStoreOwnedDisposer, EditReplay, EditReplayResult, EffectiveSupersessions, HistoryLane, MemberStoreOneItemWirePreparationFactory, MigrationProvenance, OpenToolTransaction, ReplayProgress, retire_shared_projection};

/// 🌱️ Immutable projection derived by one Store from its own event history or a provisional operation.
pub struct ArtifactDerivedSnapshot<P> {
    pub(crate) owner: Arc<P>,
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) generation: u64,
    pub(crate) revision: [u8; 32],
}

impl<P> ArtifactDerivedSnapshot<P> {
    /// 👁️ Borrows the complete typed projection without transferring its derivation authority.
    pub fn snapshot(&self) -> &P {
        self.owner.as_ref()
    }

    /// 🪞️ Borrows the immutable alias used by renderer seams.
    pub fn snapshot_owner(&self) -> &Arc<P> {
        &self.owner
    }

    /// ♻️ Transfers the projection alias to the caller's registered bounded retirement.
    pub fn into_snapshot_owner(self) -> Arc<P> {
        self.owner
    }
}

/// ⏪️ Store-owned history replay whose input edits and projection origin cannot be supplied by the caller.
pub struct ArtifactDerivedReplay<P, Mu: Mutation<P>> {
    pub(crate) replay: Option<EditReplay<P, Mu>>,
    pub(crate) plan: Option<HistoryReadPlan>,
    pub(crate) plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) generation: u64,
    pub(crate) revision: [u8; 32],
}

impl<P, Mu: Mutation<P> + OpBinary> ArtifactDerivedReplay<P, Mu> {
    /// 📈️ Reports genuine cumulative work inside the currently sealed operation producer.
    pub fn operation_preparation_progress(&self) -> ArtifactReplayPreparationProgress {
        self.replay.as_ref().map_or_else(ArtifactReplayPreparationProgress::default, |replay|replay.operation_preparation_progress())
    }

    pub(crate) fn retain_completed_plan(&mut self) -> &mut HistoryReadPlan {
        let original=self.plan.take().expect("completed replay planner");
        retain_completed_history_plan(&mut self.plan_retirement,original)
    }

    /// 📏️ Borrows the exact cleanup requirements of the completed planner's original residue.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {artifact_retirement_owner_demands(&self.plan_retirement,body)}

    /// ♻️ Closes one original planner residue frontier under the caller's independent grants.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {artifact_retirement_owner_close(&mut self.plan_retirement,grant)}
}

/// 🕰️ A generation-fenced cursor resolving a preview's inputs and folding its prefix under the caller's deadline.
pub struct ArtifactDerivedHistoryPreview<P, Mu: Mutation<P>> {
    pub(crate) plan: Option<HistoryReadPlan>,
    pub(crate) plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub(crate) drafts: protocol::HistoryInputDrafts,
    pub(crate) target: Option<(usize, usize)>,
    pub(crate) cursor: usize,
    pub(crate) operation: usize,
    pub(crate) state: Option<Arc<P>>,
    pub(crate) progress: ReplayProgress,
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) generation: u64,
    pub(crate) revision: [u8; 32],
    pub(crate) marker: PhantomData<fn() -> Mu>,
    pub(crate) finished: bool,
}

impl<P, Mu: Mutation<P>> ArtifactDerivedHistoryPreview<P, Mu> {
    /// 📶️ Work completed while planning or folding the selected prefix.
    pub fn progress(&self) -> ReplayProgress { self.plan.as_ref().map_or(self.progress, |plan| plan.progress) }

    /// 📤️ Transfers the completed projection while retaining every original input and planner residue.
    pub(crate) fn take_finished_snapshot_owner(&mut self)->Result<Arc<P>,VcsError> {
        if !self.finished{return Err(VcsError::HistoryReplaying);}
        self.state.take().ok_or(VcsError::HistoryReplaying)
    }

    pub(crate) fn retain_completed_plan(&mut self)->usize {
        let original=self.plan.take().expect("completed preview planner");
        let plan=retain_completed_history_plan(&mut self.plan_retirement,original);
        self.target=Some(plan.positions[plan.target.as_ref().expect("preview target")]);
        self.progress=plan.progress;self.progress.total=self.progress.done;
        self.drafts=std::mem::take(&mut plan.drafts);
        plan.first_input
    }

    /// 🧮️ Borrows each independent cleanup demand while preserving the original completed planner.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {artifact_retirement_owner_demands(&self.plan_retirement,body)}

    /// 🧹️ Closes one completed planner frontier without dropping any ungranted backing.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {artifact_retirement_owner_close(&mut self.plan_retirement,grant)}
}

impl<P, Mu: Mutation<P>> Drop for ArtifactDerivedHistoryPreview<P, Mu> {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            retire_shared_projection::<P, Mu>(state);
        }
    }
}

pub(crate) struct HistoryReadPlan {
    pub(crate) target: Option<MutationId>,
    pub(crate) drafts: protocol::HistoryInputDrafts,
    pub(crate) positions: protocol::HistoryFoldIndex<MutationId, (usize, usize)>,
    pub(crate) first_input: usize,
    pub(crate) last_draft: Option<usize>,
    pub(crate) order: crate::os_vcs::HistoryPageStack<String>,
    pub(crate) counts: crate::os_vcs::HistoryPageStack<u32>,
    pub(crate) cursor: usize,
    pub(crate) located_target: bool,
    pub(crate) located_draft: Option<MutationId>,
    pub(crate) located_inputs: bool,
    pub(crate) operations: u32,
    pub(crate) supersessions: EffectiveSupersessions,
    pub(crate) copied: Option<MutationId>,
    pub(crate) copying_drafts: bool,
    pub(crate) report: bool,
    pub(crate) progress: ReplayProgress,
    pub(crate) finished: bool,
}

impl HistoryReadPlan {
    pub(crate) fn new(target: Option<MutationId>, drafts: protocol::HistoryInputDrafts, edits: usize, report: bool) -> Self {
        Self {
            target, drafts, positions: protocol::HistoryFoldIndex::new(), first_input: edits, last_draft: None, order: crate::os_vcs::HistoryPageStack::empty(), counts: crate::os_vcs::HistoryPageStack::empty(), cursor: 0, located_target: false, located_draft: None, located_inputs: false, operations: 0,
            supersessions: EffectiveSupersessions::new(), copied: None, copying_drafts: false, report,
            progress: ReplayProgress { done: 0, total: 0 }, finished: false,
        }
    }
}

pub(crate) fn retain_completed_history_plan(slot:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,original:HistoryReadPlan)->&mut HistoryReadPlan {
    assert!(slot.is_none(),"completed history planner residue remains attached");
    *slot=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner declares controlled retirement")));
    slot.as_mut().unwrap().original_mut().unwrap()
}

pub(crate) struct ArtifactHistoryReadRetirement<P, Mu: Mutation<P>> {
    pub(crate) derived: std::mem::ManuallyDrop<Option<ArtifactDerivedSnapshot<P>>>,
    pub(crate) derived_alias: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub(crate) preview: std::mem::ManuallyDrop<Option<ArtifactDerivedHistoryPreview<P, Mu>>>,
    pub(crate) replay: std::mem::ManuallyDrop<Option<ArtifactDerivedReplay<P, Mu>>>,
    pub(crate) finished: std::mem::ManuallyDrop<Option<EditReplayResult<P, Mu>>>,
    pub(crate) loaded_replay: std::mem::ManuallyDrop<Option<EditReplay<P, Mu>>>,
    pub(crate) active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) plan_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>>,
    pub(crate) draft_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::HistoryInputDrafts>>>,
    pub(crate) registry_retirement: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    pub(crate) snapshots: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub(crate) mutations: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>>,
    pub(crate) factory_retirement:[Option<semio_framework_value::FactoryAuthority>;2],
}

impl<P, Mu> ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn derived_snapshot_demands(&self)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.registry_retirement.is_some(){return Some(snapshot_registry_alias_demands(&self.registry_retirement))}
        if self.derived_alias.is_some(){return Some(Ok(semio_framework_value::RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}))}
        self.derived.is_some().then_some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}))
    }
    fn close_derived_snapshot(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let Some(demand)=self.derived_snapshot_demands()else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        if self.registry_retirement.is_some(){return snapshot_registry_alias_close_step(&mut self.registry_retirement,grant).map(Some)}
        if self.derived_alias.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let Some((owner,receipt))=ReturnedSnapshotReadRetirement::admit_original(&mut self.derived_alias,self.snapshots.as_ref().ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"derived original lost its snapshot issuer"))?,child)?else{unreachable!("preflight retains derived original alias")};*self.active=Some(owner);return Ok(Some(RetainedCloneStep::Progress(receipt)))}
        let ArtifactDerivedSnapshot{owner,registry,..}=self.derived.take().unwrap();*self.derived_alias=Some(owner);*self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(registry));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))
    }
    fn plan_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.plan_retirement.is_some() {return Some(artifact_retirement_owner_demands(&self.plan_retirement,body));}
        if self.preview.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some())||self.replay.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some()) {return Some(Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}));}
        None
    }
    fn close_plan(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.plan_demands(grant.maximum_copy_bytes) else{return Ok(None)};
        let demand=demand?;if grant.maximum_items==0 {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history plan retirement requires its original owner depth"));}
        if self.plan_retirement.is_some() {return artifact_retirement_owner_close(&mut self.plan_retirement,grant).map(Some);}
        if let Some(owner)=self.preview.as_mut().and_then(|owner|owner.plan_retirement.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan_retirement.take())) {*self.plan_retirement=Some(owner);}else {
            let original=self.preview.as_mut().and_then(|owner|owner.plan.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan.take())).expect("observed original history planner");
            *self.plan_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner schema declares controlled retirement")));
        }
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn preview_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        use semio_framework_value::RetirementDemand;
        if self.draft_retirement.is_some() {return Some(artifact_retirement_owner_demands(&self.draft_retirement,body));}
        if self.registry_retirement.is_some() {return Some(snapshot_registry_alias_demands(&self.registry_retirement));}
        let preview=self.preview.as_ref()?;
        if !preview.drafts.terminal_is_empty() {return Some(Ok(RetirementDemand {depth:1,..Default::default()}));}
        if preview.state.is_some() {return Some(Ok(RetirementDemand {capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}));}
        Some(Ok(RetirementDemand {depth:1,..Default::default()}))
    }
    fn close_preview(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.preview_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history preview retirement requires its exact owner depth"));}
        if self.draft_retirement.is_some() {return artifact_retirement_owner_close(&mut self.draft_retirement,grant).map(Some);}
        if self.registry_retirement.is_some() {return snapshot_registry_alias_close_step(&mut self.registry_retirement,grant).map(Some);}
        let preview=self.preview.as_mut().expect("observed original preview");
        if !preview.drafts.terminal_is_empty() {let drafts=std::mem::take(&mut preview.drafts);*self.draft_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(drafts).unwrap_or_else(|_|unreachable!("draft schema declares controlled retirement")));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})));}
        if let Some(snapshot)=preview.state.take() {
            let factory=self.snapshots.as_ref().expect("original preview retains its snapshot issuer").clone();
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant};
            return match ReturnedSnapshotReadRetirement::admit(snapshot,factory,child_grant) {Ok((owner,progress))=>{*self.active=Some(owner);Ok(Some(RetainedCloneStep::Progress(progress)))},Err((error,snapshot,factory))=>{preview.state=Some(snapshot);drop(factory);Err(error)}};
        }
        *self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(preview.registry.clone()));drop(self.preview.take());
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn factory_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.derived.is_some()||self.derived_alias.is_some()||self.preview.is_some()||self.replay.is_some()||self.loaded_replay.is_some()||self.finished.is_some() {return None;}
        if let Some(owner)=self.factory_retirement.iter().flatten().next() {return Some(owner.demands(body).and_then(|mut demand| {demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement depth overflow"))?;Ok(demand)}));}
        (self.snapshots.is_some()||self.mutations.is_some()).then_some(Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}))
    }
    fn close_factories(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.factory_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement requires its original owner depth"));}
        if let Some(slot)=self.factory_retirement.iter_mut().find(|owner|owner.is_some()) {
            let owner=slot.as_mut().unwrap();
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant};
            let step=owner.step(child_grant)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"history factory authority")?;
            if owner.terminal_is_empty() {drop(slot.take());}
            return Ok(Some(step));
        }
        let factory:Arc<dyn semio_framework_value::FactoryRetirement>=if let Some(factory)=self.snapshots.take(){factory}else{self.mutations.take().expect("original history mutation issuer")};
        self.factory_retirement[0]=Some(semio_framework_value::FactoryAuthority::new(factory));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn raw_replay_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        let demand=if let Some(owner)=self.finished.as_ref(){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));}owner.retirement_demands(body)}else if let Some(owner)=self.loaded_replay.as_ref().or_else(||self.replay.as_ref().and_then(|owner|owner.replay.as_ref())){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));}owner.retirement_demands(body)}else{return self.replay.is_some().then_some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}));};
        Some(demand.and_then(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history raw replay retirement depth overflow"))?;Ok(demand)}))
    }
    fn close_raw_replay(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress,admit_retained_clone_close};
        let Some(demand)=self.raw_replay_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let moved=RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()});
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(owner)=self.finished.as_mut(){if owner.terminal_is_empty(){drop(self.finished.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original finished replay").map(Some);}
        if let Some(owner)=self.loaded_replay.as_mut(){if owner.terminal_is_empty(){drop(self.loaded_replay.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original loaded replay").map(Some);}
        let derived=self.replay.as_mut().expect("observed original derived replay");
        if let Some(owner)=derived.replay.as_mut(){if owner.terminal_is_empty(){drop(derived.replay.take());return Ok(Some(moved));}owner.begin_retirement();let step=owner.close_original_step(child)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original derived replay").map(Some);}
        *self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(derived.registry.clone()));drop(self.replay.take());Ok(Some(moved))
    }
}

impl<P, Mu> ErasedSnapshotRetirement for ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::RetainedCloneStep;
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.active.is_some() {return artifact_retirement_box_close_step(&mut self.active,grant);}
        let moved = |step: RetainedCloneStep| match step {
            RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: progress.copied_items.max(1), ..progress }),
            RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
        };
        if let Some(step)=self.close_derived_snapshot(grant)?{return Ok(moved(step))}
        if let Some(step) = self.close_plan(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_preview(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_raw_replay(grant)? { return Ok(moved(step)); }
        if let Some(step) = self.close_factories(grant)? { return Ok(moved(step)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual owners require exact constructor admission"))
    }
    fn terminal_is_empty(&self) -> bool { self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.copy_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual copy authority is not admitted")) }, |demand| demand.map(|demand| demand.copy_bytes)) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body).map(|demand|demand.capacity_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(body)).or_else(||self.preview_demands(body)).or_else(||self.raw_replay_demands(body)).or_else(||self.factory_demands(body)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual constructor authority is not admitted")) }, |demand| demand.map(|demand| demand.capacity_bytes)) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.release_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual release authority is not admitted")) }, |demand| demand.map(|demand| demand.release_bytes)) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.depth);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual depth authority is not admitted")) }, |demand| demand.map(|demand| demand.depth)) }
}

impl<P, Mu: Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none)), "history read retirement dropped before its exact terminal witness"); }
}

pub(crate) const SNAPSHOT_READ_LEASE_CAPACITY: usize = 1_024;
/// 📏️ The largest physical release one snapshot registry close step performs: the registry frame or one backing page.
pub(crate) const SNAPSHOT_READ_RELEASE_PAGE_BYTES: usize = 4096;
/// 📄 Lease slots per backing page; one page of slots fits one release page on every target.
pub(crate) const SNAPSHOT_READ_SLOT_PAGE: usize = 128;
/// 📄 Lease generation words per page: exactly one release page.
pub(crate) const SNAPSHOT_READ_GENERATION_PAGE: usize = SNAPSHOT_READ_RELEASE_PAGE_BYTES / size_of::<std::sync::atomic::AtomicU64>();

pub(crate) struct SnapshotReadLeaseSlot {
    pub(crate) generation: u64,
    pub(crate) owner: Arc<dyn std::any::Any + Send + Sync>,
}

type SnapshotReadSlotPage = [std::mem::MaybeUninit<SnapshotReadLeaseSlot>; SNAPSHOT_READ_SLOT_PAGE];
type SnapshotReadGenerationPage = [std::sync::atomic::AtomicU64; SNAPSHOT_READ_GENERATION_PAGE];
const _: () = assert!(size_of::<SnapshotReadSlotPage>() <= SNAPSHOT_READ_RELEASE_PAGE_BYTES && size_of::<SnapshotReadGenerationPage>() <= SNAPSHOT_READ_RELEASE_PAGE_BYTES);
const _: () = assert!(SNAPSHOT_READ_LEASE_CAPACITY % SNAPSHOT_READ_SLOT_PAGE == 0 && SNAPSHOT_READ_LEASE_CAPACITY % SNAPSHOT_READ_GENERATION_PAGE == 0);

/// 📄 The lease slot table in fixed release pages. Issuing requires every page; closing releases one page per step.
pub(crate) struct SnapshotReadSlotPages {
    pages: [Option<Box<SnapshotReadSlotPage>>; SNAPSHOT_READ_LEASE_CAPACITY / SNAPSHOT_READ_SLOT_PAGE],
}

impl SnapshotReadSlotPages {
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::new([const { std::mem::MaybeUninit::uninit() }; SNAPSHOT_READ_SLOT_PAGE]))) } }
    /// 🧱️ True while every slot page is resident, the only state in which a lease may be issued.
    pub(crate) fn resident(&self) -> bool { self.pages.iter().all(Option::is_some) }
    /// 🪪️ Identity of the first resident slot page, for physical-custody witnesses.
    pub(crate) fn backing_identity(&self) -> Option<*const ()> { self.pages.iter().flatten().next().map(|page| page.as_ptr() as *const ()) }
    /// 📏️ Bytes the next page release frees, or zero once every page is released.
    fn next_release_bytes(&self) -> usize { if self.pages.iter().any(Option::is_some) { size_of::<SnapshotReadSlotPage>() } else { 0 } }
    fn release_page(&mut self) -> usize { self.pages.iter_mut().rev().find(|page| page.is_some()).and_then(Option::take).map_or(0, |page| { drop(page); size_of::<SnapshotReadSlotPage>() }) }
}

impl std::ops::Index<usize> for SnapshotReadSlotPages {
    type Output = std::mem::MaybeUninit<SnapshotReadLeaseSlot>;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / SNAPSHOT_READ_SLOT_PAGE].as_ref().expect("resident lease slot page")[index % SNAPSHOT_READ_SLOT_PAGE] }
}

impl std::ops::IndexMut<usize> for SnapshotReadSlotPages {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.pages[index / SNAPSHOT_READ_SLOT_PAGE].as_mut().expect("resident lease slot page")[index % SNAPSHOT_READ_SLOT_PAGE] }
}

/// 📄 The lock-free lease generation words in fixed release pages, released only once the registry is exclusively owned.
pub(crate) struct SnapshotReadGenerationPages {
    pages: [Option<Box<SnapshotReadGenerationPage>>; SNAPSHOT_READ_LEASE_CAPACITY / SNAPSHOT_READ_GENERATION_PAGE],
}

impl SnapshotReadGenerationPages {
    fn new() -> Self { Self { pages: std::array::from_fn(|_| Some(Box::new(std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0))))) } }
    fn next_release_bytes(&self) -> usize { if self.pages.iter().any(Option::is_some) { size_of::<SnapshotReadGenerationPage>() } else { 0 } }
    fn release_page(&mut self) -> usize { self.pages.iter_mut().rev().find(|page| page.is_some()).and_then(Option::take).map_or(0, |page| { drop(page); size_of::<SnapshotReadGenerationPage>() }) }
}

impl std::ops::Index<usize> for SnapshotReadGenerationPages {
    type Output = std::sync::atomic::AtomicU64;
    fn index(&self, index: usize) -> &Self::Output { &self.pages[index / SNAPSHOT_READ_GENERATION_PAGE].as_ref().expect("resident lease generation page")[index % SNAPSHOT_READ_GENERATION_PAGE] }
}

pub(crate) struct SnapshotReadLeaseRegistryState {
    pub(crate) slots: SnapshotReadSlotPages,
    pub(crate) occupied: [u64; SNAPSHOT_READ_LEASE_CAPACITY / 64],
    pub(crate) free: [u16; SNAPSHOT_READ_LEASE_CAPACITY],
    pub(crate) free_read: usize,
    pub(crate) free_len: usize,
    pub(crate) next_generation: u64,
    pub(crate) cleanup_cursor: usize,
}

#[derive(Debug)]
pub(crate) enum SnapshotReadLeaseRefusal {
    Busy,
    Poisoned,
    OwnerTypeChanged,
    Admission(ValueError),
}

impl SnapshotReadLeaseRefusal {
    pub(crate) fn into_value_error(self) -> ValueError {
        match self {
            Self::Admission(error) => error,
            Self::Busy => ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "snapshot read lease registry is busy"),
            Self::Poisoned => ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease registry is poisoned"),
            Self::OwnerTypeChanged => ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease owner type changed inside its exact registry"),
        }
    }
}

pub(crate) struct SnapshotReadLeaseStateLock {
    held: std::sync::atomic::AtomicBool,
    poisoned: std::sync::atomic::AtomicBool,
    state: std::cell::UnsafeCell<SnapshotReadLeaseRegistryState>,
}

/// 🔒️ Acquire/Release guards exclusively borrow the Send registry state and never cross threads.
unsafe impl Sync for SnapshotReadLeaseStateLock where SnapshotReadLeaseRegistryState: Send {}

impl SnapshotReadLeaseStateLock {
    fn new(state: SnapshotReadLeaseRegistryState) -> Self { Self { held: std::sync::atomic::AtomicBool::new(false), poisoned: std::sync::atomic::AtomicBool::new(false), state: std::cell::UnsafeCell::new(state) } }
    pub(crate) fn try_lock(&self) -> std::sync::TryLockResult<SnapshotReadLeaseStateGuard<'_>> {
        if self.held.compare_exchange(false, true, std::sync::atomic::Ordering::Acquire, std::sync::atomic::Ordering::Relaxed).is_err() { return Err(std::sync::TryLockError::WouldBlock); }
        let guard = SnapshotReadLeaseStateGuard { lock: self, panicking_on_entry: std::thread::panicking(), not_send: PhantomData };
        if self.poisoned.load(std::sync::atomic::Ordering::Relaxed) { Err(std::sync::TryLockError::Poisoned(std::sync::PoisonError::new(guard))) } else { Ok(guard) }
    }
    fn get_mut(&mut self) -> std::sync::LockResult<&mut SnapshotReadLeaseRegistryState> {
        let state = self.state.get_mut();
        if *self.poisoned.get_mut() { Err(std::sync::PoisonError::new(state)) } else { Ok(state) }
    }
}

pub(crate) struct SnapshotReadLeaseStateGuard<'a> { lock: &'a SnapshotReadLeaseStateLock, panicking_on_entry: bool, not_send: PhantomData<*mut ()> }

impl std::ops::Deref for SnapshotReadLeaseStateGuard<'_> {
    type Target = SnapshotReadLeaseRegistryState;
    fn deref(&self) -> &Self::Target { unsafe { &*self.lock.state.get() } }
}

impl std::ops::DerefMut for SnapshotReadLeaseStateGuard<'_> {
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.lock.state.get() } }
}

impl Drop for SnapshotReadLeaseStateGuard<'_> {
    fn drop(&mut self) {
        if !self.panicking_on_entry && std::thread::panicking() { self.lock.poisoned.store(true, std::sync::atomic::Ordering::Relaxed); }
        self.lock.held.store(false, std::sync::atomic::Ordering::Release);
    }
}

pub(crate) struct SnapshotReadLeaseRegistry {
    pub(crate) state: SnapshotReadLeaseStateLock,
    pub(crate) returned: std::sync::atomic::AtomicUsize,
    pub(crate) lease_generations: SnapshotReadGenerationPages,
    authority_sequence: std::sync::atomic::AtomicU64,
    authority_generation: std::sync::atomic::AtomicU64,
    authority_revision: [std::sync::atomic::AtomicU64; 4],
}

impl SnapshotReadLeaseRegistry {
    pub(crate) fn new() -> Self {
        let mut free = [0; SNAPSHOT_READ_LEASE_CAPACITY];
        for (index, slot) in free.iter_mut().enumerate() {
            *slot = index as u16;
        }
        Self {
            state: SnapshotReadLeaseStateLock::new(SnapshotReadLeaseRegistryState {
                slots: SnapshotReadSlotPages::new(),
                occupied: [0; SNAPSHOT_READ_LEASE_CAPACITY / 64],
                free,
                free_read: 0,
                free_len: SNAPSHOT_READ_LEASE_CAPACITY,
                next_generation: 0,
                cleanup_cursor: 0,
            }),
            returned: std::sync::atomic::AtomicUsize::new(0),
            lease_generations: SnapshotReadGenerationPages::new(),
            authority_sequence: std::sync::atomic::AtomicU64::new(0),
            authority_generation: std::sync::atomic::AtomicU64::new(0),
            authority_revision: std::array::from_fn(|_| std::sync::atomic::AtomicU64::new(0)),
        }
    }

    pub(crate) fn publish_authority(&self, generation: u64, revision: [u8; 32]) -> bool {
        let sequence = self.authority_sequence.load(std::sync::atomic::Ordering::Acquire);
        let Some(terminal_sequence) = sequence.checked_add(2) else { return false };
        if sequence & 1 != 0 || self.authority_sequence.compare_exchange(sequence, sequence + 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return false;
        }
        self.authority_generation.store(generation, std::sync::atomic::Ordering::Relaxed);
        for (word, bytes) in self.authority_revision.iter().zip(revision.as_chunks::<8>().0.iter()) {
            word.store(u64::from_le_bytes(*bytes), std::sync::atomic::Ordering::Relaxed);
        }
        self.authority_sequence.store(terminal_sequence, std::sync::atomic::Ordering::Release);
        true
    }

    pub(crate) fn authority_matches(&self, generation: u64, revision: [u8; 32]) -> bool {
        let first = self.authority_sequence.load(std::sync::atomic::Ordering::Acquire);
        if first == 0 || first & 1 != 0 || self.authority_generation.load(std::sync::atomic::Ordering::Relaxed) != generation {
            return false;
        }
        for (word, bytes) in self.authority_revision.iter().zip(revision.as_chunks::<8>().0.iter()) {
            if word.load(std::sync::atomic::Ordering::Relaxed) != u64::from_le_bytes(*bytes) {
                return false;
            }
        }
        first == self.authority_sequence.load(std::sync::atomic::Ordering::Acquire)
    }

    pub(crate) fn try_take(&self, index: u16, generation: u64) -> Result<Arc<dyn std::any::Any + Send + Sync>, ValueError> {
        let Ok(mut state) = self.state.try_lock() else {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot read lease registry is busy"));
        };
        let index = index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease is no longer registered"));
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != generation {
            return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "snapshot read lease generation is stale"));
        }
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        if self.lease_generations[index].swap(0, std::sync::atomic::Ordering::AcqRel) & (1u64 << 63) != 0 {
            self.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        }
        Ok(slot.owner)
    }

    pub(crate) fn contains(&self, index: u16, generation: u64) -> bool {
        let Ok(state) = self.state.try_lock() else {
            return true;
        };
        let index = index as usize;
        let mask = 1 << (index % 64);
        state.occupied[index / 64] & mask != 0 && unsafe { state.slots[index].assume_init_ref() }.generation == generation
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.returned.load(std::sync::atomic::Ordering::Acquire) == 0 && self.state.try_lock().is_ok_and(|state| state.free_len == SNAPSHOT_READ_LEASE_CAPACITY)
    }

    pub(crate) fn has_returned(&self) -> bool {
        self.returned.load(std::sync::atomic::Ordering::Acquire) != 0
    }

    /// 📏️ Quotes the next slot page an empty shared registry releases; generation pages wait for exclusive ownership.
    pub(crate) fn empty_backing_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let state = self.state.try_lock().map_err(|reason| match reason { std::sync::TryLockError::WouldBlock => SnapshotReadLeaseRefusal::Busy.into_value_error(), std::sync::TryLockError::Poisoned(_) => SnapshotReadLeaseRefusal::Poisoned.into_value_error() })?;
        let release_bytes = state.slots.next_release_bytes();
        Ok(semio_framework_value::RetirementDemand { release_bytes, depth: usize::from(release_bytes != 0), ..Default::default() })
    }
    /// ♻️ Releases one slot page of an empty shared registry; the last page completes the shared backing.
    pub(crate) fn close_empty_backing_step(&self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        let mut state = match self.state.try_lock() { Ok(state) => state, Err(std::sync::TryLockError::WouldBlock) => return Ok(RetainedCloneStep::Progress(Default::default())), Err(std::sync::TryLockError::Poisoned(_)) => return Err(SnapshotReadLeaseRefusal::Poisoned.into_value_error()) };
        if state.free_len != SNAPSHOT_READ_LEASE_CAPACITY || self.returned.load(std::sync::atomic::Ordering::Acquire) != 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let bytes = state.slots.next_release_bytes();
        if bytes == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "registry backing requires admitted depth")); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: state.slots.release_page(), ..Default::default() };
        Ok(if state.slots.next_release_bytes() == 0 { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
    /// 📏️ Quotes the next page an exclusively owned empty registry releases: slot pages, then generation pages.
    pub(crate) fn owned_backing_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let shared = self.empty_backing_demands()?;
        if shared.release_bytes != 0 { return Ok(shared); }
        let release_bytes = self.lease_generations.next_release_bytes();
        Ok(semio_framework_value::RetirementDemand { release_bytes, depth: usize::from(release_bytes != 0), ..Default::default() })
    }
    /// ♻️ Releases one page of an exclusively owned empty registry; the last generation page completes its backing.
    pub(crate) fn close_owned_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep, RetainedCloneProgress};
        if self.empty_backing_demands()?.release_bytes != 0 { return self.close_empty_backing_step(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let bytes = self.lease_generations.next_release_bytes();
        if bytes == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "registry backing requires admitted depth")); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: self.lease_generations.release_page(), ..Default::default() };
        Ok(if self.lease_generations.next_release_bytes() == 0 { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub(crate) fn occupied_count(&self) -> usize {
        self.state.try_lock().map_or(SNAPSHOT_READ_LEASE_CAPACITY, |state| SNAPSHOT_READ_LEASE_CAPACITY - state.free_len)
    }

    /// 🪶 Releases a lease whose root is still aliased outside this registry, on the returning
    /// thread and in constant time.
    ///
    /// The retirement pump exists for DISPLACED roots: a returned read whose slot holds the last
    /// `Arc` is the only owner left, so its domain tree must be disposed in bounded steps. A read
    /// that is returned while its root is still live — every `ephemeral` hook capture, every tool
    /// job's presence/transient capture — has nothing to dispose, and the pump discovers that two
    /// turns later with an `Arc::into_inner` that simply fails (see
    /// `ReturnedSnapshotReadRetirement::close_step` and the presence local ladder). Charging a
    /// fair maintenance turn per dispatch to rediscover it starves the registry: a mounted app
    /// that dispatches once per frame issues leases faster than one fair step per frame can
    /// reclaim them, and the fixed slot table runs out mid-run.
    fn try_release_aliased(&self, index: u16, generation: u64) -> bool {
        let Ok(mut state) = self.state.try_lock() else { return false };
        let index = index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return false;
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != generation || self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) != 0 || Arc::strong_count(&slot.owner) == 1 {
            return false;
        }
        drop(self.free_slot_locked(&mut state, index));
        true
    }

    /// 🪶 Frees one occupied slot under the held registry lock and hands its owner to the caller to drop after unlocking.
    fn free_slot_locked(&self, state: &mut SnapshotReadLeaseStateGuard<'_>, index: usize) -> SnapshotReadLeaseSlot {
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !(1 << (index % 64));
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        self.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        slot
    }

    pub(crate) fn returned_admission_demands<T: Send + Sync + 'static>(&self, demand: impl FnOnce(&Arc<T>) -> semio_framework_value::RetirementDemand) -> Result<semio_framework_value::RetirementDemand, SnapshotReadLeaseRefusal> {
        let state = match self.state.try_lock() { Ok(state) => state, Err(std::sync::TryLockError::WouldBlock) => return Err(SnapshotReadLeaseRefusal::Busy), Err(std::sync::TryLockError::Poisoned(_)) => return Err(SnapshotReadLeaseRefusal::Poisoned) };
        let start = state.cleanup_cursor;
        let index = (0..=state.occupied.len()).find_map(|offset| { let word_index = (start / 64 + offset) % state.occupied.len(); let mut occupied = state.occupied[word_index]; if offset == 0 { occupied &= u64::MAX << (start % 64); } else if offset == state.occupied.len() { occupied &= (1u64 << (start % 64)) - 1; } (occupied != 0).then(|| word_index * 64 + occupied.trailing_zeros() as usize) });
        let Some(index) = index else { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); };
        if self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) == 0 { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        let owner = Arc::clone(&slot.owner).downcast::<T>().map_err(|_| SnapshotReadLeaseRefusal::OwnerTypeChanged)?;
        Ok(demand(&owner))
    }

    pub(crate) fn try_admit_one_returned<T: Send + Sync + 'static, R>(&self, grant: RetainedCloneGrant, admission: impl FnOnce(Arc<T>, RetainedCloneGrant) -> Result<(R, RetainedCloneProgress), (ValueError, Arc<T>)>) -> Result<(Option<R>, RetainedCloneProgress), SnapshotReadLeaseRefusal> {
        use semio_framework_value::retained_clone::RetainedCloneProgress;
        if grant.maximum_items == 0 { return Ok((None, Default::default())); }
        if grant.maximum_depth == 0 { return Err(SnapshotReadLeaseRefusal::Admission(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "returned read admission requires depth"))); }
        let mut state = match self.state.try_lock() {
            Ok(state) => state,
            Err(std::sync::TryLockError::WouldBlock) => return Err(SnapshotReadLeaseRefusal::Busy),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err(SnapshotReadLeaseRefusal::Poisoned),
        };
        let start = state.cleanup_cursor;
        let Some(index) = (0..=state.occupied.len()).find_map(|offset| {
            let word_index = (start / 64 + offset) % state.occupied.len();
            let mut occupied = state.occupied[word_index];
            if offset == 0 {
                occupied &= u64::MAX << (start % 64);
            } else if offset == state.occupied.len() {
                occupied &= (1u64 << (start % 64)) - 1;
            }
            (occupied != 0).then(|| word_index * 64 + occupied.trailing_zeros() as usize)
        }) else {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        };
        state.cleanup_cursor = (index + 1) % SNAPSHOT_READ_LEASE_CAPACITY;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if self.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) & (1u64 << 63) == 0 {
            return Ok((None, RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !slot.owner.is::<T>() {
            return Err(SnapshotReadLeaseRefusal::OwnerTypeChanged);
        }
        let slot = unsafe { state.slots[index].assume_init_read() };
        let original = slot.owner.downcast::<T>().expect("validated original returned read type");
        let (result, progress) = match admission(original, grant) {
            Ok(admitted) => admitted,
            Err((error, original)) => { state.slots[index].write(SnapshotReadLeaseSlot { generation: slot.generation, owner: original }); return Err(SnapshotReadLeaseRefusal::Admission(error)); },
        };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        self.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
        self.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        Ok((Some(result), progress))
    }
}

impl Drop for SnapshotReadLeaseRegistry {
    fn drop(&mut self) {
        let state = self.state.get_mut().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(std::thread::panicking() || state.free_len == SNAPSHOT_READ_LEASE_CAPACITY, "snapshot read lease registry reached Drop before every exact owner was returned and retired: {} of {SNAPSHOT_READ_LEASE_CAPACITY}", state.free_len);
        assert!(std::thread::panicking() || self.returned.load(std::sync::atomic::Ordering::Acquire) == 0, "snapshot read lease registry reached Drop with returned owners outside the bounded retirement pump");
    }
}

pub(crate) struct SnapshotReadLeaseRegistryOwner(pub(crate) Option<SnapshotReadRegistryAliasRetirement>);

impl SnapshotReadLeaseRegistryOwner {
    pub(crate) fn new() -> Self { Self(Some(SnapshotReadRegistryAliasRetirement::new(SnapshotReadRegistryHandle::new()))) }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.0.as_ref().is_none_or(|owner| owner.terminal_is_empty()) }
    pub(crate) fn closed(&self) -> bool { self.0.is_none() }
    pub(crate) fn occupied_count(&self) -> usize { self.0.as_ref().map_or(0, |owner| owner.occupied_count()) }
    pub(crate) fn empty_backing_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.0.as_ref().map_or(Ok(Default::default()), |owner| owner.empty_backing_demands()) }
    pub(crate) fn close_empty_backing_step(&self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.0.as_ref().map_or(Ok(RetainedCloneStep::Complete(Default::default())), |owner| owner.close_empty_backing_step(grant)) }
    /// 📏️ Quotes the next release of the registry's own custody: its Arc frame, then, once sole, each generation page.
    pub(crate) fn custody_byte_demand(&self) -> Result<usize, ValueError> { self.0.as_ref().map_or(Ok(0), |owner| owner.custody_byte_demand()) }
    /// ♻️ Releases the registry's own custody one frame or page at a time once it is the sole terminal owner.
    pub(crate) fn close_custody_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.0.as_ref().is_some_and(|owner| owner.strong_count() != 1 || !owner.terminal_is_empty()) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        snapshot_registry_alias_close_step(&mut self.0, grant)
    }
}

pub(crate) fn snapshot_registry_frame_bytes() -> Result<usize, ValueError> {
    std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<SnapshotReadLeaseRegistry>()).map(|(layout, _)| layout.pad_to_align().size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::AllocationFailed, "snapshot registry Arc layout overflow"))
}

impl std::ops::Deref for SnapshotReadLeaseRegistryOwner {
    type Target = SnapshotReadRegistryHandle;
    fn deref(&self) -> &Self::Target { self.0.as_ref().expect("snapshot registry is unavailable after exact owned close").alias_handle() }
}

pub(crate) struct SnapshotReadLease {
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) index: u16,
    pub(crate) generation: u64,
}

impl SnapshotReadLease {
    pub(crate) fn return_now(&mut self) -> bool {
        let mark = &self.registry.lease_generations[self.index as usize];
        if mark.load(std::sync::atomic::Ordering::Acquire) != self.generation { return false; }
        if self.registry.try_release_aliased(self.index, self.generation) { return true; }
        self.registry.returned.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
        if mark.compare_exchange(self.generation, self.generation | (1u64 << 63), std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            self.registry.returned.fetch_sub(1, std::sync::atomic::Ordering::AcqRel);
            return false;
        }
        true
    }
}

pub(crate) fn return_snapshot_read<T: ?Sized>(owner: &mut Option<Arc<T>>, lease: &mut Option<SnapshotReadLease>, after_alias_release: impl FnOnce()) -> bool {
    let Some(mut lease) = lease.take() else {
        assert!(owner.is_none(), "snapshot read alias requires its exact unreturned registry guard");
        return false;
    };
    drop(owner.take());
    after_alias_release();
    lease.return_now()
}

/// 🔐️ Returns only the exact original read while its registry slot protects the payload alias.
pub(crate) fn try_return_original_snapshot_read<T: ?Sized>(owner: &mut Option<Arc<T>>, lease: &mut Option<SnapshotReadLease>) -> Result<SnapshotReadReturn, ()> {
    let fields = lease.as_ref().ok_or(())?;
    let original_owner = owner.as_ref().ok_or(())?;
    let state = fields.registry.state.try_lock().map_err(|_| ())?;
    let index = fields.index as usize;
    if index >= SNAPSHOT_READ_LEASE_CAPACITY || !state.slots.resident() || state.occupied[index / 64] & (1 << (index % 64)) == 0 { return Err(()); }
    let slot = unsafe { state.slots[index].assume_init_ref() };
    if slot.generation != fields.generation || Arc::as_ptr(&slot.owner) as *const () != Arc::as_ptr(original_owner) as *const () { return Err(()); }
    fields.registry.lease_generations[index].compare_exchange(fields.generation, fields.generation | (1u64 << 63), std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).map_err(|_| ())?;
    fields.registry.returned.fetch_add(1, std::sync::atomic::Ordering::AcqRel);
    drop(owner.take());
    drop(state);
    let original = lease.take().expect("validated original read lease");
    Ok(SnapshotReadReturn { registry: original.registry, index: original.index, generation: original.generation })
}

pub struct SnapshotRead<T: ?Sized> {
    pub(crate) owner: Option<Arc<T>>,
    pub(crate) lease: Option<SnapshotReadLease>,
}

/// 🧾 Exact returned-read ticket retained by a mounted owner until store maintenance reclaims it.
pub struct SnapshotReadReturn {
    pub(crate) registry: SnapshotReadRegistryHandle,
    pub(crate) index: u16,
    pub(crate) generation: u64,
}

impl SnapshotReadReturn {
    pub fn terminal_is_empty(&self) -> bool {
        !self.registry.contains(self.index, self.generation)
    }
}

impl<T: ?Sized> SnapshotRead<T> {
    pub(crate) fn new(owner: Arc<T>, lease: SnapshotReadLease) -> Self {
        Self { owner: Some(owner), lease: Some(lease) }
    }

    /// 👁️ Borrows the captured immutable value without exposing its owner.
    pub fn get(&self) -> &T {
        self.owner.as_deref().expect("live snapshot read owner is present")
    }

    /// 🔐️ Revalidates this lease against the store-owned event authority without reading
    /// a session-local registry. A concurrent publication produces an odd or changed sequence and
    /// therefore fails closed.
    pub fn commit_authority_matches(&self, generation: u64, revision: [u8; 32]) -> bool {
        self.lease.as_ref().is_some_and(|lease| lease.registry.authority_matches(generation, revision))
    }

    /// 🧬️ Admits this original captured lease as source authority without replacing its registry.
    pub fn admit_retained_clone_source(self,grant:RetainedCloneGrant) -> Result<(semio_framework_value::retained_clone::RetainedCloneSource<T,Self>,RetainedCloneProgress),(ValueError,Self)>
    where
        T: Sized + semio_framework_value::retirement::RetireOwned + Sync,
    {

        semio_framework_value::retained_clone::RetainedCloneSource::admit_borrowed(self,|read:&Self|read.get(),grant)
    }

    /// 🧹 Returns this lease to its exact registry while the registry retains the snapshot
    /// root for its own one-owner maintenance cursor.
    pub fn return_to_registry(mut self) -> bool {
        return_snapshot_read(&mut self.owner, &mut self.lease, || {})
    }

    /// 🧳️ Rejected return preserves the original read rather than consuming its registry custody.
    pub fn try_return_to_registry_witness(mut self)->Result<SnapshotReadReturn,Self>{
        match try_return_original_snapshot_read(&mut self.owner, &mut self.lease) { Ok(witness) => Ok(witness), Err(()) => Err(self) }
    }
    /// 🧹 Returns this lease and hands the caller an exact generation witness for store retirement.
    pub fn return_to_registry_witness(mut self) -> Option<SnapshotReadReturn> {
        let lease = self.lease.as_ref()?;
        let witness = SnapshotReadReturn { registry: lease.registry.clone(), index: lease.index, generation: lease.generation };
        if !return_snapshot_read(&mut self.owner, &mut self.lease, || {}) {
            return None;
        }
        Some(witness)
    }
}

impl<T: Send + Sync + 'static> SnapshotRead<T> {
    /// 🪞️ Moves the original captured root and lease into erased custody without cloning or allocating.
    pub fn into_erased(mut self) -> ErasedSnapshotRead {
        ErasedSnapshotRead { owner: self.owner.take().map(|owner| owner as Arc<dyn std::any::Any + Send + Sync>), lease: self.lease.take() }
    }
}

impl<T: ?Sized> Drop for SnapshotRead<T> {
    fn drop(&mut self) {
        let _ = return_snapshot_read(&mut self.owner, &mut self.lease, || {});
    }
}

impl<T: ?Sized> std::ops::Deref for SnapshotRead<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

/// 🧰 Type-erased snapshot capability used only until a composition view selects its
/// repository-owned concrete artifact type.
pub struct ErasedSnapshotRead {
    pub(crate) owner: Option<Arc<dyn std::any::Any + Send + Sync>>,
    pub(crate) lease: Option<SnapshotReadLease>,
}

impl ErasedSnapshotRead {
    /// 🔒️ Revalidates the original erased lease against its issuer's current event authority.
    pub fn commit_authority_matches(&self,generation:u64,revision:[u8;32])->bool{
        self.owner.is_some()&&self.lease.as_ref().is_some_and(|lease|lease.registry.authority_matches(generation,revision))
    }
    /// 📏️ Generation leases use the registry's fixed inline atomics and allocate no per-read backing.
    pub const LEASE_ALLOCATION_BYTES: usize = 0;

    /// 🪪️ Quotes the original private read, fixed registry slot and exact return metadata.
    pub fn prepared_return_demands() -> semio_framework_value::RetirementDemand {
        semio_framework_value::RetirementDemand { depth: 2, ..Default::default() }
    }

    /// 🎟️ Issues a fixed registry slot without allocating or copying snapshot payload.
    pub fn prepared_read_demands() -> semio_framework_value::RetirementDemand { semio_framework_value::RetirementDemand { depth: 2, ..Default::default() } }

    pub(crate) fn new<T: Send + Sync + 'static>(owner: Arc<T>, lease: SnapshotReadLease) -> Self {
        Self { owner: Some(owner), lease: Some(lease) }
    }

    /// 🪪️ Acquires another genuine lease over this exact captured root and original issuer without allocating.
    pub fn try_duplicate(&self) -> Result<Self, ValueError> {
        let owner = self.owner.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased read has no captured root"))?;
        let lease = self.lease.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original erased read has no captured lease"))?;
        let mut state = lease.registry.state.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "original read issuer is busy"))?;
        let index = lease.index as usize;
        if state.occupied[index / 64] & (1 << (index % 64)) == 0 || lease.registry.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) != lease.generation { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original read generation is unavailable")); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != lease.generation || !Arc::ptr_eq(&slot.owner, owner) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "original read issuer root changed")); }
        if state.free_len == 0 || !state.slots.resident() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original read issuer has no vacant slot")); }
        let generation = state.next_generation.checked_add(1).filter(|generation| *generation < 1u64 << 63).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "original read issuer generation exhausted"))?;
        let next = state.free[state.free_read] as usize;
        state.free_read = (state.free_read + 1) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free_len -= 1;
        state.next_generation = generation;
        state.slots[next].write(SnapshotReadLeaseSlot { generation, owner: Arc::clone(owner) });
        lease.registry.lease_generations[next].store(generation, std::sync::atomic::Ordering::Release);
        state.occupied[next / 64] |= 1 << (next % 64);
        Ok(Self { owner: Some(Arc::clone(owner)), lease: Some(SnapshotReadLease { registry: lease.registry.clone(), index: next as u16, generation }) })
    }
    pub fn typed<T: Send + Sync + 'static>(&self) -> Option<SnapshotReadRef<'_, T>> {
        self.owner.as_deref()?.downcast_ref::<T>().map(|owner| SnapshotReadRef { owner })
    }

    /// 🪞️ Moves this original erased root and lease into typed custody; mismatch returns the exact original read.
    pub fn try_into_typed<T: Send + Sync + 'static>(mut self) -> Result<SnapshotRead<T>, Self> {
        if self.lease.is_none() { return Err(self); }
        let Some(owner) = self.owner.take() else { return Err(self); };
        match owner.downcast::<T>() {
            Ok(owner) => Ok(SnapshotRead { owner: Some(owner), lease: self.lease.take() }),
            Err(owner) => { self.owner = Some(owner); Err(self) }
        }
    }
    /// 👁️ Borrows one exact concrete owner without exposing or cloning its retained `Arc`.
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.owner.as_deref()?.downcast_ref::<T>()
    }

    pub(crate) fn admit_retirement<T: Send + Sync + 'static>(&mut self, registry: &SnapshotReadRegistryHandle, factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<T>>, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), ValueError> {
        let lease = self.lease.as_ref().filter(|lease| lease.registry.ptr_eq(registry)).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original registry"))?;
        if self.owner.as_deref().is_none_or(|owner| !owner.is::<T>()) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission requires its original concrete owner")); }
        let mut state = registry.state.try_lock().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "read admission registry is busy"))?;
        let index = lease.index as usize;
        let mask = 1 << (index % 64);
        if state.occupied[index / 64] & mask == 0 || registry.lease_generations[index].load(std::sync::atomic::Ordering::Acquire) != lease.generation { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission generation is stale")); }
        let slot = unsafe { state.slots[index].assume_init_ref() };
        if slot.generation != lease.generation || !Arc::ptr_eq(&slot.owner, self.owner.as_ref().expect("validated read owner")) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "read admission must preserve its exact original slot")); }
        let original = self.owner.take().expect("validated original read").downcast::<T>().expect("validated original type");
        let admitted = match ReturnedSnapshotReadRetirement::admit(original, Arc::clone(factory), grant) {
            Ok(admitted) => admitted,
            Err((error, original, alias)) => { self.owner = Some(original); drop(alias); return Err(error); }
        };
        let slot = unsafe { state.slots[index].assume_init_read() };
        state.occupied[index / 64] &= !mask;
        let write = (state.free_read + state.free_len) % SNAPSHOT_READ_LEASE_CAPACITY;
        state.free[write] = index as u16;
        state.free_len += 1;
        registry.lease_generations[index].store(0, std::sync::atomic::Ordering::Release);
        drop(slot);
        self.lease.take();
        Ok(admitted)
    }

    pub(crate) fn into_typed<T: Send + Sync + 'static>(mut self, registry: &SnapshotReadRegistryHandle) -> Result<Arc<T>, Self> {
        if self.lease.as_ref().is_none_or(|lease| !lease.registry.ptr_eq(registry)) || self.owner.as_deref().is_none_or(|owner| !owner.is::<T>()) {
            return Err(self);
        }
        let lease = self.lease.take().expect("validated snapshot read lease is present");
        let owner = self.owner.take().expect("validated snapshot read owner is present").downcast::<T>().expect("validated snapshot read type");
        let guard = match registry.try_take(lease.index, lease.generation) {
            Ok(guard) => guard,
            Err(_) => {
                self.owner = Some(owner);
                self.lease = Some(lease);
                return Err(self);
            }
        };
        drop(guard);
        Ok(owner)
    }
}

impl Drop for ErasedSnapshotRead {
    fn drop(&mut self) {
        let _ = return_snapshot_read(&mut self.owner, &mut self.lease, || {});
    }
}

pub struct SnapshotReadRef<'a, T: ?Sized> {
    owner: &'a T,
}

impl<T: ?Sized> std::ops::Deref for SnapshotReadRef<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.owner
    }
}

/// 🧳️ Retains the original supersession index under independent item, capacity, release and depth grants.
pub(crate) struct ArtifactStoreSupersessionRetirement {
    owner: semio_framework_value::retirement::controlled::ControlledRetirement<EffectiveSupersessions>,
}

impl ArtifactStoreSupersessionRetirement {
    pub(crate) fn new(supersessions: EffectiveSupersessions) -> Self {
        let owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(supersessions).unwrap_or_else(|_|unreachable!("supersession index declares controlled retirement"));
        Self { owner }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreSupersessionRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
}

pub(crate) struct ArtifactStoreStringRetirement {
    owner: semio_framework_value::retirement::controlled::ControlledRetirement<String>,
}

impl ArtifactStoreStringRetirement {
    pub(crate) fn new(value: String) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(value).unwrap_or_else(|_|unreachable!("String declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreStringRetirement {
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.owner.step(grant)}
    fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_copy_byte_demand()}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.owner.next_capacity_byte_demand(copy)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{self.owner.next_release_byte_demand()}
    fn next_depth_demand(&self)->Result<usize,ValueError>{self.owner.next_depth_demand()}
}

pub(crate) struct ArtifactStoreStringVectorRetirement {
    pub(crate) values: std::mem::ManuallyDrop<Option<crate::os_vcs::HistoryPageStack<String>>>,
    active: std::mem::ManuallyDrop<Option<ArtifactStoreStringRetirement>>,
}

impl ArtifactStoreStringVectorRetirement {
    pub(crate) fn new(values: impl Into<crate::os_vcs::HistoryPageStack<String>>) -> Self {
        Self { values: std::mem::ManuallyDrop::new(Some(values.into())), active: std::mem::ManuallyDrop::new(None) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreStringVectorRetirement {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> {
        self.active.as_ref().map_or(Ok(0), ErasedSnapshotRetirement::next_copy_byte_demand)
    }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> {
        self.active.as_ref().map_or(Ok(0), |active| active.next_capacity_byte_demand(copy))
    }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref() { return active.next_release_byte_demand(); }
        self.values.as_ref().filter(|values| values.is_empty()).map_or(Ok(0), |values| values.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history backing extent is unavailable")))
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> {
        if let Some(active) = self.active.as_ref() {
            return active.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "string history retirement depth overflow"));
        }
        Ok(usize::from(self.values.is_some()))
    }

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.next_depth_demand()? { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "string history retirement exceeds admitted depth")); }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
            }
            let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = active.close_step(child_grant)?;
            admit_retained_clone_close(child_grant, step, active.terminal_is_empty(), "Store string history child")?;
            return Ok(match step { RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(progress) });
        }
        let values = self.values.as_mut().expect("live string history owner");
        if let Some(value) = values.pop() {
            *self.active = Some(ArtifactStoreStringRetirement::new(value));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let release = values.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history retains owners in an empty page"))?;
        if release != 0 {
            if grant.maximum_release_bytes < release { return Ok(RetainedCloneStep::Progress(empty)); }
            if !values.release_empty_page() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "string history page changed after exact release demand")); }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: release, ..empty }));
        }
        drop(self.values.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }

    fn terminal_is_empty(&self) -> bool {
        self.values.is_none() && self.active.is_none()
    }
}

impl Drop for ArtifactStoreStringVectorRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store string-vector retirement reached Drop before its exact terminal-empty witness");
    }
}

pub(crate) struct ArtifactStoreRevisionAccumulatorRetirement {
    owner: semio_framework_value::retirement::controlled::ControlledRetirement<CursorRevisionAccumulator>,
}

impl ArtifactStoreRevisionAccumulatorRetirement {
    pub(crate) fn new(accumulator: CursorRevisionAccumulator) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(accumulator).unwrap_or_else(|_| unreachable!("Store revision schema declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreRevisionAccumulatorRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

pub(crate) struct ArtifactStoreCursorRetirement {
    pub(crate) owner: semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactCursor>,
}

impl ArtifactStoreCursorRetirement {
    pub(crate) fn new(cursor: ArtifactCursor) -> Self {
        Self { owner: semio_framework_value::retirement::controlled::ControlledRetirement::new(cursor).unwrap_or_else(|_| unreachable!("Store cursor declares controlled retirement")) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreCursorRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { self.owner.step(grant) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.owner.next_capacity_byte_demand(copy) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.owner.next_depth_demand() }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}

pub(crate) struct ArtifactStoreEditRetirement<Mutation> {
    pub(crate) forwards: std::mem::ManuallyDrop<Option<Vec<Mutation>>>,
    inverse: std::mem::ManuallyDrop<Option<semio_framework_value::list::PagedList<Mutation, {usize::MAX}>>>,
    state: std::mem::ManuallyDrop<Option<ArtifactStoreEditRetirementState>>,
}

pub(crate) type ArtifactStoreEditMetadata = (String, Option<semio_framework_value::SharedUtf8>, (Option<String>, Vec<MutationMeta>), (Option<String>, String, Option<String>));

pub(crate) type ArtifactStoreEditRetirementState = semio_framework_value::retirement::controlled::ControlledRetirement<ArtifactStoreEditMetadata>;

impl<Mutation> ArtifactStoreEditRetirement<Mutation> {
    pub(crate) fn new(edit: Edit<Mutation>) -> Result<Self, Edit<Mutation>> {
        if !edit.forwards.is_empty() || !edit.inverse.is_empty() { return Err(edit); }
        let Edit { id, actor, line, forwards, inverse, mutation_meta, verb, sequence_number: _, started_at, finished_at } = edit;
        let state=ArtifactStoreEditRetirementState::new((id,actor,(line,mutation_meta),(verb,started_at,finished_at))).unwrap_or_else(|_|unreachable!("original edit metadata declares controlled retirement"));
        Ok(Self { forwards:std::mem::ManuallyDrop::new(Some(forwards)),inverse:std::mem::ManuallyDrop::new(Some(inverse)),state:std::mem::ManuallyDrop::new(Some(state)) })
    }
    pub(crate) fn demands(&self,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        if let Some(state)=self.state.as_ref(){if state.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactStoreEditRetirementState>>(),depth:1,..Default::default()});}return Ok(RetirementDemand{copy_bytes:state.next_copy_byte_demand()?,capacity_bytes:state.next_capacity_byte_demand(copy)?,release_bytes:state.next_release_byte_demand()?,depth:state.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"original edit metadata depth overflow"))?});}
        if let Some(forwards)=self.forwards.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Vec<Mutation>>(),release_bytes:std::alloc::Layout::array::<Mutation>(forwards.capacity()).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"original empty forward backing overflow"))?.size(),depth:1,..Default::default()});}
        if let Some(inverse)=self.inverse.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<semio_framework_value::list::PagedList<Mutation,{usize::MAX}>>(),release_bytes:inverse.next_release_allocation_bytes().map_err(ValueError::from)?,depth:1,..Default::default()});}
        Ok(Default::default())
    }
}

impl<Mutation:Send+'static> ErasedSnapshotRetirement for ArtifactStoreEditRetirement<Mutation>{
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let zero=RetainedCloneProgress::default();if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(zero));}let demand=self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(zero));}
        if let Some(state)=self.state.as_mut(){if state.terminal_is_empty(){drop(self.state.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..zero}));}let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let step=state.step(child)?;semio_framework_value::retained_clone::admit_retained_clone_close(child,step,state.terminal_is_empty(),"original edit native metadata")?;return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.forwards.is_some(){drop(self.forwards.take());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..zero}));}
        if let Some(inverse)=self.inverse.as_mut(){if inverse.terminal_is_empty(){drop(self.inverse.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..zero}));}let progress=inverse.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.progressed),released_bytes:progress.released_allocation_bytes,..zero}));}
        Ok(RetainedCloneStep::Complete(zero))
    }
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    fn terminal_is_empty(&self)->bool{self.state.is_none()&&self.forwards.is_none()&&self.inverse.is_none()}
}

impl<Mutation> Drop for ArtifactStoreEditRetirement<Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.state.is_none() && self.forwards.is_none() && self.inverse.is_none()), "artifact store edit retirement reached Drop before every original typed owner and backing allocation closed");
        if self.state.is_none() && self.forwards.is_none() && self.inverse.is_none() { unsafe { std::mem::ManuallyDrop::drop(&mut self.state); std::mem::ManuallyDrop::drop(&mut self.forwards); std::mem::ManuallyDrop::drop(&mut self.inverse); } }
    }
}

pub(crate) struct ArtifactStoreHistoryMetadataRetirement {
    state: std::mem::ManuallyDrop<Option<ArtifactStoreHistoryMetadataRetirementState>>,
}

pub(crate) struct ArtifactStoreHistoryMetadataRetirementState {
    strings: [Option<String>; 8],
    string_list: Vec<String>,
    authors: Vec<Author>,
    pins: Vec<crate::os_vcs::CompositionPin>,
    active_bytes: Option<Vec<u8>>,
}

impl ArtifactStoreHistoryMetadataRetirement {
    pub(crate) fn change(change: Change) -> Self {
        let Change { id, edit_ids, description, saved_at } = change;
        Self::from_parts([Some(id), description, Some(saved_at), None, None, None, None, None], edit_ids, Vec::new(), Vec::new())
    }

    pub(crate) fn checkpoint(checkpoint: Checkpoint) -> Self {
        let Checkpoint { id, change_ids, parent_id, authors, message, timestamp, composition_pins } = checkpoint;
        Self::from_parts([Some(id), parent_id, message, Some(timestamp), None, None, None, None], change_ids, authors, composition_pins)
    }

    pub(crate) fn alternative(alternative: Alternative) -> Self {
        let Alternative { id, name, checkpoint_ids } = alternative;
        Self::from_parts([Some(id), Some(name), None, None, None, None, None, None], checkpoint_ids, Vec::new(), Vec::new())
    }

    fn from_parts(strings: [Option<String>; 8], string_list: Vec<String>, authors: Vec<Author>, pins: Vec<crate::os_vcs::CompositionPin>) -> Self {
        Self { state: std::mem::ManuallyDrop::new(Some(ArtifactStoreHistoryMetadataRetirementState { strings, string_list, authors, pins, active_bytes: None })) }
    }
}

impl ArtifactStoreHistoryMetadataRetirement {
    fn retained_backing_bytes(state: &ArtifactStoreHistoryMetadataRetirementState) -> Result<usize, ValueError> {
        let mut bytes = 0usize;
        for extent in [state.string_list.capacity().checked_mul(size_of::<String>()), state.authors.capacity().checked_mul(size_of::<Author>()), state.pins.capacity().checked_mul(size_of::<crate::os_vcs::CompositionPin>())] {
            bytes = extent.and_then(|extent| bytes.checked_add(extent)).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history metadata backing extent overflow"))?;
        }
        Ok(bytes)
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreHistoryMetadataRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        let Some(state) = self.state.as_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history metadata requires admitted depth")); }
        if let Some(bytes) = state.active_bytes.as_ref() {
            let released_bytes = bytes.capacity();
            if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
            drop(state.active_bytes.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }));
        }
        if let Some(value) = state.strings.iter_mut().rev().find_map(Option::take).or_else(|| state.string_list.pop()) {
            state.active_bytes = Some(value.into_bytes());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(author) = state.authors.pop() {
            state.strings[0] = Some(author.id);
            state.strings[1] = Some(author.name);
            state.strings[2] = author.avatar;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if let Some(pin) = state.pins.pop() {
            state.strings[0] = Some(pin.child_ref.artifact_id);
            state.strings[1] = Some(pin.child_ref.dialect.artifact_kind);
            state.strings[2] = Some(pin.child_ref.dialect.standard);
            state.strings[3] = Some(pin.child_ref.dialect.subset);
            state.strings[4] = Some(pin.checkpoint_id);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let released_bytes = Self::retained_backing_bytes(state)?;
        if grant.maximum_release_bytes < released_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(self.state.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.state.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        let Some(state) = self.state.as_ref() else { return Ok(0); };
        if let Some(bytes) = state.active_bytes.as_ref() { return Ok(bytes.capacity()); }
        if state.strings.iter().any(Option::is_some) || !state.string_list.is_empty() || !state.authors.is_empty() || !state.pins.is_empty() { return Ok(0); }
        Self::retained_backing_bytes(state)
    }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.state.is_some())) }
}

impl Drop for ArtifactStoreHistoryMetadataRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.state.is_none()), "artifact store history metadata retirement reached Drop before every nested owner was terminal-empty");
    }
}

pub(crate) struct ArtifactStoreMutationDagRetirement {
    dag: std::mem::ManuallyDrop<Option<crate::os_spr::MutationDag>>,
    envelope: std::mem::ManuallyDrop<Option<ArtifactStoreConflictRetirement>>,
    identity: std::mem::ManuallyDrop<Option<ArtifactStoreStringRetirement>>,
}

impl ArtifactStoreMutationDagRetirement {
    pub(crate) fn new(dag: crate::os_spr::MutationDag) -> Self {
        Self { dag: std::mem::ManuallyDrop::new(Some(dag)), envelope: std::mem::ManuallyDrop::new(None), identity: std::mem::ManuallyDrop::new(None) }
    }
    fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.envelope.is_some() { return artifact_retirement_owner_demands(&self.envelope, copy); }
        if self.identity.is_some() { return artifact_retirement_owner_demands(&self.identity, copy); }
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.dag.as_ref().map_or(0, |dag| dag.next_backing_release_byte_demand()), depth: usize::from(self.dag.is_some()), ..Default::default() })
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreMutationDagRetirement {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth < self.demands(grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "mutation DAG exceeds admitted original owner depth")); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.envelope.is_some() { return artifact_retirement_owner_close(&mut self.envelope, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if self.identity.is_some() { return artifact_retirement_owner_close(&mut self.identity, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        let dag = self.dag.as_mut().unwrap();
        if let Some(owner) = dag.take_one_close_owner() {
            match owner {
                crate::os_spr::MutationDagCloseOwner::Envelope(envelope) => *self.envelope = Some(ArtifactStoreConflictRetirement::causal_envelope(envelope)),
                crate::os_spr::MutationDagCloseOwner::Identity(id) => *self.identity = Some(ArtifactStoreStringRetirement::new(id)),
            }
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if !dag.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "mutation DAG lost an exact owner before terminal retirement")); }
        if !dag.backing_is_empty() {
            let (copied_items, released_bytes, _) = dag.close_backing_step(1, grant.maximum_release_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items, released_bytes, ..empty }));
        }
        drop(self.dag.take());
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.dag.is_none() && self.envelope.is_none() && self.identity.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl Drop for ArtifactStoreMutationDagRetirement {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "mutation DAG retirement reached Drop before every exact nested owner was terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.dag); std::mem::ManuallyDrop::drop(&mut self.envelope); std::mem::ManuallyDrop::drop(&mut self.identity); } }
    }
}

pub(crate) enum ArtifactStoreVcsRetirementPhase {
    Mutations,
    Edits,
    Changes,
    Checkpoints,
    Alternatives,
    InitialSnapshot,
    Complete,
}

/// 📏️ Observes every current child demand and the separate physical cursor-shell release.
pub fn artifact_retirement_box_demands(owner: &Box<dyn ErasedSnapshotRetirement>, maximum_body_bytes: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    if owner.terminal_is_empty() { return Ok(semio_framework_value::RetirementDemand { release_bytes: size_of_val(owner.as_ref()), depth: 1, ..Default::default() }); }
    let mut demand = owner.next_demand(maximum_body_bytes)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "boxed retirement depth overflow"))?;
    Ok(demand)
}

/// 🪆️ Quotes only the depth axis of the boxed turn, so a depth admission never re-derives the other three currencies.
pub(crate) fn artifact_retirement_box_depth(owner: &Box<dyn ErasedSnapshotRetirement>) -> Result<usize, ValueError> {
    if owner.terminal_is_empty() { return Ok(1); }
    owner.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "boxed retirement depth overflow"))
}

/// 📦️ Retires a current nested payload and its original box in separate fully granted turns.
pub fn artifact_retirement_box_close_step(slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_close};
    let empty = RetainedCloneProgress::default();
    let Some(owner) = slot.as_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
    if grant.maximum_depth < artifact_retirement_box_depth(owner)? { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "boxed retirement exceeds admitted depth")); }
    if owner.terminal_is_empty() {
        let extent = size_of_val(owner.as_ref());
        if grant.maximum_release_bytes < extent { return Ok(RetainedCloneStep::Progress(empty)); }
        drop(slot.take());
        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: extent, ..empty }));
    }
    let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
    let step = owner.close_step(child_grant)?;
    admit_retained_clone_close(child_grant, step, owner.terminal_is_empty(), "boxed retirement child")?;
    Ok(RetainedCloneStep::Progress(step.progress()))
}

/// 🎫️ Prices a current original typed owner before its controlled child frame is born.
pub fn artifact_retirement_owned_birth_demands<T: semio_framework_value::retirement::RetireOwned>(pending: &Option<T>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    if pending.is_none() { return Ok(Default::default()); }
    if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "typed retirement has no controlled owner authority")); }
    Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::owned_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
}

/// 📋️ Admits the complete original derived snapshot and registry under unchanged caller authority.
pub fn admit_original_snapshot_alias<P,Mu>(original:&mut Option<ArtifactDerivedSnapshot<P>>,snapshots:&Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:&Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>,grant:RetainedCloneGrant)->Result<Option<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress)>,ValueError>
where P:Send+Sync+'static,Mu:Mutation<P>+Send+'static {
    history_read_admission::admit_history_read_retirement(original,snapshots,mutations,grant,|owner,original|*owner.derived=Some(original))
}

/// 🧳️ Admits a typed child without losing the exact original pending owner on refusal.
pub fn artifact_retirement_admit_owned<T: semio_framework_value::retirement::RetireOwned>(pending: &mut Option<T>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let empty = RetainedCloneProgress::default();
    if active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "typed admission requires an empty original child slot")); }
    if pending.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
    let demand = artifact_retirement_owned_birth_demands(pending)?;
    if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "typed child birth exceeds admitted parent depth")); }
    if grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
    let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
    let original = pending.take().expect("typed birth retains its observed original owner");
    match semio_framework_value::retirement::admit_owned_retirement(original, child) {
        Ok((owner, progress)) => {
            *active = Some(owner);
            semio_framework_value::retained_clone::admit_retained_clone_progress(child, progress, "typed child birth")?;
            if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "typed child birth receipt differs from its exact current frame quote")); }
            Ok(RetainedCloneStep::Progress(progress))
        }
        Err((error, original)) => { *pending = Some(original); Err(error) }
    }
}

/// 🔗️ Prices the lease frame of a shared root the holder never owns uniquely: only the last alias pays the deep release.
pub fn artifact_retirement_shared_lease_birth_demands<T: semio_framework_value::retirement::RetireOwned + Sync>(pending: &Option<Arc<T>>) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    if pending.is_none() { return Ok(Default::default()); }
    if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "shared lease has no controlled owner authority")); }
    Ok(semio_framework_value::RetirementDemand { capacity_bytes: semio_framework_value::retirement::shared::shared_retirement_birth_bytes::<T>(), depth: 2, ..Default::default() })
}

/// 🔗️ Admits a shared root as a lease without losing the exact original alias on refusal.
pub fn artifact_retirement_admit_shared_lease<T: semio_framework_value::retirement::RetireOwned + Sync>(pending: &mut Option<Arc<T>>, active: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let empty = RetainedCloneProgress::default();
    if active.is_some() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "lease admission requires an empty original child slot")); }
    if pending.is_none() { return Ok(RetainedCloneStep::Complete(empty)); }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
    let demand = artifact_retirement_shared_lease_birth_demands(pending)?;
    if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "lease birth exceeds admitted parent depth")); }
    if grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
    let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
    match semio_framework_value::retirement::shared::admit_shared_retirement(pending.take().expect("lease birth retains its observed original"), child, true) {
        Ok((owner, progress)) => { *active = Some(owner); Ok(RetainedCloneStep::Progress(progress)) }
        Err((error, original)) => { *pending = Some(original); Err(error) }
    }
}

/// 🪆️ Observes an inline retained child's fallible demands with its logical parent depth.
pub fn artifact_retirement_owner_demands<O: ErasedSnapshotRetirement>(owner: &Option<O>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let Some(owner) = owner.as_ref() else { return Ok(Default::default()); };
    let mut demand = owner.next_demand(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement depth overflow"))?;
    Ok(demand)
}

/// 🧷️ Closes an inline owner under the original grant and removes it only after its exact terminal witness.
pub fn artifact_retirement_owner_close<O: ErasedSnapshotRetirement>(slot: &mut Option<O>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneStep, admit_retained_clone_close};
    if slot.is_none() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth < artifact_retirement_owner_demands(slot, grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement exceeds admitted depth")); }
    let owner = slot.as_mut().unwrap();
    let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
    let step = owner.close_step(child_grant)?;
    admit_retained_clone_close(child_grant, step, owner.terminal_is_empty(), "inline retirement child")?;
    if matches!(step, RetainedCloneStep::Complete(_)) { drop(slot.take()); }
    Ok(step)
}

pub(crate) struct ArtifactStoreHistoryLedgerRetirement<T> {
    ledger: std::mem::ManuallyDrop<Option<ArtifactHistoryLedger<T>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pending: std::mem::ManuallyDrop<Option<T>>,
    visibility: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<crate::os_vcs::ArtifactGroupVisibility>>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>>,
    closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<T: Send + 'static> ArtifactStoreHistoryLedgerRetirement<T> {
    pub(crate) fn new(ledger: ArtifactHistoryLedger<T>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<T>>) -> Self {
        Self { ledger: std::mem::ManuallyDrop::new(Some(ledger)), active: std::mem::ManuallyDrop::new(None), pending: std::mem::ManuallyDrop::new(None), visibility: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }
    fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        if self.visibility.is_some() { return artifact_retirement_owner_demands(&self.visibility, copy); }
        if let Some(ledger) = self.ledger.as_ref() {
            if let Some(value) = self.pending.as_ref().or_else(|| ledger.last()) { return Ok(semio_framework_value::RetirementDemand { capacity_bytes: self.factory.as_ref().unwrap().retirement_birth_bytes(value), depth: 2, ..Default::default() }); }
            if !ledger.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger retains an unresolved original group or reservation")); }
            if ledger.retained_terminal_slot_count() != 0 { ledger.next_terminal_slot_visibility()?; return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            return Ok(semio_framework_value::RetirementDemand { release_bytes: ledger.next_empty_page_release_byte_demand()?, depth: 1, ..Default::default() });
        }
        if self.factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(factory) = self.closing_factory.as_ref() {
            let mut demand = factory.demands(copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history factory capability depth overflow"))?;
            return Ok(demand);
        }
        Ok(Default::default())
    }
}

impl<T: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreHistoryLedgerRetirement<T> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, admit_retained_clone_progress};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history ledger exceeds admitted original owner depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.visibility.is_some() { return artifact_retirement_owner_close(&mut self.visibility, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(ledger) = self.ledger.as_mut() {
            if let Some(value) = self.pending.take().or_else(|| ledger.pop()) {
                let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
                return match self.factory.as_ref().unwrap().retire_owned(value, child) {
                    Ok((owner, progress)) => { *self.active = Some(owner); admit_retained_clone_progress(child, progress, "original history entry constructor")?; if progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history entry factory changed its exact declared birth")); } Ok(RetainedCloneStep::Progress(progress)) }
                    Err((error, value)) => { *self.pending = Some(value); Err(error) }
                };
            }
            if ledger.retained_terminal_slot_count() != 0 {
                let (visibility, progress) = ledger.retire_terminal_slot(grant)?;
                *self.visibility = visibility.map(semio_framework_value::retirement::shared::SharedControlledRetirement::lease);
                return Ok(RetainedCloneStep::Progress(progress));
            }
            if !ledger.backing_is_empty() { return ledger.release_empty_page(grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
            drop(self.ledger.take());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        if self.factory.is_some() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.factory.take().unwrap();
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let factory = self.closing_factory.as_mut().unwrap();
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        let step = factory.step(child)?;
        semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original history factory capability")?;
        if factory.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<T> Drop for ArtifactStoreHistoryLedgerRetirement<T> {
    fn drop(&mut self) {
        let empty = self.ledger.is_none() && self.active.is_none() && self.pending.is_none() && self.visibility.is_none() && self.factory.is_none() && self.closing_factory.is_none();
        assert!(std::thread::panicking() || empty, "original history ledger retains its payloads, visibility, native backing and factory until exact terminal");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.ledger); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.visibility); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}

pub(crate) struct ArtifactStoreVcsRetirement<P, Mutation> {
    pub(crate) vcs: std::mem::ManuallyDrop<Option<ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>>>,
    pub(crate) active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub(crate) phase: ArtifactStoreVcsRetirementPhase,
    pub(crate) initial_snapshot_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub(crate) mutation_factory: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>>>,
    pub(crate) closing_factories: std::mem::ManuallyDrop<[Option<semio_framework_value::FactoryAuthority>; 2]>,
}

/// 🧾️ Retains the actual native forward or inverse collection without rebuilding its scaffold.
pub(crate) enum ArtifactStoreOperationRowOwners<Mu> {
    Forwards(Vec<Mu>),
    Inverse(semio_framework_value::list::PagedList<Mu, {usize::MAX}>),
}

impl<Mu> From<Vec<Mu>> for ArtifactStoreOperationRowOwners<Mu> {
    fn from(rows: Vec<Mu>) -> Self { Self::Forwards(rows) }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct ArtifactStoreCheckpointRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Checkpoint> for ArtifactStoreCheckpointRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Checkpoint) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Checkpoint, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Checkpoint)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::checkpoint)
    }
}

#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub(crate) struct ArtifactStoreAlternativeRetirementFactory;

impl ArtifactOwnedValueRetirementFactory<Alternative> for ArtifactStoreAlternativeRetirementFactory {
    fn retirement_birth_bytes(&self, _value: &Alternative) -> usize { size_of::<ArtifactStoreHistoryMetadataRetirement>() }
    fn retire_owned(&self, value: Alternative, grant: RetainedCloneGrant) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, Alternative)> {
        admit_artifact_retirement(value, grant, ArtifactStoreHistoryMetadataRetirement::alternative)
    }
}

pub(crate) fn artifact_history_ledger_birth<T: Send + 'static, F: semio_framework_value::FactoryPayloadRetirement>() -> Result<usize, ValueError> {
    size_of::<ArtifactStoreHistoryLedgerRetirement<T>>().checked_add(semio_framework_value::factory_arc_birth_bytes::<F>()).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history owner constructor extent overflow"))
}

pub(crate) fn artifact_history_ledger_handoff<T: Send + 'static, F: ArtifactOwnedValueRetirementFactory<T> + semio_framework_value::FactoryPayloadRetirement>(ledger: &mut ArtifactHistoryLedger<T>, factory: F, slot: &mut Option<Box<dyn ErasedSnapshotRetirement>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
    let capacity = artifact_history_ledger_birth::<T, F>()?;
    if grant.maximum_items == 0 || grant.maximum_capacity_bytes < capacity { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth < 2 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history owner constructor requires original child depth")); }
    let original = std::mem::replace(ledger, ArtifactHistoryLedger::empty());
    *slot = Some(Box::new(ArtifactStoreHistoryLedgerRetirement::new(original, Arc::new(factory))));
    Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: capacity, ..Default::default() }))
}

impl<P, Mutation> ArtifactStoreVcsRetirement<P, Mutation> {
    pub(crate) fn new(vcs: ArtifactVcs<AdmittedArtifactGenesis<P>, Mutation>, initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        Self { vcs: std::mem::ManuallyDrop::new(Some(vcs)), active: std::mem::ManuallyDrop::new(None), phase: ArtifactStoreVcsRetirementPhase::Mutations, initial_snapshot_factory: std::mem::ManuallyDrop::new(Some(initial_snapshot_factory)), mutation_factory: std::mem::ManuallyDrop::new(Some(mutation_factory)), closing_factories: std::mem::ManuallyDrop::new([None, None]) }
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreVcsRetirement<P, Mutation> {
    pub(crate) fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, copy); }
        let Some(vcs) = self.vcs.as_ref() else {
            if self.initial_snapshot_factory.is_some() || self.mutation_factory.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
            if let Some(factory) = self.closing_factories.iter().flatten().next() { let mut demand = factory.demands(copy)?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS factory depth overflow"))?; return Ok(demand); }
            return Ok(Default::default());
        };
        let capacity_bytes = match self.phase {
            ArtifactStoreVcsRetirementPhase::Edits if !vcs.edits.backing_is_empty() || !vcs.edits.terminal_is_empty() => artifact_history_ledger_birth::<Edit<Mutation>, ArtifactStoreDecodedEditRetirementFactory<Mutation>>()?,
            ArtifactStoreVcsRetirementPhase::Changes if !vcs.changes.backing_is_empty() || !vcs.changes.terminal_is_empty() => artifact_history_ledger_birth::<Change, ArtifactStoreChangeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Checkpoints if !vcs.checkpoints.backing_is_empty() || !vcs.checkpoints.terminal_is_empty() => artifact_history_ledger_birth::<Checkpoint, ArtifactStoreCheckpointRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::Alternatives if !vcs.alternatives.backing_is_empty() || !vcs.alternatives.terminal_is_empty() => artifact_history_ledger_birth::<Alternative, ArtifactStoreAlternativeRetirementFactory>()?,
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => size_of::<ArtifactGenesisRetirement<P>>(),
            _ => 0,
        };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: if capacity_bytes == 0 { 1 } else { 2 }, ..Default::default() })
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreVcsRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "VCS retirement exceeds admitted original depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let grant = RetainedCloneGrant { maximum_items: 1, ..grant };
        if self.active.is_some() { return artifact_retirement_box_close_step(&mut self.active, grant); }
        if self.vcs.is_none() {
            if self.initial_snapshot_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.initial_snapshot_factory.take().unwrap(); self.closing_factories[0] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            if self.mutation_factory.is_some() { let factory: Arc<dyn semio_framework_value::FactoryRetirement> = self.mutation_factory.take().unwrap(); self.closing_factories[1] = Some(semio_framework_value::FactoryAuthority::new(factory)); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty })); }
            let slot = self.closing_factories.iter_mut().find(|slot| slot.is_some()).unwrap();
            let factory = slot.as_mut().unwrap();
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = factory.step(child)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "original VCS factory capability")?;
            if factory.terminal_is_empty() { drop(slot.take()); }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        let vcs = self.vcs.as_mut().unwrap();
        match self.phase {
            ArtifactStoreVcsRetirementPhase::Mutations => self.phase = ArtifactStoreVcsRetirementPhase::Edits,
            ArtifactStoreVcsRetirementPhase::Edits => {
                self.phase = ArtifactStoreVcsRetirementPhase::Changes;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.edits, ArtifactStoreDecodedEditRetirementFactory { mutation_factory: self.mutation_factory.as_ref().unwrap().clone() }, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Changes => {
                self.phase = ArtifactStoreVcsRetirementPhase::Checkpoints;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.changes, ArtifactStoreChangeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Checkpoints => {
                self.phase = ArtifactStoreVcsRetirementPhase::Alternatives;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.checkpoints, ArtifactStoreCheckpointRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::Alternatives => {
                self.phase = ArtifactStoreVcsRetirementPhase::InitialSnapshot;
                if demand.capacity_bytes != 0 { return artifact_history_ledger_handoff(&mut vcs.alternatives, ArtifactStoreAlternativeRetirementFactory, &mut self.active, grant); }
            }
            ArtifactStoreVcsRetirementPhase::InitialSnapshot => {
                if !(vcs.edits.terminal_is_empty() && vcs.edits.backing_is_empty() && vcs.changes.terminal_is_empty() && vcs.changes.backing_is_empty() && vcs.checkpoints.terminal_is_empty() && vcs.checkpoints.backing_is_empty() && vcs.alternatives.terminal_is_empty() && vcs.alternatives.backing_is_empty()) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "VCS genesis handoff still retains original history backing")); }
                let ArtifactVcs { genesis, edits, changes, checkpoints, alternatives } = self.vcs.take().unwrap();
                drop((edits, changes, checkpoints, alternatives));
                *self.active = Some(Box::new(ArtifactGenesisRetirement::new(genesis, self.initial_snapshot_factory.as_ref().unwrap().clone())));
                self.phase = ArtifactStoreVcsRetirementPhase::Complete;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..empty }));
            }
            ArtifactStoreVcsRetirementPhase::Complete => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "completed VCS phase still retains its original VCS")),
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }))
    }
    fn terminal_is_empty(&self) -> bool { self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreVcsRetirement<P, Mutation> {
    fn drop(&mut self) {
        let empty = self.vcs.is_none() && self.active.is_none() && self.initial_snapshot_factory.is_none() && self.mutation_factory.is_none() && self.closing_factories.iter().all(Option::is_none);
        assert!(std::thread::panicking() || empty, "VCS retirement reached Drop before every exact history backing and factory became terminal-empty");
        if empty { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.initial_snapshot_factory); std::mem::ManuallyDrop::drop(&mut self.mutation_factory); std::mem::ManuallyDrop::drop(&mut self.closing_factories); } }
    }
}

/// 📏️ Quotes the exact cursor frame [`retire_document_envelope`] births before any original field or factory transfers.
pub fn retire_document_envelope_birth_bytes<P: Send + Sync + 'static, Mutation: Send + 'static>() -> usize {
    size_of::<ArtifactStoreEnvelopeRetirement<P, Mutation>>()
}

/// ♻️ Admits whole detached document custody before any original field or factory transfers.
pub fn retire_document_envelope<P: Send + Sync + 'static, Mutation: Send + 'static>(
    envelope: ArtifactEnvelope<P, Mutation>,
    initial_snapshot_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    mutation_factory: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>,
    grant: RetainedCloneGrant,
) -> Result<(Box<dyn ErasedSnapshotRetirement>, RetainedCloneProgress), (ValueError, ArtifactEnvelope<P, Mutation>, Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>)> {
    admit_artifact_retirement((envelope, initial_snapshot_factory, mutation_factory), grant, |(envelope, snapshots, mutations)| ArtifactStoreEnvelopeRetirement::new(envelope, snapshots, mutations))
        .map_err(|(error, (envelope, snapshots, mutations))| (error, envelope, snapshots, mutations))
}

pub(crate) struct ArtifactEnvelopeMetadata {
    pub(crate) schema: String,
    pub(crate) id: String,
    pub(crate) backbone: Option<ArtifactBackboneRef>,
    pub(crate) active_alternative_id: Option<String>,
    pub(crate) cursor: Option<ArtifactCursor>,
    pub(crate) dialect: Option<semio_framework_artifact_reference::ArtifactDialect>,
    pub(crate) migrated_from: Option<MigrationProvenance>,
    pub(crate) owner: Option<OwnerRef>,
    pub(crate) lanes: protocol::HistoryFoldIndex<String, HistoryLane>,
    pub(crate) viewer_checkpoint_id: Option<String>,
    pub(crate) conflicts: Vec<crate::os_spr::Conflict>,
    pub(crate) transitions: Vec<crate::os_spr::MutationEnvelope>,
    pub(crate) open_transaction: Option<OpenToolTransaction>,
}

pub(crate) struct ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub(crate) vcs: std::mem::ManuallyDrop<Option<ArtifactStoreVcsRetirement<P, Mutation>>>,
    pub(crate) messages: std::mem::ManuallyDrop<Option<ArtifactEditMessageLedgerRetirement>>,
    pub(crate) metadata: std::mem::ManuallyDrop<Option<ArtifactEnvelopeMetadata>>,
    pub(crate) active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
}

impl<P, Mutation> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    pub(crate) fn new(envelope: ArtifactEnvelope<P, Mutation>, snapshots: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>, mutations: Arc<dyn ArtifactOwnedValueRetirementFactory<Mutation>>) -> Self {
        let ArtifactEnvelopeOwners { schema, id, vcs, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, edit_messages, conflicts, transitions, history_shape: _, open_transaction } = envelope.into_owners();
        Self {
            vcs: std::mem::ManuallyDrop::new(Some(ArtifactStoreVcsRetirement::new(vcs, snapshots, mutations))),
            messages: std::mem::ManuallyDrop::new(Some(ArtifactEditMessageLedgerRetirement::new(edit_messages))),
            metadata: std::mem::ManuallyDrop::new(Some(ArtifactEnvelopeMetadata { schema, id, backbone, active_alternative_id, cursor, dialect, migrated_from, owner, lanes, viewer_checkpoint_id, conflicts, transitions, open_transaction })),
            active: std::mem::ManuallyDrop::new(None),
        }
    }
    fn ownership_is_empty(&self) -> bool { self.vcs.is_none() && self.messages.is_none() && self.metadata.is_none() && self.active.is_none() }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.active.as_ref() { return artifact_retirement_box_demands(active, body); }
        if self.vcs.is_some() { return artifact_retirement_owner_demands(&self.vcs, body); }
        if self.messages.is_some() { return artifact_retirement_owner_demands(&self.messages, body); }
        artifact_retirement_owned_birth_demands(&self.metadata)
    }
}

impl<P: Send + Sync + 'static, Mutation: Send + 'static> ErasedSnapshotRetirement for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.ownership_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let step = if self.active.is_some() { artifact_retirement_box_close_step(&mut self.active, grant) }
            else if self.vcs.is_some() { artifact_retirement_owner_close(&mut self.vcs, grant) }
            else if self.messages.is_some() { artifact_retirement_owner_close(&mut self.messages, grant) }
            else { artifact_retirement_admit_owned(&mut self.metadata, &mut self.active, grant) }?;
        Ok(if self.ownership_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.ownership_is_empty() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Ok(self.demands(body)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P, Mutation> Drop for ArtifactStoreEnvelopeRetirement<P, Mutation> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.ownership_is_empty(), "whole envelope reached Drop before every original VCS, message backing, metadata and factory owner closed");
        if self.ownership_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.vcs); std::mem::ManuallyDrop::drop(&mut self.messages); std::mem::ManuallyDrop::drop(&mut self.metadata); std::mem::ManuallyDrop::drop(&mut self.active); } }
    }
}

pub(crate) struct ArtifactGenesisRetirement<P: Send + Sync + 'static> {
    snapshot: std::mem::ManuallyDrop<Option<ReturnedSnapshotReadRetirement<P>>>,
    pack: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::shared::SharedControlledRetirement<Vec<u8>>>>,
}

impl<P: Send + Sync + 'static> ArtifactGenesisRetirement<P> {
    pub(crate) fn new(genesis: AdmittedArtifactGenesis<P>, factory: Arc<dyn ArtifactOwnedValueRetirementFactory<P>>) -> Self {
        let (snapshot, pack) = genesis.into_owners();
        Self { snapshot: std::mem::ManuallyDrop::new(Some(ReturnedSnapshotReadRetirement::new(snapshot, factory))), pack: std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::shared::SharedControlledRetirement::lease(pack))) }
    }
    fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.snapshot.is_some() { artifact_retirement_owner_demands(&self.snapshot, copy) } else { artifact_retirement_owner_demands(&self.pack, copy) }
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactGenesisRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneStep};
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let grant = RetainedCloneGrant { maximum_items: grant.maximum_items.min(1), ..grant };
        let step = if self.snapshot.is_some() { artifact_retirement_owner_close(&mut self.snapshot, grant)? } else { artifact_retirement_owner_close(&mut self.pack, grant)? };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.snapshot.is_none() && self.pack.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactGenesisRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "genesis retired before both immutable owners became terminal-empty");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.snapshot); std::mem::ManuallyDrop::drop(&mut self.pack); } }
    }
}

/// 📦️ Retains a displaced shared snapshot with its exact factory until a granted turn constructs its retirement frame.
pub struct ArtifactStoreDisplacedSnapshotRetirement<P: Send + Sync + 'static> {
    pending: std::mem::ManuallyDrop<Option<Arc<P>>>,
    active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory: std::mem::ManuallyDrop<Option<Arc<dyn SnapshotRetirementFactory<P>>>>,
    closing_factory: std::mem::ManuallyDrop<Option<semio_framework_value::FactoryAuthority>>,
}

impl<P: Send + Sync + 'static> ArtifactStoreDisplacedSnapshotRetirement<P> {
    pub fn new(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self { pending: std::mem::ManuallyDrop::new(Some(snapshot)), active: std::mem::ManuallyDrop::new(None), factory: std::mem::ManuallyDrop::new(Some(factory)), closing_factory: std::mem::ManuallyDrop::new(None) }
    }

    pub fn boxed(snapshot: Arc<P>, factory: Arc<dyn SnapshotRetirementFactory<P>>) -> Box<dyn ErasedSnapshotRetirement> { Box::new(Self::new(snapshot, factory)) }

    fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        let nested = |mut demand: RetirementDemand| -> Result<RetirementDemand, ValueError> { demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot depth overflow"))?; Ok(demand) };
        if let Some(snapshot) = self.pending.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.factory.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot lost its original factory"))?.retirement_birth_bytes(snapshot), depth: 2, ..Default::default() }); }
        if let Some(active) = self.active.as_ref() { return nested(semio_framework_value::factory_ticket_demands(active, copy)?); }
        if self.factory.is_some() { return Ok(RetirementDemand { depth: 1, ..Default::default() }); }
        if let Some(authority) = self.closing_factory.as_ref() { return nested(authority.demands(copy)?); }
        Ok(Default::default())
    }
}

impl<P: Send + Sync + 'static> ErasedSnapshotRetirement for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::admit_retained_clone_close;
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced snapshot exceeds admitted depth")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(snapshot) = self.pending.take() {
            return match self.factory.as_ref().expect("pending displaced snapshot retains its original factory").retire(snapshot, child) {
                Ok((owner, progress)) => {
                    *self.active = Some(owner);
                    if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced snapshot constructor changed its admitted receipt")); }
                    Ok(RetainedCloneStep::Progress(progress))
                }
                Err((error, snapshot)) => { *self.pending = Some(snapshot); Err(error) }
            };
        }
        if self.active.is_some() { return semio_framework_value::close_factory_ticket(&mut self.active, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(factory) = self.factory.take() {
            *self.closing_factory = Some(semio_framework_value::FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..empty }));
        }
        let authority = self.closing_factory.as_mut().expect("displaced snapshot retains its closing factory");
        let step = authority.step(child)?;
        let step = admit_retained_clone_close(child, step, authority.terminal_is_empty(), "displaced snapshot factory capability")?;
        if authority.terminal_is_empty() { drop(self.closing_factory.take()); }
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }
    fn terminal_is_empty(&self) -> bool { self.pending.is_none() && self.active.is_none() && self.factory.is_none() && self.closing_factory.is_none() }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { Ok(self.demands(copy)?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.demands(0)?.depth) }
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { self.demands(body) }
}

impl<P: Send + Sync + 'static> Drop for ArtifactStoreDisplacedSnapshotRetirement<P> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "displaced snapshot retirement abandoned its original snapshot or factory capability");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.pending); std::mem::ManuallyDrop::drop(&mut self.active); std::mem::ManuallyDrop::drop(&mut self.factory); std::mem::ManuallyDrop::drop(&mut self.closing_factory); } }
    }
}

pub(crate) fn advance_returned_snapshot_read<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: &Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    grant: RetainedCloneGrant,
) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::RetainedCloneStep;
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if active.is_some() { return artifact_retirement_box_close_step(active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
    if !registry.has_returned() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    match registry.try_admit_one_returned::<P, _>(grant, |root, grant| match ReturnedSnapshotReadRetirement::admit(root, factory.clone(), grant) { Ok(admitted) => Ok(admitted), Err((error, root, alias)) => { drop(alias); Err((error, root)) } }) {
        Ok((owner, receipt)) => { *active = owner; if !receipt.fits(grant) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned read admission exceeded its original grant")); } Ok(RetainedCloneStep::Progress(receipt)) },
        Err(SnapshotReadLeaseRefusal::Busy) => Ok(RetainedCloneStep::Progress(Default::default())),
        Err(reason) => Err(reason.into_value_error()),
    }
}

pub(crate) fn advance_returned_snapshot_read_root<P: Send + Sync + 'static>(
    registry: &SnapshotReadLeaseRegistry,
    active: &mut Option<Box<dyn ErasedSnapshotRetirement>>,
    factory: &dyn SnapshotRetirementFactory<P>,
    grant: RetainedCloneGrant,
) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::RetainedCloneStep;
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if active.is_some() { return artifact_retirement_box_close_step(active, grant).map(|step| RetainedCloneStep::Progress(step.progress())); }
    if !registry.has_returned() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    match registry.try_admit_one_returned::<P, _>(grant, |root, grant| factory.retire(root, grant)) {
        Ok((owner, receipt)) => { *active = owner; if !receipt.fits(grant) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "returned read admission exceeded its original grant")); } Ok(RetainedCloneStep::Progress(receipt)) },
        Err(SnapshotReadLeaseRefusal::Busy) => Ok(RetainedCloneStep::Progress(Default::default())),
        Err(reason) => Err(reason.into_value_error()),
    }
}

pub(crate) const ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY: usize = 1_024;

pub(crate) const ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY: usize = 8;

/// 🌡️ One-item retirement steps a change spends on the displaced queue while it is under pressure (`relieve_displaced_pressure`).
pub(crate) const DISPLACED_RELIEF_STEPS: usize = 16;

/// 🌡️ Displaced-owner occupancy at which a store reports maintenance PRESSURE — a quarter of the
/// fixed queue. Every commit displaces its envelope, current snapshot and dag (three owners, more
/// with a tail undo entry) and the round-robin maintenance pump drains ONE owner step per visit, so
/// a live cadence of coalesced commits (a playback tick, a gumball drag step) fills the queue a
/// hundred times faster than the fair rotation empties it. Above this mark the host drains the
/// pressured queue out of turn and in bursts until it is back under.
pub const ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY: usize = ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY / 4;

pub(crate) struct ArtifactStoreDisplacedRetirements {
    pub(crate) owners: std::mem::ManuallyDrop<VecDeque<Box<dyn ErasedSnapshotRetirement>>>,
    owner_reservations: [Option<(u64, usize)>; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY],
    owner_generation: u64,
}

pub(crate) struct ArtifactStoreDisplacedOwnerReservation {
    slot: u8,
    generation: u64,
    remaining: usize,
}

impl ArtifactStoreDisplacedRetirements {
    pub(crate) fn new() -> Self {
        Self { owners: std::mem::ManuallyDrop::new(VecDeque::with_capacity(ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY)), owner_reservations: [None; ARTIFACT_STORE_DISPLACED_RESERVATION_CAPACITY], owner_generation: 0 }
    }

    pub(crate) fn reserve(&self, count: usize) -> Result<(), VcsError> {
        let occupied = self.owners.len().checked_add(self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>()).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner occupancy overflowed".into()))?;
        if count > ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(occupied) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner fixed retirement authority is saturated".into()));
        }
        Ok(())
    }

    pub(crate) fn reserve_owner_slots(&mut self, count: usize) -> Result<ArtifactStoreDisplacedOwnerReservation, VcsError> {
        if count == 0 {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is empty".into()));
        }
        let slot = self.owner_reservations.iter().position(Option::is_none).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation registry is saturated".into()))?;
        self.reserve(count)?;
        let generation = self.owner_generation.checked_add(1).ok_or_else(|| VcsError::ValidationFailed("artifact store displaced-owner reservation generation exhausted".into()))?;
        self.owner_generation = generation;
        self.owner_reservations[slot] = Some((generation, count));
        Ok(ArtifactStoreDisplacedOwnerReservation { slot: slot as u8, generation, remaining: count })
    }

    pub(crate) fn push_owner_reserved(&mut self, reservation: &mut ArtifactStoreDisplacedOwnerReservation, owner: Box<dyn ErasedSnapshotRetirement>) -> Result<(), Box<dyn ErasedSnapshotRetirement>> {
        let slot = reservation.slot as usize;
        if reservation.remaining == 0 || self.owner_reservations.get(slot).copied().flatten() != Some((reservation.generation, reservation.remaining)) || self.owners.len() >= ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY {
            return Err(owner);
        }
        reservation.remaining -= 1;
        self.owner_reservations[slot] = (reservation.remaining != 0).then_some((reservation.generation, reservation.remaining));
        self.owners.push_back(owner);
        Ok(())
    }

    pub(crate) fn release_owner_slots(&mut self, reservation: ArtifactStoreDisplacedOwnerReservation) -> Result<(), VcsError> {
        if reservation.remaining == 0 {
            if self.owner_reservations[reservation.slot as usize].is_some_and(|(generation, _)| generation == reservation.generation) {
                return Err(VcsError::ValidationFailed("artifact store consumed displaced-owner reservation retained live credits".into()));
            }
            return Ok(());
        }
        if self.owner_reservations[reservation.slot as usize] != Some((reservation.generation, reservation.remaining)) {
            return Err(VcsError::ValidationFailed("artifact store displaced-owner reservation is stale or partially consumed".into()));
        }
        self.owner_reservations[reservation.slot as usize] = None;
        Ok(())
    }

    pub(crate) fn push_reserved(&mut self, owner: Box<dyn ErasedSnapshotRetirement>) {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        assert!(self.owners.len() < ARTIFACT_STORE_DISPLACED_RETIREMENT_CAPACITY.saturating_sub(reserved), "artifact store displaced-owner admission token was consumed after its fixed slot changed");
        self.owners.push_back(owner);
    }

    pub(crate) fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(owner) = self.owners.front() {
            let mut demand = artifact_retirement_box_demands(owner, copy)?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue retirement depth overflow"))?;
            return Ok(demand);
        }
        if self.owner_reservations.iter().any(Option::is_some) { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "displaced queue retains reserved original owner slots")); }
        Ok(Default::default())
    }

    pub(crate) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        let Some(owner) = self.owners.front_mut() else { return Ok(RetainedCloneStep::Complete(empty)); };
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced queue exceeds admitted original child depth")); }
        if grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes || (demand.copy_bytes != 0 && grant.maximum_copy_bytes == 0) { return Ok(RetainedCloneStep::Progress(empty)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if owner.terminal_is_empty() {
            let bytes = size_of_val(owner.as_ref());
            drop(self.owners.pop_front());
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }));
        }
        let step = owner.close_step(child)?;
        let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, owner.terminal_is_empty(), "original displaced queue child")?;
        if matches!(step, RetainedCloneStep::Complete(_)) && !owner.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced child reported completion while original ownership remains")); }
        Ok(RetainedCloneStep::Progress(step.progress()))
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.owners.is_empty() && self.owner_reservations.iter().all(Option::is_none)
    }

    pub(crate) fn backing_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "displaced backing remains occupied by original owners or reservations")); }
        std::alloc::Layout::array::<Box<dyn ErasedSnapshotRetirement>>(self.owners.capacity()).map(|layout| layout.size()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "displaced original queue backing extent overflow"))
    }

    pub(crate) fn release_empty_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        let bytes = self.backing_release_byte_demand()?;
        if bytes == 0 { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < bytes { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "displaced backing requires original frame depth")); }
        drop(std::mem::take(&mut *self.owners));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..empty }))
    }

    /// 🔭️ Why this queue is not terminal — a queued owner the closer is draining, or a RESERVED
    /// slot no owner ever filled. The two answer `Blocked` from the same arm and need different
    /// cures, so the census names them apart.
    pub(crate) fn witness(&self) -> String {
        let reserved = self.owner_reservations.iter().flatten().map(|(_, count)| *count).sum::<usize>();
        format!("owners={}/reserved={}", self.owners.len(), reserved)
    }

    pub(crate) fn under_pressure(&self) -> bool {
        self.owners.len() >= ARTIFACT_STORE_DISPLACED_PRESSURE_OCCUPANCY
    }
}

impl Drop for ArtifactStoreDisplacedRetirements {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || (self.terminal_is_empty()), "artifact store displaced-owner authority reached Drop before bounded retirement completed");
        let owners = unsafe { std::mem::ManuallyDrop::take(&mut self.owners) };
        drop(owners);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ArtifactStoreCursorDisposerPhase {
    ReturnedReads,
    Displaced,
    RuntimeOwners(u8),
    PrefixSnapshots,
    TailSnapshot,
    CurrentSnapshot,
    Backbone,
    Causal,
    Envelope,
    Complete,
}

pub(crate) const ARTIFACT_STORE_UNSCHEDULED_CLOSE_STEPS: usize = 1 << 20;
