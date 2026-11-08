//! 🪪️ Erased read custody preserves the original typed lease and its independently funded frame.
use super::{ReadLease,ReadLeaseId};
use crate::{RetainedWorkOutcome,ErasedSnapshotRetirement,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep},retirement::{RetireOwned,RetirementCursor,RetirementStep}};
use std::{any::Any,mem::ManuallyDrop};

trait LeaseView:ErasedSnapshotRetirement {
    fn root(&self)->Option<&dyn Any>;
    fn id(&self)->ReadLeaseId;
    fn work(&self)->Option<crate::retained_clone::OriginalWorkBindingIdentity>;
    fn matches(&self,generation:u64,revision:[u8;32])->bool;
}
impl<T:RetireOwned+Sync> LeaseView for ReadLease<T> {
    fn root(&self)->Option<&dyn Any> {self.owner.as_ref().map(|owner|owner.get() as &dyn Any)}
    fn id(&self)->ReadLeaseId {self.id()}
    fn work(&self)->Option<crate::retained_clone::OriginalWorkBindingIdentity>{self.work}
    fn matches(&self,generation:u64,revision:[u8;32])->bool {self.commit_authority_matches(generation,revision)}
}

/// 🔒️ One admitted concrete lease frame retains its original root, registry and closing cursor.
pub struct ErasedReadLease {lease:ManuallyDrop<Option<Box<dyn LeaseView>>>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>>>,route:Option<(crate::retained_clone::OriginalEffectInvocationIdentity,crate::retained_clone::OriginalWorkActionBoundary,RetainedCloneGrant,u64,Option<(semio_framework_value_control_contract::NativeAllocationIdentity,Option<u64>,crate::retained_clone::OriginalWorkBindingIdentity)>,usize)>}
impl ErasedReadLease {
    pub fn terminal_is_empty(&self)->bool {self.lease.is_none()&&self.reply.is_none()&&self.route.is_none()}
    pub fn next_depth_demand(&self)->Result<usize,ValueError> {Ok(self.demands(0)?.depth)}
    /// 📏️ Quotes the resident typed input, native allocation and erased destination.
    pub fn admission_demand<T:RetireOwned+Sync>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<ReadLease<T>>>()+size_of::<ReadLease<T>>()+size_of::<Option<Self>>()+2*size_of::<Option<usize>>(),capacity_bytes:size_of::<ReadLease<T>>(),depth:1,..Default::default()}}
    /// 📥️ Publishes the actual typed lease in its paid erased native owner.
    pub fn admit_into<'r,T:RetireOwned+Sync>(input:&mut Option<ReadLease<T>>,output:&mut Option<Self>,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{let lease=input.as_ref().ok_or_else(||ValueError::invariant("erased read input absent"))?;if !lease.work.is_some_and(|work|work.matches_effect(effect)){return Err(ValueError::invariant("erased read changed original Work"))}if output.is_some()||effect.actual().is_some(){return Err(ValueError::invariant("erased read destination occupied"))}let demand=Self::admission_demand::<T>();if effect.remaining_work_units()?<1||!crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"erased read admission exceeded original Work").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}let layout=std::alloc::Layout::new::<ReadLease<T>>();*effect.actual()=Some(0);let raw=unsafe{std::alloc::alloc(layout)}.cast::<ReadLease<T>>();if raw.is_null(){return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"erased read native allocation refused").into())))}*effect.actual()=Some(layout.size());unsafe{raw.write(input.take().expect("validated erased read input"))};let lease:Box<dyn LeaseView>=unsafe{Box::from_raw(raw)};*output=Some(Self{lease:ManuallyDrop::new(Some(lease)),reply:ManuallyDrop::new(None),route:None});Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))))}
    pub(super) fn original_work(&self)->Option<crate::retained_clone::OriginalWorkBindingIdentity>{self.lease.as_ref().and_then(|lease|lease.work())}
    pub fn get<T:'static>(&self)->Option<&T> {self.lease.as_ref()?.root()?.downcast_ref()}
    pub fn id(&self)->Option<ReadLeaseId> {self.lease.as_ref().map(|lease|lease.id())}
    pub fn commit_authority_matches(&self,generation:u64,revision:[u8;32])->bool {self.lease.as_ref().is_some_and(|lease|lease.matches(generation,revision))}
    pub fn demands(&self,copy:usize)->Result<RetirementDemand,ValueError> {
        let Some(lease)=self.lease.as_ref()else{return Ok(Default::default());};
        if lease.terminal_is_empty(){let release=lease.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"read lease has no native terminal header witness"))?;return release.demand(lease.as_ref())}
        lease.next_demand(copy)
    }
}
type ReadReply<'r>=crate::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>;
type ReadComplete<'r>=crate::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,semio_framework_value_retirement::OriginalRetirementFailure>;
type CursorReply<'r>=crate::retained_clone::OriginalEffectReply<'r,RetirementStep,semio_framework_value_retirement::OriginalRetirementFailure>;
type CursorComplete<'r>=crate::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,semio_framework_value_retirement::OriginalRetirementFailure>;
impl ErasedReadLease{
 fn dispatch<'r>(lease:&mut Option<Box<dyn LeaseView>>,mut original:Option<&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>,recover:bool)->Result<ReadComplete<'r>,ValueError>{if let Some(loan)=original.as_mut(){let work=loan.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("read lease original Work changed"))}}let Some(owner)=lease.as_mut()else{if recover{return Err(ValueError::invariant("read lease recovery owner absent"))}return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))};if recover{return match original{Some(loan)=>owner.recover_close_original_into(loan,body,effect,reply.progress()),None=>owner.recover_close_into(body,effect,reply.progress())}}if owner.terminal_is_empty(){if original.is_some(){return Err(ValueError::invariant("read lease terminal header does not need original control"))}let release=owner.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"read lease has no native terminal header witness"))?;return release.release_into(lease,effect,reply.progress())}match original{Some(loan)=>owner.close_original_into(loan,body,effect,reply.progress()),None=>owner.close_into(body,effect,reply.progress())}}
 fn cursor_drive<'r>(&mut self,mut original:Option<&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CursorReply<'r>,recover:bool)->Result<CursorComplete<'r>,ValueError>{use crate::retained_clone::OriginalEffectCallbackMode;let invocation=effect.invocation().ok_or_else(||ValueError::invariant("read lease requires original invocation"))?;let mode=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive};if invocation.mode()!=mode||invocation.source_turn()!=effect.work_source_turn(){return Err(ValueError::invariant("read lease invocation changed"))}let binding=match original.as_mut(){Some(loan)=>{let work=loan.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(ValueError::invariant("read lease original Work changed"))}Some((loan.ledger_identity(),loan.retirement_recipient_identity(),work))},None=>None};let key=if recover{let key=self.route.ok_or_else(||ValueError::invariant("read lease recovery absent"))?;if key.0!=invocation.identity()||key.2!=effect.grant()||key.3!=effect.maximum_work_units()||key.4!=binding||key.5!=body{return Err(ValueError::invariant("read lease recovery changed original request"))}key}else{if self.route.is_some(){return Err(ValueError::invariant("read lease action occupied"))}(invocation.identity(),effect.action_boundary(),effect.grant(),effect.maximum_work_units(),binding,body)};let Self{lease,reply:child,route}=self;let mut scope=reply.scope(route,key)?;if child.is_none(){let inner=crate::retained_clone::OriginalEffectReply::borrow(child)?;drop(Self::dispatch(lease,original,body,effect,inner,recover)?)}if child.is_none(){return Ok(scope.forwarded())}let work=effect.performed_since_action(key.1)?;let output=scope.borrow()?;let Some(result)=child.take()else{return Err(ValueError::invariant("recorded read child reply disappeared"))};let result=result.map(|step|RetirementStep::Progress(step.progress(),work));output.complete(result);Ok(scope.forwarded())}
}
impl ErasedSnapshotRetirement for ErasedReadLease {
 fn native_header_release(&self)->Option<semio_framework_value_retirement::RetirementHeaderRelease>{self.terminal_is_empty().then(||unsafe{semio_framework_value_retirement::RetirementHeaderRelease::native(self,0,0)})}
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{if self.route.is_some()||self.reply.is_some(){return Err(ValueError::invariant("read lease retains a cursor action"))}Self::dispatch(&mut self.lease,None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{if self.route.is_some()||self.reply.is_some(){return Err(ValueError::invariant("read lease retains a cursor action"))}Self::dispatch(&mut self.lease,None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{if self.route.is_some()||self.reply.is_some(){return Err(ValueError::invariant("read lease retains a cursor action"))}Self::dispatch(&mut self.lease,Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadComplete<'r>,ValueError>{if self.route.is_some()||self.reply.is_some(){return Err(ValueError::invariant("read lease retains a cursor action"))}Self::dispatch(&mut self.lease,Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.reply.is_none()&&self.lease.as_ref().is_some_and(|owner|owner.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->semio_framework_value_retirement::OriginalRetirementPoll{match self.lease.as_mut(){Some(owner)=>owner.close_original_birth_step(original,grant),None=>crate::retained_clone::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::invariant("read lease Birth absent").into()),work:0})}}
 fn requires_original_control(&self)->bool{if let Some(route)=self.route{return route.4.is_some()}self.lease.as_ref().is_some_and(|owner|owner.requires_original_control())}
 fn close_step(&mut self,_:usize,_:&mut crate::RetainedAllocationEffect<'_>)->semio_framework_value_retirement::OriginalRetirementPoll{crate::retained_clone::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"erased read requires borrowed close").into()),work:0})}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(route)=self.route{if route.5!=body{return Err(ValueError::invariant("read close Work query changed original body"))}return Ok(route.3)}let Some(owner)=self.lease.as_ref()else{return Ok(0)};if owner.terminal_is_empty(){owner.native_header_release().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"read terminal witness absent"))?.work()}else{owner.next_computational_work(body)}}
 fn next_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{Ok(self.demands(copy)?.capacity_bytes)}
 fn next_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.depth)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some((_,_,grant,_,_,selected_body))=self.route{if selected_body!=body{return Err(ValueError::invariant("read close query changed original body"))}return Ok(RetirementDemand{copy_bytes:grant.maximum_copy_bytes,capacity_bytes:grant.maximum_capacity_bytes,release_bytes:grant.maximum_release_bytes,depth:grant.maximum_depth})}self.demands(body)}
}
impl RetirementCursor for ErasedReadLease {
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CursorReply<'r>)->Result<CursorComplete<'r>,ValueError>{if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}self.cursor_drive(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CursorReply<'r>)->Result<CursorComplete<'r>,ValueError>{self.cursor_drive(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CursorReply<'r>)->Result<CursorComplete<'r>,ValueError>{self.cursor_drive(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:CursorReply<'r>)->Result<CursorComplete<'r>,ValueError>{self.cursor_drive(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{ErasedSnapshotRetirement::requires_original_birth(self)}
 fn close_original_birth_step(&mut self,original:&mut semio_framework_value_retirement::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{match ErasedSnapshotRetirement::close_original_birth_step(self,original,grant){crate::retained_clone::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),crate::retained_clone::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}}
 fn requires_original_control(&self)->bool{ErasedSnapshotRetirement::requires_original_control(self)}
 fn terminal_is_empty(&self)->bool{Self::terminal_is_empty(self)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{ErasedSnapshotRetirement::next_demand(self,body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{ErasedSnapshotRetirement::next_computational_work(self,body)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demands(0)?.copy_bytes)}
 fn next_birth_bytes(&self,copy:usize)->Option<usize>{Some(self.demands(copy).ok()?.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.demands(0).ok()?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{ErasedSnapshotRetirement::next_depth_demand(self)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl RetireOwned for ErasedReadLease {
    type RetirementCursor=Self;fn retirement_seed(self)->Self {self}
    fn retirement_birth_bytes(&self)->Option<usize> {Some(size_of::<Self>())}
    fn controlled_retirement_supported()->bool {true}
}
impl Drop for ErasedReadLease {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"erased read must close its original lease and funded frame");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.lease);ManuallyDrop::drop(&mut self.reply);}}}}
