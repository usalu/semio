//! 🌳️ A returned Arc root elects its unique payload, converts it once into original shared storage and retires it through its captured issuer.
use crate::{OriginalRetirementCloseLoan,ArtifactOwnedValueRetirementFactory,SnapshotRetirementFactory,ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,OriginalRetirementPoll,OriginalRetirementFailure,RetainedAllocationEffect,FactoryProducerReply};
use crate::retirement::{record_retirement_work,arc_custody::{arc_bytes,original::SharedOriginalStorage}};
use crate::retained_clone::{retained_clone_grant_admits,admit_retained_clone_close,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_retirement::{FactoryTicketClose,RetirementHeaderRelease};
use super::FactorySharedRetirement;
use std::{mem::{size_of,size_of_val,ManuallyDrop},sync::Arc};
type Retired=Box<dyn ErasedSnapshotRetirement>;
fn done(result:Result<RetainedCloneStep,OriginalRetirementFailure>,work:u128)->OriginalRetirementPoll{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
fn refuse(message:&'static str)->OriginalRetirementPoll{done(Err(ValueError::invariant(message).into()),0)}
/// 📦️ Quotes a boxed child owner, one depth level deeper than the child.
pub fn box_demands(owner:&Retired,maximum_body_bytes:usize)->Result<RetirementDemand,ValueError>{
    if owner.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}
    let mut demand=owner.next_demand(maximum_body_bytes)?;
    demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"boxed retirement depth overflow"))?;
    Ok(demand)
}
/// 📦️ Quotes the depth of a boxed child owner.
pub fn box_depth(owner:&Retired)->Result<usize,ValueError>{
    if owner.terminal_is_empty(){return Ok(1)}
    owner.next_depth_demand()?.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"boxed retirement depth overflow"))
}
/// 📦️ Advances a boxed child owner one funded step and releases its box once it is terminal.
pub fn box_close_step(slot:&mut Option<Retired>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
    let grant=effect.grant();let empty=RetainedCloneProgress::default();
    let Some(owner)=slot.as_mut()else{return done(Ok(RetainedCloneStep::Complete(empty)),0)};
    if grant.maximum_items==0||effect.maximum_work_units()==0{return done(Ok(RetainedCloneStep::Progress(empty)),0)}
    let depth=match box_depth(owner){Ok(depth)=>depth,Err(error)=>return done(Err(error.into()),0)};
    if grant.maximum_depth<depth{return done(Err(ValueError::literal(ValueRefusalKind::DepthLimit,"boxed retirement exceeds admitted depth").into()),0)}
    if owner.terminal_is_empty(){
        let extent=size_of_val(owner.as_ref());
        if grant.maximum_release_bytes<extent{return done(Ok(RetainedCloneStep::Progress(empty)),0)}
        let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};
        drop(slot.take());
        return done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:extent,..empty})),work)
    }
    let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
    let turn=match owner.close_step(body,&mut effect.nested(child)){RetainedWorkPoll::Pending{work}=>return RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>turn};
    let step=match turn.result{Ok(step)=>step,Err(error)=>return done(Err(error),turn.work)};
    match admit_retained_clone_close(child,step,owner.terminal_is_empty(),"boxed retirement child"){Ok(step)=>done(Ok(RetainedCloneStep::Progress(step.progress())),turn.work),Err(error)=>done(Err(error.into()),turn.work)}
}
enum RootIssuer<P:Send+Sync+'static>{Owned(Arc<dyn ArtifactOwnedValueRetirementFactory<P>>),Snapshot(Arc<dyn SnapshotRetirementFactory<P>>)}
/// 🏛️ The one Arc-root retirement: alias, elected payload, shared storage and captured issuer stay resident until physical return.
pub struct ArcRootRetirement<P:Send+Sync+'static>{alias:ManuallyDrop<Option<Arc<P>>>,elected:ManuallyDrop<Option<P>>,storage:ManuallyDrop<Option<SharedOriginalStorage<P>>>,issuer:Option<RootIssuer<P>>,active:ManuallyDrop<Option<Retired>>,ticket:FactoryTicketClose}
impl<P:Send+Sync+'static> ArcRootRetirement<P>{
    pub const fn constructor_capacity_bytes()->usize{size_of::<Self>()}
    pub fn admission_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<Arc<P>>>()+size_of::<Option<RootIssuer<P>>>()+size_of::<Self>()+size_of::<Option<Retired>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
    pub fn admit_original_source_work_demand(original:&Option<Arc<P>>)->u64{u64::from(original.is_some())}
    pub fn original_admission_demands(original:&Option<Arc<P>>)->RetirementDemand{if original.is_none(){Default::default()}else{Self::admission_demand()}}
    fn from_parts(alias:Option<Arc<P>>,issuer:Option<RootIssuer<P>>)->Self{Self{alias:ManuallyDrop::new(alias),elected:ManuallyDrop::new(None),storage:ManuallyDrop::new(None),issuer,active:ManuallyDrop::new(None),ticket:FactoryTicketClose::vacant()}}
    /// 🆕️ Takes custody of a root and its owned-value issuer before any election.
    pub fn new(alias:Arc<P>,issuer:Arc<dyn ArtifactOwnedValueRetirementFactory<P>>)->Self{Self::from_parts(Some(alias),Some(RootIssuer::Owned(issuer)))}
    /// 🆕️ Takes custody of a root and its snapshot issuer before any election.
    pub fn new_snapshot(alias:Arc<P>,issuer:Arc<dyn SnapshotRetirementFactory<P>>)->Self{Self::from_parts(Some(alias),Some(RootIssuer::Snapshot(issuer)))}
    fn admit(original:&mut Option<Arc<P>>,issuer:Option<RootIssuer<P>>,output:&mut Option<Retired>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        if output.is_some()||effect.actual().is_some(){return refuse("store root output or allocation marker occupied")}
        if original.is_none(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
        if issuer.is_none(){return refuse("store root issuer is absent")}
        let demand=Self::admission_demand();
        if effect.maximum_work_units()==0||!retained_clone_grant_admits(effect.grant(),demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
        let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};
        *effect.actual()=Some(0);let layout=std::alloc::Layout::new::<Self>();
        let Some(pointer)=std::ptr::NonNull::new(unsafe{std::alloc::alloc(layout)}.cast::<Self>())else{return done(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"store root frame allocation failed").into()),work)};
        *effect.actual()=Some(layout.size());
        let owner=unsafe{pointer.as_ptr().write(Self::from_parts(original.take(),issuer));Box::from_raw(pointer.as_ptr())};*output=Some(owner);
        done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:layout.size(),..Default::default()})),work)
    }
    /// 🪑️ Moves the root and an owned-value issuer into one resident frame.
    pub fn admit_original(original:&mut Option<Arc<P>>,factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<P>>>,output:&mut Option<Retired>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{Self::admit(original,factory.take().map(RootIssuer::Owned),output,effect)}
    /// 🪑️ Moves the root and a snapshot issuer into one resident frame.
    pub fn admit_snapshot(original:&mut Option<Arc<P>>,factory:&mut Option<Arc<dyn SnapshotRetirementFactory<P>>>,output:&mut Option<Retired>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{Self::admit(original,factory.take().map(RootIssuer::Snapshot),output,effect)}
    fn terminal(&self)->bool{self.alias.is_none()&&self.elected.is_none()&&self.storage.is_none()&&self.active.is_none()&&self.ticket.is_empty()}
    fn lost()->ValueError{ValueError::invariant("store root storage lost its issuer")}
    fn child_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        let mut demand=self.ticket.next_demand(&self.active,body)?;
        demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"store root child depth overflow"))?;
        Ok(demand)
    }
    fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{
        if self.active.is_none()&&self.ticket.is_empty(){
            if recover{return Err(ValueError::invariant("store root has no original child to recover"))}
            return match self.stage(body,effect){RetainedWorkPoll::Complete(turn)=>Ok(reply.complete(turn.result)),RetainedWorkPoll::Pending{..}=>Err(ValueError::invariant("store root stage left its producer reply pending"))}
        }
        let grant=effect.grant();
        if grant.maximum_items==0||effect.maximum_work_units()==0{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
        let depth=grant.maximum_depth.checked_sub(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"store root child depth absent"))?;
        let remaining=effect.remaining_work_units()?;let mut child=effect.reborrow(RetainedCloneGrant{maximum_depth:depth,..grant},remaining)?;let reply=reply.progress();
        match(original,recover){
            (Some(original),true)=>self.ticket.recover_close_original_into(&mut self.active,original,body,&mut child,reply),
            (Some(original),false)=>self.ticket.close_original_into(&mut self.active,original,body,&mut child,reply),
            (None,true)=>self.ticket.recover_close_into(&mut self.active,body,&mut child,reply),
            (None,false)=>self.ticket.close_into(&mut self.active,body,&mut child,reply),
        }
    }
    fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(alias)=self.alias.as_ref(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Arc<P>>>()+size_of::<Option<P>>(),release_bytes:if Arc::weak_count(alias)==0{arc_bytes::<P>()}else{0},depth:1,..Default::default()})}
        if self.active.is_some(){return self.child_demands(body)}
        if let Some(storage)=self.storage.as_ref(){return match self.issuer.as_ref(){Some(RootIssuer::Snapshot(issuer))=>issuer.retire_demands(storage),Some(RootIssuer::Owned(_))=>Ok(FactorySharedRetirement::<P>::original_admission_demands(&self.storage)),None=>Err(Self::lost())}}
        if self.elected.is_some(){return SharedOriginalStorage::<P>::source_admission_demand()}
        Ok(Default::default())
    }
    fn work_demand(&self,body:usize)->Result<u64,ValueError>{
        if self.alias.is_some(){return Ok(1)}
        if self.active.is_some(){return self.ticket.next_work(&self.active,body)}
        if let Some(storage)=self.storage.as_ref(){return match self.issuer.as_ref(){Some(RootIssuer::Snapshot(issuer))=>issuer.retire_work_demand(storage),Some(RootIssuer::Owned(_))=>Ok(FactorySharedRetirement::<P>::admit_original_source_work_demand(&self.storage)),None=>Err(Self::lost())}}
        if self.elected.is_some(){return Ok(SharedOriginalStorage::<P>::source_admission_work_demand())}
        Ok(0)
    }
    fn elect(&mut self,effect:&mut RetainedAllocationEffect<'_>,demand:RetirementDemand)->OriginalRetirementPoll{
        let work=match record_retirement_work(effect,1){Ok(work)=>work,Err((error,work))=>return done(Err(error.into()),work)};
        let value=Arc::into_inner(self.alias.take().expect("funded store root alias"));
        let released_bytes=if value.is_some(){demand.release_bytes}else{0};
        *self.elected=value;
        done(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes,..Default::default()})),work)
    }
    fn convert(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        let turn=SharedOriginalStorage::admit_from(&mut self.elected,&mut self.storage,effect);
        done(turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into),turn.work)
    }
    fn retire(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        let boundary=effect.performed_boundary();let(mut output,mut reply,mut attempted)=(None,None,None);let mut slot=None;
        let answer=(||->Result<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>,ValueError>{
            match self.issuer.as_mut(){
                Some(RootIssuer::Snapshot(issuer))=>{let borrowed=FactoryProducerReply::borrow(&mut output,&mut reply,&mut attempted)?;let _=issuer.retire(&mut self.storage,effect,borrowed);Ok(reply.take().map(|turn|turn.result.map(|progress|{if let Some(owner)=output.take(){*self.active=Some(owner)}RetainedCloneStep::Progress(progress)})))}
                Some(RootIssuer::Owned(factory))=>{let mut captured=Some(Arc::clone(factory));let reply=OriginalEffectReply::borrow(&mut slot)?;drop(FactorySharedRetirement::<P>::admit_original_into(&mut self.storage,&mut captured,&mut self.active,effect,reply)?);Ok(slot.take())}
                None=>Err(Self::lost()),
            }
        })();
        let work=match effect.performed_since(boundary){Ok(work)=>work,Err(error)=>return done(Err(error.into()),0)};
        match answer{Ok(Some(result))=>done(result,work),Ok(None)=>done(Err(ValueError::invariant("store root retirement left its producer reply pending").into()),work),Err(error)=>done(Err(error.into()),work)}
    }
    fn stage(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
        if self.terminal(){return done(Ok(RetainedCloneStep::Complete(Default::default())),0)}
        let grant=effect.grant();
        let(demand,work)=match(self.demands(body),self.work_demand(body)){(Ok(demand),Ok(work))=>(demand,work),(Err(error),_)|(_,Err(error))=>return done(Err(error.into()),0)};
        if grant.maximum_items==0||effect.maximum_work_units()<work||!retained_clone_grant_admits(grant,demand){return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
        if self.alias.is_some(){return self.elect(effect,demand)}
        if self.storage.is_some(){return self.retire(effect)}
        self.convert(effect)
    }
}
impl<P:Send+Sync+'static> ErasedSnapshotRetirement for ArcRootRetirement<P>{
    fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.work_demand(body)}
    fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demands(body)}
    fn native_header_release(&self)->Option<RetirementHeaderRelease>{self.terminal().then(||unsafe{RetirementHeaderRelease::native(self,0,0)})}
    fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
    fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
    fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
    fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
    fn requires_original_birth(&self)->bool{self.active.as_ref().is_some_and(|child|!child.terminal_is_empty()&&child.requires_original_birth())}
    fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
        let Some(depth)=grant.maximum_depth.checked_sub(1)else{return RetainedWorkPoll::Pending{work:0}};
        match self.active.as_mut(){Some(child)=>child.close_original_birth_step(original,RetainedCloneGrant{maximum_depth:depth,..grant}),None=>refuse("store root has no selected original birth")}
    }
    fn requires_original_control(&self)->bool{self.active.as_ref().is_some_and(|child|!child.terminal_is_empty()&&child.requires_original_control())}
    fn close_step(&mut self,_body:usize,_effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{done(Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"store root retirement requires its borrowed producer").into()),0)}
    fn terminal_is_empty(&self)->bool{self.terminal()}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
    fn next_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.demands(body)?.capacity_bytes)}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
}
impl<P:Send+Sync+'static> Drop for ArcRootRetirement<P>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal(),"store root retirement abandoned its payload");if self.terminal(){unsafe{ManuallyDrop::drop(&mut self.alias);ManuallyDrop::drop(&mut self.elected);ManuallyDrop::drop(&mut self.storage);ManuallyDrop::drop(&mut self.active);}}}}
