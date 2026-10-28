//! 🪟️ Canonical history rows share one original publication decision without changing their keys or order.
use super::{ArtifactHistoryKey,ArtifactHistoryLedger};
use semio_framework_value::{FactoryAuthority,RetainedPublicationDecision,RetainedPublicationRoot,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::{mem::size_of,sync::Arc};
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn grant_admits(g:RetainedCloneGrant,d:RetirementDemand)->bool{semio_framework_value::retained_clone::retained_clone_grant_admits(g,d)}
impl<T> ArtifactHistoryLedger<T>{
 /// 📐️ Quotes both real candidate allocations and fixed original row transfers before any ownership changes.
 pub fn replacement_stage_demands(&self,key:ArtifactHistoryKey)->Result<RetirementDemand,ValueError>{
  self.get_key(key).ok_or_else(||fault("replacement requires its exact visible original history generation"))?;
  if self.reservation.is_some()||self.group.is_some(){return Err(fault("replacement requires its original ledger suffix to be settled"))}
  let slot=self.slot(key.index);if slot.read.is_some()||slot.replacement.is_some()||slot.value.is_none(){return Err(fault("replacement requires an unborrowed original row"))}
  let mut d=RetainedPublicationRoot::<T>::stage_demands();d.capacity_bytes+=size_of::<RetainedPublicationRoot<T>>();d.copy_bytes+=2*size_of::<T>()+2*size_of::<RetainedPublicationRoot<T>>()+size_of::<usize>();Ok(d)
 }
 /// 🎟️ Stages one real typed row under the original pending decision; every read keeps selecting the old row until publication.
 pub fn stage_replacement(&mut self,key:ArtifactHistoryKey,candidate:&mut Option<T>,decision:&mut Option<Arc<dyn RetainedPublicationDecision>>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  let d=self.replacement_stage_demands(key)?;if candidate.is_none()||decision.as_ref().is_none_or(|view|!view.pending()){return Err(fault("history replacement requires its original candidate and pending decision"))}let identity=Arc::as_ptr(decision.as_ref().unwrap())as*const()as usize;if self.replacement_decision.is_some_and(|original|original!=identity){return Err(fault("history replacement changed its original group decision"))}if !grant_admits(grant,d){return Ok(Default::default())}
  let slot=self.slot_mut(key.index);let mut root=Box::new(RetainedPublicationRoot::new(slot.value.take().unwrap()));root.stage(candidate,decision,grant)?;slot.replacement=Some(root);self.replacement_count+=1;self.replacement_decision=Some(identity);Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})
 }
 /// 🧷️ The actual candidates retain every alias; this constant observation fences their one shared decision and extent.
 pub fn replacement_group_identity(&self)->Option<(usize,usize)>{self.replacement_decision.map(|identity|(identity,self.replacement_count))}
 /// 🪪️ Borrows the exact shared decision allocation retained by this original row candidate.
 pub fn replacement_decision_identity(&self,key:ArtifactHistoryKey)->Result<Option<usize>,ValueError>{self.get_key(key).ok_or_else(||fault("replacement decision requires its exact original generation"))?;Ok(self.slot(key.index).replacement.as_ref().and_then(|root|root.decision_identity()))}
 /// 📐️ Quotes return of the displaced original and both candidate frame releases before settlement.
 pub fn replacement_settlement_demands(&self,key:ArtifactHistoryKey)->Result<RetirementDemand,ValueError>{
  self.get_key(key).ok_or_else(||fault("replacement settlement requires its exact visible original generation"))?;let Some(root)=self.slot(key.index).replacement.as_ref()else{return Ok(Default::default())};let mut d=root.settlement_demands();d.release_bytes+=size_of::<RetainedPublicationRoot<T>>();d.copy_bytes+=2*size_of::<T>()+2*size_of::<RetainedPublicationRoot<T>>()+size_of::<usize>();Ok(d)
 }
 /// 🤝️ Returns the displaced row and original decision retirement after publication or abort, preserving every direct reader.
 pub fn settle_replacement(&mut self,key:ArtifactHistoryKey,displaced:&mut Option<T>,decision:&mut Option<FactoryAuthority>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
  if displaced.is_some()||decision.is_some(){return Err(fault("replacement settlement requires empty original recipient slots"))}let d=self.replacement_settlement_demands(key)?;if self.slot(key.index).replacement.is_none()||!grant_admits(grant,d){return Ok(Default::default())}
  let root=self.slot_mut(key.index).replacement.as_mut().unwrap();let p=root.settle(displaced,decision,grant)?;if p.copied_items==0{return Ok(Default::default())}let root=self.slot_mut(key.index).replacement.take().unwrap();let root=*root;let original=match root.into_original(){Ok(original)=>original,Err(_)=>return Err(fault("replacement settlement did not return its original row"))};self.slot_mut(key.index).value=Some(original);self.replacement_count-=1;if self.replacement_count==0{self.replacement_decision=None;}Ok(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,released_bytes:d.release_bytes,..Default::default()})
 }
}
