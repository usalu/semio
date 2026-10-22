//! 📖️ Derived snapshots, replays and history read plan: the derived snapshot, replay and history preview, the history read plan and the history read retirement. See [`super`].

use super::*;

/// 🌱️ Immutable projection derived by one Store from its own event history or a provisional operation.
pub struct ArtifactDerivedSnapshot<P> {
    pub owner: Arc<P>,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
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
        pub history_source:Option<semio_framework_os_store_replay_operation::native::history::OriginalHistorySource<semio_framework_os_spr::Edit<Mu>>>,
    pub history_input:Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mu>>,
    pub history_issuer:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<semio_framework_os_spr::Edit<Mu>>>>,
    pub history_issuer_close:Option<semio_framework_value::FactoryAuthority>,
    pub history_operation:Option<(usize,usize)>,
    pub failed_operation:Option<semio_framework_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>,
    pub preparation_receipt:RetainedCloneProgress,
pub replay: Option<EditReplay<P, Mu>>,
    pub plan: Option<HistoryReadPlan>,
    pub plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
}

impl<P: Clone, Mu: Mutation<P> + semio_framework_os_spr::ColdMutation<P> + OpBinary> ArtifactDerivedReplay<P, Mu> {
        /// 🧾️ Exposes the original caller-granted replay work to the mounted history controller.
    pub fn take_preparation_receipt(&mut self)->RetainedCloneProgress{std::mem::take(&mut self.preparation_receipt)}
/// 📈️ Reports genuine cumulative work inside the currently sealed operation producer.
    pub fn operation_preparation_progress(&self) -> ArtifactReplayPreparationProgress {
        self.replay.as_ref().map_or_else(ArtifactReplayPreparationProgress::default, |replay|replay.operation_preparation_progress())
    }

    pub fn retain_completed_plan(&mut self) -> &mut HistoryReadPlan {
        let original=self.plan.take().expect("completed replay planner");
        retain_completed_history_plan(&mut self.plan_retirement,original)
    }

    /// 📏️ Borrows the exact cleanup requirements of the completed planner's original residue.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {retirement_owner_demands(&self.plan_retirement,body)}

    /// ♻️ Closes one original planner residue frontier under the caller's independent grants.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {retirement_owner_close(&mut self.plan_retirement,grant)}
}

/// 🕰️ A generation-fenced cursor resolving a preview's inputs and folding its prefix under the caller's deadline.
pub struct ArtifactDerivedHistoryPreview<P, Mu: Mutation<P>> {
        pub derived_operation:Option<semio_framework_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>,
    pub history_source:Option<semio_framework_os_store_replay_operation::native::history::OriginalHistorySource<semio_framework_os_spr::Edit<Mu>>>,
    pub history_input:Option<semio_framework_value::retained_clone::RetainedOwnedProjection<Mu>>,
    pub history_issuer:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<semio_framework_os_spr::Edit<Mu>>>>,
    pub history_issuer_close:Option<semio_framework_value::FactoryAuthority>,
    pub displaced_operation:Option<Arc<P>>,
    pub operation_close:Option<Box<dyn ErasedSnapshotRetirement>>,
    pub operation_identity:Option<MutationId>,
    pub operation_adopted:bool,
    pub preparation_receipt:RetainedCloneProgress,
pub plan: Option<HistoryReadPlan>,
    pub plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub drafts: protocol::HistoryInputDrafts,
    pub target: Option<(usize, usize)>,
    pub cursor: usize,
    pub operation: usize,
    pub state: Option<Arc<P>>,
    pub progress: ReplayProgress,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
    pub marker: PhantomData<fn() -> Mu>,
    pub finished: bool,
}

impl<P, Mu: Mutation<P>> ArtifactDerivedHistoryPreview<P, Mu> {
    /// 📶️ Work completed while planning or folding the selected prefix.
    pub fn progress(&self) -> ReplayProgress { self.plan.as_ref().map_or(self.progress, |plan| plan.progress) }

        /// 🧾️ Transfers the actual most recent native preparation turn to its mounted caller.
    pub fn take_preparation_receipt(&mut self)->RetainedCloneProgress{std::mem::take(&mut self.preparation_receipt)}
/// 📤️ Transfers the completed projection while retaining every original input and planner residue.
    pub fn take_finished_snapshot_owner(&mut self)->Result<Arc<P>,VcsError> {
        if !self.finished{return Err(VcsError::HistoryReplaying);}
        self.state.take().ok_or(VcsError::HistoryReplaying)
    }

    pub fn retain_completed_plan(&mut self)->usize {
        let original=self.plan.take().expect("completed preview planner");
        let plan=retain_completed_history_plan(&mut self.plan_retirement,original);
        self.target=Some(plan.positions[plan.target.as_ref().expect("preview target")]);
        self.progress=plan.progress;self.progress.total=self.progress.done;
        self.drafts=std::mem::take(&mut plan.drafts);
        plan.first_input
    }

    /// 🧮️ Borrows each independent cleanup demand while preserving the original completed planner.
    pub fn planning_retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {retirement_owner_demands(&self.plan_retirement,body)}

    /// 🧹️ Closes one completed planner frontier without dropping any ungranted backing.
    pub fn retire_completed_plan(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {retirement_owner_close(&mut self.plan_retirement,grant)}
}

impl<P, Mu: Mutation<P>> Drop for ArtifactDerivedHistoryPreview<P, Mu> {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            retire_shared_projection::<P, Mu>(state);
        }
    }
}

pub struct HistoryReadPlan {
    pub target: Option<MutationId>,
    pub drafts: protocol::HistoryInputDrafts,
    pub positions: protocol::HistoryFoldIndex<MutationId, (usize, usize)>,
    pub first_input: usize,
    pub last_draft: Option<usize>,
    pub order: semio_framework_os_vcs::HistoryPageStack<String>,
    pub counts: semio_framework_os_vcs::HistoryPageStack<u32>,
    pub cursor: usize,
    pub located_target: bool,
    pub located_draft: Option<MutationId>,
    pub located_inputs: bool,
    pub operations: u32,
    pub supersessions: EffectiveSupersessions,
    pub copied: Option<MutationId>,
    pub copying_drafts: bool,
    pub report: bool,
    pub progress: ReplayProgress,
    pub finished: bool,
}

impl HistoryReadPlan {
    pub fn new(target: Option<MutationId>, drafts: protocol::HistoryInputDrafts, edits: usize, report: bool) -> Self {
        Self {
            target, drafts, positions: protocol::HistoryFoldIndex::new(), first_input: edits, last_draft: None, order: semio_framework_os_vcs::HistoryPageStack::empty(), counts: semio_framework_os_vcs::HistoryPageStack::empty(), cursor: 0, located_target: false, located_draft: None, located_inputs: false, operations: 0,
            supersessions: EffectiveSupersessions::new(), copied: None, copying_drafts: false, report,
            progress: ReplayProgress { done: 0, total: 0 }, finished: false,
        }
    }
}

pub fn retain_completed_history_plan(slot:&mut Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,original:HistoryReadPlan)->&mut HistoryReadPlan {
    assert!(slot.is_none(),"completed history planner residue remains attached");
    *slot=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner declares controlled retirement")));
    slot.as_mut().unwrap().original_mut().unwrap()
}

pub struct ArtifactHistoryReadRetirement<P, Mu: Mutation<P>> {
    pub derived: std::mem::ManuallyDrop<Option<ArtifactDerivedSnapshot<P>>>,
    pub derived_alias: std::mem::ManuallyDrop<Option<Arc<P>>>,
    pub preview: std::mem::ManuallyDrop<Option<ArtifactDerivedHistoryPreview<P, Mu>>>,
    pub replay: std::mem::ManuallyDrop<Option<ArtifactDerivedReplay<P, Mu>>>,
    pub finished: std::mem::ManuallyDrop<Option<EditReplayResult<P, Mu>>>,
    pub loaded_replay: std::mem::ManuallyDrop<Option<EditReplay<P, Mu>>>,
    pub active: std::mem::ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    pub plan_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>>,
    pub draft_retirement: std::mem::ManuallyDrop<Option<semio_framework_value::retirement::controlled::ControlledRetirement<protocol::HistoryInputDrafts>>>,
    pub registry_retirement: std::mem::ManuallyDrop<Option<SnapshotReadRegistryAliasRetirement>>,
    pub snapshots: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>>,
    pub mutations: std::mem::ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<Mu>>>>,
    pub factory_retirement:[Option<semio_framework_value::FactoryAuthority>;2],
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
        if self.plan_retirement.is_some() {return Some(retirement_owner_demands(&self.plan_retirement,body));}
        if self.preview.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some())||self.replay.as_ref().is_some_and(|owner|owner.plan.is_some()||owner.plan_retirement.is_some()) {return Some(Ok(semio_framework_value::RetirementDemand {depth:1,..Default::default()}));}
        None
    }
    fn close_plan(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.plan_demands(grant.maximum_copy_bytes) else{return Ok(None)};
        let demand=demand?;if grant.maximum_items==0 {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history plan retirement requires its original owner depth"));}
        if self.plan_retirement.is_some() {return retirement_owner_close(&mut self.plan_retirement,grant).map(Some);}
        if let Some(owner)=self.preview.as_mut().and_then(|owner|owner.plan_retirement.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan_retirement.take())) {*self.plan_retirement=Some(owner);}else {
            let original=self.preview.as_mut().and_then(|owner|owner.plan.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan.take())).expect("observed original history planner");
            *self.plan_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner schema declares controlled retirement")));
        }
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,..Default::default()})))
    }
    fn preview_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        use semio_framework_value::RetirementDemand;
        if self.draft_retirement.is_some() {return Some(retirement_owner_demands(&self.draft_retirement,body));}
        if self.registry_retirement.is_some() {return Some(snapshot_registry_alias_demands(&self.registry_retirement));}
        let preview=self.preview.as_ref()?;
        if !preview.drafts.terminal_is_empty() {return Some(Ok(RetirementDemand {depth:1,..Default::default()}));}
                if let Some(owner)=preview.derived_operation.as_ref(){return Some(if owner.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>>(),depth:1,..Default::default()})}else if owner.is_closing(){owner.next_demand(body)}else{Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})});}
        if let Some(owner)=preview.operation_close.as_ref(){return Some(semio_framework_value::factory_ticket_demands(owner,body));}
        if preview.displaced_operation.is_some(){return Some(Ok(RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()}));}
        if let Some(id)=preview.operation_identity.as_ref(){return Some(Ok(RetirementDemand{copy_bytes:size_of::<MutationId>(),release_bytes:id.0.capacity(),depth:1,..Default::default()}));}
if preview.state.is_some() {return Some(Ok(RetirementDemand {capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}));}
        Some(Ok(RetirementDemand {depth:1,..Default::default()}))
    }
    fn close_preview(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let Some(demand)=self.preview_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history preview retirement requires its exact owner depth"));}
        if self.draft_retirement.is_some() {return retirement_owner_close(&mut self.draft_retirement,grant).map(Some);}
        if self.registry_retirement.is_some() {return snapshot_registry_alias_close_step(&mut self.registry_retirement,grant).map(Some);}
        let preview=self.preview.as_mut().expect("observed original preview");
                if let Some(owner)=preview.derived_operation.as_mut(){if owner.terminal_is_empty(){drop(preview.derived_operation.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}if !owner.is_closing(){owner.cancel();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}return owner.close_step(grant).map(Some);}
        if preview.operation_close.is_some(){return semio_framework_value::close_factory_ticket(&mut preview.operation_close,grant).map(Some);}
        if preview.displaced_operation.is_some(){let Some((owner,p))=ReturnedSnapshotReadRetirement::<P>::admit_original(&mut preview.displaced_operation,self.snapshots.as_ref().unwrap(),grant)?else{return Ok(Some(RetainedCloneStep::Progress(Default::default())))};preview.operation_close=Some(owner);return Ok(Some(RetainedCloneStep::Progress(p)));}
        if preview.operation_identity.is_some(){drop(preview.operation_identity.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})));}
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
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body);}self.derived_snapshot_demands().or_else(||self.plan_demands(body)).or_else(||self.preview_demands(body)).or_else(||self.raw_replay_demands(body)).or_else(||self.factory_demands(body)).unwrap_or_else(||if self.terminal_is_empty(){Ok(Default::default())}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read residual authority is not admitted"))})}

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
