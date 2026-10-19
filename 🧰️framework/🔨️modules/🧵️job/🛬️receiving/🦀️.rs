//! 🛬️ Original Job decoder custody preserves its consuming ledger and exact recipient between caller turns.
use semio_framework_value::{NativeDecodeControl,NativeDecodeProgress,NativeDecodeRetirementRecipient,ErasedSnapshotRetirement,ValueError,ValueRefusalKind,RetirementDemand,native_decoding::{NativeDecodeConstructionRecipient,NativeDetachedRetirementDecodeContinuation},retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::size_of;
/// 🧳️ One job owns the same decoder continuation and original retirement recipient until terminal closure.
pub struct OriginalJobDecodeOwner{receipt:Option<NativeDetachedRetirementDecodeContinuation>,recipient:NativeDecodeRetirementRecipient,closing:bool}
impl OriginalJobDecodeOwner{
 /// 🪑️ Creates one inline original recipient without allocating or granting decode capacity.
 pub fn empty()->Self{Self{receipt:None,recipient:NativeDecodeRetirementRecipient::new(),closing:false}}
 /// 🧵️ Reborrows the same consuming decoder, exact recipient and caller construction authority for one original turn.
 pub fn receive<T,E:From<ValueError>>(&mut self,grant:RetainedCloneGrant,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,construction:&mut dyn NativeDecodeConstructionRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
  if self.closing{return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job decoder is closing")));}if grant.maximum_items==0||grant.maximum_depth==0{return Err(E::from(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job receiving requires one original caller turn")));}
  let mut continuation=if let Some(receipt)=self.receipt.take(){match receipt.bind(&mut self.recipient){Ok(bound)=>bound,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(E::from(error));}}}else{let receipt=NativeDecodeControl::new_retained(callback).pause().map_err(E::from)?;receipt.with_retirement_recipient(&mut self.recipient).map_err(|(error,_)|E::from(error))?};
  let mut outcome=None;let result=continuation.decode(callback,|control|{control.admit_turn_capacity(grant.maximum_capacity_bytes)?;control.checkpoint()?;outcome=Some(control.with_construction_recipient(construction,operation));Ok(())});let(detached,_)=continuation.detach();self.receipt=Some(detached);result.map_err(E::from)?;outcome.ok_or_else(||E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job receiving did not visit its semantic producer")))?
 }
 /// ♻️ Loans only retirement charging for already born custody under the original grant, deadline and observer.
 pub fn retire<T>(&mut self,grant:RetainedCloneGrant,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,deadline:&mut dyn FnMut()->bool,operation:impl FnOnce(&mut semio_framework_value::native_decoding::NativeRetirementDecodeControl<'_, '_>)->Result<T,ValueError>)->Result<T,ValueError>{
  if !deadline(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job retirement deadline reached"));}if grant.maximum_items==0||grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job retirement requires its original caller turn"));}
  let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job retirement requires its existing decoder receipt"))?;let mut continuation=match receipt.bind(&mut self.recipient){Ok(bound)=>bound,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(error);}};
  let result=continuation.retire(grant,callback,operation);let(detached,_)=continuation.detach();self.receipt=Some(detached);result
 }
 /// 📊️ Reads cumulative original receiving allocation without granting another credit.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().map_or(0,NativeDetachedRetirementDecodeContinuation::owned_bytes)}
 /// 📏️ Quotes actual recipient closure before separately removing its scalar continuation.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.recipient.terminal_is_empty(){return self.recipient.next_demand(body);}Ok(RetirementDemand{copy_bytes:if self.receipt.is_some(){size_of::<Option<NativeDetachedRetirementDecodeContinuation>>()}else{0},depth:usize::from(self.receipt.is_some()),..Default::default()})}
 /// ♻️ Closes original returned construction ownership under independent caller grants before deleting the consuming receipt.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.closing=true;let d=self.retirement_demands(grant.maximum_copy_bytes)?;if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(RetainedCloneStep::Progress(Default::default()));}if !self.recipient.terminal_is_empty(){return self.recipient.close_step(grant);}if self.receipt.is_some(){self.receipt=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}));}Ok(RetainedCloneStep::Complete(Default::default()))}
 /// 🫙️ Every original decoder recipient has closed and its consuming receipt has been removed.
 pub fn terminal_is_empty(&self)->bool{self.receipt.is_none()&&self.recipient.terminal_is_empty()}
}
impl Drop for OriginalJobDecodeOwner{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original Job decoder requires funded recipient and scalar closure");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
