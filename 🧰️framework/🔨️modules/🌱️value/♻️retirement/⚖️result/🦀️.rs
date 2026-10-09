//! ⚖️ Native Result variants retain their original payload through independent retirement admissions.
use super::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,admit_retained_clone_close}};
use std::mem::{ManuallyDrop,size_of};

enum ActiveResult<T:RetireOwned,E:RetireOwned>{Ok(ControlledRetirement<T>),Err(ControlledRetirement<E>)}
struct OriginalResult<T:RetireOwned,E:RetireOwned>{original:ManuallyDrop<Option<Result<T,E>>>,active:ManuallyDrop<Option<ActiveResult<T,E>>>}
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,"original Result retirement exceeds its declared caller authority")}
fn permits(d:RetirementDemand,g:RetainedCloneGrant)->bool{g.maximum_items>0&&d.copy_bytes<=g.maximum_copy_bytes&&d.capacity_bytes<=g.maximum_capacity_bytes&&d.release_bytes<=g.maximum_release_bytes&&d.depth<=g.maximum_depth}
impl<T:RetireOwned,E:RetireOwned> ActiveResult<T,E>{
 fn terminal_is_empty(&self)->bool{match self{Self::Ok(v)=>v.terminal_is_empty(),Self::Err(v)=>v.terminal_is_empty()}}
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  macro_rules! demand{($v:expr)=>{Ok(RetirementDemand{copy_bytes:$v.next_copy_byte_demand()?,capacity_bytes:$v.next_capacity_byte_demand(body)?,release_bytes:$v.next_release_byte_demand()?,depth:$v.next_depth_demand()?.checked_add(1).ok_or_else(refusal)?})};}
  match self{Self::Ok(v)=>demand!(v),Self::Err(v)=>demand!(v)}
 }
 fn step(&mut self,g:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{match self{Self::Ok(v)=>v.step(g),Self::Err(v)=>v.step(g)}}
}
impl<T:RetireOwned,E:RetireOwned> OriginalResult<T,E>{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.original.is_some(){return Ok(RetirementDemand{copy_bytes:size_of::<Option<Result<T,E>>>()+size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()});}
  match self.active.as_ref(){None=>Ok(Default::default()),Some(v)if v.terminal_is_empty()=>Ok(RetirementDemand{copy_bytes:size_of::<Option<ActiveResult<T,E>>>(),depth:1,..Default::default()}),Some(v)=>v.demand(body)}
 }
}
impl<T:RetireOwned,E:RetireOwned> RetireOwned for Result<T,E>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(OriginalResult{original:ManuallyDrop::new(Some(self)),active:ManuallyDrop::new(None)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<OriginalResult<T,E>>())}
 fn controlled_retirement_supported()->bool{T::controlled_retirement_supported()&&E::controlled_retirement_supported()}
}
impl<T:RetireOwned,E:RetireOwned> RetirementCursor for OriginalResult<T,E>{
 fn close_step(&mut self,g:RetainedCloneGrant)->RetirementStep{
  if self.terminal_is_empty(){return RetirementStep::Complete;}
  let d=match self.demand(g.maximum_copy_bytes){Ok(d)=>d,Err(e)=>return RetirementStep::Failure(e)};
  if !permits(d,g){return RetirementStep::BudgetExhausted;}
  let step=if let Some(original)=self.original.take(){
   let active=match original{Ok(v)=>match ControlledRetirement::new(v){Ok(v)=>ActiveResult::Ok(v),Err((e,v))=>{*self.original=Some(Ok(v));return RetirementStep::Failure(e);}},Err(v)=>match ControlledRetirement::new(v){Ok(v)=>ActiveResult::Err(v),Err((e,v))=>{*self.original=Some(Err(v));return RetirementStep::Failure(e);}}};
   *self.active=Some(active);RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})
  }else if self.active.as_ref().unwrap().terminal_is_empty(){
   *self.active=None;RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})
  }else{let child=RetainedCloneGrant{maximum_depth:g.maximum_depth-1,..g};match self.active.as_mut().unwrap().step(child){Ok(step)=>RetainedCloneStep::Progress(step.progress()),Err(e)=>return RetirementStep::Failure(e)}};
  match admit_retained_clone_close(g,step,self.terminal_is_empty(),"original Result retirement") {Ok(step)=>RetirementStep::Progress(step.progress()),Err(e)=>RetirementStep::Failure(e)}
 }
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
