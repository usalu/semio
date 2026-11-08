//! ♻️ Ordered metadata keeps every elected original and arbitrary payload in resident custody.
use super::{OrderedMap,Retirement,SharedOwner,UpdateCursor,LookupCursor};
use crate::{ValueError,OriginalRetirementFailure,OriginalRetirementCloseLoan,OriginalEffectReply,OriginalEffectReplyComplete,RetainedAllocationEffect,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use std::mem::{ManuallyDrop,size_of};

trait RootOwner<V>:Send{
 fn native_demand(&self)->Result<RetirementDemand,ValueError>;
 fn native_work(&self)->Result<u64,ValueError>;
 fn native_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep>;
 fn payload(&self)->&Option<V>;
 fn payload_mut(&mut self)->&mut Option<V>;
 fn is_empty(&self)->bool;
}
impl<V:RetireOwned+Sync> RootOwner<V> for Retirement<V>{
 fn native_demand(&self)->Result<RetirementDemand,ValueError>{Retirement::native_demand(self)}
 fn native_work(&self)->Result<u64,ValueError>{Retirement::native_work(self)}
 fn native_step(&mut self,e:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep>{Retirement::native_step(self,e)}
 fn payload(&self)->&Option<V>{&self.payload}fn payload_mut(&mut self)->&mut Option<V>{&mut self.payload}
 fn is_empty(&self)->bool{self.terminal_is_empty()}
}
macro_rules! cursor_root{
 ($cursor:ident)=>{impl<V:RetireOwned+Sync> RootOwner<V> for $cursor<V>{
  fn native_demand(&self)->Result<RetirementDemand,ValueError>{
   if !self.state.closing{return Ok(crate::retained_clone::retained_close_start_demand(false))}
   if !self.state.retirement.is_empty(){return self.state.retirement.native_demand()}
   Ok(RetirementDemand{copy_bytes:self.native_close_header_bytes(),depth:usize::from(!self.terminal_is_empty()),..Default::default()})
  }
  fn native_work(&self)->Result<u64,ValueError>{if !self.state.retirement.is_empty(){self.state.retirement.native_work()}else{Ok(u64::from(!self.terminal_is_empty()))}}
  fn native_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->crate::RetainedWorkOutcome<RetainedCloneStep>{
   if !self.state.closing{let turn=crate::retained_clone::start_retained_close(&mut self.state.closing,effect);return crate::RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}}
   if !self.state.retirement.is_empty(){return self.state.retirement.native_step(effect)}
   let before=effect.performed_boundary();let result=(||{if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let demand=RootOwner::native_demand(self)?;if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<1{return Ok(RetainedCloneStep::Progress(Default::default()))}if !effect.performed(1)?{return Err(ValueError::invariant("ordered cursor metadata transfer exceeded original Work"))}self.state.close_step(effect.grant());Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))})();crate::RetainedWorkOutcome{result,work:effect.performed_since(before).expect("same ordered cursor metadata receipt")}
  }
  fn payload(&self)->&Option<V>{&self.state.retirement.payload}fn payload_mut(&mut self)->&mut Option<V>{&mut self.state.retirement.payload}
  fn is_empty(&self)->bool{self.terminal_is_empty()}
 }}
}
cursor_root!(UpdateCursor);cursor_root!(LookupCursor);

#[derive(Clone,Copy,PartialEq,Eq)]
struct CloseRoute{body:usize,demand:RetirementDemand,boundary:crate::OriginalWorkActionBoundary,invocation:crate::OriginalEffectInvocationIdentity,grant:RetainedCloneGrant,work:u64,original:bool,loan:Option<(crate::NativeAllocationIdentity,Option<u64>)>}
#[doc(hidden)]
pub struct TypedOwner<V:RetireOwned+Sync,R:RootOwner<V>>{
 root:R,payload:ManuallyDrop<Option<ControlledRetirement<V>>>,answer:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,route:Option<CloseRoute>,
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> TypedOwner<V,R>{
 fn new(root:R)->Self{Self{root,payload:ManuallyDrop::new(None),answer:ManuallyDrop::new(None),route:None}}
 fn relay_bytes()->usize{size_of::<Option<CloseRoute>>()+size_of::<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>()}
 fn drive<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{
  let before=effect.performed_boundary();
  if self.route.is_none(){
   if recover{return Err(ValueError::invariant("ordered retirement has no pending child to recover"))}
   if self.terminal_is_empty(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
   let demand=self.next_demand(body)?;let work=self.next_computational_work(body)?;
   if !crate::retained_clone::retained_clone_grant_admits(effect.grant(),demand)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
   if self.payload.as_ref().is_none_or(|owner|owner.terminal_is_empty()){
    if self.payload.is_some(){let owner=self.payload.as_ref().expect("selected empty payload");if crate::ErasedSnapshotRetirement::native_header_release(owner).is_none(){return Err(ValueError::invariant("ordered controlled payload has no terminal native release"))}if !effect.performed(1)?{return Err(ValueError::invariant("ordered payload header clear exceeded original Work"))}self.payload.take();return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},effect.performed_since(before)?))))}
    if self.root.payload().is_some(){if !effect.performed(1)?{return Err(ValueError::invariant("ordered payload placement exceeded original Work"))}*self.payload=Some(ControlledRetirement::retain(self.root.payload_mut().take().expect("funded original payload")));return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},effect.performed_since(before)?))))}
    let outcome=self.root.native_step(effect);return Ok(reply.complete(outcome.result.map(|step|if matches!(step,RetainedCloneStep::Complete(_))&&self.terminal_is_empty(){RetirementStep::Complete(outcome.work)}else{RetirementStep::Progress(step.progress(),outcome.work)}).map_err(Into::into)))
   }
  }
  let grant=effect.grant();let identity=effect.invocation().ok_or_else(||ValueError::invariant("ordered child close has no genuine invocation"))?.identity();let loan=original.as_ref().map(|o|(o.ledger_identity(),o.retirement_recipient_identity()));
  let fresh=self.route.is_none();let route=match self.route{Some(route)=>{if !recover||route.body!=body||route.invocation!=identity||route.grant!=grant||route.original!=original.is_some()||route.loan!=loan{return Err(ValueError::invariant("ordered retirement changed its actual child invocation"))}route},None=>CloseRoute{body,demand:self.next_demand(body)?,boundary:effect.action_boundary(),invocation:identity,grant,work:self.next_computational_work(body)?,original:original.is_some(),loan}};
  if fresh&&!effect.performed(1)?{return Err(ValueError::invariant("ordered child route installation exceeded original Work"))}
  let mut scope=reply.scope(&mut self.route,route)?;
  if self.answer.is_none(){
   let child_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.checked_sub(Self::relay_bytes()).ok_or_else(||ValueError::invariant("ordered relay headers are not funded"))?,..grant};
   let spent=effect.performed_since_action(route.boundary)?;let remaining=u128::from(route.work.checked_sub(1).ok_or_else(||ValueError::invariant("ordered reply Work is not funded"))?).checked_sub(spent).and_then(|work|u64::try_from(work).ok()).ok_or_else(||ValueError::invariant("ordered child exceeded its original Work"))?;let mut child_effect=effect.reborrow(child_grant,remaining)?;
   let child=OriginalEffectReply::borrow(&mut self.answer)?;let owner=self.payload.as_mut().expect("frozen ordered child owner");
   match(original.as_deref_mut(),recover){(Some(o),true)=>owner.recover_step_original_into(o,route.body.saturating_sub(Self::relay_bytes()),&mut child_effect,child)?,(Some(o),false)=>owner.step_original_into(o,route.body.saturating_sub(Self::relay_bytes()),&mut child_effect,child)?,(None,true)=>owner.recover_step_into(route.body.saturating_sub(Self::relay_bytes()),&mut child_effect,child)?,(None,false)=>owner.step_into(route.body.saturating_sub(Self::relay_bytes()),&mut child_effect,child)?};
  }
  if self.answer.is_none(){return Ok(scope.forwarded())}
  if !effect.performed(1)?{return Err(ValueError::invariant("ordered child reply transfer exceeded original Work"))}
  let work=effect.performed_since(before)?;let result=self.answer.take().expect("recorded original child reply");let result=result.map(|step|{let mut progress=step.progress();progress.copied_bytes=progress.copied_bytes.checked_add(Self::relay_bytes()).expect("preflight ordered reply extent");RetirementStep::Progress(progress,work)});
  scope.borrow()?.complete(result);Ok(scope.forwarded())
 }
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> RetirementCursor for TypedOwner<V,R>{
 fn close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,false)}
 fn recover_close_into<'r>(&mut self,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(None,body,e,r,true)}
 fn close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,false)}
 fn recover_close_original_into<'r>(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,body:usize,e:&mut RetainedAllocationEffect<'_>,r:OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>,ValueError>{self.drive(Some(o),body,e,r,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.payload.as_ref().is_some_and(|owner|owner.requires_original_birth())}
 fn requires_original_control(&self)->bool{self.route.map_or_else(||self.payload.as_ref().is_some_and(|owner|owner.requires_original_control()),|r|r.original)}
 fn close_original_birth_step(&mut self,o:&mut OriginalRetirementCloseLoan<'_>,g:RetainedCloneGrant)->RetirementStep{let Some(owner)=self.payload.as_mut()else{return RetirementStep::Failure(ValueError::invariant("ordered retirement has no selected original Birth").into(),0)};crate::retirement::erased_cursor_turn(owner.step_original_birth(o,g))}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(route)=self.route{if route.body!=body{return Err(ValueError::invariant("ordered retirement changed selected body"))}return Ok(route.demand)}
  if let Some(owner)=self.payload.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<ControlledRetirement<V>>>(),depth:1,..Default::default()})}let mut d=owner.next_demand(body.saturating_sub(Self::relay_bytes()))?;d.copy_bytes=d.copy_bytes.checked_add(Self::relay_bytes()).ok_or_else(||ValueError::invariant("ordered child relay extent overflow"))?;return Ok(d)}
  if self.root.payload().is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<V>>()+size_of::<Option<ControlledRetirement<V>>>(),depth:1,..Default::default()})}
  self.root.native_demand()
 }
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(route)=self.route{if route.body!=body{return Err(ValueError::invariant("ordered retirement changed selected body"))}return Ok(route.work)}if let Some(owner)=self.payload.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.next_computational_work(body.saturating_sub(Self::relay_bytes()))?.checked_add(2).ok_or_else(||ValueError::invariant("ordered child relay Work overflow"))}}if self.root.payload().is_some(){Ok(1)}else{self.root.native_work()}}
 fn terminal_is_empty(&self)->bool{self.root.is_empty()&&self.payload.is_none()&&self.answer.is_none()&&self.route.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.next_demand(0).ok().map(|d|d.release_bytes)}
 fn next_birth_bytes(&self,b:usize)->Option<usize>{self.next_demand(b).ok().map(|d|d.capacity_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<V:RetireOwned+Sync,R:RootOwner<V>> Drop for TypedOwner<V,R>{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"ordered typed owner retains its actual original custody");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.payload);ManuallyDrop::drop(&mut self.answer)}}}}

macro_rules! ordered_retirement{
 ($owner:ident,$root:ty,$seed:expr)=>{impl<V:RetireOwned+Sync> RetireOwned for $owner<V>{
  type RetirementCursor=TypedOwner<V,$root>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(($seed)(self))}
  fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
  fn controlled_retirement_supported()->bool{V::controlled_retirement_supported()}
 }}
}
ordered_retirement!(OrderedMap,Retirement<V>,|owner:OrderedMap<V>|owner.retire());
ordered_retirement!(Retirement,Retirement<V>,|owner|owner);
ordered_retirement!(UpdateCursor,UpdateCursor<V>,|owner|owner);
ordered_retirement!(LookupCursor,LookupCursor<V>,|owner|owner);
impl RetireOwned for super::OrderedSet{
 type RetirementCursor=TypedOwner<(),Retirement<()>>;fn retirement_seed(self)->Self::RetirementCursor{TypedOwner::new(self.retire())}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
#[cfg(test)]#[path="../🎟️work/♻️retirement/🧪️tests/🦀️.rs"]mod actual_work_tests;
