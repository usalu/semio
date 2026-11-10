//! 🔗️ Original history slots yield genuine retained projections and return the same owner under independent grants.
use super::{ArtifactHistoryKey,ArtifactHistoryLedger};
use semio_framework_value::{ArtifactOwnedValueRetirementFactory,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneRef,RetainedCloneStep,RetainedOwnedProjectionAdmission};
use semio_framework_value::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect};
use std::sync::Arc;
fn fault(message:&'static str)->ValueError {ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
type ReadReply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type ReadDone<'r>=OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct ArtifactHistoryReadKey {pub authority:usize,pub issuance:u64,pub key:ArtifactHistoryKey}

impl<T> ArtifactHistoryLedger<T> {
 /// 🗝️ Captures this exact ledger backing, slot generation and visible group decision.
 pub fn retained_read_key_at(&self,position:usize)->Option<ArtifactHistoryReadKey>{self.retained_read_key(self.key_at(position)?)}
 /// 🪪️ Binds an existing revision index's original slot without scanning or recreating history.
 pub fn retained_read_key(&self,key:ArtifactHistoryKey)->Option<ArtifactHistoryReadKey>{self.get_key(key)?;if self.slot(key.index).replacement.is_some(){return None}Some(ArtifactHistoryReadKey{authority:self.authority(),issuance:self.read_issuance,key})}
 /// 🧷️ Borrows the exact already-captured issuer when another original reader owns this same slot.
 pub fn retained_read_issuer(&self,key:ArtifactHistoryReadKey)->Result<Option<&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,ValueError>{Ok(self.original_read_slot(key)?.original.read_issuer())}
 fn original_read_slot(&self,key:ArtifactHistoryReadKey)->Result<&super::ArtifactHistorySlot<T>,ValueError>{if key.authority!=self.authority()||key.issuance!=self.read_issuance{return Err(fault("history read belongs to another original ledger"));}self.get_key(key.key).ok_or_else(||fault("history read slot generation or visibility changed"))?;let slot=self.slot(key.key.index);if slot.replacement.is_some(){return Err(fault("history read requires its original replacement to settle"))}Ok(slot)}
 /// 🧮️ Queries the original frame or current source birth without taking its value.
 pub fn retained_read_demands(&self,key:ArtifactHistoryReadKey,body:usize)->Result<RetirementDemand,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.original.demands(slot.original.key(),body)}
 /// ⛽️ Quotes the Work units of the same selected read action.
 pub fn retained_read_work_demand(&self,key:ArtifactHistoryReadKey,body:usize)->Result<u64,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.original.read_work_demand(slot.original.key(),body)}
 /// 🎟️ Moves the actual slot value and installed issuer into its granted read birth under the caller's recorded reply.
 pub fn advance_retained_read_into<'r>(&mut self,key:ArtifactHistoryReadKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadDone<'r>,ValueError> where T:Send+Sync+'static {
  self.original_read_slot(key)?;let index=key.key.index;let was=self.slot(index).original.has_read();
  let slot=self.slot_mut(index);let slot_key=slot.original.key();let done=slot.original.advance_read_into(slot_key,factory,effect,reply);
  if !was&&self.slot(index).original.has_read(){self.link_read(index)}
  done
 }
 /// 🔁️ Recovers the unrecorded read birth step under its unchanged invocation.
 pub fn recover_advance_retained_read_into<'r>(&mut self,key:ArtifactHistoryReadKey,factory:&Arc<dyn ArtifactOwnedValueRetirementFactory<T>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadDone<'r>,ValueError> where T:Send+Sync+'static {
  self.original_read_slot(key)?;let slot=self.slot_mut(key.key.index);let slot_key=slot.original.key();slot.original.recover_advance_read_into(slot_key,factory,effect,reply)
 }
 fn link_read(&mut self,index:u32){let head=self.read_head;let slot=self.slot_mut(index);slot.read_next=head;slot.read_previous=None;if let Some(head)=head{self.slot_mut(head).read_previous=Some(index);}self.read_head=Some(index);self.read_count+=1;}
 fn unlink_read(&mut self,index:u32){let slot=self.slot_mut(index);let previous=slot.read_previous.take();let next=slot.read_next.take();if let Some(previous)=previous{self.slot_mut(previous).read_next=next;}else{self.read_head=next;}if let Some(next)=next{self.slot_mut(next).read_previous=previous;}self.read_count-=1;}
 /// 🤲️ Borrows a genuine root lease; callers project their recorded forward operation by field and index.
 pub fn retained_read(&self,key:ArtifactHistoryReadKey)->Result<RetainedCloneRef<'_,T>,ValueError>{let slot=self.original_read_slot(key)?;slot.original.borrow(slot.original.key())}
 /// 📏️ Quotes the retained projection of this original read into the caller's admitted output.
 pub fn retain_read_demand(&self,key:ArtifactHistoryReadKey,output:&RetainedOwnedProjectionAdmission<T>)->Result<RetirementDemand,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.original.retain_read_demand(slot.original.key(),output)}
 /// ⛽️ Quotes the Work units of the same retained projection.
 pub fn retain_read_work_demand(&self,key:ArtifactHistoryReadKey,output:&RetainedOwnedProjectionAdmission<T>)->Result<u64,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.original.retain_read_work_demand(slot.original.key(),output)}
 /// 📽️ Retains the genuine slot source into the caller's admission under the recorded reply.
 pub fn retain_read_into<'r>(&self,key:ArtifactHistoryReadKey,output:&mut RetainedOwnedProjectionAdmission<T>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError> where T:Send+Sync+'static {let slot=self.original_read_slot(key)?;slot.original.retain_read_into(slot.original.key(),output,effect,reply)}
 /// 🧭️ Observes the next original leased slot in constant time for Store cancellation.
 pub fn next_retained_read_key(&self)->Option<ArtifactHistoryReadKey>{let index=self.read_head?;Some(ArtifactHistoryReadKey{authority:self.authority(),issuance:self.read_issuance,key:ArtifactHistoryKey{index,generation:self.slot(index).generation}})}
 /// 🚪️ Queries exact source return through the caller's unchanged body allowance.
 pub fn retained_read_return_demands(&self,key:ArtifactHistoryReadKey,body:usize)->Result<RetirementDemand,ValueError>{let slot=self.original_read_slot(key)?;slot.original.return_demands(slot.original.key(),body)}
 /// ⛽️ Quotes the Work units of the same exact source return.
 pub fn retained_read_return_work_demand(&self,key:ArtifactHistoryReadKey,body:usize)->Result<u64,ValueError>{let slot=self.original_read_slot(key)?;slot.original.return_work_demand(slot.original.key(),body)}
 /// ↩️ Restores the same original slot and returns its captured issuer alias for controlled cleanup.
 pub fn return_retained_read_into<'r>(&mut self,key:ArtifactHistoryReadKey,returned_factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadDone<'r>,ValueError> where T:Send+Sync+'static {
  self.original_read_slot(key)?;let index=key.key.index;let was=self.slot(index).original.has_read();
  let slot=self.slot_mut(index);let slot_key=slot.original.key();let done=slot.original.return_read_into(slot_key,returned_factory,effect,reply);
  if was&&!self.slot(index).original.has_read(){self.unlink_read(index)}
  done
 }
 /// 🔁️ Recovers the unrecorded read return step under its unchanged invocation.
 pub fn recover_return_retained_read_into<'r>(&mut self,key:ArtifactHistoryReadKey,returned_factory:&mut Option<Arc<dyn ArtifactOwnedValueRetirementFactory<T>>>,effect:&mut RetainedAllocationEffect<'_>,reply:ReadReply<'r>)->Result<ReadDone<'r>,ValueError> where T:Send+Sync+'static {
  self.original_read_slot(key)?;let index=key.key.index;let was=self.slot(index).original.has_read();
  let slot=self.slot_mut(index);let slot_key=slot.original.key();let done=slot.original.recover_return_read_into(slot_key,returned_factory,effect,reply);
  if was&&!self.slot(index).original.has_read(){self.unlink_read(index)}
  done
 }
}
/// 🧷️ Runs one reply-shaped read child to its recorded result inside the caller's native action; a suspended child violates the native owner contract.
pub fn complete_in_action<T,E>(call:impl for<'r> FnOnce(OriginalEffectReply<'r,T,E>)->Result<OriginalEffectReplyComplete<'r,T,E>,ValueError>)->Result<Result<T,E>,ValueError>{let mut slot=None;drop(call(OriginalEffectReply::borrow(&mut slot)?)?);slot.ok_or_else(||fault("history original read suspended inside a native action"))}
