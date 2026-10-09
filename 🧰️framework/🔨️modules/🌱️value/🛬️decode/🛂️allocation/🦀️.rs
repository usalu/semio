//! 🛂️ Decoder reservations retain original receiving allocation and retirement authority.
use super::{NativeDecodeControl,NativeDecodeContinuation,NativeDecodeProgress,NativeDecodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};
/// 📦️ Describes one reservation before storage or cumulative accounting changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeDecodeAllocation{pub bytes:usize,pub owned_bytes:usize,pub next_owned_bytes:usize,pub maximum_bytes:usize}
/// 🚚️ Carries original receiving authority through native erased futures.
#[cfg(not(target_arch="wasm32"))]
pub type NativeDecodeAllocationPort<'a>=dyn FnMut(NativeDecodeAllocation)->Result<(),ValueError>+Send+'a;
/// 🌐️ Keeps original receiving authority on the local guest executor.
#[cfg(target_arch="wasm32")]
pub type NativeDecodeAllocationPort<'a>=dyn FnMut(NativeDecodeAllocation)->Result<(),ValueError>+'a;
pub(super) enum Binding<'a>{Local,Forwarded(&'a mut NativeDecodeAllocationPort<'a>)}
/// 🧵️ Retains original ports, retirement slot and cumulative receipt across callback lifetimes.
pub struct NativeForwardedDecodeContinuation<'a>{receipt:Option<NativeDecodeContinuation>,allocate:&'a mut NativeDecodeAllocationPort<'a>,retirement:Option<&'a mut NativeDecodeRetirementRecipient>}
/// 🧮️ Exposes canonical parsing under original receiving allocation admission.
pub struct NativeForwardedDecodeControl<'a>{inner:NativeDecodeControl<'a>}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedDecodeReceipt{receipt:NativeDecodeContinuation,recipient:usize}
impl NativeForwardedDecodeReceipt{
 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}
impl<'a> NativeDecodeControl<'a>{
 /// 🏭️ Binds every reservation to the original caller allocation port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut NativeDecodeAllocationPort<'a>)->NativeForwardedDecodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=Binding::Forwarded(allocate);NativeForwardedDecodeControl{inner}}
}
impl<'a> NativeForwardedDecodeControl<'a>{
 /// ⏸️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(self)->Result<NativeForwardedDecodeReceipt,(ValueError,Self)>{
  let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};
  let Some(recipient)=self.inner.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};
  Ok(NativeForwardedDecodeReceipt{receipt,recipient:std::ptr::from_ref(&**recipient)as usize})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedDecodeReceipt,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool,allocate:&'a mut NativeDecodeAllocationPort<'a>,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedDecodeReceipt)>{
  if receipt.recipient!=std::ptr::from_ref(&*recipient)as usize||receipt.receipt.owned_bytes>receipt.receipt.maximum_bytes||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let mut inner=NativeDecodeControl::resume(receipt.receipt,callback).expect("validated original native receiving receipt");inner.allocation=Binding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves original allocation and retirement authority together with the cumulative receipt.
 pub fn pause(mut self)->Result<NativeForwardedDecodeContinuation<'a>,ValueError>{let receipt=self.inner.continuation()?;let Binding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,Binding::Local)else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded decoder lacks original allocation port"))};Ok(NativeForwardedDecodeContinuation{receipt:Some(receipt),allocate,retirement:self.inner.retirement.take()})}
 /// ▶️ Rebinds only progress while retaining original allocation and retirement authority.
 pub fn resume(receipt:NativeForwardedDecodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeDecodeProgress)->bool)->Result<Self,ValueError>{let mut inner=NativeDecodeControl::resume(receipt.receipt.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving"))?,callback)?;inner.allocation=Binding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
impl<'a> NativeForwardedDecodeContinuation<'a>{
 /// 🏭️ Binds the original caller port directly without creating replacement progress authority.
 pub fn new(maximum_bytes:usize,allocate:&'a mut NativeDecodeAllocationPort<'a>)->Self{Self{receipt:Some(NativeDecodeContinuation{receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🫴️ Installs the original caller return slot before retained decoder ownership exists.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeDecodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded decoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 🔁️ Reborrows original ports and retirement slot for one synchronous receiving hop.
 pub fn decode<T>(&mut self,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded decoder is already receiving"))?;let mut control=NativeDecodeControl::resume(receipt,callback)?;control.allocation=Binding::Forwarded(&mut *self.allocate);control.retirement=self.retirement.as_deref_mut();let result=control.scoped_stage(operation);self.receipt=Some(control.continuation()?);result}
 /// 📏️ Returns the original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").maximum_bytes}
 /// 📊️ Returns cumulative original receiving admission.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("decoder receipt present outside receiving hop").owned_bytes}
}
impl<'a> std::ops::Deref for NativeForwardedDecodeControl<'a>{type Target=NativeDecodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedDecodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
