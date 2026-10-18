//! 👥️ Member owners and visitors of the tool run ledger: the composed member owners, the member run, fold, emission, generation and retire visitors and the erased retirement helpers. See [`super`].

use super::*;

//#region 🔖️Member
/// 🧬️ The typed owners of a member run (`ToolRunDefinition::member`): the member snapshot the run rests on, the overlay its
/// provisional ops fold to, and every alias it let go — all owed to that member store's alias retirement, never dropped plainly.
#[doc(hidden)]
pub trait ToolRunMemberOwners: Send {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
    fn has_displaced(&self) -> bool;
    fn is_empty(&self) -> bool;
}

struct ToolRunMemberState<P> {
    pub base: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub overlay: Option<store::os_store::snapshot_read::ArtifactDerivedSnapshot<P>>,
    pub displaced: Vec<Arc<P>>,
    pending_alias: Option<Arc<P>>,
}

impl<P: Send + Sync + 'static> ToolRunMemberOwners for ToolRunMemberState<P> {
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }

    fn has_displaced(&self) -> bool {
        !self.displaced.is_empty() || self.pending_alias.is_some()
    }

    fn is_empty(&self) -> bool {
        self.base.is_none() && self.overlay.is_none() && self.displaced.is_empty() && self.displaced.capacity()==0 && self.pending_alias.is_none()
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
pub(crate) struct ToolRunMemberFolded {
    pub shown: usize,
    pub conflicts: u32,
    pub read: Option<(store::os_store::snapshot_read::ErasedSnapshotRead, [u8; 32])>,
}

/// 🔁️ Folds a member run's ops onto its overlay within the turn wall budget: a rebase restarts from the member head.
#[doc(hidden)]
pub struct ToolRunMemberFold<'a> {
    pub owners: &'a mut Option<Box<dyn ToolRunMemberOwners>>,
    pub ops: &'a [Vec<u8>],
    pub shown: usize,
    pub rebase: bool,
    pub deadline_us: u64,
}

impl store::MemberStoreVisitor for ToolRunMemberFold<'_> {
    type Output = Result<ToolRunMemberFolded, Fault>;

    fn visit<P, Mu>(self, store: &ArtifactStore<P, Mu>) -> Self::Output
    where
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
    {
        let owners = self.owners.get_or_insert_with(|| Box::new(ToolRunMemberState::<P> { base: Some(store.derived_snapshot_head()), overlay: None, displaced: Vec::new(), pending_alias: None }));
        let state = owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-store-kind"), "a member tool run's owners belong to another store kind"))?;
        let mut shown = self.shown;
        if self.rebase {
            let previous = state.base.replace(store.derived_snapshot_head());
            state.displaced.extend(previous.map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
            state.displaced.extend(state.overlay.take().map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner));
            shown = 0;
        }
        let mut conflicts = 0u32;
        while shown < self.ops.len() {
            let op = <Mu as ::protocol::OpBinary>::decode_op(&self.ops[shown]).map_err(|error| error.into_fault())?;
            let source = state.overlay.as_ref().or(state.base.as_ref()).ok_or_else(|| Fault::new(FaultOrigin::Plugin, FaultCode::new("toolRun.member-base-lost"), "a member tool run lost its base"))?;
            let next = store.derive_provisional_snapshot(source, &op);
            <Mu as ::protocol::Mutation<P>>::retire_cold(op);
            match next.map_err(|error| error.into_fault())? {
                Some(next) => state.displaced.extend(state.overlay.replace(next).map(store::os_store::snapshot_read::ArtifactDerivedSnapshot::into_snapshot_owner)),
                None => conflicts = conflicts.saturating_add(1),
            }
            shown += 1;
            if shown % TOOL_RUN_REFOLD_CHECK_OPS == 0 && shown < self.ops.len() && semio_framework_job::default_now_us().is_none_or(|now| now >= self.deadline_us) {
                return Ok(ToolRunMemberFolded { shown, conflicts, read: None });
            }
        }
        let read = match state.overlay.as_ref().filter(|_| !self.ops.is_empty()) {
            Some(overlay) => Some((store.snapshot_read_derived(overlay).map_err(|error| error.into_fault())?, store.content_revision())),
            None => None,
        };
        Ok(ToolRunMemberFolded { shown, conflicts, read })
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

pub(super) fn tool_erased_retirement_demand(owner:&Option<Box<dyn store::ErasedSnapshotRetirement>>,body:usize)->Result<RetirementDemand,ValueError>{
 let Some(owner)=owner.as_ref()else{return Ok(Default::default())};
 if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),copy_bytes:size_of::<Option<Box<dyn store::ErasedSnapshotRetirement>>>(),depth:1,..Default::default()});}
 Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"tool retained close depth overflow"))?})
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
            return Ok(RetirementDemand{copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"member original refusal close depth overflow"))?});
        }
        if self.retirement_refusal.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<ControlledRetirement<ValueError>>()+size_of::<Option<ValueError>>(),depth:1,..Default::default()});}
        if let Some(schema)=self.schema.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<String>>(),release_bytes:schema.capacity(),depth:1,..Default::default()});}
        if self.retirement.is_some(){return tool_erased_retirement_demand(&self.retirement,body);}
        ::protocol::protocol_error_retirement_demand(&self.refusal)
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
        ::protocol::close_protocol_error_one(&mut self.refusal,grant).map_err(FaultFrom::into_fault)
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
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
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
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
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
        P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + semio_framework_schema_composition::ArtifactCompositionFields + Send + Sync + 'static,
        Mu: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + ::protocol::Mutation<P> + protocol::SemanticMutation<P> + ::protocol::OpBinary + ::protocol::OpText + Send + 'static,
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
        P:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot+semio_framework_schema_composition::ArtifactCompositionFields+Send+Sync+'static,
        Mu:Clone+semio_framework_value::ToValue+semio_framework_value::FromValue+::protocol::Mutation<P>+protocol::SemanticMutation<P>+::protocol::OpBinary+::protocol::OpText+Send+'static,
    {
        if self.grant.maximum_items==0{return Ok((None,Default::default()));}
        let state=self.owners.as_any_mut().downcast_mut::<ToolRunMemberState<P>>().ok_or_else(||plugin_sdk_fault("member alias retirement owns another exact Store type"))?;
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
