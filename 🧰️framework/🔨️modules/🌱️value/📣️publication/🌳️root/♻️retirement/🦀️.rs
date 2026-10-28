//! ♻️ Closure retains both actual roots independently of publication state and releases the original decision through its defining issuer.
use super::*;
use crate::retirement::{RetireOwned,RetirementCursor,RetirementStep,controlled::ControlledRetirement};
use crate::{RetainedCloneStep};
#[derive(crate::RetireOwned)]
#[value(crate="crate")]
enum RootPayload<T:RetireOwned>{Original(RetainedOriginalSlot<T>),Candidate(T)}
struct RootRetirement<T:RetireOwned>{root:RetainedPublicationRoot<T>,active:Option<ControlledRetirement<RootPayload<T>>>,decision:Option<FactoryAuthority>}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.depth=d.depth.checked_add(1).ok_or_else(||invalid("publication root retirement depth overflow"))?;Ok(d)}
fn header<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
impl<T:RetireOwned> RootRetirement<T>{
 fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(active)=self.active.as_ref(){return if active.terminal_is_empty(){Ok(header::<Option<ControlledRetirement<RootPayload<T>>>>())}else{active.next_demand(body).and_then(nested)}}if let Some(active)=self.decision.as_ref(){return if active.terminal_is_empty(){Ok(header::<Option<FactoryAuthority>>())}else{active.demands(body).and_then(nested)}}if self.root.original.is_some(){return Ok(header::<RootPayload<T>>())}if let Some(candidate)=self.root.candidate.as_ref(){if candidate.root.is_some(){return Ok(header::<T>())}if candidate.decision.is_some(){return Ok(header::<FactoryAuthority>())}return Ok(RetirementDemand{release_bytes:size_of::<Candidate<T>>(),depth:1,..Default::default()})}Ok(Default::default())
 }
 fn step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()))}let d=self.demand(grant.maximum_copy_bytes)?;if !permits(d,grant){return Ok(RetainedCloneStep::Progress(Default::default()))}if let Some(active)=self.active.as_mut(){if !active.terminal_is_empty(){return active.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}self.active=None;}else if let Some(active)=self.decision.as_mut(){if !active.terminal_is_empty(){return active.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|RetainedCloneStep::Progress(step.progress()))}self.decision=None;}else if let Some(original)=self.root.original.take(){self.active=Some(ControlledRetirement::retain(RootPayload::Original(original)));}else if let Some(candidate)=self.root.candidate.as_mut(){if let Some(original)=candidate.root.take(){self.active=Some(ControlledRetirement::retain(RootPayload::Candidate(original)));}else if let Some(decision)=candidate.decision.take(){self.decision=Some(decision.into_retirement());}else{drop(self.root.candidate.take());}}Ok(RetainedCloneStep::Progress(progress(d)))
 }
}
impl<T:RetireOwned> RetirementCursor for RootRetirement<T>{
 fn next_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{self.demand(body)}
 fn close_step(&mut self,grant:RetainedCloneGrant)->RetirementStep{match self.step(grant){Ok(RetainedCloneStep::Complete(p))if p==Default::default()=>RetirementStep::Complete,Ok(step)=>RetirementStep::Progress(step.progress()),Err(error)=>RetirementStep::Failure(error)}}
 fn terminal_is_empty(&self)->bool{self.root.original.is_none()&&self.root.candidate.is_none()&&self.active.is_none()&&self.decision.is_none()}
 fn next_work_byte_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.copy_bytes)}
 fn next_birth_bytes(&self,body:usize)->Option<usize>{self.demand(body).ok().map(|d|d.capacity_bytes)}
 fn next_close_byte_demand(&self)->Option<usize>{self.demand(0).ok().map(|d|d.release_bytes)}
 fn next_depth_demand(&self)->Result<usize,ValueError>{Ok(self.demand(0)?.depth)}
 fn terminal_release_bytes(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl<T:RetireOwned> RetireOwned for RetainedPublicationRoot<T>{
 fn retirement(self)->Box<dyn RetirementCursor>{Box::new(RootRetirement{root:self,active:None,decision:None})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<RootRetirement<T>>())}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()}
}
