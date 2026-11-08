//! ♻️ Read Source retirement retains its actual child and borrowed reply.
use super::*;
use crate::{ErasedSnapshotRetirement,RetainedAllocationEffect,RetirementDemand,retained_clone::{OriginalEffectCallbackMode,OriginalEffectInvocationIdentity,OriginalWorkActionBoundary,OriginalWorkBindingIdentity,RetainedCloneGrant,RetainedWorkPoll}};
use semio_framework_value_retirement::{OriginalRetirementCloseLoan,OriginalRetirementFailure};

type Reply<'r>=OriginalEffectReply<'r,RetirementStep,OriginalRetirementFailure>;
type Complete<'r>=OriginalEffectReplyComplete<'r,RetirementStep,OriginalRetirementFailure>;
type Key=(OriginalEffectInvocationIdentity,OriginalWorkActionBoundary,RetainedCloneGrant,u64,Option<(semio_framework_value_control_contract::NativeAllocationIdentity,Option<u64>,OriginalWorkBindingIdentity)>,usize);
fn invalid(message:&'static str)->ValueError{ValueError::invariant(message)}

trait Capsule:Send+'static{
 fn child(&self)->Option<&dyn ErasedSnapshotRetirement>;
 fn child_mut(&mut self)->Option<&mut dyn ErasedSnapshotRetirement>;
 fn work(&self)->Option<OriginalWorkBindingIdentity>;
 fn clear_demand()->RetirementDemand;
 fn clear(&mut self);
}
impl<T:RetireOwned+Sync> Capsule for OriginalReadSource<T>{
 fn child(&self)->Option<&dyn ErasedSnapshotRetirement>{self.read.as_ref().map(|read|read as &dyn ErasedSnapshotRetirement)}
 fn child_mut(&mut self)->Option<&mut dyn ErasedSnapshotRetirement>{self.read.as_mut().map(|read|read as &mut dyn ErasedSnapshotRetirement)}
 fn work(&self)->Option<OriginalWorkBindingIdentity>{self.read.as_ref().and_then(|read|read.work)}
 fn clear_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<ReadLease<T>>>()+size_of::<Option<OriginalWorkBindingIdentity>>(),depth:1,..Default::default()}}
 fn clear(&mut self){drop(self.read.take())}
}
impl<T:RetireOwned+Sync> Capsule for OriginalErasedReadSource<T>{
 fn child(&self)->Option<&dyn ErasedSnapshotRetirement>{self.read.as_ref().map(|read|read as &dyn ErasedSnapshotRetirement)}
 fn child_mut(&mut self)->Option<&mut dyn ErasedSnapshotRetirement>{self.read.as_mut().map(|read|read as &mut dyn ErasedSnapshotRetirement)}
 fn work(&self)->Option<OriginalWorkBindingIdentity>{self.read.as_ref().and_then(ErasedReadLease::original_work)}
 fn clear_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<Option<ErasedReadLease>>()+size_of::<Option<OriginalWorkBindingIdentity>>(),depth:1,..Default::default()}}
 fn clear(&mut self){drop(self.read.take())}
}

/// 🧳️ The Source cursor retains the original read, selected invocation and child reply.
pub struct Cursor<S:Send+'static>{original:ManuallyDrop<S>,reply:ManuallyDrop<Option<Result<RetainedCloneStep,OriginalRetirementFailure>>>,route:Option<Key>,work:Option<OriginalWorkBindingIdentity>}
impl<S:Capsule> Cursor<S>{
 pub(super) fn new(original:S)->Self{let work=original.work();Self{original:ManuallyDrop::new(original),reply:ManuallyDrop::new(None),route:None,work}}
 fn drive<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Complete<'r>,ValueError>{
  if !self.work.is_some_and(|work|work.matches_effect(effect)){return Err(invalid("read Source cursor changed original Work"))}
  let invocation=effect.invocation().ok_or_else(||invalid("read Source cursor requires original invocation"))?;
  if invocation.mode()!=if recover{OriginalEffectCallbackMode::Recover}else{OriginalEffectCallbackMode::Receive}||invocation.source_turn()!=effect.work_source_turn(){return Err(invalid("read Source cursor invocation changed"))}
  let binding=match original.as_mut(){Some(loan)=>{let work=loan.original_work().binding_identity()?;if !work.matches_effect(effect){return Err(invalid("read Source cursor original Work changed"))}Some((loan.ledger_identity(),loan.retirement_recipient_identity(),work))},None=>None};
  let key=if recover{let key=self.route.ok_or_else(||invalid("read Source recovery absent"))?;if key.0!=invocation.identity()||key.2!=effect.grant()||key.3!=effect.maximum_work_units()||key.4!=binding||key.5!=body{return Err(invalid("read Source recovery changed original action"))}key}else{if self.route.is_some(){return Err(invalid("read Source action occupied"))}(invocation.identity(),effect.action_boundary(),effect.grant(),effect.maximum_work_units(),binding,body)};
  let Self{original:owner,reply:child,route,..}=self;let mut scope=reply.scope(route,key)?;
  if child.is_none(){let inner=OriginalEffectReply::borrow(child)?;let owner=owner.child_mut().ok_or_else(||invalid("read Source child absent"))?;let done=match(original,recover){(None,false)=>owner.close_into(body,effect,inner)?,(None,true)=>owner.recover_close_into(body,effect,inner)?,(Some(loan),false)=>owner.close_original_into(loan,body,effect,inner)?,(Some(loan),true)=>owner.recover_close_original_into(loan,body,effect,inner)?};drop(done)}
  if child.is_none(){return Ok(scope.forwarded())}let work=effect.performed_since_action(key.1)?;let output=scope.borrow()?;let Some(result)=child.take()else{return Err(invalid("read Source child reply disappeared"))};output.complete(result.map(|step|RetirementStep::Progress(step.progress(),work)));Ok(scope.forwarded())
 }
}
impl<S:Capsule> RetirementCursor for Cursor<S>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{
  if self.route.is_some()||self.reply.is_some(){return Err(invalid("read Source cursor requires original recovery"))}
  let Some(child)=self.original.child()else{return Ok(reply.complete(Ok(RetirementStep::Complete(0))))};
  if !child.terminal_is_empty(){return self.drive(None,body,effect,reply,false)}
  if !self.work.is_some_and(|work|work.matches_effect(effect)){return Err(invalid("read Source terminal Work changed"))}
  let demand=S::clear_demand();if effect.remaining_work_units()?<1||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return Ok(reply.complete(Err(ValueError::literal(ValueRefusalKind::WorkLimit,"read Source terminal clear exceeded Work").into()))),Err(error)=>return Ok(reply.complete(Err(error.into())))}
  self.original.clear();self.work=None;Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},1))))
 }
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Result<Complete<'r>,ValueError>{self.drive(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.route.is_none()&&self.original.child().is_some_and(|child|child.requires_original_birth())}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{match self.original.child_mut(){Some(child)=>match child.close_original_birth_step(original,grant){RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}},None=>RetirementStep::Failure(invalid("read Source Birth absent").into(),0)}}
 fn requires_original_control(&self)->bool{self.route.map_or_else(||self.original.child().is_some_and(|child|child.requires_original_control()),|route|route.4.is_some())}
 fn terminal_is_empty(&self)->bool{self.original.child().is_none()&&self.reply.is_none()&&self.route.is_none()}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(route)=self.route{if route.5!=body{return Err(invalid("read Source query changed original body"))}return Ok(RetirementDemand{copy_bytes:route.2.maximum_copy_bytes,capacity_bytes:route.2.maximum_capacity_bytes,release_bytes:route.2.maximum_release_bytes,depth:route.2.maximum_depth})}match self.original.child(){None=>Ok(Default::default()),Some(child)if child.terminal_is_empty()=>Ok(S::clear_demand()),Some(child)=>child.next_demand(body)}}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{if let Some(route)=self.route{if route.5!=body{return Err(invalid("read Source work query changed original body"))}return Ok(route.3)}match self.original.child(){None=>Ok(0),Some(child)if child.terminal_is_empty()=>Ok(1),Some(child)=>child.next_computational_work(body)}}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{Some(self.next_demand(body).ok()?.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.next_demand(0).ok()?.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.next_demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<S:Send+'static> Drop for Cursor<S>{fn drop(&mut self){if std::thread::panicking(){return}assert!(self.reply.is_none()&&self.route.is_none()&&self.work.is_none(),"read Source cursor retains its original action");unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.reply)}}}
