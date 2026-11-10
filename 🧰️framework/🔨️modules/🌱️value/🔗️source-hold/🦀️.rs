//! 🔗️ Keeps one original Source authority, its projection action and its handback in resident places across edit turns and cancellation.
use semio_framework_value::{ValueRefusalKind,OriginalRetirementCloseLoan,OriginalRetirementFailure,RetainedWorkPoll,RetirementDemand,ValueError,retained_clone::{OriginalEffectAction,OriginalEffectCallbackMode,OriginalEffectInvocationIdentity,OriginalEffectReply,OriginalSourceMode,OriginalSourceOwner,OriginalSourceProjection,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneStep,retained_clone_grant_admits},retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement}};
use std::mem::{ManuallyDrop,size_of};

type EntityReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type EntityComplete<'r>=semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;
fn route_fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
type ValueReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,ValueError>;
type Route=(OriginalEffectInvocationIdentity,OriginalSourceMode,usize);
fn forward<'r>(resident:&mut Option<Result<RetainedCloneStep,ValueError>>,reply:EntityReply<'r>)->Result<EntityComplete<'r>,ValueError>{match resident.take(){Some(result)=>Ok(reply.progress().complete(result.map_err(OriginalRetirementFailure::from))),None=>Ok(reply.pending())}}

/// 🔗️ Owns the staged original, the actual Source owner and the projection action; its retirement hands every Source back and retires the original.
pub struct SourceHold<T:Sync+'static,A:OriginalSourceProjection<T>+Send>{staged:ManuallyDrop<Option<A>>,owner:OriginalSourceOwner<T,A>,inner:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>,inner_reply:ManuallyDrop<Option<Result<RetainedCloneStep,ValueError>>>,rest:ManuallyDrop<Option<ControlledRetirement<A>>>,route:Option<Route>}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> SourceHold<T,A>{
 pub fn new(original:A)->Self{Self{staged:ManuallyDrop::new(Some(original)),owner:OriginalSourceOwner::vacant(),inner:None,inner_reply:ManuallyDrop::new(None),rest:ManuallyDrop::new(None),route:None}}
 fn captured(&self)->bool{!self.owner.terminal_is_empty()}
 pub fn ready(&self)->bool{self.captured()&&self.route.is_none()&&self.owner.demand(OriginalSourceMode::Original,0).is_ok_and(|demand|demand.is_none())}
 pub fn original(&self)->Result<RetainedCloneRef<'_,T>,ValueError>{self.owner.original()}
 pub fn quote(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.captured(){return Ok(OriginalSourceOwner::<T,A>::capture_demand());}
  Ok(self.owner.demand(OriginalSourceMode::Original,body)?.unwrap_or_default())
 }
 pub fn work_quote(&self,body:usize)->Result<u64,ValueError>{if !self.captured(){return Ok(1);}self.owner.work_demand(OriginalSourceMode::Original,body)}
 pub fn step_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  if !self.captured(){
   if recover{return Err(route_fault("source hold capture has no unrecorded callback"));}
   let mut inner=None;drop(self.owner.capture_into(&mut self.staged,effect,ValueReply::borrow(&mut inner)?)?);return forward(&mut inner,reply);
  }
  self.drive(OriginalSourceMode::Original,body,None,effect,reply,recover)
 }
 fn drive<'r>(&mut self,mode:OriginalSourceMode,body:usize,loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  let invocation=effect.invocation().ok_or_else(||route_fault("source hold lacks its native invocation"))?;
  if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}{return Err(route_fault("source hold callback mode changed"));}
  let key=(invocation.identity(),mode,body);if recover&&self.route!=Some(key)||!recover&&self.route.is_some(){return Err(route_fault("source hold route changed"));}
  let state=self;let mut scope=reply.progress().scope(&mut state.route,key)?;
  if state.owner.projection_drive_required_for(mode)?{
   let frame=state.owner.projection_frame(effect)?;let mut context=frame.context()?;let ready=frame.authority()?.drive(&mut context,effect,&mut state.inner)?;
   if ready{
    if !state.inner.as_mut().ok_or_else(||route_fault("source hold projection action is absent"))?.transfer_reply(&mut state.inner_reply)?{return Ok(scope.forwarded());}
    frame.authority()?.finish(&mut state.inner)?;drop(forward(&mut state.inner_reply,scope.borrow()?)?);
   }
  }else{
   let done=match(loan,recover){(None,false)=>state.owner.prepare_into(mode,body,effect,scope.borrow()?)?,(None,true)=>state.owner.recover_prepare_into(mode,body,effect,scope.borrow()?)?,(Some(loan),false)=>state.owner.prepare_original_into(mode,body,loan,effect,scope.borrow()?)?,(Some(loan),true)=>state.owner.recover_prepare_original_into(mode,body,loan,effect,scope.borrow()?)?};drop(done);
  }
  Ok(scope.forwarded())
 }
 fn unwind_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(rest)=self.rest.as_ref(){return Ok(if rest.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<A>>>(),depth:1,..Default::default()}}else{rest.selected_demand(body)?});}
  if self.captured(){return Ok(match self.owner.demand(OriginalSourceMode::Unique,body)?{Some(demand)=>demand,None=>self.owner.return_demand()});}
  Ok(if self.staged.is_some(){RetirementDemand{copy_bytes:size_of::<Option<A>>()+size_of::<ControlledRetirement<A>>(),depth:1,..Default::default()}}else{Default::default()})
 }
 fn unwind_work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(rest)=self.rest.as_ref(){return if rest.terminal_is_empty(){Ok(1)}else{rest.next_computational_work(body)};}
  if self.captured(){return if self.owner.demand(OriginalSourceMode::Unique,body)?.is_some(){self.owner.work_demand(OriginalSourceMode::Unique,body)}else{Ok(1)};}
  Ok(u64::from(self.staged.is_some()))
 }
 fn unwind<'r>(&mut self,body:usize,mut loan:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:EntityReply<'r>,recover:bool)->Result<EntityComplete<'r>,ValueError>{
  if recover{return self.drive(OriginalSourceMode::Unique,body,loan,effect,reply,true);}
  let demand=self.unwind_demand(body)?;let units=self.unwind_work(body)?;
  if units==0{return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))));}
  if !retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<units{return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))));}
  let item=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
  if let Some(rest)=self.rest.as_mut(){
   if rest.terminal_is_empty(){if !effect.performed(1)?{return Err(route_fault("source hold original clear exceeded Work"));}self.rest=ManuallyDrop::new(None);return Ok(reply.complete(Ok(RetainedCloneStep::Progress(item))));}
   return match loan{Some(loan)=>rest.step_original_into(loan,body,effect,reply),None=>rest.step_into(body,effect,reply)};
  }
  if self.captured(){
   if self.owner.demand(OriginalSourceMode::Unique,body)?.is_none(){let mut inner=None;drop(self.owner.return_into(&mut self.staged,effect,ValueReply::borrow(&mut inner)?)?);return forward(&mut inner,reply);}
   return self.drive(OriginalSourceMode::Unique,body,loan.take(),effect,reply,false);
  }
  if !effect.performed(1)?{return Err(route_fault("source hold original wrap exceeded Work"));}
  let original=self.staged.take().ok_or_else(||route_fault("source hold original is absent"))?;
  match ControlledRetirement::new(original){Ok(rest)=>self.rest=ManuallyDrop::new(Some(rest)),Err((error,original))=>{self.staged=ManuallyDrop::new(Some(original));return Err(error);}}
  Ok(reply.complete(Ok(RetainedCloneStep::Progress(item))))
 }
 fn empty(&self)->bool{self.staged.is_none()&&self.rest.is_none()&&self.owner.terminal_is_empty()&&self.inner.is_none()&&self.inner_reply.is_none()&&self.route.is_none()}
}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> Drop for SourceHold<T,A>{fn drop(&mut self){if std::thread::panicking(){return;}assert!(self.empty(),"source hold retains its original authority or action");unsafe{ManuallyDrop::drop(&mut self.staged);ManuallyDrop::drop(&mut self.inner_reply);ManuallyDrop::drop(&mut self.rest);}}}

#[doc(hidden)]
pub struct HoldRetirement<T:Sync+'static,A:OriginalSourceProjection<T>+Send>{hold:ManuallyDrop<SourceHold<T,A>>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,route:Option<u128>}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> HoldRetirement<T,A>{
 fn relay<'r>(&mut self,body:usize,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  let invalid=||route_fault("source hold retirement lost its retained reply");
  if recover{if self.route.is_none(){return Err(invalid());}}else{if self.route.is_some()||self.reply.is_some(){return Err(invalid());}self.route=Some(effect.performed_work());}
  if self.reply.is_none(){let done=self.hold.unwind(body,original.take(),effect,OriginalEffectReply::borrow(&mut self.reply)?,recover)?;drop(done);}
  let Some(result)=self.reply.take()else{return Ok(reply.pending());};
  let start=self.route.take().ok_or_else(invalid)?;let work=effect.performed_work().checked_sub(start).ok_or_else(invalid)?;
  Ok(reply.complete(result.map(|step|RetirementStep::Progress(step.progress(),work))))
 }
}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> RetirementCursor for HoldRetirement<T,A>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.hold.unwind_demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.hold.unwind_work(body)}
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,None,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<semio_framework_value::retained_clone::OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.relay(body,Some(original),effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.hold.owner.requires_original_birth()}
 fn requires_original_control(&self)->bool{self.hold.owner.requires_original_control()||self.hold.rest.as_ref().is_some_and(|rest|rest.requires_original_control())}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{match self.hold.owner.original_birth_step(original,grant){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}}
 fn terminal_is_empty(&self)->bool{self.route.is_none()&&self.reply.is_none()&&self.hold.empty()}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_selected_work_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.next_demand(body)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|demand|demand.release_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.next_demand(body).ok().map(|demand|demand.capacity_bytes)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> Drop for HoldRetirement<T,A>{fn drop(&mut self){if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.hold);ManuallyDrop::drop(&mut self.reply)}}else if !std::thread::panicking(){panic!("source hold retirement retains its original authority")}}}
impl<T:Sync+'static,A:OriginalSourceProjection<T>+Send> RetireOwned for SourceHold<T,A>{
 type RetirementCursor=HoldRetirement<T,A>;
 fn retirement_seed(self)->Self::RetirementCursor{HoldRetirement{hold:ManuallyDrop::new(self),reply:ManuallyDrop::new(None),route:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<HoldRetirement<T,A>>())}
 fn controlled_retirement_supported()->bool{A::controlled_retirement_supported()}
}
