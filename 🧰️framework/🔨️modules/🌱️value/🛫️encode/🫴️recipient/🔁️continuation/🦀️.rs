//! 🧵️ Native consuming receipts retain the actual caller recipient without foreign allocation authority.
use super::{NativeEncodeControl,NativeEncodeContinuation,NativeEncodeProgress,NativeEncodeRetirementRecipient,allocation};
use crate::{ValueError,ValueRefusalKind,ErasedSnapshotRetirement};
/// 🫴️ Preserves original local admission beside its original borrowed retirement recipient.
pub struct NativeRetirementEncodeContinuation<'recipient>{receipt:Option<NativeEncodeContinuation>,recipient:&'recipient mut NativeEncodeRetirementRecipient}
/// 🔐️ Preserves a consuming local receipt beside its unique original recipient identity.
pub struct NativeDetachedRetirementEncodeContinuation{receipt:NativeEncodeContinuation,recipient:u64}
impl NativeEncodeContinuation{
 /// 📏️ Observes the original caller ceiling without granting new authority.
 pub fn maximum_bytes(&self)->usize{self.maximum_bytes}
 /// 📊️ Observes cumulative admission from the original consuming receipt.
 pub fn owned_bytes(&self)->usize{self.owned_bytes}

 /// 🪑️ Binds the actual caller return slot without adding a port, ceiling or progress observer.
 pub fn with_retirement_recipient<'recipient>(self,recipient:&'recipient mut NativeEncodeRetirementRecipient)->Result<NativeRetirementEncodeContinuation<'recipient>,(ValueError,NativeEncodeContinuation)>{if !recipient.terminal_is_empty(){return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"native continuation requires its empty original retirement recipient"),self))}if self.owned_bytes>self.maximum_bytes||(self.total!=0&&self.completed>self.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native retirement continuation has invalid original receipt"),self))}Ok(NativeRetirementEncodeContinuation{receipt:Some(self),recipient})}
}
impl<'control> NativeEncodeControl<'control>{
 /// ⏸️ Transfers local admission and the installed original return slot together.
 pub fn pause_with_retirement(mut self)->Result<NativeRetirementEncodeContinuation<'control>,ValueError>{if !matches!(self.allocation,allocation::Binding::Local){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"foreign encoding requires its original allocation continuation"))}let recipient=self.retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native encoding has no installed original retirement recipient"))?;let receipt=self.pause()?;Ok(NativeRetirementEncodeContinuation{receipt:Some(receipt),recipient})}
}
struct Scope<'slot,'control>{control:NativeEncodeControl<'control>,slot:&'slot mut Option<NativeEncodeContinuation>}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.slot=Some(NativeEncodeContinuation{receiving:self.control.receiving,maximum_bytes:self.control.maximum_bytes,owned_bytes:self.control.owned_bytes,completed:self.control.completed,total:self.control.total,stage:self.control.stage});}}
impl<'recipient> NativeRetirementEncodeContinuation<'recipient>{
 /// 🧵️ Detaches the original scalar receipt while retaining the unique recipient identity.
 pub fn detach(mut self)->(NativeDetachedRetirementEncodeContinuation,&'recipient mut NativeEncodeRetirementRecipient){let recipient=self.recipient.identity;(NativeDetachedRetirementEncodeContinuation{receipt:self.receipt.take().expect("original encoder receipt exists between hops"),recipient},self.recipient)}
 /// 🫴️ Returns the unique original admission receipt and caller recipient together.
 pub fn into_parts(mut self)->(NativeEncodeContinuation,&'recipient mut NativeEncodeRetirementRecipient){(self.receipt.take().expect("native retirement receipt exists between hops"),self.recipient)}
 /// 🔁️ Runs one original local hop while preserving recipient custody and cumulative allocation admission.
 pub fn encode<T>(&mut self,progress:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native retirement continuation is already receiving"))?;let mut control=NativeEncodeControl::resume(receipt,progress)?;control.retirement=Some(&mut *self.recipient);let mut scope=Scope{control,slot:&mut self.receipt};let result=scope.control.scoped_stage(operation);scope.control.continuation()?;result}
 /// 📏️ Returns the complete original caller allowance.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("native retirement receipt exists between hops").maximum_bytes}
 /// 📊️ Observes exact admission retained across every original encode and close hop.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("native retirement receipt exists between hops").owned_bytes}
}
impl NativeDetachedRetirementEncodeContinuation{
 /// 🧾️ Observes unchanged original cumulative allocation admission.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 🔗️ Rebinds only the original recipient without consuming a mismatched receipt.
 pub fn bind<'recipient>(self,recipient:&'recipient mut NativeEncodeRetirementRecipient)->Result<NativeRetirementEncodeContinuation<'recipient>,(ValueError,Self)>{if self.recipient!=recipient.identity{return Err((ValueError::literal(ValueRefusalKind::OwnershipLimit,"detached encoder requires its uniquely identified original recipient"),self))}Ok(NativeRetirementEncodeContinuation{receipt:Some(self.receipt),recipient})}
}
