//! 👥️ Member owners and visitors of the tool run ledger: the composed member owners, the member run, fold, emission, generation and retire visitors and the erased retirement helpers. See [`super`].

use super::*;
use std::mem::ManuallyDrop;
use semio_framework_value_retirement::retirement::controlled::ControlledRetirement;
use semio_framework::ArtifactDialect;
use semio_framework_plugin_decl_views::child_view::ChildContentView;
use semio_framework_plugin_decl_emission::ChildEmit;
use semio_framework_os_kernel::os_store::ArtifactStore;
use semio_framework::Fault;

//#region 🔖️Member
/// 🧬️ The typed owners of a member run (`ToolRunDefinition::member`): the member snapshot the run rests on, the overlay its
/// provisional ops fold to, and every alias it let go — all owed to that member store's alias retirement, never dropped plainly.
#[doc(hidden)]
pub trait ToolRunMemberOwners: Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn has_displaced(&self) -> bool;
    fn is_empty(&self) -> bool;
}

struct ToolRunMemberState<P,M:Send+Sync+'static> {
    operation:Option<M>,
    operation_source:Option<store::os_store::ArtifactMutationSource<M>>,
    operation_input:Option<semio_framework_value::retained_clone::RetainedOwnedProjection<M>>,
    operation_consumed:bool,
    preparation:Option<store::os_store::ArtifactDerivedOperation<P,M>>,
    operation_retirement:Option<Box<dyn store::ErasedSnapshotRetirement>>,
    rebase_pending:bool,
    pub base: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub overlay: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub displaced: Vec<Arc<P>>,
    pending_alias: Option<Arc<P>>,
}

impl<P: Send + Sync + 'static,M:Send+Sync+'static> ToolRunMemberOwners for ToolRunMemberState<P,M> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn has_displaced(&self) -> bool {
        !self.displaced.is_empty() || self.pending_alias.is_some()||self.operation_retirement.is_some()||((self.operation_consumed||self.rebase_pending)&&(self.operation_source.is_some()||self.operation_input.is_some()))||self.preparation.as_ref().is_some_and(|owner|owner.is_closing())
    }

    fn is_empty(&self) -> bool {
        self.operation.is_none()&&self.operation_source.is_none()&&self.operation_input.is_none()&&self.preparation.is_none()&&self.operation_retirement.is_none()&&self.base.is_none() && self.overlay.is_none() && self.displaced.is_empty() && self.displaced.capacity()==0 && self.pending_alias.is_none()
    }
}

/// 🧩️ A run whose provisional ops are one owned member's ops (design §12, §20.15): where they land, the member generation
/// they rest on, how many the overlay already shows, the typed owners they fold through, and the composed read — the live
/// children with the overlay in the member's place — the render seams show while the run holds them.
#[doc(hidden)]
pub struct ToolRunMemberRun {
    pub slot: String,
    pub child_id: String,
    pub dialect: ArtifactDialect,
    pub base_generation: u64,
    pub ops: Vec<Vec<u8>>,
    pub shown: usize,
    pub rebase: bool,
    pub owners: Option<Box<dyn ToolRunMemberOwners>>,
    pub children: Option<ChildContentView>,
    pub stale: Vec<ChildContentView>,
    pub emit:Option<ChildEmit>,
    pub emission_owner:Option<Box<dyn ToolRunMemberEmissionOwner>>,
    pub retired_emits:std::collections::VecDeque<ChildEmit>,
    pub retired_emission_owners:std::collections::VecDeque<Box<dyn ToolRunMemberEmissionOwner>>,
    pub retired_ops:std::collections::VecDeque<Vec<Vec<u8>>>,
}

impl ToolRunMemberRun {
    /// 🏃️ Owed work: ops the overlay does not show yet, a refold from the member head, or reads, aliases, released ops
    /// and published emissions to retire.
    pub fn is_work(&self) -> bool {
        self.rebase || self.shown != self.ops.len() || !self.stale.is_empty() || !self.retired_ops.is_empty() || !self.retired_emits.is_empty() || !self.retired_emission_owners.is_empty() || self.owners.as_deref().is_some_and(ToolRunMemberOwners::has_displaced)
    }

    /// 🏁️ Every typed owner, queued emission, operation and composed read of this member is returned; only its address metadata remains.
    pub fn is_retired(&self) -> bool {
        self.ops.is_empty() && self.ops.capacity() == 0 && self.emit.is_none() && self.emission_owner.is_none() && self.retired_emits.is_empty() && self.retired_emission_owners.is_empty() && self.retired_ops.is_empty() && self.retired_emits.capacity() == 0 && self.retired_emission_owners.capacity() == 0 && self.retired_ops.capacity() == 0 && self.children.is_none() && self.stale.is_empty() && self.owners.as_deref().is_none_or(ToolRunMemberOwners::is_empty)
    }

    /// 📐️ Bytes of the address strings and the empty queues a retired member still holds.
    pub fn address_backing_bytes(&self) -> usize {
        self.slot.capacity().saturating_add(self.child_id.capacity()).saturating_add(self.stale.capacity().saturating_mul(size_of::<ChildContentView>()))
    }

    /// 🧹️ Drops every provisional op: the composed read retires and the next refresh rests on the member head.
    pub fn release(&mut self) {
        if !self.ops.is_empty()||self.ops.capacity()!=0{self.retired_ops.push_back(std::mem::take(&mut self.ops));}
        self.shown = 0;
        self.rebase = true;
        self.retired_emits.extend(self.emit.take());
        self.retired_emission_owners.extend(self.emission_owner.take());
        self.stale.extend(self.children.take());
    }
}

/// 📏️ Where one member fold left the overlay: the ops it shows, the ops that did not apply, and — once every op is shown —
/// the leased read of the overlay with the member's revision (`None`: no op, the render seams read the live member).
pub struct ToolRunMemberFolded {
    pub(crate) pending:bool,
    pub(crate) progress:RetainedCloneProgress,
    pub shown: usize,
    pub conflicts: u32,
    pub read: Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>,
}

impl ToolRunMemberFolded{
 /// 📏️ The exact retained receipt belongs to the containing original driver.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// ⏳️ An unfinished semantic source keeps its original member recipient resident.
 pub fn pending(&self)->bool{self.pending}
}

/// 🔁️ Folds a member run's ops onto its overlay within the turn wall budget: a rebase restarts from the member head.
#[doc(hidden)]
pub struct ToolRunMemberFold<'a, 'b> {
    pub owners: &'a mut Option<Box<dyn ToolRunMemberOwners>>,
    pub ops: &'a [Vec<u8>],
    pub shown: usize,
    pub rebase: bool,
    pub deadline_us: u64,
    pub grant: RetainedCloneGrant,
    pub receiving: &'a mut semio_framework_value::NativeDecodeControl<'b>,
}

/// 🚫️ Why one member fold step stopped: a native value refusal that keeps the retained progress it already paid for, or a
/// semantic fault of the member store (a decode, a lost base, a derive refusal) that owes no retained receipt.
#[doc(hidden)]
pub enum ToolRunMemberFoldRefusal {
    Native(ValueError),
    Semantic(Fault),
}

impl From<Fault> for ToolRunMemberFoldRefusal {
    fn from(fault: Fault) -> Self {
        Self::Semantic(fault)
    }
}

impl store::MemberStoreVisitor for ToolRunMemberFold<'_,'_> {
    type Output = Result<ToolRunMemberFolded, ToolRunMemberFoldRefusal>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        let idle=RetainedCloneProgress::default();
        if self.grant.maximum_items==0{return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:idle,conflicts:0,read:None});}
        if self.owners.is_none(){let demand=RetirementDemand{capacity_bytes:size_of::<ToolRunMemberState<P,Mu>>(),depth:1,..Default::default()};if !tool_grant_funds(self.grant,demand){return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:idle,conflicts:0,read:None});}*self.owners=Some(Box::new(ToolRunMemberState::<P,Mu>{operation:None,operation_source:None,operation_input:None,operation_consumed:false,preparation:None,operation_retirement:None,rebase_pending:self.rebase,base:Some(store.derived_snapshot_head()),overlay:None,displaced:Vec::new(),pending_alias:None}));return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:demand.capacity_bytes,..idle},conflicts:0,read:None});}
        let state=self.owners.as_mut().unwrap().as_any_mut().downcast_mut::<ToolRunMemberState<P,Mu>>().ok_or_else(||plugin_sdk_fault("member fold owns another exact Store type"))?;
        state.rebase_pending|=self.rebase;
        let mut deadline=||semio_framework_job::default_now_us().is_none_or(|now|now>=self.deadline_us);
        if state.rebase_pending{if let Some(owner)=state.preparation.as_mut(){owner.cancel();}}
        if let Some(owner)=state.preparation.as_mut(){
            if owner.is_closing(){let step=owner.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if owner.terminal_is_empty(){drop(state.preparation.take());}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
        }
        if state.operation_consumed||state.rebase_pending{
            if let Some(input)=state.operation_input.as_mut(){let step=input.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if input.terminal_is_empty(){state.operation_input=None;}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
            if let Some(source)=state.operation_source.as_mut(){source.begin_close();let step=source.close_step(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;if source.terminal_is_empty(){state.operation_source=None;}return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
        }
        if state.operation_retirement.is_some(){let step=semio_framework_value::close_factory_ticket(&mut state.operation_retirement,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?;return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:step.progress(),conflicts:0,read:None});}
        if state.operation.is_some()&&(state.operation_consumed||state.rebase_pending){let p=store.retire_owned_mutation(&mut state.operation,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.map_or(idle,|(owner,p)|{state.operation_retirement=Some(owner);p});return Ok(ToolRunMemberFolded{shown:self.shown,pending:true,progress:p,conflicts:0,read:None});}
        let mut shown=self.shown;
        if state.rebase_pending{
            let copy=3*size_of::<Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>>();if !tool_grant_funds(self.grant,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()}){return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}
            if state.base.is_some()||state.overlay.is_some(){let needed=usize::from(state.base.is_some())+usize::from(state.overlay.is_some());if state.displaced.capacity()-state.displaced.len()<needed{let before=state.displaced.capacity();let bytes=(state.displaced.len()+needed)*size_of::<Arc<P>>();if self.grant.maximum_capacity_bytes<bytes||self.grant.maximum_release_bytes<before*size_of::<Arc<P>>(){return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}state.displaced.try_reserve_exact(needed).map_err(|_|plugin_sdk_fault("member original displaced slots cannot reserve"))?;return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,retained_capacity_bytes:state.displaced.capacity()*size_of::<Arc<P>>(),released_bytes:before*size_of::<Arc<P>>(),..idle},conflicts:0,read:None});}}
            state.displaced.extend(state.base.replace(store.derived_snapshot_head()).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));state.displaced.extend(state.overlay.take().map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));state.rebase_pending=false;shown=0;return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:copy,..idle},conflicts:0,read:None});
        }
        if shown<self.ops.len(){
            if state.operation.is_none()&&state.operation_source.is_none()&&state.preparation.is_none(){let bytes=self.ops[shown].len();if self.grant.maximum_copy_bytes<bytes||bytes>4096{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});}state.operation_consumed=false;state.operation=Some(<Mu as ::protocol::OpBinary>::decode_op(&self.ops[shown]).map_err(FaultFrom::into_fault)?);return Ok(ToolRunMemberFolded{shown,pending:true,progress:RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..idle},conflicts:0,read:None});}
            if state.preparation.is_none(){
                if state.operation_source.is_none(){let factory=store.owned_mutation_retirement_factory().map_err(ToolRunMemberFoldRefusal::Native)?;let Some((source,p))=store::os_store::ArtifactMutationSource::<Mu>::admit(&mut state.operation,&factory,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?else{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});};state.operation_source=Some(source);return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
                if state.operation_input.is_none(){let source=state.operation_source.as_mut().unwrap();let p=if source.ready(){source.take(&mut state.operation_input,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.unwrap_or(idle)}else{source.advance(self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.progress()};return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
            }
            if state.preparation.is_none(){let source=state.overlay.as_ref().or(state.base.as_ref()).ok_or_else(||plugin_sdk_fault("member fold retains its original base"))?;let p=store.begin_derived_operation(&mut state.preparation,source,&mut state.operation_input,u32::try_from(shown).unwrap_or(u32::MAX),store::os_store::ArtifactDerivedOperationMode::Provisional,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?.unwrap_or(idle);return Ok(ToolRunMemberFolded{shown,pending:true,progress:p,conflicts:0,read:None});}
            let owner=state.preparation.as_mut().unwrap();
            if owner.is_finished(){let Some(((next,_),p))=store.take_derived_operation(owner,self.grant).map_err(ToolRunMemberFoldRefusal::Native)?else{return Ok(ToolRunMemberFolded{shown,pending:true,progress:idle,conflicts:0,read:None});};state.operation_consumed=true;let conflicts=u32::from(next.is_none());if let Some(next)=next{if let Some(previous)=state.overlay.replace(next){state.pending_alias=Some(previous.into_snapshot_owner());}}return Ok(ToolRunMemberFolded{shown:shown+1,pending:true,progress:p,conflicts,read:None});}
            let step=store.step_derived_operation(owner,self.grant.maximum_copy_bytes,self.grant,self.receiving,&mut deadline).map_err(ToolRunMemberFoldRefusal::Native)?;return Ok(ToolRunMemberFolded{shown,pending:true,progress:step.progress(),conflicts:0,read:None});
        }
        let read=state.overlay.as_ref().filter(|_|!self.ops.is_empty()).map(|overlay|store.snapshot_read_derived(overlay).map(|read|(read,store.content_revision()))).transpose().map_err(FaultFrom::into_fault)?;
        Ok(ToolRunMemberFolded{shown,pending:false,progress:idle,conflicts:0,read})
    }
}

pub(super) fn ledger_backing_is_empty<T>(owner:&Vec<T>)->bool{owner.is_empty()&&(owner.capacity()==0||size_of::<T>()==0)}

pub(super) fn ledger_empty_backing_demand<T>(owner:&Vec<T>)->Result<Option<RetirementDemand>,ValueError>{
    if !owner.is_empty()||ledger_backing_is_empty(owner){return Ok(None);}
    Ok(Some(RetirementDemand{copy_bytes:2*size_of::<Vec<T>>(),release_bytes:owner.capacity().checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tool ledger backing demand overflow"))?,depth:1,..Default::default()}))
}

pub(super) fn close_ledger_backing<T>(owner:&mut Vec<T>,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
    let Some(demand)=ledger_empty_backing_demand(owner)?else{return Ok(None);};
    if !tool_grant_funds(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
    drop(std::mem::take(owner));Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})))
}

#[doc(hidden)]
pub fn tool_grant_funds(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{
 grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth
}

pub(super) fn close_tool_erased_retirement(owner:&mut Option<Box<dyn store::ErasedSnapshotRetirement>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
 if owner.is_none(){return Ok(RetainedCloneStep::Complete(Default::default()));}
 if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
 let demand=tool_erased_retirement_demand(owner,grant.maximum_copy_bytes)?;
 if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
 let current=owner.as_mut().expect("tool retained owner admitted in place");
 if current.terminal_is_empty(){drop(owner.take());return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}
 let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=current.close_step(child)?;
 let step=admit_retained_clone_close(child,step,current.terminal_is_empty(),"tool original retirement")?;
 Ok(RetainedCloneStep::Progress(step.progress()))
}

#[doc(hidden)]
pub trait ToolRunMemberEmissionOwner:Send{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any;
    fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>;
    fn is_empty(&self)->bool;
}

struct ToolRunMemberEmissionState<M>{
    pub current:ManuallyDrop<Option<M>>,
    pub retirement:ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    pub schema:ManuallyDrop<Option<String>>,
    pub refusal:ManuallyDrop<Option<::protocol::ProtocolError>>,
    pub retirement_refusal:ManuallyDrop<Option<ValueError>>,
    retirement_refusal_close:ManuallyDrop<Option<ControlledRetirement<ValueError>>>,
}

impl<M> Default for ToolRunMemberEmissionState<M>{
    fn default()->Self{Self{current:ManuallyDrop::new(None),retirement:ManuallyDrop::new(None),schema:ManuallyDrop::new(None),refusal:ManuallyDrop::new(None),retirement_refusal:ManuallyDrop::new(None),retirement_refusal_close:ManuallyDrop::new(None)}}
}

impl<M:Send+'static> ToolRunMemberEmissionOwner for ToolRunMemberEmissionState<M>{
    fn as_any_mut(&mut self)->&mut dyn std::any::Any{self}
    fn next_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.current.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"decoded member mutation retains its exact Store retirement issuer"));}
        if let Some(owner)=self.retirement_refusal_close.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<ValueError>>>(),depth:1,..Default::default()});}
            return Ok({let mut demand=owner.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"member original refusal close depth overflow"))?;demand});
        }
        if self.retirement_refusal.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<ValueError>>()+size_of::<Option<ValueError>>(),depth:1,..Default::default()});}
        if let Some(schema)=self.schema.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<String>>(),release_bytes:schema.capacity(),depth:1,..Default::default()});}
        if self.retirement.is_some(){return tool_erased_retirement_demand(&self.retirement,body);}
        replication::protocol_error_retirement_demand(&self.refusal)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,Fault>{
        if self.is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let demand=self.next_demands(grant.maximum_copy_bytes).map_err(FaultFrom::into_fault)?;
        if !tool_grant_funds(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()};
        if let Some(owner)=self.retirement_refusal_close.as_mut(){
            if owner.terminal_is_empty(){drop(self.retirement_refusal_close.take());return Ok(RetainedCloneStep::Progress(progress));}
            let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
            let step=owner.step(child).map_err(FaultFrom::into_fault)?;let step=admit_retained_clone_close(child,step,owner.terminal_is_empty(),"member original typed refusal").map_err(FaultFrom::into_fault)?;return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if let Some(original)=self.retirement_refusal.take(){
            match ControlledRetirement::new(original){Ok(owner)=>*self.retirement_refusal_close=Some(owner),Err((error,original))=>{*self.retirement_refusal=Some(original);return Err(error.into_fault());}}
            return Ok(RetainedCloneStep::Progress(progress));
        }
        if self.schema.is_some(){drop(self.schema.take());return Ok(RetainedCloneStep::Progress(progress));}
        if self.retirement.is_some(){return close_tool_erased_retirement(&mut self.retirement,grant).map_err(FaultFrom::into_fault);}
        replication::close_protocol_error_one(&mut self.refusal,grant).map_err(FaultFrom::into_fault)
    }
    fn is_empty(&self)->bool{self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()&&self.retirement_refusal_close.is_none()}
}

impl<M> Drop for ToolRunMemberEmissionState<M>{
    fn drop(&mut self){
        assert!(std::thread::panicking()||(self.current.is_none()&&self.retirement.is_none()&&self.schema.is_none()&&self.refusal.is_none()&&self.retirement_refusal.is_none()&&self.retirement_refusal_close.is_none()),"member emission dropped its decoded mutation or exact refusal before retirement");
        unsafe{ManuallyDrop::drop(&mut self.current);ManuallyDrop::drop(&mut self.retirement);ManuallyDrop::drop(&mut self.schema);ManuallyDrop::drop(&mut self.refusal);ManuallyDrop::drop(&mut self.retirement_refusal);ManuallyDrop::drop(&mut self.retirement_refusal_close);}
    }
}

#[doc(hidden)]
pub struct ToolRunMemberEmit<'a>{
    pub slot:&'a str,
    pub child_id:&'a str,
    pub ops:&'a [Vec<u8>],
    pub emit:&'a mut Option<ChildEmit>,
    pub owner:&'a mut Option<Box<dyn ToolRunMemberEmissionOwner>>,
    pub grant:RetainedCloneGrant,
}

impl store::MemberStoreVisitor for ToolRunMemberEmit<'_>{
    type Output=Result<(bool,RetainedCloneProgress),Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        if self.grant.maximum_items==0{return Ok((false,Default::default()));}
        if self.owner.is_none(){
            let demand=RetirementDemand{copy_bytes:size_of::<Option<Box<dyn ToolRunMemberEmissionOwner>>>(),capacity_bytes:size_of::<ToolRunMemberEmissionState<Mu>>(),depth:1,..Default::default()};
            if !tool_grant_funds(self.grant,demand){return Ok((false,Default::default()));}
            *self.owner=Some(Box::new(ToolRunMemberEmissionState::<Mu>::default()));
            return Ok((false,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}));
        }
        let typed=self.owner.as_mut().unwrap().as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member emission owns another exact mutation type"))?;
        if let Some(refusal)=typed.refusal.as_ref(){return Err(refusal.to_fault());}
        if typed.current.is_some(){
            if let Some((retirement,progress))=store.retire_owned_mutation(&mut typed.current,self.grant).map_err(FaultFrom::into_fault)?{*typed.retirement=Some(retirement);return Ok((false,progress));}
            return Ok((false,Default::default()));
        }
        if !typed.is_empty(){return Ok((false,typed.close_step(self.grant)?.progress()));}
        if self.emit.as_ref().is_some_and(|emit|emit.ops.len()==self.ops.len()){return Ok((true,Default::default()));}
        Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"tool member emission requires genuinely granted typed operation decoding and wire preparation").into_fault())
    }
}

#[doc(hidden)]
pub struct ToolRunMemberEmissionRetire<'a>{pub owner:&'a mut(dyn ToolRunMemberEmissionOwner+'static),pub grant:RetainedCloneGrant}

impl store::MemberStoreVisitor for ToolRunMemberEmissionRetire<'_>{
    type Output=Result<RetainedCloneStep,Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        if self.grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let typed=self.owner.as_any_mut().downcast_mut::<ToolRunMemberEmissionState<Mu>>().ok_or_else(||plugin_sdk_fault("member retirement owns another exact mutation type"))?;
        if typed.current.is_some(){
            return Ok(RetainedCloneStep::Progress(match store.retire_owned_mutation(&mut typed.current,self.grant).map_err(FaultFrom::into_fault)?{Some((retirement,progress))=>{*typed.retirement=Some(retirement);progress},None=>Default::default()}));
        }
        typed.close_step(self.grant)
    }
}

/// 🔢️ The member store's generation, the base a member run watches instead of the document's.
#[doc(hidden)]
pub struct ToolRunMemberGeneration;

impl store::MemberStoreVisitor for ToolRunMemberGeneration {
    type Output = u64;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        store.generation()
    }
}

/// ♻️ Hands one alias of a member run's owners to the member store's bounded alias retirement: displaced aliases first,
/// and with `everything` the base and overlay too (the run left its slot).
#[doc(hidden)]
pub struct ToolRunMemberRetire<'a>{pub owners:&'a mut(dyn ToolRunMemberOwners+'static),pub everything:bool,pub grant:RetainedCloneGrant}

impl store::MemberStoreVisitor for ToolRunMemberRetire<'_>{
    type Output=Result<(Option<Box<dyn store::ErasedSnapshotRetirement>>,RetainedCloneProgress),Fault>;
    fn visit<P,Mu>(self,store:&ArtifactStore<P,Mu>)->Self::Output
    where
        P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + ::semio_framework_os_kernel::StoreState,
        Mu: protocol::SemanticMutation<P> + ::semio_framework_os_kernel::StoreMutationFor<P>,
    {
        if self.grant.maximum_items==0{return Ok((None,Default::default()));}
        let state=self.owners.as_any_mut().downcast_mut::<ToolRunMemberState<P,Mu>>().ok_or_else(||plugin_sdk_fault("member alias retirement owns another exact Store type"))?;
        if let Some(owner)=state.preparation.as_mut(){if self.everything{owner.cancel();}if owner.is_closing(){let step=owner.close_step(self.grant).map_err(ValueError::into_fault)?;if owner.terminal_is_empty(){drop(state.preparation.take());}return Ok((None,step.progress()));}}
        if self.everything||state.operation_consumed{
            if let Some(input)=state.operation_input.as_mut(){let step=input.close_step(self.grant).map_err(ValueError::into_fault)?;if input.terminal_is_empty(){state.operation_input=None;}return Ok((None,step.progress()));}
            if let Some(source)=state.operation_source.as_mut(){source.begin_close();let step=source.close_step(self.grant).map_err(ValueError::into_fault)?;if source.terminal_is_empty(){state.operation_source=None;}return Ok((None,step.progress()));}
        }
        if state.operation_retirement.is_some(){return semio_framework_value::close_factory_ticket(&mut state.operation_retirement,self.grant).map(|step|(None,step.progress())).map_err(ValueError::into_fault);}
        if state.operation.is_some()&&(self.everything||state.operation_consumed){return Ok(store.retire_owned_mutation(&mut state.operation,self.grant).map_err(ValueError::into_fault)?.map_or((None,Default::default()),|(frame,p)|(Some(frame),p)));}
        if state.pending_alias.is_some(){return Ok(store.retire_snapshot_alias(&mut state.pending_alias,self.grant).map_err(FaultFrom::into_fault)?.map_or((None,Default::default()),|(owner,progress)|(Some(owner),progress)));}
        let copy=2*size_of::<Option<Arc<P>>>();
        if self.grant.maximum_copy_bytes<copy||self.grant.maximum_depth==0{return Ok((None,Default::default()));}
        let alias=state.displaced.pop().or_else(||if self.everything{state.overlay.take().or_else(||state.base.take()).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner)}else{None});
        if let Some(alias)=alias{state.pending_alias=Some(alias);return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}));}
        let release=state.displaced.capacity().checked_mul(size_of::<Arc<P>>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"member original alias backing overflow").into_fault())?;
        if release!=0&&self.grant.maximum_release_bytes>=release{let original=std::mem::take(&mut state.displaced);drop(original);return Ok((None,RetainedCloneProgress{copied_items:1,copied_bytes:copy,released_bytes:release,..Default::default()}));}
        Ok((None,Default::default()))
    }
}
//#endregion 🔖️Member
