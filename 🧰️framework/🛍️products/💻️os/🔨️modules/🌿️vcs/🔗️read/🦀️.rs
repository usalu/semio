//! 🔗️ Original history slots yield genuine retained projections and return the same owner under independent grants.
use super::{ArtifactHistoryKey,ArtifactHistoryLedger};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value_factory::owned::FactoryOwnedSourceAuthority;
use semio_framework_value::retained_clone::{RetainedCloneBorrowAuthority,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneRef,RetainedCloneSourceTake,RetainedOwnedProjection,RetainedCloneStep,retained_clone_grant_admits};
use std::sync::Arc;
fn fault(message:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
use semio_framework_value_original_slot::{RetainedOriginalReadOwner,RetainedOriginalRead,RetainedOriginalReadReturn};

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ArtifactHistoryReadKey {authority:usize,issuance:u64,key:ArtifactHistoryKey}

impl<T> ArtifactHistoryLedger<T> {
 /// 🗝️ Captures this exact ledger backing, slot generation and visible group decision.
 pub fn retained_read_key_at(&self,position:usize)->Option<ArtifactHistoryReadKey>{self.retained_read_key(self.key_at(position)?)}
 /// 🪪️ Binds an existing revision index's original slot without scanning or recreating history.
 pub fn retained_read_key(&self,key:ArtifactHistoryKey)->Option<ArtifactHistoryReadKey>{self.get_key(key)?;Some(ArtifactHistoryReadKey{authority:self.authority(),issuance:self.read_issuance,key})}
 /// 🧷️ Borrows the exact already-captured issuer when another original reader owns this same slot.
 pub fn retained_read_issuer(&self,key:ArtifactHistoryReadKey)->Result<Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,ValueError>{Ok(self.original_read_slot(key)?.read.as_ref().map(|read|read.factory()))}
 fn original_read_slot(&self,key:ArtifactHistoryReadKey)->Result<&super::ArtifactHistorySlot<T>,ValueError>{if key.authority!=self.authority()||key.issuance!=self.read_issuance{return Err(fault("history read belongs to another original ledger"));}self.get_key(key.key).ok_or_else(||fault("history read slot generation or visibility changed"))?;Ok(self.slot(key.key.index))}
 /// 🧮️ Queries the original frame or current source birth without taking its value.
 pub fn retained_read_demands(&self,key:ArtifactHistoryReadKey)->Result<RetirementDemand,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.read.as_ref().map_or(Ok(RetainedOriginalRead::<T>::initial_demands()),|read|read.birth_demands())}
 /// 🎟️ Moves the actual slot value and installed issuer into two independently granted read births.
 pub fn advance_retained_read(&mut self,key:ArtifactHistoryReadKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> where T:Send+Sync+'static {
  let demand=self.retained_read_demands(key)?;
  if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
  let read_head=self.read_head;
  let slot=self.slot_mut(key.key.index);
  if let Some(read)=slot.read.as_mut(){if !read.matches_factory(factory){return Err(fault("history read issuer changed from its original catalog"));}return read.advance(grant);}
  let original=slot.value.take().ok_or_else(||fault("history read lost its original slot value"))?;
  slot.read=Some(Box::new(RetainedOriginalRead::new(original,Arc::clone(factory))));
  slot.read_next=read_head;slot.read_previous=None;
  if let Some(head)=read_head{self.slot_mut(head).read_previous=Some(key.key.index);}
  self.read_head=Some(key.key.index);self.read_count+=1;
  Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,..Default::default()}))
 }
 /// 🧷️ Borrows a genuine root lease; callers project their recorded forward operation by field and index.
 pub fn retained_read(&self,key:ArtifactHistoryReadKey)->Result<RetainedCloneRef<'_,T>,ValueError>{self.original_read_slot(key)?.read.as_ref().ok_or_else(||fault("history read requires granted original source birth"))?.borrow()}
 /// 🪪️ Retains a genuine owning alias of the exact root lease so a recorded forward operation can be projected from it later.
 pub fn retain_read(&self,key:ArtifactHistoryReadKey,grant:RetainedCloneGrant)->Result<(RetainedOwnedProjection<T>,RetainedCloneProgress),ValueError> where T:Sync{self.original_read_slot(key)?.read.as_ref().ok_or_else(||fault("history read requires granted original source birth"))?.retain_original(grant)}
 /// 🧭️ Observes the next original leased slot in constant time for Store cancellation.
 pub fn next_retained_read_key(&self)->Option<ArtifactHistoryReadKey>{let index=self.read_head?;Some(ArtifactHistoryReadKey{authority:self.authority(),issuance:self.read_issuance,key:ArtifactHistoryKey{index,generation:self.slot(index).generation}})}
 /// 🚪️ Queries exact source return through the caller's unchanged body allowance.
 pub fn retained_read_return_demands(&self,key:ArtifactHistoryReadKey,body:usize)->Result<RetirementDemand,ValueError>{self.original_read_slot(key)?.read.as_ref().map_or(Ok(Default::default()),|read|read.return_demands(body))}
 /// ↩️ Restores the same original slot and returns its captured issuer alias for controlled cleanup.
 pub fn return_retained_read(&mut self,key:ArtifactHistoryReadKey,returned_factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
  self.original_read_slot(key)?;
  if returned_factory.is_some(){return Err(fault("history source issuer return requires an empty original slot"));}
  let slot=self.slot_mut(key.key.index);
  let Some(read)=slot.read.as_mut()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
  match read.return_original(grant)?{
   RetainedOriginalReadReturn::Pending(receipt)=>Ok(RetainedCloneStep::Progress(receipt)),
   RetainedOriginalReadReturn::Ready(original,factory,receipt)=>{if !read.terminal_is_empty(){return Err(fault("history source claimed original return before terminal custody"));}slot.value=Some(original);*returned_factory=Some(factory);drop(slot.read.take());let previous=slot.read_previous.take();let next=slot.read_next.take();if let Some(previous)=previous{self.slot_mut(previous).read_next=next;}else{self.read_head=next;}if let Some(next)=next{self.slot_mut(next).read_previous=previous;}self.read_count-=1;Ok(RetainedCloneStep::Complete(receipt))}
  }
 }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
