//! ⚖️ Native Result variants retain their original payload through independent retirement admissions.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of};

enum ActiveResult<T:RetireOwned,E:RetireOwned>{Ok(ControlledRetirement<T>),Err(ControlledRetirement<E>)}
#[doc(hidden)]
pub struct OriginalResult<T:RetireOwned,E:RetireOwned>{original:ManuallyDrop<Option<Result<T,E>>>,active:ManuallyDrop<Option<ActiveResult<T,E>>>}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Result retirement exceeds its declared caller authority")}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
impl<T:RetireOwned,E:RetireOwned> ActiveResult<T,E>{
 fn terminal_is_empty(&self)->bool{match self{Self::Ok(v)=>v.terminal_is_empty(),Self::Err(v)=>v.terminal_is_empty()}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! demand{($v:expr)=>{{let mut d=$v.next_demand(body)?;d.depth=d.depth.checked_add(1).ok_or_else(refusal)?;Ok(d)}};}
  match self{Self::Ok(v)=>demand!(v),Self::Err(v)=>demand!(v)}
 }
 fn last_turn_work(&self)->u128{match self{Self::Ok(v)=>v.last_turn_work(),Self::Err(v)=>v.last_turn_work()}}
 fn step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{match self{Self::Ok(v)=>v.step(effect),Self::Err(v)=>v.step(effect)}}
 fn next_work(&self,body:usize)->Result<u64,ValueError>{match self{Self::Ok(v)=>v.next_computational_work(body),Self::Err(v)=>v.next_computational_work(body)}}
 fn requires_original_control(&self)->bool{match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::requires_original_control(v),Self::Err(v)=>crate::ErasedSnapshotRetirement::requires_original_control(v)}}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->crate::OriginalRetirementPoll{let g=effect.grant();match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::close_step_controlled(v,&mut effect.nested(g),control),Self::Err(v)=>crate::ErasedSnapshotRetirement::close_step_controlled(v,&mut effect.nested(g),control)}}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll{let g=effect.grant();match self{Self::Ok(v)=>crate::ErasedSnapshotRetirement::close_original_step(v,original,&mut effect.nested(g)),Self::Err(v)=>crate::ErasedSnapshotRetirement::close_original_step(v,original,&mut effect.nested(g))}}
}
impl<T:RetireOwned,E:RetireOwned> OriginalResult<T,E>{
 fn step_with(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,invoke:impl FnOnce(&mut ActiveResult<T,E>,&mut crate::RetainedAllocationEffect<'_>)->crate::OriginalRetirementPoll)->RetirementStep{let g=effect.grant();
  let work=match self.next_work(g.maximum_copy_bytes){Ok(work)=>work,Err(error)=>return RetirementStep::Failure(error.into(),0)};if work>effect.maximum_work_units(){return RetirementStep::BudgetExhausted}
  if self.terminal_is_empty(){return RetirementStep::Complete(0);}
  let d=match self.demand(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetirementStep::Failure(e.into(),0)};
  if !permits(d,g){return RetirementStep::BudgetExhausted;}
  let(step,work)=if let Some(original)=self.original.take(){
   let active=match original{Ok(v)=>match ControlledRetirement::new(v){Ok(v)=>ActiveResult::Ok(v),Err((e,v))=>{*self.original=Some(Ok(v));return RetirementStep::Failure(e.into(),1);}},Err(v)=>match ControlledRetirement::new(v){Ok(v)=>ActiveResult::Err(v),Err((e,v))=>{*self.original=Some(Err(v));return RetirementStep::Failure(e.into(),1);}}};
   *self.active=Some(active);(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),1)
  }else if self.active.as_ref().is_some_and(|active|active.terminal_is_empty()){
   *self.active=None;(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}),1)
  }else{let child=RetainedCloneGrant{maximum_depth:g.maximum_depth.saturating_sub(1),..g};let Some(active)=self.active.as_mut()else{return RetirementStep::Failure(ValueError::invariant("original Result retirement lost its active cursor").into(),0)};let mut child=effect.nested(child);let turn=match invoke(active,&mut child){crate::RetainedWorkPoll::Pending{work}=>return RetirementStep::Pending(work),crate::RetainedWorkPoll::Complete(turn)=>turn};let work=turn.work;match turn.result{Ok(step)=>(RetainedCloneStep::Progress(step.progress()),work),Err(e)=>return RetirementStep::Failure(e,work)}};
  match admit_retained_clone_close(g,step,self.terminal_is_empty(),"original Result retirement"){Ok(step)=>RetirementStep::Progress(step.progress(),work),Err(e)=>RetirementStep::Failure(e.into(),work)}
 }
 fn next_work(&self,body:usize)->Result<u64,ValueError>{if self.terminal_is_empty(){return Ok(0)}match self.active.as_ref(){Some(active)if !active.terminal_is_empty()=>active.next_work(body),_=>Ok(1)}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Result<T,E>>>()+size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()});}
  match self.active.as_ref(){None=>Ok(Default::default()),Some(v)if v.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()}),Some(v)=>v.demand(body)}
 }
}
impl<T:RetireOwned,E:RetireOwned> RetireOwned for Result<T,E>{
 fn retirement_ready(&self)->bool{match self{Ok(value)=>value.retirement_ready(),Err(error)=>error.retirement_ready()}}
 type RetirementCursor=OriginalResult<T,E>;fn retirement_seed(self)->Self::RetirementCursor{OriginalResult{original:ManuallyDrop::new(Some(self)),active:ManuallyDrop::new(None)}}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalResult<T,E>>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()&&path.supports::<E>()}
}
impl<T:RetireOwned,E:RetireOwned> RetirementCursor for OriginalResult<T,E>{
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.next_work(body)}
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
 fn requires_original_control(&self)->bool{self.original.is_none()&&self.active.as_ref().is_some_and(|v|!v.terminal_is_empty()&&v.requires_original_control())}
 fn close_step(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let g=effect.grant();self.step_with(effect,|active,effect|{active.step(effect)})}
 fn close_step_controlled(&mut self,effect:&mut crate::RetainedAllocationEffect<'_>,control:&mut dyn crate::OriginalRetirementControl)->RetirementStep{let g=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"result has no selected original child").into(),0)}self.step_with(effect,|active,effect|active.close_step_controlled(effect,control))}
 fn close_original_step(&mut self,original:&mut semio_framework_value_control_contract::OriginalRetirementCloseLoan<'_>,effect:&mut crate::RetainedAllocationEffect<'_>)->RetirementStep{let g=effect.grant();if !self.requires_original_control(){return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"result has no selected original child").into(),0)}self.step_with(effect,|active,effect|active.close_original_step(original,effect))}
 fn terminal_is_empty(&self)->bool{self.original.is_none()&&self.active.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
 fn allows_admitted_narrow_work(&self)->bool{true}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned,E:RetireOwned> Drop for OriginalResult<T,E>{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original Result abandoned before terminal-empty");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.original);ManuallyDrop::drop(&mut self.active);}}}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
