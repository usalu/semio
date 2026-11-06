//! 🧵️ Local decoder receipts preserve the uniquely identified original retirement recipient.
use super::{NativeDecodeControl,NativeDecodeContinuation,NativeDecodeProgress,NativeDecodeRetirementRecipient,allocation};
use crate::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
/// 🧳️ Retains an owned output or operation error together with an independent context refusal.
#[derive(Debug)]
pub struct NativeDecodeCallFailure<T,E>{pub output:Option<T>,pub operation:Option<E>,pub context:Option<ValueError>,pub receipt_refusal:Option<ValueError>}
impl<T:crate::retirement::RetireOwned,E:crate::retirement::RetireOwned> crate::retirement::RetireOwned for NativeDecodeCallFailure<T,E>{
 fn retirement(self)->Box<dyn crate::retirement::RetirementCursor>{crate::retirement::sequence(vec![crate::retirement::deferred(self.output),crate::retirement::deferred(self.operation),crate::retirement::deferred(self.context),crate::retirement::deferred(self.receipt_refusal)])}
 fn retirement_birth_bytes(&self)->Option<usize>{crate::retirement::sequence_birth_bytes(&[crate::retirement::deferred_birth_bytes_for(&self.output),crate::retirement::deferred_birth_bytes_for(&self.operation),crate::retirement::deferred_birth_bytes_for(&self.context),crate::retirement::deferred_birth_bytes_for(&self.receipt_refusal)])}
 fn controlled_retirement_supported()->bool{crate::retirement::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&crate::retirement::RetirementSupport<'_>)->bool{path.supports::<T>()&&path.supports::<E>()}
}
/// 🫴️ Loans one original local receipt and its actual retained recipient.
pub struct NativeRetirementDecodeContinuation<'recipient>{receipt:Option<NativeDecodeContinuation>,recipient:&'recipient mut NativeDecodeRetirementRecipient}
/// 🔐️ Detaches a consuming local receipt with its unique original recipient identity.
pub struct NativeDetachedRetirementDecodeContinuation{receipt:NativeDecodeContinuation,recipient:u64}
impl NativeDecodeContinuation{
 /// 🪑️ Installs the initial empty original recipient without granting additional authority.
 pub fn with_retirement_recipient<'recipient>(self,recipient:&'recipient mut NativeDecodeRetirementRecipient)->Result<NativeRetirementDecodeContinuation<'recipient>,(ValueError,Self)>{if !recipient.terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoder continuation requires its empty original recipient"),self))}if (self.total!=0&&self.completed>self.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native decoder retirement continuation has invalid original receipt"),self))}Ok(NativeRetirementDecodeContinuation{receipt:Some(self),recipient})}
}
impl<'control> NativeDecodeControl<'control>{
 /// ⏸️ Transfers the same local admission and installed recipient together after a hop.
 pub fn pause_with_retirement(mut self)->Result<NativeRetirementDecodeContinuation<'control>,ValueError>{if self.construction.is_some(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction loan cannot pause outside its original receiving hop"));}if !matches!(self.allocation,allocation::Binding::Local){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign decoding requires its original allocation continuation"))}let receipt=self.continuation()?;let recipient=self.retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decoding has no installed original retirement recipient"))?;Ok(NativeRetirementDecodeContinuation{receipt:Some(receipt),recipient})}
}
struct Scope<'slot,'control>{control:NativeDecodeControl<'control>,slot:&'slot mut Option<NativeDecodeContinuation>}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.slot=Some(NativeDecodeContinuation{actual_work:self.control.work.actual,reserved_work:self.control.work.reserved,reserved_issuer:self.control.work.reserved_issuer,reserved_action:self.control.work.reserved_action,work_quote_pending:self.control.work.quote_pending,ledger:self.control.ledger,receiving:self.control.receiving,maximum_bytes:self.control.maximum_bytes,owned_bytes:self.control.owned_bytes,completed:self.control.completed,total:self.control.total,stage:self.control.stage});self.control.work.reserved=0;self.control.work.reserved_issuer=None;self.control.work.reserved_action=None;self.control.work.quote_pending=false;}}
impl<'recipient> NativeRetirementDecodeContinuation<'recipient>{
 /// ♻️ Loans the same consuming receipt and exact recipient for funded retirement after semantic cancellation.
 pub fn retire<T,E:From<ValueError>>(&mut self,grant:crate::retained_clone::RetainedCloneGrant,progress:&mut dyn FnMut(NativeDecodeProgress)->bool,work:&mut super::NativeDecodeWorkLoan<'_>,operation:impl FnOnce(&mut super::NativeRetirementDecodeControl<'_, '_>)->Result<T,E>)->Result<T,NativeDecodeCallFailure<T,E>>{self.decode(progress,|original|original.with_work_recipient(work,|same|same.with_retirement(grant,operation)))}
 /// 📊️ Observes cumulative allocation retained across original decode and retirement hops.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("original decoder receipt exists between hops").owned_bytes}
 /// ⏱️ Observes cumulative actual work on the original decoding ledger.
 pub fn actual_work(&self)->u64{self.receipt.as_ref().expect("original decoder receipt exists between hops").actual_work}
 /// ⏳️ Observes an unresolved original work reservation without replacing its payer.
 pub fn pending_work(&self)->u64{self.receipt.as_ref().map_or(0,|receipt|receipt.reserved_work)}
 /// ⏳️ Includes a pending quote or an admitted zero-work reservation.
 pub fn has_pending_work(&self)->bool{self.receipt.as_ref().is_some_and(|receipt|receipt.reserved_issuer.is_some()||receipt.work_quote_pending)}
 /// 🔁️ Runs an original local hop with the same recipient and cumulative admission.
 pub fn decode<T,E>(&mut self,progress:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,NativeDecodeCallFailure<T,E>>{
  let context=|error|NativeDecodeCallFailure{output:None,operation:None,context:Some(error),receipt_refusal:None};let Some(receipt)=self.receipt.as_ref()else{return Err(context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original decoder receipt is already receiving")))};if (receipt.total!=0&&receipt.completed>receipt.total){return Err(context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original decode resume preserves invalid cumulative receipt")))}
  let Some(receipt)=self.receipt.take()else{return Err(context(ValueError::invariant("validated decoder receipt absent")))};let mut control=NativeDecodeControl::resume_validated(receipt,progress);control.retirement=Some(&mut *self.recipient);let mut scope=Scope{control,slot:&mut self.receipt};let result=scope.control.scoped_stage(operation);let context=scope.control.continuation().err();match(result,context){(Ok(output),None)=>Ok(output),(Ok(output),Some(error))=>Err(NativeDecodeCallFailure{output:Some(output),operation:None,context:Some(error),receipt_refusal:None}),(Err(error),context)=>Err(NativeDecodeCallFailure{output:None,operation:Some(error),context,receipt_refusal:None})}
 }
 /// 🪪️ Separates the scalar receipt from its original recipient while preserving unique identity.
 pub fn detach(mut self)->(NativeDetachedRetirementDecodeContinuation,&'recipient mut NativeDecodeRetirementRecipient){let recipient=self.recipient.identity;(NativeDetachedRetirementDecodeContinuation{receipt:self.receipt.take().expect("original decoder receipt exists between hops"),recipient},self.recipient)}
}
impl NativeDetachedRetirementDecodeContinuation{
 /// 🪪️ Borrows the same consuming decoder ledger and uniquely retained recipient without issuing any authority.
 pub fn identity(&self)->(u64,u64){(self.receipt.ledger,self.recipient)}
 /// ⏳️ Reports work still reserved by the original caller.
 pub fn pending_work(&self)->u64{self.receipt.reserved_work}
 /// ⏳️ Includes a pending quote or an admitted zero-work reservation.
 pub fn has_pending_work(&self)->bool{self.receipt.reserved_issuer.is_some()||self.receipt.work_quote_pending}
 /// ♻️ Releases the exact empty original recipient box under its caller grant and observer.
 pub fn release_recipient_box(&mut self,recipient:&mut Option<Box<NativeDecodeRetirementRecipient>>,grant:crate::RetainedCloneGrant,progress:&mut dyn FnMut(NativeDecodeProgress)->bool)->Result<crate::RetainedCloneProgress,ValueError>{let original=recipient.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original recipient box was already released"))?;if original.identity!=self.recipient{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"recipient box release requires original unique identity"))}if !original.terminal_is_empty(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original recipient box retains unfinished retirement"))}if self.has_pending_work(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original recipient retains unresolved work reservation"))}if (self.receipt.total!=0&&self.receipt.completed>self.receipt.total){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"recipient release preserves invalid original receipt"))}if grant.maximum_items==0{return Ok(Default::default())}if grant.maximum_depth<1{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original recipient release requires admitted depth"))}let bytes=size_of::<NativeDecodeRetirementRecipient>();if bytes>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original recipient release exceeds caller release grant"))}if !progress(NativeDecodeProgress{completed:self.receipt.completed,total:self.receipt.total,owned_bytes:self.receipt.owned_bytes}){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original recipient release canceled"))}drop(recipient.take().unwrap());Ok(crate::RetainedCloneProgress{copied_items:1,released_bytes:bytes,..Default::default()})}
 /// 🧾️ Observes the unchanged original cumulative allocation receipt.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// ⏱️ Observes cumulative actual work on the detached original ledger.
 pub fn actual_work(&self)->u64{self.receipt.actual_work}
 /// 🔗️ Rebinds only the original recipient, preserving the consuming receipt on mismatch.
 pub fn bind<'recipient>(self,recipient:&'recipient mut NativeDecodeRetirementRecipient)->Result<NativeRetirementDecodeContinuation<'recipient>,(ValueError,Self)>{if self.recipient!=recipient.identity{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"detached decoder requires its uniquely identified original recipient"),self))}Ok(NativeRetirementDecodeContinuation{receipt:Some(self.receipt),recipient})}
}
