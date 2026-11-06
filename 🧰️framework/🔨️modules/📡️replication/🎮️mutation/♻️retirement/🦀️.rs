//! ♻️ Native pending replay owners admit constructors and physical backing release independently.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, FactoryAuthority, close_factory_ticket, factory_ticket_demands, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, factory::FactorySharedRetirement};
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

pub trait ArtifactReplayRetirementFactory<P,M>:semio_framework_value::FactoryRetirement+Send+Sync {
    fn snapshot_birth_bytes(&self)->usize;
    fn mutations_birth_bytes(&self)->usize;
    fn snapshot(&self,owner:Arc<P>,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<P>)>;
    fn mutations(&self,owners:PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,PagedList<M,{usize::MAX}>)>;
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RegisteredReplayRetirement<P,M> {
    #[factory_child]
    snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    #[factory_child]
    mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactReplayRetirementFactory<P,M> for RegisteredReplayRetirement<P,M> {
    fn snapshot_birth_bytes(&self)->usize {FactorySharedRetirement::<P>::constructor_capacity_bytes()}
    fn mutations_birth_bytes(&self)->usize {size_of::<ReplayMutationsRetirement<M>>()}
    fn snapshot(&self,owner:Arc<P>,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,Arc<P>)> {
        let turn=FactorySharedRetirement::admit(owner,Arc::clone(&self.snapshots),grant);semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|(error,owner,factory)|{drop(factory);(error,owner)}),work:turn.work}
    }
    fn mutations(&self,owners:PagedList<M,{usize::MAX}>,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<(Box<dyn ErasedSnapshotRetirement>,RetainedCloneProgress),(ValueError,PagedList<M,{usize::MAX}>)> {
        let bytes=self.mutations_birth_bytes();
        if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes {return semio_framework_value::RetainedWorkOutcome{result:Err((refusal("replay inverse frame requires exact constructor admission"),owners)),work:0};}
        let layout=std::alloc::Layout::new::<ReplayMutationsRetirement<M>>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<ReplayMutationsRetirement<M>>())else{return semio_framework_value::RetainedWorkOutcome{result:Err((ValueError::literal(ValueRefusalKind::AllocationFailed,"replay inverse frame allocation failed"),owners)),work:1};};
        let owner=unsafe{pointer.as_ptr().write(ReplayMutationsRetirement::new(owners,Arc::clone(&self.mutations)));Box::from_raw(pointer.as_ptr())};
        semio_framework_value::RetainedWorkOutcome{result:Ok((owner,RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})),work:1}
    }
}
/// 📏️ The defining shared factory quotes its exact original system allocation before any child issuer aliases move.
pub fn registered_replay_retirement_factory_allocation_bytes<P,M>()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<RegisteredReplayRetirement<P,M>>()).expect("original replay issuer layout").0.pad_to_align().size()}
pub fn registered_replay_retirement_factory<P:Send+Sync+'static,M:Send+'static>(snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->Arc<dyn ArtifactReplayRetirementFactory<P,M>> {Arc::new(RegisteredReplayRetirement{snapshots,mutations})}

/// 🧳️ Two pending native lanes preserve originals while each admitted child physically closes.
pub struct ReplayRetirement<P, M> {
    snapshot:ManuallyDrop<Option<Arc<P>>>,
    mutations:ManuallyDrop<Option<PagedList<M,{usize::MAX}>>>,
    active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactReplayRetirementFactory<P,M>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
}
enum ReplayCloseControl<'a,'loan>{Controlled(&'a mut dyn semio_framework_value::OriginalRetirementControl),Original(&'a mut semio_framework_value::OriginalRetirementCloseLoan<'loan>)}
impl<P,M> ReplayRetirement<P,M> {
    pub fn new(factory:Arc<dyn ArtifactReplayRetirementFactory<P,M>>)->Self {Self{snapshot:ManuallyDrop::new(None),mutations:ManuallyDrop::new(None),active:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(factory)),closing_factory:ManuallyDrop::new(None)}}
    pub fn stage_snapshot(&mut self,owner:Arc<P>){assert!(self.snapshot.is_none()&&self.factory.is_some(),"replay snapshot lane must close before reuse");*self.snapshot=Some(owner);}
    pub fn stage_mutations(&mut self,owners:PagedList<M,{usize::MAX}>){assert!(self.mutations.is_none()&&self.factory.is_some(),"replay inverse lane must close before reuse");*self.mutations=Some(owners);}
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(active)=self.active.as_ref(){return nested(factory_ticket_demands(active,copy)?);}
        if self.snapshot.is_some(){return Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().unwrap().snapshot_birth_bytes(),depth:2,..Default::default()});}
        if self.mutations.is_some(){return Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().unwrap().mutations_birth_bytes(),depth:2,..Default::default()});}
        if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        self.closing_factory.as_ref().map_or(Ok(Default::default()),|factory|nested(factory.demands(copy)?))
    }
    pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    pub fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    pub fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    pub fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
    pub fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}if self.snapshot.is_some()||self.mutations.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"replay source and factory handoff needs its exact selected-work quote"))}if self.factory.is_some(){return Ok(1)}match self.closing_factory.as_ref(){Some(factory) if factory.terminal_is_empty()=>Ok(1),Some(factory)=>factory.next_computational_work(body),None=>Ok(0)}}
    pub fn terminal_is_empty(&self)->bool{self.snapshot.is_none()&&self.mutations.is_none()&&self.active.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()}
    pub fn requires_original_control(&self)->bool{if let Some(active)=self.active.as_ref(){return !active.terminal_is_empty()&&active.requires_original_control()}if self.snapshot.is_some()||self.mutations.is_some()||self.factory.is_some(){return false}self.closing_factory.as_ref().is_some_and(|factory|!factory.terminal_is_empty()&&factory.requires_original_control())}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{self.close_inner(grant,None)}
    pub fn close_step_controlled(&mut self,grant:RetainedCloneGrant,control:&mut dyn semio_framework_value::OriginalRetirementControl)->semio_framework_value::OriginalRetirementOutcome{let mut control=ReplayCloseControl::Controlled(control);self.close_inner(grant,Some(&mut control))}
    pub fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{let mut control=ReplayCloseControl::Original(original);self.close_inner(grant,Some(&mut control))}
    fn close_inner(&mut self,grant:RetainedCloneGrant,mut control:Option<&mut ReplayCloseControl<'_,'_>>)->semio_framework_value::OriginalRetirementOutcome{let mut work=0;let result=(||->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}
        if self.requires_original_control()&&control.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"replay retirement selected child requires original control").into())}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.active.is_some(){let turn=match control.as_deref_mut(){Some(ReplayCloseControl::Controlled(control))=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,child,*control),Some(ReplayCloseControl::Original(original))=>semio_framework_value::close_factory_ticket_original(&mut self.active,*original,child),None=>close_factory_ticket(&mut self.active,child)};work=turn.work;return turn.result;}
        if self.snapshot.is_some(){let turn=self.factory.as_ref().unwrap().snapshot(self.snapshot.take().unwrap(),child);work=turn.work;match turn.result{Ok((owner,progress))=>{*self.active=Some(owner);return admit_birth(progress,child,demand.capacity_bytes).map_err(Into::into);},Err((error,owner))=>{*self.snapshot=Some(owner);return Err(error.into());}}}
        if self.mutations.is_some(){let turn=self.factory.as_ref().unwrap().mutations(self.mutations.take().unwrap(),child);work=turn.work;match turn.result{Ok((owner,progress))=>{*self.active=Some(owner);return admit_birth(progress,child,demand.capacity_bytes).map_err(Into::into);},Err((error,owner))=>{*self.mutations=Some(owner);return Err(error.into());}}}
        if self.factory.is_some(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.factory.take().unwrap();*self.closing_factory=Some(FactoryAuthority::new(factory));work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        let factory=self.closing_factory.as_mut().ok_or_else(||ValueError::invariant("closing_factory absent"))?;if factory.terminal_is_empty(){drop(self.closing_factory.take());work=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}let turn=match control.as_deref_mut(){Some(ReplayCloseControl::Controlled(control))=>factory.step_controlled(child,*control),Some(ReplayCloseControl::Original(original))=>factory.step_original(*original,child),None=>factory.step(child)};work=turn.work;let step=turn.result?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
    })();semio_framework_value::RetainedWorkOutcome{result,work}}
}
impl<P,M> Drop for ReplayRetirement<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"replay cleanup requires exact terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot);ManuallyDrop::drop(&mut self.mutations);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}}}}

struct ReplayMutationsRetirement<M>{
    owners:ManuallyDrop<Option<PagedList<M,{usize::MAX}>>>,
    pending:ManuallyDrop<Option<M>>,
    active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
}
impl<M> ReplayMutationsRetirement<M>{
    fn new(owners:PagedList<M,{usize::MAX}>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->Self{Self{owners:ManuallyDrop::new(Some(owners)),pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(factory)),closing_factory:ManuallyDrop::new(None)}}
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(active)=self.active.as_ref(){return nested(factory_ticket_demands(active,copy)?);}
        if let Some(value)=self.pending.as_ref(){return Ok(RetirementDemand{capacity_bytes:self.factory.as_ref().unwrap().retirement_birth_bytes(value),depth:2,..Default::default()});}
        if let Some(owners)=self.owners.as_ref(){return Ok(RetirementDemand{copy_bytes:if owners.is_empty(){0}else{size_of::<M>()},release_bytes:if owners.is_empty(){owners.next_release_allocation_bytes().map_err(ValueError::from)?}else{0},depth:1,..Default::default()});}
        if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        self.closing_factory.as_ref().map_or(Ok(Default::default()),|factory|nested(factory.demands(copy)?))
    }
    fn empty(&self)->bool{self.owners.is_none()&&self.pending.is_none()&&self.active.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()}
}
impl<M:Send+'static> ErasedSnapshotRetirement for ReplayMutationsRetirement<M>{
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}if let Some(value)=self.pending.as_ref(){return self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?.retire_owned_work_demand(value)}if self.owners.is_some()||self.factory.is_some(){return Ok(1)}match self.closing_factory.as_ref(){Some(factory) if factory.terminal_is_empty()=>Ok(1),Some(factory)=>factory.next_computational_work(body),None=>Ok(0)}}
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
    fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::OriginalRetirementOutcome{
        let mut work=0;let result=(||->Result<RetainedCloneStep,semio_framework_value::OriginalRetirementFailure>{
        let empty=RetainedCloneProgress::default();if self.empty(){return Ok(RetainedCloneStep::Complete(empty));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.active.is_some(){let turn=close_factory_ticket(&mut self.active,child);work=turn.work;return turn.result;}
        if self.pending.is_some(){let turn=match self.factory.as_ref(){Some(v)=>v,None=>return Err(ValueError::invariant("factory absent").into())}.retire_owned(match self.pending.take(){Some(v)=>v,None=>return Err(ValueError::invariant("pending absent").into())},child);work=turn.work;match turn.result{Ok((owner,progress))=>{*self.active=Some(owner);return admit_birth(progress,child,demand.capacity_bytes).map_err(Into::into);},Err((error,owner))=>{*self.pending=Some(owner);return Err(error);}}}
        if let Some(owners)=self.owners.as_mut(){
            if !owners.is_empty(){*self.pending=owners.pop();work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<M>(),..empty}));}
            if owners.terminal_is_empty(){drop(self.owners.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let progress=owners.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
            work=progress.work;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.progressed),released_bytes:progress.released_allocation_bytes,..empty}));
        }
        if self.factory.is_some(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=self.factory.take().unwrap();*self.closing_factory=Some(FactoryAuthority::new(factory));work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        let factory=match self.closing_factory.as_mut(){Some(v)=>v,None=>return Err(ValueError::invariant("closing_factory absent").into())};if factory.terminal_is_empty(){drop(self.closing_factory.take());work=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}let turn=factory.step(child);work=turn.work;let step=turn.result?;
        Ok(if self.empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
        })();
        semio_framework_value::RetainedWorkOutcome{result,work}
    }
    fn terminal_is_empty(&self)->bool{self.empty()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<M> Drop for ReplayMutationsRetirement<M>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"native inverse retirement abandoned owners");}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"replay cleanup nested depth overflow"))?;Ok(demand)}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,message)}
fn admit_birth(progress:RetainedCloneProgress,grant:RetainedCloneGrant,bytes:usize)->Result<RetainedCloneStep,ValueError>{if !progress.fits(grant)||progress.retained_capacity_bytes!=bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"replay cleanup constructor changed its receipt"));}Ok(RetainedCloneStep::Progress(progress))}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
