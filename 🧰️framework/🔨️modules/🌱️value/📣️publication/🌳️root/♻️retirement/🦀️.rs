//! ♻️ Closure retains both actual roots independently of publication state and releases the original decision through its defining issuer.
use super::*;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetainedCloneStep,RetainedWorkPoll,OriginalRetirementPoll,RetainedAllocationEffect};
#[doc(hidden)]
pub struct RootRetirement<T:RetireOwned>{root:RetainedPublicationRoot<T>,original:Option<ControlledRetirement<RetainedOriginalSlot<T>>>,candidate:Option<ControlledRetirement<T>>,decision:Option<FactoryAuthority>}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("publication root retirement depth overflow"))?;Ok(d)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
fn complete(result:Result<RetainedCloneStep,crate::OriginalRetirementFailure>,work:u128)->OriginalRetirementPoll{RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result,work})}
fn pump<P:RetireOwned>(active:&mut Option<ControlledRetirement<P>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,child:RetainedCloneGrant)->Option<OriginalRetirementPoll>{let cursor=active.as_mut()?;if cursor.terminal_is_empty(){return None}Some(cursor.step(body,&mut effect.nested(child)).map(|step|RetainedCloneStep::Progress(step.progress())))}
impl<T:RetireOwned> RootRetirement<T>{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.original.as_ref(){return if active.terminal_is_empty(){Ok(header::<Option<ControlledRetirement<RetainedOriginalSlot<T>>>>())}else{active.next_demand(body).and_then(nested)}}if let Some(active)=self.candidate.as_ref(){return if active.terminal_is_empty(){Ok(header::<Option<ControlledRetirement<T>>>())}else{active.next_demand(body).and_then(nested)}}if let Some(active)=self.decision.as_ref(){return if active.terminal_is_empty(){Ok(header::<Option<FactoryAuthority>>())}else{active.demands(body).and_then(nested)}}if self.root.original.is_some(){return Ok(header::<RetainedOriginalSlot<T>>())}if let Some(candidate)=self.root.candidate.as_ref(){if candidate.root.is_some(){return Ok(header::<T>())}if candidate.decision.is_some(){return Ok(header::<FactoryAuthority>())}return Ok(RetirementDemand{release_bytes:size_of::<Candidate<T>>(),depth:1,..Default::default()})}Ok(Default::default())
 }
 fn work(&self,body:usize)->Result<u64,ValueError>{
  if let Some(active)=self.original.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}if let Some(active)=self.candidate.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}if let Some(active)=self.decision.as_ref(){return if active.terminal_is_empty(){Ok(1)}else{active.next_computational_work(body)}}Ok(u64::from(!self.terminal_is_empty()))
 }
 fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  let grant=effect.grant();let fail=|error:ValueError|complete(Err(error.into()),0);
  if self.terminal_is_empty(){return complete(Ok(RetainedCloneStep::Complete(Default::default())),0)}let d=match self.demand(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return fail(error)};if !permits(d,grant){return complete(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
  if let Some(poll)=pump(&mut self.original,grant.maximum_copy_bytes,effect,child){return poll}if let Some(poll)=pump(&mut self.candidate,grant.maximum_copy_bytes,effect,child){return poll}
  if let Some(active)=self.decision.as_mut(){if !active.terminal_is_empty(){return active.step(grant.maximum_copy_bytes,&mut effect.nested(child)).map(|step|RetainedCloneStep::Progress(step.progress()))}}
  match effect.performed(1){Ok(true)=>{},Ok(false)=>return fail(ValueError::literal(ValueRefusalKind::WorkLimit,"publication root retirement exceeded its original work")),Err(error)=>return fail(error)}
  if self.original.is_some(){self.original=None}else if self.candidate.is_some(){self.candidate=None}else if self.decision.is_some(){self.decision=None}else if let Some(original)=self.root.original.take(){self.original=Some(ControlledRetirement::retain(original))}else if let Some(candidate)=self.root.candidate.as_mut(){if let Some(value)=candidate.root.take(){self.candidate=Some(ControlledRetirement::retain(value))}else if let Some(decision)=candidate.decision.take(){self.decision=Some(FactoryAuthority::new(decision))}else{drop(self.root.candidate.take())}}
  complete(Ok(RetainedCloneStep::Progress(progress(d))),1)
 }
}
impl<T:RetireOwned> RootRetirement<T>{
 fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->RetirementStep{crate::retirement::erased_cursor_turn(self.step(effect))}
}
impl<T:RetireOwned> RetirementCursor for RootRetirement<T>{
    semio_framework_value_retirement::native_retirement_reply!();
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
 fn next_computational_work(&self,body:usize)->Result<u64,ValueError>{self.work(body)}
 fn terminal_is_empty(&self)->bool{self.root.original.is_none()&&self.root.candidate.is_none()&&self.original.is_none()&&self.candidate.is_none()&&self.decision.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> RetireOwned for RetainedPublicationRoot<T>{
 type RetirementCursor=RootRetirement<T>;
 fn retirement_seed(self)->Self::RetirementCursor{RootRetirement{root:self,original:None,candidate:None,decision:None}}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
