//! 🛂️ Foreign physical factories obtain original receiving admission before each reservation.
use super::{NativeEncodeControl,NativeEncodeContinuation,NativeEncodeProgress,NativeEncodeRetirementRecipient};
use crate::{ValueError,ValueRefusalKind};

/// 📦️ Describes one exact requested allocation before either ledger or storage changes.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct NativeEncodeAllocation{pub bytes:usize,pub owned_bytes:usize,pub next_owned_bytes:usize,pub maximum_bytes:usize}

/// 🚚️ Carries original receiving authority through native erased futures.
#[cfg(not(target_arch="wasm32"))]
pub type NativeEncodeAllocationPort<'a>=dyn FnMut(NativeEncodeAllocation)->Result<(),ValueError>+Send+'a;
/// 🌐️ Keeps the same original receiving authority on the local guest executor.
#[cfg(target_arch="wasm32")]
pub type NativeEncodeAllocationPort<'a>=dyn FnMut(NativeEncodeAllocation)->Result<(),ValueError>+'a;

pub(super) enum Binding<'a>{Local,Forwarded(&'a mut NativeEncodeAllocationPort<'a>)}

/// 🧵️ Retains the original allocation port and cumulative ledger across progress callback lifetimes.
pub struct NativeForwardedEncodeContinuation<'a>{receipt:Option<NativeEncodeContinuation>,allocate:&'a mut NativeEncodeAllocationPort<'a>,retirement:Option<&'a mut NativeEncodeRetirementRecipient>}

/// 🧮️ Exposes the canonical controlled serializer while preserving foreign allocation provenance.
pub struct NativeForwardedEncodeControl<'a>{inner:NativeEncodeControl<'a>}

/// 🧾️ Consumes the same scalar ledger while its original recipient remains physically retained.
pub struct NativeForwardedEncodeReceipt{receipt:NativeEncodeContinuation,recipient:usize}
impl NativeForwardedEncodeReceipt{
 /// 🔗️ Reborrows the original retained port without borrowing a progress observer.
 pub fn bind<'a>(self,allocate:&'a mut NativeEncodeAllocationPort<'a>,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<NativeForwardedEncodeContinuation<'a>,(ValueError,Self)>{
  if self.recipient!=std::ptr::from_ref(&*recipient)as usize||self.receipt.owned_bytes>self.receipt.maximum_bytes||(self.receipt.total!=0&&self.receipt.completed>self.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan mismatches original recipient or ledger"),self))}
  Ok(NativeForwardedEncodeContinuation{receipt:Some(self.receipt),allocate,retirement:Some(recipient)})
 }

 /// 📊️ Reports already admitted ownership without copying its authority.
 pub fn owned_bytes(&self)->usize{self.receipt.owned_bytes}
 /// 📏️ Reports the unchanged original receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.maximum_bytes}
}

impl<'a> NativeEncodeControl<'a>{
 /// 🏭️ Binds every requested reservation to the caller's original receiving port.
 pub fn new_forwarded(maximum_bytes:usize,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeAllocationPort<'a>)->NativeForwardedEncodeControl<'a>{let mut inner=Self::new(maximum_bytes,callback);inner.allocation=Binding::Forwarded(allocate);NativeForwardedEncodeControl{inner}}
}
impl<'a> NativeForwardedEncodeControl<'a>{
 /// ⏸️ Ends this import loan while retaining the actual recipient identity and consuming ledger.
 pub fn detach(self)->Result<NativeForwardedEncodeReceipt,(ValueError,Self)>{
  let receipt=match self.inner.continuation(){Ok(receipt)=>receipt,Err(error)=>return Err((error,self))};
  let Some(recipient)=self.inner.retirement.as_ref()else{return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving requires its original retained recipient"),self))};
  Ok(NativeForwardedEncodeReceipt{receipt,recipient:std::ptr::from_ref(&**recipient)as usize})
 }
 /// ▶️ Reborrows the original receiving imports and exact retained recipient without resetting admission.
 pub fn rebind(receipt:NativeForwardedEncodeReceipt,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool,allocate:&'a mut NativeEncodeAllocationPort<'a>,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<Self,(ValueError,NativeForwardedEncodeReceipt)>{
  if receipt.recipient!=std::ptr::from_ref(&*recipient)as usize||receipt.receipt.owned_bytes>receipt.receipt.maximum_bytes||(receipt.receipt.total!=0&&receipt.receipt.completed>receipt.receipt.total){return Err((ValueError::literal(ValueRefusalKind::InvariantViolated,"detached native receiving mismatches its original recipient or ledger"),receipt))}
  let mut inner=NativeEncodeControl::resume(receipt.receipt,callback).expect("validated original native receiving receipt");inner.allocation=Binding::Forwarded(allocate);inner.retirement=Some(recipient);Ok(Self{inner})
 }
 /// ⏸️ Moves the original allocation port together with its cumulative accounting.
 pub fn pause(mut self)->Result<NativeForwardedEncodeContinuation<'a>,ValueError>{let Binding::Forwarded(allocate)=std::mem::replace(&mut self.inner.allocation,Binding::Local)else{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"forwarded native continuation lacks original allocation port"))};let retirement=self.inner.retirement.take();let receipt=self.inner.pause()?;Ok(NativeForwardedEncodeContinuation{receipt:Some(receipt),allocate,retirement})}
 /// ▶️ Rebinds only progress while the original allocation port remains unchanged.
 pub fn resume(receipt:NativeForwardedEncodeContinuation<'a>,callback:&'a mut dyn FnMut(NativeEncodeProgress)->bool)->Result<Self,ValueError>{let mut inner=NativeEncodeControl::resume(receipt.receipt.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation is not at a receiving boundary"))?,callback)?;inner.allocation=Binding::Forwarded(receipt.allocate);inner.retirement=receipt.retirement;Ok(Self{inner})}
}
impl<'a> NativeForwardedEncodeContinuation<'a>{
 /// 🫴️ Returns the same cumulative receiving receipt while the owner keeps its original ports.
 pub fn detach(mut self)->Result<NativeForwardedEncodeReceipt,ValueError>{
  let recipient=self.retirement.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan lacks its original retirement recipient"))?;
  let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"native receiving loan lacks its consuming receipt"))?;
  Ok(NativeForwardedEncodeReceipt{receipt,recipient:std::ptr::from_ref(&*recipient)as usize})
 }

 /// 🎟️ Retains the original allocation port before the first physical hop binds its progress observer.
 pub fn new(maximum_bytes:usize,allocate:&'a mut NativeEncodeAllocationPort<'a>)->Self{Self{receipt:Some(NativeEncodeContinuation{receiving:false,maximum_bytes,owned_bytes:0,completed:0,total:0,stage:0}),allocate,retirement:None}}
 /// 🔁️ Runs one synchronous hop under a shorter progress borrow and preserves the original port on refusal.
 pub fn encode<T>(&mut self,callback:&mut dyn FnMut(NativeEncodeProgress)->bool,operation:impl FnOnce(&mut NativeEncodeControl<'_>)->Result<T,ValueError>)->Result<T,ValueError>{let receipt=self.receipt.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"forwarded native continuation is already receiving"))?;let mut control=NativeEncodeControl::resume(receipt,callback)?;control.allocation=Binding::Forwarded(&mut *self.allocate);control.retirement=self.retirement.as_deref_mut();let result=control.scoped_stage(operation);self.receipt=Some(control.continuation()?);result}
 /// 🫴️ Installs the same caller return slot for every subsequent physical hop.
 pub fn install_retirement_recipient(&mut self,recipient:&'a mut NativeEncodeRetirementRecipient)->Result<(),ValueError>{if self.retirement.is_some()||!crate::ErasedSnapshotRetirement::terminal_is_empty(recipient){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"forwarded encoder requires one empty original retirement slot"))}self.retirement=Some(recipient);Ok(())}
 /// 📏️ Returns the same complete receiving ceiling.
 pub fn maximum_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").maximum_bytes}
 /// 📊️ Returns ownership admitted by the original receiving port through every hop.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().expect("forwarded native receipt present outside receiving hop").owned_bytes}
}
impl<'a> std::ops::Deref for NativeForwardedEncodeControl<'a>{type Target=NativeEncodeControl<'a>;fn deref(&self)->&Self::Target{&self.inner}}
impl<'a> std::ops::DerefMut for NativeForwardedEncodeControl<'a>{fn deref_mut(&mut self)->&mut Self::Target{&mut self.inner}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
