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
        pub history_source:Option<semio_os_store_replay_operation::native::history::OriginalHistorySource<semio_os_spr::Edit<Mu>>>,
    pub replacement_wire:semio_os_store_operation_wire::OriginalOperationWireComparison<Mu>,
    pub replacement_foreign:bool,
    pub original_history_issuance:u64,
    pub original_decode:Option<(semio_framework_value::retained_clone::NativeAllocationIdentity,u64)>,
    pub original_selection:Option<semio_os_vcs::OriginalHistoryEditSelection>,
    pub original_order:Option<(usize,usize)>,
    pub target:semio_os_store_operation_wire::OriginalCommonTarget,
    pub target_operation:Option<(usize,usize,u8)>,
    pub replacement:semio_os_store_operation_wire::OriginalReplacementReceiving<Mu>,
    pub replacement_operation:Option<(usize,usize,bool)>,
    pub replacement_phase:u8,
    pub history_input:semio_os_store_replay_operation::native::history::OriginalHistoryInput<Mu>,
    pub history_issuer:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<semio_os_spr::Edit<Mu>>>>,
    pub history_issuer_close:Option<semio_framework_value::FactoryAuthority>,
    pub history_operation:Option<(usize,usize)>,
    pub failed_operation:Option<semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>,
    pub preparation_receipt:RetainedCloneProgress,
    pub preparation_work:Option<u64>,
pub replay: Option<EditReplay<P, Mu>>,
    pub plan: Option<HistoryReadPlan>,
    pub plan_retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>,
    pub registry: SnapshotReadRegistryHandle,
    pub generation: u64,
    pub revision: [u8; 32],
}

impl<P, Mu: Mutation<P> + OpBinary> ArtifactDerivedReplay<P, Mu> {
        /// 🧾️ Exposes the original caller-granted replay work to the mounted history controller.
    pub fn take_preparation_receipt(&mut self)->RetainedCloneProgress{std::mem::take(&mut self.preparation_receipt)}
    /// 🪙️ Transfers the independently measured work of the original source turn, when selected.
    pub fn take_preparation_work(&mut self)->Option<u64>{self.preparation_work.take()}
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
    pub fn retire_completed_plan(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll {retirement_owner_close(&mut self.plan_retirement,effect)}
}

/// 📍️ A borrowed checkpoint request selects exact original change and edit ledger positions.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ArtifactCheckpointSelection{pub identity:(usize,usize),pub phase:u8,pub checkpoint:usize,pub scan:usize,pub change:usize,pub edit:usize,pub current_change:Option<usize>,pub current_edit:Option<usize>,pub comparison:usize,pub complete:bool}
/// 📚️ The owning Store lends exact checkpoint, change and edit identities from its fenced original ledgers.
pub trait ArtifactCheckpointReadSource{
 fn checkpoint_id(&self,position:usize)->Option<&str>;
 fn checkpoint_change_id(&self,checkpoint:usize,change:usize)->Option<&str>;
 fn change_id(&self,position:usize)->Option<&str>;
 fn change_edit_id(&self,change:usize,edit:usize)->Option<&str>;
 fn edit_id(&self,position:usize)->Option<&str>;
}
impl ArtifactCheckpointSelection{
 pub fn new(id:&str)->Self{Self{identity:(id.as_ptr()as usize,id.len()),phase:0,checkpoint:0,scan:0,change:0,edit:0,current_change:None,current_edit:None,comparison:0,complete:false}}
 pub fn validate(&self,id:&str)->Result<(),ValueError>{if self.identity!=(id.as_ptr()as usize,id.len()){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"checkpoint read requires its original borrowed selection"))}Ok(())}
 pub fn advance_edit(&mut self){self.current_edit=None;self.edit+=1;self.scan=0;self.comparison=0;self.phase=2;}
 pub fn next_demand(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<Self>()+128,depth:1,..Default::default()}}
 pub fn step(&mut self,source:&impl ArtifactCheckpointReadSource,id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  self.validate(id)?;let demand=self.next_demand();if self.complete||self.current_edit.is_some()||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Ok(Default::default())}let invalid=||ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"checkpoint references an absent checkpoint, change or edit");
  match self.phase{
   0=>{let candidate=source.checkpoint_id(self.scan).ok_or_else(invalid)?;match compare_fragment(candidate,id,&mut self.comparison){Some(true)=>{self.checkpoint=self.scan;self.scan=0;self.phase=1;},Some(false)=>self.scan+=1,None=>{}}},
   1=>{if let Some(change_id)=source.checkpoint_change_id(self.checkpoint,self.change){match compare_fragment(source.change_id(self.scan).ok_or_else(invalid)?,change_id,&mut self.comparison){Some(true)=>{self.current_change=Some(self.scan);self.scan=0;self.phase=2;},Some(false)=>self.scan+=1,None=>{}}}else{self.complete=true;}},
   2=>{if let Some(edit_id)=source.change_edit_id(self.current_change.ok_or_else(invalid)?,self.edit){match compare_fragment(source.edit_id(self.scan).ok_or_else(invalid)?,edit_id,&mut self.comparison){Some(true)=>{self.current_edit=Some(self.scan);self.phase=3;},Some(false)=>self.scan+=1,None=>{}}}else{self.current_change=None;self.change+=1;self.edit=0;self.scan=0;self.phase=1;}},
   _=>return Err(invalid()),
  }Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
 }

}

/// 🕰️ A generation-fenced cursor resolving a preview's inputs and folding its prefix under the caller's deadline.
pub struct ArtifactDerivedHistoryPreview<P, Mu: Mutation<P>> {
        pub derived_operation:Option<semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>,
    pub history_source:Option<semio_os_store_replay_operation::native::history::OriginalHistorySource<semio_os_spr::Edit<Mu>>>,
    pub checkpoint:Option<ArtifactCheckpointSelection>,
    pub history_input:semio_os_store_replay_operation::native::history::OriginalHistoryInput<Mu>,
    pub history_issuer:Option<Arc<dyn ArtifactOwnedValueRetirementFactory<semio_os_spr::Edit<Mu>>>>,
    pub history_issuer_close:Option<semio_framework_value::FactoryAuthority>,
    pub displaced_operation:Option<Arc<P>>,
    pub operation_close:Option<Box<dyn ErasedSnapshotRetirement>>,
    pub operation_identity:Option<MutationId>,
    pub operation_adopted:bool,
    pub preparation_receipt:RetainedCloneProgress,
    pub preparation_work:Option<u64>,
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
    /// 🪙️ Transfers independently measured original source work for the selected turn.
    pub fn take_preparation_work(&mut self)->Option<u64>{self.preparation_work.take()}
/// 📤️ Transfers the completed projection while retaining every original input and planner residue.
    pub fn take_finished_snapshot_owner(&mut self)->Result<Arc<P>,VcsError> {
        if !self.finished{return Err(VcsError::HistoryReplaying);}
        self.state.take().ok_or_else(||VcsError::HistoryReplaying)
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
    pub fn retire_completed_plan(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll {retirement_owner_close(&mut self.plan_retirement,effect)}
}

impl<P, Mu: Mutation<P>> Drop for ArtifactDerivedHistoryPreview<P, Mu> {
    fn drop(&mut self) {
        if let Some(state) = self.state.take() {
            retire_shared_projection::<P, Mu>(state);
        }
    }
}

pub struct HistoryReadPlan {
    pub original_selection:Option<semio_os_vcs::OriginalHistoryEditSelection>,
    pub original_order:Option<(usize,usize)>,
    pub original_supersessions:bool,
    pub native_phase:u8,
    pub native_row:usize,
    pub native_schema:String,
    pub target: Option<MutationId>,
    pub drafts: protocol::HistoryInputDrafts,
    pub positions: protocol::HistoryFoldIndex<MutationId, (usize, usize)>,
    pub first_input: usize,
    pub last_draft: Option<usize>,
    pub order: semio_os_vcs::HistoryPageStack<String>,
    pub counts: semio_os_vcs::HistoryPageStack<u32>,
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
            original_selection:None,original_order:None,original_supersessions:false,native_phase:0,native_row:0,native_schema:String::new(),target, drafts, positions: protocol::HistoryFoldIndex::new(), first_input: edits, last_draft: None, order: semio_os_vcs::HistoryPageStack::empty(), counts: semio_os_vcs::HistoryPageStack::empty(), cursor: 0, located_target: false, located_draft: None, located_inputs: false, operations: 0,
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

enum HistoryCloseControl<'a,'loan>{Controlled(&'a mut dyn semio_framework_value::OriginalRetirementControl),Original(&'a mut semio_framework_value::OriginalRetirementCloseLoan<'loan>)}

impl<P, Mu> ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn derived_snapshot_demands(&self)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.registry_retirement.is_some(){return Some(snapshot_registry_alias_demands(&self.registry_retirement))}
        if self.derived_alias.is_some(){return Some(Ok(semio_framework_value::RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}))}
        self.derived.is_some().then_some(Ok(semio_framework_value::RetirementDemand{depth:1,..Default::default()}))
    }
    fn close_derived_snapshot(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit>{
        use semio_framework_value::retained_clone::{RetainedCloneProgress,RetainedCloneStep};
        let grant=effect.grant();
        let Some(demand)=self.derived_snapshot_demands()else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())))}
        if self.registry_retirement.is_some(){return semio_framework_value::RetirementPollExit::resume(snapshot_registry_alias_close_step(&mut self.registry_retirement,effect)).map(Some)}
        if self.derived_alias.is_some(){let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let mut factory=self.snapshots.clone();if factory.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"derived original lost its snapshot issuer").into())}return semio_framework_value::RetirementPollExit::resume(ReturnedSnapshotReadRetirement::admit_original(&mut self.derived_alias,&mut factory,&mut self.active,&mut effect.nested(child)).map_err(Into::into)).map(Some)}
        let ArtifactDerivedSnapshot{owner,registry,..}=self.derived.take().unwrap();*self.derived_alias=Some(owner);*self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(registry));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})))
    }
    fn plan_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.plan_retirement.is_some() {return Some(retirement_owner_demands(&self.plan_retirement,body));}
        if self.preview.as_ref().is_some_and(|owner|owner.plan_retirement.is_some())||self.replay.as_ref().is_some_and(|owner|owner.plan_retirement.is_some()){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::retirement::controlled::ControlledRetirement<HistoryReadPlan>>>(),depth:1,..Default::default()}))}if self.preview.as_ref().is_some_and(|owner|owner.plan.is_some())||self.replay.as_ref().is_some_and(|owner|owner.plan.is_some()){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<HistoryReadPlan>>(),depth:1,..Default::default()}))}
        None
    }
    fn close_plan(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:Option<&mut HistoryCloseControl<'_,'_>>)->Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let grant=effect.grant();
        let Some(demand)=self.plan_demands(grant.maximum_copy_bytes) else{return Ok(None)};
        let demand=demand?;if grant.maximum_items==0 {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history plan retirement requires its original owner depth" ).into());}
        if self.plan_retirement.is_some(){let poll=match control{Some(HistoryCloseControl::Controlled(control))=>retirement_owner_close_controlled(&mut self.plan_retirement,effect,*control),Some(HistoryCloseControl::Original(original))=>retirement_owner_close_original(&mut self.plan_retirement,effect,*original),None=>retirement_owner_close(&mut self.plan_retirement,effect)};return semio_framework_value::RetirementPollExit::resume(poll).map(Some)}
        if let Some(owner)=self.preview.as_mut().and_then(|owner|owner.plan_retirement.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan_retirement.take())) {*self.plan_retirement=Some(owner);}else {
            let original=self.preview.as_mut().and_then(|owner|owner.plan.take()).or_else(||self.replay.as_mut().and_then(|owner|owner.plan.take())).expect("observed original history planner");
            *self.plan_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|unreachable!("history planner schema declares controlled retirement")));
        }
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }
    fn preview_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        use semio_framework_value::RetirementDemand;
        if self.draft_retirement.is_some() {return Some(retirement_owner_demands(&self.draft_retirement,body));}
        if self.registry_retirement.is_some() {return Some(snapshot_registry_alias_demands(&self.registry_retirement));}
        let preview=self.preview.as_ref()?;
        if preview.history_source.is_some()||preview.history_input.occupied()||preview.history_issuer.is_some()||preview.history_issuer_close.is_some(){return Some(Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"preview retains Store-owned history source or issuer")))}
        if preview.checkpoint.is_some(){return Some(Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<ArtifactCheckpointSelection>>(),depth:1,..Default::default()}))}
                if let Some(owner)=preview.derived_operation.as_ref(){return Some(if owner.terminal_is_empty(){Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_os_store_replay_operation::ArtifactDerivedOperation<P,Mu>>>(),depth:1,..Default::default()})}else if owner.is_closing(){owner.next_demand(body)}else{Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})});}
        if let Some(owner)=preview.operation_close.as_ref(){return Some(semio_framework_value::factory_ticket_demands(owner,body));}
        if preview.displaced_operation.is_some(){return Some(Ok(RetirementDemand{capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:1,..Default::default()}));}
        if let Some(id)=preview.operation_identity.as_ref(){return Some(Ok(RetirementDemand{copy_bytes:size_of::<MutationId>(),release_bytes:id.0.capacity(),depth:1,..Default::default()}));}
        if !preview.drafts.terminal_is_empty(){return Some(Ok(RetirementDemand{copy_bytes:size_of::<protocol::HistoryInputDrafts>(),depth:1,..Default::default()}))}
if preview.state.is_some() {return Some(Ok(RetirementDemand {capacity_bytes:ReturnedSnapshotReadRetirement::<P>::constructor_capacity_bytes(),depth:2,..Default::default()}));}
        Some(Ok(RetirementDemand{copy_bytes:size_of::<Option<ArtifactDerivedHistoryPreview<P,Mu>>>()+size_of::<SnapshotReadRegistryHandle>(),depth:1,..Default::default()}))
    }
    fn close_preview(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:Option<&mut HistoryCloseControl<'_,'_>>)->Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let grant=effect.grant();
        let Some(demand)=self.preview_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history preview retirement requires its exact owner depth").into());}
        if self.draft_retirement.is_some(){let poll=match control{Some(HistoryCloseControl::Controlled(control))=>retirement_owner_close_controlled(&mut self.draft_retirement,effect,*control),Some(HistoryCloseControl::Original(original))=>retirement_owner_close_original(&mut self.draft_retirement,effect,*original),None=>retirement_owner_close(&mut self.draft_retirement,effect)};return semio_framework_value::RetirementPollExit::resume(poll).map(Some)}
        if self.registry_retirement.is_some(){return semio_framework_value::RetirementPollExit::resume(snapshot_registry_alias_close_step(&mut self.registry_retirement,effect)).map(Some)}
        let preview=self.preview.as_mut().expect("observed original preview");
        if preview.checkpoint.is_some(){preview.checkpoint.take();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
        if let Some(owner)=preview.derived_operation.as_mut(){if owner.terminal_is_empty(){drop(preview.derived_operation.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}if !owner.is_closing(){owner.cancel();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}let poll=match control{Some(HistoryCloseControl::Controlled(control))=>ErasedSnapshotRetirement::close_step_controlled(owner,effect.grant().maximum_copy_bytes,effect,*control),Some(HistoryCloseControl::Original(original))=>ErasedSnapshotRetirement::close_original_step(owner,*original,effect.grant().maximum_copy_bytes,effect),None=>ErasedSnapshotRetirement::close_step(owner,effect.grant().maximum_copy_bytes,effect)};return semio_framework_value::RetirementPollExit::resume(poll).map(Some);}
        if preview.operation_close.is_some(){let poll=match control{Some(HistoryCloseControl::Controlled(control))=>semio_framework_value::close_factory_ticket_controlled(&mut preview.operation_close,effect.grant().maximum_copy_bytes,effect,*control),Some(HistoryCloseControl::Original(original))=>semio_framework_value::close_factory_ticket_original(&mut preview.operation_close,*original,effect.grant().maximum_copy_bytes,effect),None=>semio_framework_value::close_factory_ticket(&mut preview.operation_close,effect.grant().maximum_copy_bytes,effect)};return semio_framework_value::RetirementPollExit::resume(poll).map(Some)}
        if preview.displaced_operation.is_some(){let mut factory=self.snapshots.clone();if factory.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview lost its original snapshot issuer").into())}return semio_framework_value::RetirementPollExit::resume(ReturnedSnapshotReadRetirement::admit_original(&mut preview.displaced_operation,&mut factory,&mut preview.operation_close,effect).map_err(Into::into)).map(Some)}
        if preview.operation_identity.is_some(){drop(preview.operation_identity.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))}
if !preview.drafts.terminal_is_empty() {let drafts=std::mem::take(&mut preview.drafts);*self.draft_retirement=Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(drafts).unwrap_or_else(|_|unreachable!("draft schema declares controlled retirement")));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})));}
        if preview.state.is_some(){let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let mut factory=self.snapshots.clone();if factory.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"preview lost its original snapshot issuer").into())}return semio_framework_value::RetirementPollExit::resume(ReturnedSnapshotReadRetirement::admit_original(&mut preview.state,&mut factory,&mut self.active,&mut effect.nested(child_grant)).map_err(Into::into)).map(Some)}
        semio_framework_value::RetirementPollExit::pay(effect,2)?;*self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(preview.registry.clone()));drop(self.preview.take());
        Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }
    fn factory_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.derived.is_some()||self.derived_alias.is_some()||self.preview.is_some()||self.replay.is_some()||self.loaded_replay.is_some()||self.finished.is_some() {return None;}
        if let Some(owner)=self.factory_retirement.iter().flatten().next() {if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<semio_framework_value::FactoryAuthority>>(),depth:1,..Default::default()}))}return Some(owner.demands(body).and_then(|mut demand| {demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement depth overflow"))?;Ok(demand)}));}
        (self.snapshots.is_some()||self.mutations.is_some()).then_some(Ok(semio_framework_value::RetirementDemand {copy_bytes:size_of::<Option<Arc<dyn semio_framework_value::FactoryRetirement>>>(),depth:1,..Default::default()}))
    }
    fn close_factories(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:Option<&mut HistoryCloseControl<'_,'_>>)->Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress};
        let grant=effect.grant();
        let Some(demand)=self.factory_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes {return Ok(Some(RetainedCloneStep::Progress(Default::default())));}if grant.maximum_depth<demand.depth {return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history factory retirement requires its original owner depth").into());}
        if let Some(slot)=self.factory_retirement.iter_mut().find(|owner|owner.is_some()) {
            let owner=slot.as_mut().unwrap();
            if owner.terminal_is_empty(){drop(slot.take());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))}
            let child_grant=RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant};
            let mut child=effect.nested(child_grant);
            let poll=match control{Some(HistoryCloseControl::Controlled(control))=>owner.step_controlled(child.grant().maximum_copy_bytes,&mut child,*control),Some(HistoryCloseControl::Original(original))=>owner.step_original(*original,child.grant().maximum_copy_bytes,&mut child),None=>owner.step(child.grant().maximum_copy_bytes,&mut child)};let step=semio_framework_value::RetirementPollExit::resume(poll)?;
            semio_framework_value::retained_clone::admit_retained_clone_close(child_grant,step,owner.terminal_is_empty(),"history factory authority")?;
            return Ok(Some(step));
        }
        let factory:Arc<dyn semio_framework_value::FactoryRetirement>=if let Some(factory)=self.snapshots.take(){factory}else{self.mutations.take().expect("original history mutation issuer")};
        self.factory_retirement[0]=Some(semio_framework_value::FactoryAuthority::new(factory));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})))
    }
    fn raw_replay_demands(&self,body:usize)->Option<Result<semio_framework_value::RetirementDemand,ValueError>> {
        if self.finished.is_none()&&self.loaded_replay.is_none(){if let Some(replay)=self.replay.as_ref(){if !replay.replacement_wire.terminal_is_empty()||!replay.target.terminal_is_empty()||!replay.replacement.terminal_is_empty()||replay.failed_operation.is_some()||replay.history_source.is_some()||replay.history_input.occupied()||replay.history_issuer.is_some()||replay.history_issuer_close.is_some(){return Some(Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"replay retains Store-owned history or replacement authority")))}}}
        let demand=if let Some(owner)=self.finished.as_ref(){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<EditReplayResult<P,Mu>>>(),depth:1,..Default::default()}));}if !owner.retirement_started(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}));}owner.retirement_demands(body)}else if let Some(owner)=self.loaded_replay.as_ref().or_else(||self.replay.as_ref().and_then(|owner|owner.replay.as_ref())){if owner.terminal_is_empty(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<EditReplay<P,Mu>>>(),depth:1,..Default::default()}));}if !owner.retirement_started(){return Some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()}));}owner.retirement_demands(body)}else{return self.replay.is_some().then_some(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<Option<ArtifactDerivedReplay<P,Mu>>>()+size_of::<SnapshotReadRegistryHandle>(),depth:1,..Default::default()}));};
        Some(demand.and_then(|mut demand|{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history raw replay retirement depth overflow"))?;Ok(demand)}))
    }
    fn close_raw_replay(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut control:Option<&mut HistoryCloseControl<'_,'_>>)->Result<Option<RetainedCloneStep>,semio_framework_value::RetirementPollExit> {
        use semio_framework_value::retained_clone::{RetainedCloneStep,RetainedCloneProgress,admit_retained_clone_close};
        let grant=effect.grant();
        let Some(demand)=self.raw_replay_demands(grant.maximum_copy_bytes)else{return Ok(None)};let demand=demand?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
        let moved=RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()});
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if let Some(owner)=self.finished.as_mut(){if owner.terminal_is_empty(){drop(self.finished.take());return Ok(Some(moved));}if !owner.retirement_started(){owner.begin_retirement();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})))}let mut nested=effect.nested(child);let poll=match control.as_deref_mut(){Some(HistoryCloseControl::Controlled(control))=>owner.close_original_step_controlled(&mut nested,*control),Some(HistoryCloseControl::Original(original))=>ErasedSnapshotRetirement::close_original_step(owner,*original,nested.grant().maximum_copy_bytes,&mut nested),None=>owner.close_original_step(&mut nested)};let step=semio_framework_value::RetirementPollExit::resume(poll)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original finished replay").map(Some).map_err(Into::into);}
        if let Some(owner)=self.loaded_replay.as_mut(){if owner.terminal_is_empty(){drop(self.loaded_replay.take());return Ok(Some(moved));}if !owner.retirement_started(){owner.begin_retirement();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})))}let mut nested=effect.nested(child);let poll=match control.as_deref_mut(){Some(HistoryCloseControl::Controlled(control))=>owner.close_original_step_controlled(&mut nested,*control),Some(HistoryCloseControl::Original(original))=>ErasedSnapshotRetirement::close_original_step(owner,*original,nested.grant().maximum_copy_bytes,&mut nested),None=>owner.close_original_step(&mut nested)};let step=semio_framework_value::RetirementPollExit::resume(poll)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original loaded replay").map(Some).map_err(Into::into);}
        let derived=self.replay.as_mut().expect("observed original derived replay");
        if let Some(owner)=derived.replay.as_mut(){if owner.terminal_is_empty(){drop(derived.replay.take());return Ok(Some(moved));}if !owner.retirement_started(){owner.begin_retirement();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()})))}let mut nested=effect.nested(child);let poll=match control.as_deref_mut(){Some(HistoryCloseControl::Controlled(control))=>owner.close_original_step_controlled(&mut nested,*control),Some(HistoryCloseControl::Original(original))=>ErasedSnapshotRetirement::close_original_step(owner,*original,nested.grant().maximum_copy_bytes,&mut nested),None=>owner.close_original_step(&mut nested)};let step=semio_framework_value::RetirementPollExit::resume(poll)?;return admit_retained_clone_close(child,step,owner.terminal_is_empty(),"history original derived replay").map(Some).map_err(Into::into);}
        semio_framework_value::RetirementPollExit::pay(effect,2)?;*self.registry_retirement=Some(SnapshotReadRegistryAliasRetirement::new(derived.registry.clone()));drop(self.replay.take());Ok(Some(moved))
    }
}

impl<P, Mu> ErasedSnapshotRetirement for ArtifactHistoryReadRetirement<P, Mu>
where P: Send + Sync + 'static, Mu: Mutation<P> + Send + 'static {
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{
        if self.terminal_is_empty(){return Ok(0)}
        if let Some(owner)=self.active.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}
        if let Some(demand)=self.derived_snapshot_demands(){demand?;if self.registry_retirement.is_some(){return snapshot_registry_alias_work_demand(&self.registry_retirement)}if self.derived_alias.is_some(){return Ok(ReturnedSnapshotReadRetirement::<P>::admit_original_source_work_demand(&self.derived_alias))}return Ok(1)}
        if let Some(demand)=self.plan_demands(body){demand?;return if self.plan_retirement.is_some(){retirement_owner_work_demand(&self.plan_retirement,body)}else{Ok(1)}}
        if let Some(demand)=self.preview_demands(body){demand?;if self.draft_retirement.is_some(){return retirement_owner_work_demand(&self.draft_retirement,body)}if self.registry_retirement.is_some(){return snapshot_registry_alias_work_demand(&self.registry_retirement)}let preview=self.preview.as_ref().ok_or_else(||ValueError::invariant("selected preview vanished"))?;if preview.checkpoint.is_some(){return Ok(1)}if let Some(owner)=preview.derived_operation.as_ref(){return if owner.terminal_is_empty()||!owner.is_closing(){Ok(1)}else{owner.next_computational_work(body)}}if let Some(owner)=preview.operation_close.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body)}}if preview.displaced_operation.is_some(){return Ok(ReturnedSnapshotReadRetirement::<P>::admit_original_source_work_demand(&preview.displaced_operation))}if preview.operation_identity.is_some()||!preview.drafts.terminal_is_empty(){return Ok(1)}if preview.state.is_some(){return Ok(ReturnedSnapshotReadRetirement::<P>::admit_original_source_work_demand(&preview.state))}return Ok(2)}
        if let Some(demand)=self.raw_replay_demands(body){demand?;if let Some(owner)=self.finished.as_ref(){return if owner.terminal_is_empty()||!owner.retirement_started(){Ok(1)}else{ErasedSnapshotRetirement::next_computational_work(owner,body)}}if let Some(owner)=self.loaded_replay.as_ref().or_else(||self.replay.as_ref().and_then(|replay|replay.replay.as_ref())){return if owner.terminal_is_empty()||!owner.retirement_started(){Ok(1)}else{ErasedSnapshotRetirement::next_computational_work(owner,body)}}return Ok(2)}
        if let Some(demand)=self.factory_demands(body){demand?;return match self.factory_retirement.iter().flatten().next(){Some(owner)if !owner.terminal_is_empty()=>owner.next_computational_work(body),_=>Ok(1)}}
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read residual authority is not admitted"))
    }
    fn requires_original_control(&self)->bool{
        if let Some(owner)=self.active.as_ref(){return !owner.terminal_is_empty()&&owner.requires_original_control()}
        if self.derived_snapshot_demands().is_some(){return false}
        if self.plan_demands(0).is_some(){return self.plan_retirement.as_ref().is_some_and(|owner|!owner.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(owner))}
        if self.preview_demands(0).is_some(){
            if let Some(owner)=self.draft_retirement.as_ref(){return !owner.terminal_is_empty()&&ErasedSnapshotRetirement::requires_original_control(owner)}
            if self.registry_retirement.is_some(){return false}
            if let Some(preview)=self.preview.as_ref(){
                if preview.checkpoint.is_some(){return false}
                if let Some(owner)=preview.derived_operation.as_ref(){return !owner.terminal_is_empty()&&owner.is_closing()&&owner.requires_original_control()}
                if let Some(owner)=preview.operation_close.as_ref(){return !owner.terminal_is_empty()&&owner.requires_original_control()}
                return false
            }
        }
        if self.raw_replay_demands(0).is_some(){if let Some(owner)=self.finished.as_ref(){return !owner.terminal_is_empty()&&owner.retirement_started()&&owner.requires_original_control()}if let Some(owner)=self.loaded_replay.as_ref(){return !owner.terminal_is_empty()&&owner.retirement_started()&&owner.requires_original_control()}return self.replay.as_ref().and_then(|owner|owner.replay.as_ref()).is_some_and(|owner|!owner.terminal_is_empty()&&owner.retirement_started()&&owner.requires_original_control())}
        self.factory_retirement.iter().flatten().next().is_some_and(|owner|owner.requires_original_control())
    }
        fn next_demand(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body);}self.derived_snapshot_demands().or_else(||self.plan_demands(body)).or_else(||self.preview_demands(body)).or_else(||self.raw_replay_demands(body)).or_else(||self.factory_demands(body)).unwrap_or_else(||if self.terminal_is_empty(){Ok(Default::default())}else{Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read residual authority is not admitted"))})}

fn close_step(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{let result=self.close_inner(effect,None);semio_framework_value::RetirementPollExit::settle(effect,result)}
fn close_step_controlled(&mut self,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementPoll{if !self.requires_original_control(){return semio_framework_value::RetirementPollExit::settle(effect,Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read selected turn does not own an original child loan").into()))}let mut control=HistoryCloseControl::Controlled(control);let result=self.close_inner(effect,Some(&mut control));semio_framework_value::RetirementPollExit::settle(effect,result)}
fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,_body:usize,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{if !self.requires_original_control(){return semio_framework_value::RetirementPollExit::settle(effect,Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"history read selected turn does not own an original child loan").into()))}let mut control=HistoryCloseControl::Original(original);let result=self.close_inner(effect,Some(&mut control));semio_framework_value::RetirementPollExit::settle(effect,result)}

    fn terminal_is_empty(&self) -> bool { self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.copy_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual copy authority is not admitted")) }, |demand| demand.map(|demand| demand.copy_bytes)) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,body).map(|demand|demand.capacity_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(body)).or_else(||self.preview_demands(body)).or_else(||self.raw_replay_demands(body)).or_else(||self.factory_demands(body)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual constructor authority is not admitted")) }, |demand| demand.map(|demand| demand.capacity_bytes)) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.release_bytes);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual release authority is not admitted")) }, |demand| demand.map(|demand| demand.release_bytes)) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if let Some(active)=self.active.as_ref(){return artifact_retirement_box_demands(active,0).map(|demand|demand.depth);} self.derived_snapshot_demands().or_else(||self.plan_demands(0)).or_else(||self.preview_demands(0)).or_else(||self.raw_replay_demands(0)).or_else(||self.factory_demands(0)).map_or_else(|| if self.terminal_is_empty() { Ok(0) } else { Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual depth authority is not admitted")) }, |demand| demand.map(|demand| demand.depth)) }
}

impl<P, Mu: Mutation<P>> Drop for ArtifactHistoryReadRetirement<P, Mu> {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.derived.is_none() && self.derived_alias.is_none() && self.preview.is_none() && self.replay.is_none() && self.loaded_replay.is_none() && self.finished.is_none() && self.active.is_none() && self.plan_retirement.is_none() && self.draft_retirement.is_none() && self.registry_retirement.is_none() && self.snapshots.is_none() && self.mutations.is_none() && self.factory_retirement.iter().all(Option::is_none)), "history read retirement dropped before its exact terminal witness"); }
}

impl<P:Send+Sync+'static,Mu:Mutation<P>+Send+'static> ArtifactHistoryReadRetirement<P,Mu>{
    fn close_inner(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>,mut control:Option<&mut HistoryCloseControl<'_,'_>>)->Result<RetainedCloneStep,semio_framework_value::RetirementPollExit>{
        let grant=effect.grant();
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if self.active.is_some(){let mut child=grant;child.maximum_depth=child.maximum_depth.checked_sub(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"history boxed child requires enclosing depth"))?;let mut nested=effect.nested(child);let poll=match control.as_deref_mut(){Some(HistoryCloseControl::Controlled(control))=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,nested.grant().maximum_copy_bytes,&mut nested,*control),Some(HistoryCloseControl::Original(original))=>semio_framework_value::close_factory_ticket_original(&mut self.active,*original,nested.grant().maximum_copy_bytes,&mut nested),None=>artifact_retirement_box_close_step(&mut self.active,nested.grant().maximum_copy_bytes,&mut nested)};return semio_framework_value::RetirementPollExit::resume(poll)}
        let moved = |step: RetainedCloneStep| match step {
            RetainedCloneStep::Complete(progress) => RetainedCloneStep::Progress(progress),
            RetainedCloneStep::Progress(progress) => RetainedCloneStep::Progress(progress),
        };
        if let Some(step)=self.close_derived_snapshot(effect)?{return Ok(moved(step))}
        if let Some(step) = self.close_plan(effect,control.as_deref_mut())? { return Ok(moved(step)); }
        if let Some(step) = self.close_preview(effect,control.as_deref_mut())? { return Ok(moved(step)); }
        if let Some(step) = self.close_raw_replay(effect,control.as_deref_mut())? { return Ok(moved(step)); }
        if let Some(step) = self.close_factories(effect,control.as_deref_mut())? { return Ok(moved(step)); }
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "history read residual owners require exact constructor admission").into())
    }
}

#[cfg(test)]
#[path="📸️checkpoint-selection/🦀️.rs"]
mod checkpoint_selection_tests;

fn compare_fragment(left:&str,right:&str,offset:&mut usize)->Option<bool>{if left.len()!=right.len(){*offset=0;return Some(false)}let end=(*offset+64).min(left.len());if left.as_bytes()[*offset..end]!=right.as_bytes()[*offset..end]{*offset=0;Some(false)}else if end==left.len(){*offset=0;Some(true)}else{*offset=end;None}}
