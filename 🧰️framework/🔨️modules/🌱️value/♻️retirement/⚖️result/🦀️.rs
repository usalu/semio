//! ⚖️ Native Result variants retain their original payload through independent retirement admissions.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of};
use semio_framework_value_control_contract::{OriginalEffectReply,OriginalEffectReplyComplete,OriginalEffectInvocationIdentity};

enum ActiveResult<T:RetireOwned,E:RetireOwned>{Ok(ControlledRetirement<T>),Err(ControlledRetirement<E>)}
#[doc(hidden)]
pub struct OriginalResult<T:RetireOwned,E:RetireOwned>{original:ManuallyDrop<Option<Result<T,E>>>,active:ManuallyDrop<Option<ActiveResult<T,E>>>,child_reply:ManuallyDrop<Option<Result<RetainedCloneStep,crate::OriginalRetirementFailure>>>,invocation:Option<OriginalEffectInvocationIdentity>,grant:Option<RetainedCloneGrant>,selected:Option<(RetirementDemand,u64)>,original_mode:Option<bool>,body:Option<usize>,loan:Option<(crate::NativeAllocationIdentity,Option<u64>)>}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Result retirement exceeds its declared caller authority")}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
impl<T:RetireOwned,E:RetireOwned> ActiveResult<T,E>{

 fn native_header_empty(&self)->bool{match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::native_header_release(v).is_some(),Self::Err(v)=>crate::ErasedSnapshotRetirement::native_header_release(v).is_some()}}
 fn requires_original_birth(&self)->bool{match self{Self::Ok(v)=>v.requires_original_birth(),Self::Err(v)=>v.requires_original_birth()}}
 fn birth(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{let poll=match self{Self::Ok(v)=>v.step_original_birth(original,grant),Self::Err(v)=>v.step_original_birth(original,grant)};match poll{crate::RetainedWorkPoll::Pending{work}=>RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>match turn.result{Ok(step)=>RetirementStep::Progress(step.progress(),turn.work),Err(error)=>RetirementStep::Failure(error,turn.work)}}}
 fn close_into<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,crate::OriginalRetirementFailure>,ValueError>{macro_rules! child{($value:expr)=>{match original{Some(original)=>if recover{$value.recover_step_original_into(original,body,effect,reply)}else{$value.step_original_into(original,body,effect,reply)},None=>if recover{$value.recover_step_into(body,effect,reply)}else{$value.step_into(body,effect,reply)}}}}match self{Self::Ok(value)=>child!(value),Self::Err(value)=>child!(value)}}
 fn terminal_is_empty(&self)->bool{match self{Self::Ok(v)=>v.terminal_is_empty(),Self::Err(v)=>v.terminal_is_empty()}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! demand{($v:expr)=>{{let mut d=$v.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(refusal)?;Ok(d)}};}
  match self{Self::Ok(v)=>demand!(v),Self::Err(v)=>demand!(v)}
 }
 fn last_turn_work(&self)->u128{match self{Self::Ok(v)=>v.last_turn_work(),Self::Err(v)=>v.last_turn_work()}}
 fn step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{match self{Self::Ok(v)=>v.step(body,effect),Self::Err(v)=>v.step(body,effect)}}
 fn next_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Ok(v)=>v.next_computational_work(body),Self::Err(v)=>v.next_computational_work(body)}}
 fn requires_original_control(&self)->bool{match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::requires_original_control(v),Self::Err(v)=>crate::ErasedSnapshotRetirement::requires_original_control(v)}}
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let g=effect.grant();match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::close_step_controlled(v,body,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control),Self::Err(v)=>crate::ErasedSnapshotRetirement::close_step_controlled(v,body,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"),control)}}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::close_original_step(v,original,body,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect")),Self::Err(v)=>crate::ErasedSnapshotRetirement::close_original_step(v,original,body,&mut effect.reborrow(g,effect.maximum_work_units()).expect("retirement child exceeds original effect"))}}
}
impl<T:RetireOwned,E:RetireOwned> OriginalResult<T,E>{

 /// 👁️ Borrows the same original variant before its payload moves into a typed child.
 pub fn original(&self)->Option<Result<&T,&E>>{if let Some(value)=self.original.as_ref(){return Some(match value{Ok(value)=>Ok(value),Err(error)=>Err(error)})}match self.active.as_ref(){Some(ActiveResult::Ok(value))=>value.original().map(Ok),Some(ActiveResult::Err(value))=>value.original().map(Err),None=>None}}
 fn close_reply<'r>(&mut self,original:Option<&mut crate::OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>,recover:bool)->Result<OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{
  let boundary=effect.performed_boundary();let grant=effect.grant();let mode=original.is_some();let loan=original.as_ref().map(|original|(original.ledger_identity(),original.retirement_recipient_identity()));
  if self.invocation.is_none(){
   if recover{return Err(ValueError::invariant("Result recovery has no original child invocation"))}
   let demand=self.demand(body)?;let work=self.next_work(body)?;if !permits(demand,grant)||effect.remaining_work_units()?<work{return Ok(reply.complete(Ok(RetirementStep::BudgetExhausted)))}
   if self.original.is_some(){if let Err((error,_))=crate::record_retirement_work(effect,1){return Ok(reply.complete(Err(error.into())))}let value=self.original.take().expect("original Result remains resident");*self.active=Some(match value{Ok(value)=>ActiveResult::Ok(ControlledRetirement::retain(value)),Err(error)=>ActiveResult::Err(ControlledRetirement::retain(error))});return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},effect.performed_since(boundary)?))))}
   if self.active.as_ref().is_some_and(ActiveResult::terminal_is_empty){if !self.active.as_ref().is_some_and(ActiveResult::native_header_empty){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Result child retains its original native lifecycle"))}if let Err((error,_))=crate::record_retirement_work(effect,1){return Ok(reply.complete(Err(error.into())))}*self.active=None;return Ok(reply.complete(Ok(RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()},effect.performed_since(boundary)?))))}
   if self.active.is_none(){return Ok(reply.complete(Ok(RetirementStep::Complete(0))))}
   let identity=effect.invocation().ok_or_else(||ValueError::invariant("Result child requires its original producer invocation"))?.identity();self.invocation=Some(identity);self.grant=Some(grant);self.selected=Some((demand,work));self.original_mode=Some(mode);self.body=Some(body);self.loan=loan;
  }
  let identity=effect.invocation().ok_or_else(||ValueError::invariant("Result child lost its original producer invocation"))?.identity();if self.invocation!=Some(identity)||self.grant!=Some(grant)||self.original_mode!=Some(mode)||self.body!=Some(body)||self.loan!=loan{return Err(ValueError::invariant("Result child changed its original invocation, grant or control"))}
  if self.child_reply.is_none(){let active=self.active.as_mut().ok_or_else(||ValueError::invariant("Result original child is absent"))?;let nested=RetainedCloneGrant{maximum_depth:grant.maximum_depth.checked_sub(1).ok_or_else(refusal)?,..grant};let maximum=effect.remaining_work_units()?;let mut child=effect.reborrow(nested,maximum)?;drop(active.close_into(original,body,&mut child,OriginalEffectReply::borrow(&mut self.child_reply)?,recover)?);}
  if self.child_reply.is_none(){return Ok(reply.pending())}let work=effect.performed_since(boundary)?;let mut scope=reply.scope(&mut self.invocation,identity)?;let parent=scope.borrow()?;self.grant=None;self.selected=None;self.original_mode=None;self.body=None;self.loan=None;let result=self.child_reply.take().expect("actual Result child reply remains resident");parent.complete(result.map(|step|RetirementStep::Progress(step.progress(),work)));Ok(scope.forwarded())
 }
 fn step_with(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut ActiveResult<T,E>,&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->RetirementStep{let g=effect.grant();
  let work=match self.next_work(body){Ok(work)=>work,Err(error)=>return RetirementStep::Failure(error.into(),0)};if work>effect.maximum_work_units(){return RetirementStep::BudgetExhausted}
  if self.terminal_is_empty(){return RetirementStep::Complete(0);}
  let d=match self.demand(body){Ok(d)=>d,Err(e)=>return RetirementStep::Failure(e.into(),0)};
  if !permits(d,g){return RetirementStep::BudgetExhausted;}
  if self.active.as_ref().is_some_and(ActiveResult::terminal_is_empty)&&!self.active.as_ref().is_some_and(ActiveResult::native_header_empty){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"Result child retains its original lifecycle").into(),0)}let metadata=self.original.is_some()||self.active.as_ref().is_some_and(ActiveResult::native_header_empty);if metadata{if let Err((error,work))=crate::record_retirement_work(effect,1){return RetirementStep::Failure(error.into(),work)}}let(step,work)=if let Some(original)=self.original.take(){
   let active=match original{Ok(v)=>ActiveResult::Ok(ControlledRetirement::retain(v)),Err(v)=>ActiveResult::Err(ControlledRetirement::retain(v))};
   *self.active=Some(active);(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),1)
  }else if self.active.as_ref().is_some_and(|active|active.terminal_is_empty()){
   *self.active=None;(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),1)
  }else{let child=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};let Some(active)=self.active.as_mut()else{return RetirementStep::Failure(ValueError::invariant("original Result retirement lost its active cursor").into(),0)};let mut child=match effect.reborrow(child,effect.maximum_work_units()){Ok(child)=>child,Err(error)=>return RetirementStep::Failure(error.into(),0)};let turn=match invoke(active,&mut child){crate::RetainedWorkPoll::Pending{work}=>return RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>turn};let work=turn.work;match turn.result{Ok(step)=>(RetainedCloneStep::Progress(step.progress()),work),Err(e)=>return RetirementStep::Failure(e,work)}};
  match admit_retained_clone_close(g,step,self.terminal_is_empty(),"original Result retirement"){Ok(step)=>RetirementStep::Progress(step.progress(),work),Err(e)=>RetirementStep::Failure(e.into(),work)}
 }
 fn next_work(&self,body:usize)->Result<u64,ValueError>{if self.invocation.is_some(){if self.body!=Some(body){return Err(ValueError::invariant("Result recovery changed its selected body"))}return self.selected.map(|s|s.1).ok_or_else(||ValueError::invariant("Result original work quote missing"))}if self.terminal_is_empty(){return Ok(0)}match self.active.as_ref(){Some(active)if !active.terminal_is_empty()=>active.next_work(body),_=>Ok(1)}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.invocation.is_some(){return self.selected.map(|s|s.0).ok_or_else(||ValueError::invariant("Result original physical quote missing"))}
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Result<T,E>>>()+size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()});}
  match self.active.as_ref(){None=>Ok(Default::default()),Some(v)if v.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()}),Some(v)=>v.demand(body)}
 }
}
impl<T:RetireOwned,E:RetireOwned> RetireOwned for Result<T,E>{
 fn retirement_ready(&self)->bool{match self{Ok(value)=>value.retirement_ready(),Err(error)=>error.retirement_ready()}}
 type RetirementCursor=OriginalResult<T,E>;fn retirement_seed(self)->Self::RetirementCursor{OriginalResult{original:ManuallyDrop::new(Some(self)),active:ManuallyDrop::new(None),child_reply:ManuallyDrop::new(None),invocation:None,grant:None,selected:None,original_mode:None,body:None,loan:None}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalResult<T,E>>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()&&path.supports::<E>()}
}
impl<T:RetireOwned,E:RetireOwned> RetirementCursor for OriginalResult<T,E>{
 fn close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetirementStep,crate::OriginalRetirementFailure>)->Result<OriginalEffectReplyComplete<'r,RetirementStep,crate::OriginalRetirementFailure>,ValueError>{self.close_reply(Some(original),body,effect,reply,true)}
 fn requires_original_birth(&self)->bool{self.invocation.is_none()&&self.original.is_none()&&self.active.as_ref().is_some_and(ActiveResult::requires_original_birth)}
 fn close_original_birth_step(&mut self,original:&mut crate::OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->RetirementStep{if !self.requires_original_birth()||grant.maximum_depth==0{return RetirementStep::BudgetExhausted}self.active.as_mut().expect("original Result birth child remains resident").birth(original,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})}

 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_work(body)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
 fn requires_original_control(&self)->bool{if self.invocation.is_some(){return self.original_mode==Some(true)}self.original.is_none()&&self.active.as_ref().is_some_and(|v|!v.terminal_is_empty()&&v.requires_original_control())}
 
 fn close_step_controlled(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let g=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"result has no selected original child").into(),0)}self.step_with(body,effect,|active,effect|active.close_step_controlled(body,effect,control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_retirement_contract::OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let g=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"result has no selected original child").into(),0)}self.step_with(body,effect,|active,effect|active.close_original_step(original,body,effect))}
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.active.is_none()&&self.child_reply.is_none()&&self.invocation.is_none()&&self.grant.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned,E:RetireOwned> OriginalResult<T,E>{
 fn close_step(&mut self,body:usize,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let g=effect.grant();self.step_with(body,effect,|active,effect|{active.step(body,effect)})}
}

impl<T:RetireOwned,E:RetireOwned> Drop for OriginalResult<T,E>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original Result abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.active);ManuallyDrop::drop(&mut self.child_reply);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;


