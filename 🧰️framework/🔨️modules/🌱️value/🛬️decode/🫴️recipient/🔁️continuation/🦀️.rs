//! 🧵️ Local decoder receipts preserve the uniquely identified original retirement recipient.
use super::{NativeDecodeControl,NativeDecodeContinuation,NativeDecodeProgress,NativeDecodeRetirementRecipient,allocation};
use crate::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement};
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
/// 🫴️ Loans one original local receipt and its actual retained recipient.
pub struct NativeRetirementDecodeContinuation<'recipient>{receipt:Option<NativeDecodeContinuation>,recipient:&'recipient mut NativeDecodeRetirementRecipient}
/// 🔐️ Detaches a consuming local receipt with its unique original recipient identity.
pub struct NativeDetachedRetirementDecodeContinuation{receipt:NativeDecodeContinuation,recipient:u64}
impl NativeDecodeContinuation{
 /// 🪑️ Installs the initial empty original recipient without granting additional authority.
 pub fn with_retirement_recipient<'recipient>(self,recipient:&'recipient mut NativeDecodeRetirementRecipient)->Result<NativeRetirementDecodeContinuation<'recipient>,(ValueError,Self)>{if !recipient.terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoder continuation requires its empty original recipient"),self))}if self.owned_bytes>self.maximum_bytes||(self.total!=0&&self.completed>self.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native decoder retirement continuation has invalid original receipt"),self))}Ok(NativeRetirementDecodeContinuation{receipt:Some(self),recipient})}
}
impl<'control> NativeDecodeControl<'control>{
 /// ⏸️ Transfers the same local admission and installed recipient together after a hop.
 pub fn pause_with_retirement(mut self)->Result<NativeRetirementDecodeContinuation<'control>,ValueError>{if !matches!(self.allocation,allocation::Binding::Local){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign decoding requires its original allocation continuation"))}let receipt=self.continuation()?;let recipient=self.retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native decoding has no installed original retirement recipient"))?;Ok(NativeRetirementDecodeContinuation{receipt:Some(receipt),recipient})}
}
struct Scope<'slot,'control>{control:NativeDecodeControl<'control>,slot:&'slot mut Option<NativeDecodeContinuation>}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.slot=Some(NativeDecodeContinuation{receiving:self.control.receiving,maximum_bytes:self.control.maximum_bytes,owned_bytes:self.control.owned_bytes,completed:self.control.completed,total:self.control.total,stage:self.control.stage});}}
impl<'recipient> NativeRetirementDecodeContinuation<'recipient>{
 /// 🧾️ Observes cumulative allocation retained across original decode and retirement hops.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("original decoder receipt exists between hops").owned_bytes}
 /// 🔁️ Runs an original local hop with the same recipient and cumulative admission.
 pub fn decode<T>(&mut self,progress:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original decoder receipt is already receiving"))?;let mut control=NativeDecodeControl::resume(receipt,progress)?;control.retirement=Some(&mut *self.recipient);let mut scope=Scope{control,slot:&mut self.receipt};let result=scope.control.scoped_stage(operation);scope.control.continuation()?;result}
 /// 🧵️ Separates the scalar receipt from its original recipient while preserving unique identity.
 pub fn detach(mut self)->(NativeDetachedRetirementDecodeContinuation,&'recipient mut NativeDecodeRetirementRecipient){let recipient=self.recipient.identity;(NativeDetachedRetirementDecodeContinuation{receipt:self.receipt.take().expect("original decoder receipt exists between hops"),recipient},self.recipient)}
}
impl NativeDetachedRetirementDecodeContinuation{
 /// 🧾️ Observes the unchanged original cumulative allocation receipt.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 🔗️ Rebinds only the original recipient, preserving the consuming receipt on mismatch.
 pub fn bind<'recipient>(self,recipient:&'recipient mut NativeDecodeRetirementRecipient)->Result<NativeRetirementDecodeContinuation<'recipient>,(ValueError,Self)>{if self.recipient!=recipient.identity{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"detached decoder requires its uniquely identified original recipient"),self))}Ok(NativeRetirementDecodeContinuation{receipt:Some(self.receipt),recipient})}
}
