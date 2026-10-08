//! ⏪️ Original actor retirement quotes and admits each retained physical frontier independently.
use super::*;
use semio_framework_value::{RetirementDemand,ValueError,ValueRefusalKind,FactoryAuthority,FactoryOwnedRetirement,ArtifactOwnedValueRetirementFactory,close_factory_ticket,factory_ticket_demands};
use std::mem::{size_of,size_of_val};
fn original_refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,message)}
fn nested(mut demand:RetirementDemand)->Result<RetirementDemand,ValueError>{demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"actor retirement depth overflow"))?;Ok(demand)}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn progress(step:RetainedCloneStep)->RetainedCloneStep{RetainedCloneStep::Progress(step.progress())}
impl<P,Mu> TimeTravelStoreState<P,Mu> where P:Send+Sync+'static,Mu: ::protocol::Mutation<P>+Send+'static {
 pub(crate) fn original_retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if let Some(owner)=self.retirements.back(){return if owner.terminal_is_empty(){Ok(RetirementDemand{release_bytes:size_of_val(owner.as_ref()),depth:1,..Default::default()})}else{nested(factory_ticket_demands(owner,body)?)};}
  if self.retirements.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.retirements.capacity().checked_mul(size_of::<Box<dyn store::ErasedSnapshotRetirement>>()).ok_or_else(||original_refusal("actor original queue capacity overflow"))?,depth:1,..Default::default()});}
  if let Some(owner)=self.discarded_active.as_ref(){return nested(factory_ticket_demands(owner,body)?);}
  if self.discarded_pending.is_some(){self.discarded_factory.as_ref().ok_or_else(||original_refusal("actor original mutation retains its missing issuer"))?;return Ok(RetirementDemand{capacity_bytes:FactoryOwnedRetirement::<Mu>::constructor_capacity_bytes(),depth:2,..Default::default()});}
  if !self.discarded.is_empty(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  if self.discarded.capacity()!=0{return Ok(RetirementDemand{release_bytes:self.discarded.capacity().checked_mul(size_of::<Mu>()).ok_or_else(||original_refusal("actor original mutation vector capacity overflow"))?,depth:1,..Default::default()});}
  if self.discarded_factory.is_some(){return Ok(RetirementDemand{depth:1,..Default::default()});}
  self.discarded_factory_close.as_ref().map_or(Ok(Default::default()),|owner|nested(owner.demands(body)?))
 }
 pub(crate) fn original_retirement_step(&mut self,grant:RetainedCloneGrant)->Result<Option<RetainedCloneStep>,ValueError>{
  if grant.maximum_items==0{return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
  let demand=self.original_retirement_demands(grant.maximum_copy_bytes)?;if !permits(grant,demand){return Ok(Some(RetainedCloneStep::Progress(Default::default())));}
  if let Some(owner)=self.retirements.back_mut(){
   if owner.terminal_is_empty(){drop(self.retirements.pop_back());return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
   let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=owner.close_step(child)?;let step=semio_framework_value::retained_clone::admit_retained_clone_close(child,step,owner.terminal_is_empty(),"actor original snapshot frame")?;return Ok(Some(progress(step)));
  }
  if self.retirements.capacity()!=0{drop(std::mem::take(&mut self.retirements));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
  if self.discarded_active.is_some(){return close_factory_ticket(&mut self.discarded_active,RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant}).map(|step|Some(progress(step)));}
  if self.discarded_pending.is_some(){let factory=self.discarded_factory.as_ref().unwrap();let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};if let Some((owner,receipt))=FactoryOwnedRetirement::admit_original(&mut self.discarded_pending,factory,child)?{self.discarded_active=Some(owner);if !receipt.fits(child){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actor mutation issuer exceeded its original admission"));}return Ok(Some(RetainedCloneStep::Progress(receipt)));}return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actor mutation issuer lost its original"));}
  if !self.discarded.is_empty(){self.discarded_pending=self.discarded.pop();return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));}
  if self.discarded.capacity()!=0{drop(std::mem::take(&mut self.discarded));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes:demand.release_bytes,..Default::default()})));}
  if let Some(factory)=self.discarded_factory.take(){self.discarded_factory_close=Some(FactoryAuthority::new(factory));return Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})));}
  if let Some(owner)=self.discarded_factory_close.as_mut(){let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};let step=owner.step(child)?;if !step.progress().fits(child){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"actor issuer exceeded its retained retirement grant"));}if owner.terminal_is_empty(){drop(self.discarded_factory_close.take());}return Ok(Some(progress(step)));}
  Ok(None)
 }
}
