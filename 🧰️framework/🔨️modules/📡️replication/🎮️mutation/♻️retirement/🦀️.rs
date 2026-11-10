//! ♻️ Native pending replay owners admit constructors and physical backing release independently.
use semio_framework_value::{ArtifactOwnedValueRetirementFactory, ErasedSnapshotRetirement};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkOutcome, RetainedWorkPoll, RetirementPollExit, ValueError, ValueRefusalKind, RetirementDemand, FactoryAuthority, close_factory_ticket, factory_ticket_demands, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}, };
use semio_framework_value_persistent::list::PagedList;
use std::{mem::{ManuallyDrop,size_of},sync::Arc};

pub trait ArtifactReplayRetirementFactory<P,M>:semio_framework_value::FactoryRetirement+Send+Sync {
    fn snapshot_birth_bytes(&self)->usize;
    fn mutations_birth_bytes(&self)->usize;
    fn snapshot(&self,owner:&mut Option<Arc<P>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
    fn mutations(&self,owners:&mut Option<PagedList<M,{usize::MAX}>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll;
}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
struct RegisteredReplayRetirement<P,M> {
    #[factory_child]
    snapshots:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>,
    #[factory_child]
    mutations:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
}
impl<P:Send+Sync+'static,M:Send+'static> ArtifactReplayRetirementFactory<P,M> for RegisteredReplayRetirement<P,M> {
    fn snapshot_birth_bytes(&self)->usize {size_of::<ReplaySnapshotRetirement<P>>()}
    fn mutations_birth_bytes(&self)->usize {size_of::<ReplayMutationsRetirement<M>>()}
    fn snapshot(&self,owner:&mut Option<Arc<P>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        birth(owner,output,effect,self.snapshot_birth_bytes(),|owner|ReplaySnapshotRetirement::new(owner,Arc::clone(&self.snapshots)),"replay snapshot")
    }
    fn mutations(&self,owners:&mut Option<PagedList<M,{usize::MAX}>>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        birth(owners,output,effect,self.mutations_birth_bytes(),|owners|ReplayMutationsRetirement::new(owners,Arc::clone(&self.mutations)),"replay inverse")
    }
}
/// 🧱️ Allocates one fixed native frame for a staged lane before taking its owners.
fn birth<O,T:ErasedSnapshotRetirement+'static>(owners:&mut Option<O>,output:&mut Option<Box<dyn ErasedSnapshotRetirement>>,effect:&mut RetainedAllocationEffect<'_>,bytes:usize,make:impl FnOnce(O)->T,lane:&'static str)->OriginalRetirementPoll {
    let complete=|result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
    let idle=||Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
    if owners.is_none(){return complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),0);}
    if output.is_some(){return complete(Err(ValueError::invariant("replay frame output slot is occupied").into()),0);}
    let grant=effect.grant();
    if effect.maximum_work_units()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_capacity_bytes<bytes {return complete(idle(),0);}
    if effect.actual().is_some(){return complete(Err(ValueError::invariant("replay frame allocation marker is occupied").into()),0);}
    match effect.performed(1){Ok(true)=>{},Ok(false)=>return complete(Err(refusal("replay frame exceeded its work ceiling").into()),1),Err(error)=>return complete(Err(error.into()),1)}
    let layout=std::alloc::Layout::new::<T>();
    *effect.actual()=Some(0);
    let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<T>())else{return complete(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,lane).into()),1);};
    *effect.actual()=Some(layout.size());
    let Some(original)=owners.take()else{return complete(Err(ValueError::invariant("replay frame owners vanished").into()),1);};
    *output=Some(unsafe{pointer.as_ptr().write(make(original));Box::from_raw(pointer.as_ptr())});
    complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()})),1)
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
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{self.close_inner(body,effect,None)}
    pub fn close_step_controlled(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,control:&mut dyn semio_framework_value::OriginalRetirementControl)->OriginalRetirementPoll{let mut control=ReplayCloseControl::Controlled(control);self.close_inner(body,effect,Some(&mut control))}
    pub fn close_original_step(&mut self,original:&mut semio_framework_value::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{let mut control=ReplayCloseControl::Original(original);self.close_inner(body,effect,Some(&mut control))}
    fn close_inner(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,mut control:Option<&mut ReplayCloseControl<'_,'_>>)->OriginalRetirementPoll{
        let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
        let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
        let empty=RetainedCloneProgress::default();
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(empty));}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;
        if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}
        if self.requires_original_control()&&control.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"replay retirement selected child requires original control").into())}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.active.is_some(){let poll=match control.as_deref_mut(){Some(ReplayCloseControl::Controlled(control))=>semio_framework_value::close_factory_ticket_controlled(&mut self.active,body,&mut effect.nested(child),*control),Some(ReplayCloseControl::Original(original))=>semio_framework_value::close_factory_ticket_original(&mut self.active,*original,body,&mut effect.nested(child)),None=>close_factory_ticket(&mut self.active,body,&mut effect.nested(child))};let turn=RetirementPollExit::settled(poll)?;work=turn.work;return turn.result.map_err(Into::into);}
        if self.snapshot.is_some(){let issuer=self.factory.as_ref().ok_or_else(||ValueError::invariant("replay factory absent"))?;let turn=RetirementPollExit::settled(issuer.snapshot(&mut *self.snapshot,&mut *self.active,&mut effect.nested(child)))?;work=turn.work;let step=turn.result?;return Ok(admit_birth(step.progress(),child,demand.capacity_bytes)?);}
        if self.mutations.is_some(){let issuer=self.factory.as_ref().ok_or_else(||ValueError::invariant("replay factory absent"))?;let turn=RetirementPollExit::settled(issuer.mutations(&mut *self.mutations,&mut *self.active,&mut effect.nested(child)))?;work=turn.work;let step=turn.result?;return Ok(admit_birth(step.progress(),child,demand.capacity_bytes)?);}
        if let Some(factory)=self.factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;*self.closing_factory=Some(FactoryAuthority::new(factory));work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        let factory=self.closing_factory.as_mut().ok_or_else(||ValueError::invariant("closing_factory absent"))?;if factory.terminal_is_empty(){drop(self.closing_factory.take());work=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}
        let poll=match control.as_deref_mut(){Some(ReplayCloseControl::Controlled(control))=>factory.step_controlled(body,&mut effect.nested(child),*control),Some(ReplayCloseControl::Original(original))=>factory.step_original(*original,body,&mut effect.nested(child)),None=>factory.step(body,&mut effect.nested(child))};
        let turn=RetirementPollExit::settled(poll)?;work=turn.work;let step=turn.result?;
        Ok(if self.terminal_is_empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
        })();settle_exit(effect,base,result,work)}
}
impl<P,M> Drop for ReplayRetirement<P,M>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"replay cleanup requires exact terminal custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.snapshot);ManuallyDrop::drop(&mut self.mutations);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.factory);ManuallyDrop::drop(&mut self.closing_factory);}}}}

trait Staged<V>:Sized{
    fn demand(&self)->Result<RetirementDemand,ValueError>;
    fn finished(&self)->bool;
    fn step(&mut self,pending:&mut Option<V>,child:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<(RetainedCloneStep,u128),ValueError>;
}
struct ArcStage<P>(Option<Arc<P>>);
fn arc_bytes<P>()->usize{std::alloc::Layout::new::<[usize;2]>().extend(std::alloc::Layout::new::<P>()).expect("arc layout").0.pad_to_align().size()}
impl<P> Staged<P> for ArcStage<P>{
    fn demand(&self)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand{copy_bytes:size_of::<P>(),release_bytes:arc_bytes::<P>(),depth:1,..Default::default()})}
    fn finished(&self)->bool{self.0.is_none()}
    fn step(&mut self,pending:&mut Option<P>,_:RetainedCloneGrant,_:&mut RetainedAllocationEffect<'_>)->Result<(RetainedCloneStep,u128),ValueError>{
        *pending=self.0.take().and_then(Arc::into_inner);
        Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:if pending.is_some(){size_of::<P>()}else{0},released_bytes:if pending.is_some(){arc_bytes::<P>()}else{0},..Default::default()}),1))
    }
}
impl<M> Staged<M> for PagedList<M,{usize::MAX}>{
    fn demand(&self)->Result<RetirementDemand,ValueError>{if !self.is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<M>(),depth:1,..Default::default()})}Ok(RetirementDemand{copy_bytes:self.next_release_copy_byte_demand().map_err(ValueError::from)?,release_bytes:self.next_release_allocation_bytes().map_err(ValueError::from)?,depth:self.next_release_depth_demand().map_err(ValueError::from)?.max(1),..Default::default()})}
    fn finished(&self)->bool{self.terminal_is_empty()}
    fn step(&mut self,pending:&mut Option<M>,child:RetainedCloneGrant,effect:&mut RetainedAllocationEffect<'_>)->Result<(RetainedCloneStep,u128),ValueError>{
        if !self.is_empty(){*pending=self.pop();return Ok((RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<M>(),..Default::default()}),1));}
        let turn=self.release_empty_page(&mut effect.nested(child));
        Ok((match turn.result{
            Ok(semio_framework_value_persistent::list::PagedListReleaseOutcome::Denied)=>RetainedCloneStep::Progress(Default::default()),
            Ok(semio_framework_value_persistent::list::PagedListReleaseOutcome::Ready(progress))=>RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.progressed),released_bytes:progress.released_allocation_bytes,..Default::default()}),
            Err(failure)=>return Err(release_error(failure)),
        },turn.work))
    }
}
struct ReplayLane<V:'static,S:Staged<V>+'static>{
    staged:ManuallyDrop<Option<S>>,
    pending:ManuallyDrop<Option<V>>,
    active:ManuallyDrop<Option<Box<dyn ErasedSnapshotRetirement>>>,
    factory:ManuallyDrop<Option<Arc<dyn ArtifactOwnedValueRetirementFactory<V>>>>,
    closing_factory:ManuallyDrop<Option<FactoryAuthority>>,
}
type ReplaySnapshotRetirement<P>=ReplayLane<P,ArcStage<P>>;
type ReplayMutationsRetirement<M>=ReplayLane<M,PagedList<M,{usize::MAX}>>;
impl<V:'static,S:Staged<V>+'static> ReplayLane<V,S>{
    fn from_stage(staged:S,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<V>>)->Self{Self{staged:ManuallyDrop::new(Some(staged)),pending:ManuallyDrop::new(None),active:ManuallyDrop::new(None),factory:ManuallyDrop::new(Some(factory)),closing_factory:ManuallyDrop::new(None)}}
    fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        if let Some(active)=self.active.as_ref(){return nested(factory_ticket_demands(active,copy)?);}
        if let Some(value)=self.pending.as_ref(){return nested(self.factory.as_ref().unwrap().retire_owned_demands(value)?);}
        if let Some(staged)=self.staged.as_ref(){return nested(staged.demand()?);}
        if self.factory.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(),depth:1,..Default::default()});}
        self.closing_factory.as_ref().map_or(Ok(Default::default()),|factory|nested(factory.demands(copy)?))
    }
    fn empty(&self)->bool{self.staged.is_none()&&self.pending.is_none()&&self.active.is_none()&&self.factory.is_none()&&self.closing_factory.is_none()}
}
impl<M> ReplayLane<M,PagedList<M,{usize::MAX}>>{fn new(owners:PagedList<M,{usize::MAX}>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<M>>)->Self{Self::from_stage(owners,factory)}}
impl<P> ReplayLane<P,ArcStage<P>>{fn new(owner:Arc<P>,factory:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>)->Self{Self::from_stage(ArcStage(Some(owner)),factory)}}
impl<V:Send+'static,S:Staged<V>+Send+'static> ErasedSnapshotRetirement for ReplayLane<V,S>{
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}if let Some(value)=self.pending.as_ref(){return self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?.retire_owned_work_demand(value)}if self.staged.is_some()||self.factory.is_some(){return Ok(1)}match self.closing_factory.as_ref(){Some(factory) if factory.terminal_is_empty()=>Ok(1),Some(factory)=>factory.next_computational_work(body),None=>Ok(0)}}
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
    fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        let grant=effect.grant();let base=effect.performed_work();let mut work:u128=0;
        let result=(||->Result<RetainedCloneStep,RetirementPollExit>{
        let empty=RetainedCloneProgress::default();if self.empty(){return Ok(RetainedCloneStep::Complete(empty));}if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(empty));}
        let demand=self.demands(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(RetainedCloneStep::Progress(empty));}
        let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};
        if self.active.is_some(){let turn=RetirementPollExit::settled(close_factory_ticket(&mut self.active,body,&mut effect.nested(child)))?;work=turn.work;return turn.result.map_err(Into::into);}
        if self.pending.is_some(){
            let factory=self.factory.as_ref().ok_or_else(||ValueError::invariant("factory absent"))?;
            let turn=match semio_framework_value::factory::factory_retire_owned(&**factory,&mut *self.pending,&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>return Err(RetirementPollExit::Pending(work)),RetainedWorkPoll::Complete(turn)=>turn};
            work=turn.work;let(owner,progress)=turn.result?;*self.active=Some(owner);return Ok(admit_birth(progress,child,demand.capacity_bytes)?);
        }
        if let Some(staged)=self.staged.as_mut(){
            if staged.finished(){drop(self.staged.take());work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..empty}));}
            let(step,spent)=staged.step(&mut self.pending,child,effect)?;work=spent;return Ok(step);
        }
        if let Some(factory)=self.factory.take(){let factory:Arc<dyn semio_framework_value::FactoryRetirement>=factory;*self.closing_factory=Some(FactoryAuthority::new(factory));work=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..empty}));}
        let factory=self.closing_factory.as_mut().ok_or_else(||ValueError::invariant("closing_factory absent"))?;if factory.terminal_is_empty(){drop(self.closing_factory.take());work=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..empty}))}
        let turn=RetirementPollExit::settled(factory.step(body,&mut effect.nested(child)))?;work=turn.work;let step=turn.result?;
        Ok(if self.empty(){RetainedCloneStep::Complete(step.progress())}else{RetainedCloneStep::Progress(step.progress())})
        })();settle_exit(effect,base,result,work)
    }
    fn terminal_is_empty(&self)->bool{self.empty()}
    fn native_header_release(&self)->Option<semio_framework_value::RetirementHeaderRelease>{self.empty().then(||unsafe{semio_framework_value::RetirementHeaderRelease::native(self,0,0)})}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<V:'static,S:Staged<V>+'static> Drop for ReplayLane<V,S>{fn drop(&mut self){assert!(std::thread::panicking()||self.empty(),"native replay lane abandoned owners");}}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"replay cleanup nested depth overflow"))?;Ok(demand)}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::WorkLimit,message)}
/// 🧾️ Runs one physical-only erased close and settles its reported work on the shared effect.
pub(crate) fn erased_close(effect:&mut RetainedAllocationEffect<'_>,close:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,ValueError>)->OriginalRetirementPoll{
    let grant=effect.grant();let base=effect.performed_work();let turn=close(grant);
    settle_exit(effect,base,turn.result.map_err(Into::into),turn.work)
}
/// ⚠️ Keeps the Work refusal or the page release refusal of an empty-page release as one typed value error.
pub(crate) fn release_error(failure:semio_framework_value_persistent::list::PagedListReleaseFailure)->ValueError{match failure{semio_framework_value_persistent::list::PagedListReleaseFailure::Work(error)=>error,semio_framework_value_persistent::list::PagedListReleaseFailure::Release(error)=>ValueError::from(error)}}
/// ⏱️ Records the work a reported total still owes the shared effect, keeping every refusal and Pending child.
pub(crate) fn settle_exit(effect:&mut RetainedAllocationEffect<'_>,base:u128,result:Result<RetainedCloneStep,RetirementPollExit>,work:u128)->OriginalRetirementPoll{
    let done=effect.performed_work().saturating_sub(base);
    let settlement:Result<(),ValueError>=if done>work{Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"replay cleanup recorded more work than it reports"))}else{u64::try_from(work-done).map_err(|_|refusal("replay cleanup work does not fit one original unit")).and_then(|owed|if owed==0||effect.performed(owed)?{Ok(())}else{Err(refusal("replay cleanup work exceeded its original action"))})};
    let complete=|result|RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work});
    match(result,settlement){
        (Ok(step),Ok(()))=>complete(Ok(step)),
        (Ok(step),Err(error))=>complete(Err(OriginalRetirementFailure::from_work_refusal(error.with_retained_progress(step.progress())))),
        (Err(RetirementPollExit::Failed(failure)),Ok(()))=>complete(Err(failure)),
        (Err(RetirementPollExit::Failed(failure)),Err(error))=>complete(Err(OriginalRetirementFailure::from_owned_parts(None,Some(error),None,None,Some(Box::new(failure)),None))),
        (Err(RetirementPollExit::Pending(child)),Ok(()))=>RetirementPollExit::finish::<RetainedCloneStep>(Err(RetirementPollExit::Pending(child)),work),
        (Err(RetirementPollExit::Pending(_)),Err(error))=>complete(Err(OriginalRetirementFailure::from_work_refusal(error))),
    }
}
fn admit_birth(progress:RetainedCloneProgress,grant:RetainedCloneGrant,bytes:usize)->Result<RetainedCloneStep,ValueError>{if !progress.fits(grant)||progress.retained_capacity_bytes!=bytes{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"replay cleanup constructor changed its receipt"));}Ok(RetainedCloneStep::Progress(progress))}

#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
