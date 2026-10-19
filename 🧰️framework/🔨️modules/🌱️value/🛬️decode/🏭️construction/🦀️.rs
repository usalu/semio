//! 🏭️ Retained native construction requires original physical grants and retains born output before receipt acceptance.
use super::NativeDecodeControl;
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::shared::sealed::{SealedShared,SealedSharedBirth,SharedIssuer}};
use std::mem::size_of;
#[path="🧾️result/🦀️.rs"]
mod result;
pub use result::{NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};
/// 🫴️ The owning native operation supplies grants and accepts actual receipts through this mandatory borrowed port.
pub trait NativeDecodeConstructionRecipient:Send{
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>;
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;}}
pub(super) fn run<T,E:From<ValueError>>(original:&mut NativeDecodeControl<'_>,recipient:&mut dyn NativeDecodeConstructionRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 if original.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction cannot substitute its original receiving authority")));}
 let control=NativeDecodeControl{ledger:original.ledger,scope_maximum:original.scope_maximum,construction:Some(recipient),receiving:original.receiving,maximum_bytes:original.maximum_bytes,owned_bytes:original.owned_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut *original.callback,retirement:original.retirement.as_deref_mut(),allocation:match &mut original.allocation{super::allocation::Binding::Local=>super::allocation::Binding::Local,super::allocation::Binding::Forwarded(port)=>super::allocation::Binding::Forwarded(&mut **port)}};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,owned:&mut original.owned_bytes,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth};operation(&mut scope.control)
}
impl NativeDecodeControl<'_>{
 /// 🛬️ One original physical recipient funds and records a bounded typed construction transition.
 pub fn receive_construction_step(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->Result<crate::retained_clone::RetainedCloneStep,ValueError>)->Result<crate::retained_clone::RetainedCloneStep,NativeConstructionStepFailure<ValueError>>{
  self.receive_construction_step_typed(demand,|control,grant|{let result=operation(control,grant);let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};NativeConstructionStepResult{result,progress}}).into_step_result()
 }
 /// 🧬️ Typed output or primary failure remains resident even when its physical recipient refuses the same effects.
 pub fn receive_construction_step_typed<T,E:From<ValueError>>(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->NativeConstructionStepResult<T,E>)->NativeConstructionStepOutcome<T,E>{
  let admission=self.checkpoint().and_then(|()|self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction step requires its original recipient"))?.grant(demand));
  let grant=match admission{Ok(grant)=>grant,Err(error)=>return NativeConstructionStepOutcome{result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default()}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return NativeConstructionStepOutcome{result:None,receipt_refusal:None,progress:Default::default()}}
  let NativeConstructionStepResult{result,progress}=operation(self,grant);
  let receipt_refusal=self.construction.as_deref_mut().expect("original scoped construction recipient").accept(grant,progress).err().map(|error|error.with_retained_progress(progress));
  NativeConstructionStepOutcome{result:Some(result),receipt_refusal,progress}
 }
 /// 🧾️ Records already-born bounded construction effects before reporting original cancellation or allocation receipt refusal.
 pub fn record_constructed_allocation(&mut self,bytes:usize)->Result<(),ValueError>{
  if self.construction.is_none(){return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"constructed allocation requires its original receiving recipient"));}
  let before=self.owned_bytes;let next=before.checked_add(bytes).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"constructed allocation extent overflow"))?;
  self.owned_bytes=next;
  if let super::allocation::Binding::Forwarded(port)=&mut self.allocation{port(super::NativeDecodeAllocation{bytes,owned_bytes:before,next_owned_bytes:next,maximum_bytes:self.maximum_bytes})?;}
  if next>self.maximum_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"constructed allocation exceeds its original decoder ceiling"))}
  if !self.started||bytes>65536{self.checkpoint()?;}Ok(())
 }
 /// 🫴️ Lends one original construction recipient through this receiving hop without changing its cumulative decoder receipt.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut dyn NativeDecodeConstructionRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{run(self,recipient,operation)}
 /// 🧷️ Admits sealed field ownership only under this hop's explicitly supplied original construction recipient.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>)->Result<RetainedCloneProgress,ValueError>{
  if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction requires one original input and an empty output slot"));}
  self.checkpoint()?;let demand=NativeOwnedDecodeControl::shared_birth_demand::<T>();let grant=self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native sealed field requires its original construction recipient"))?.grant(demand)?;
  let progress=receive_granted(self,input,output,issuer,grant)?;self.construction.as_deref_mut().expect("original scoped construction recipient").accept(grant,progress).map_err(|error|error.with_retained_progress(progress))?;Ok(progress)
 }
}
fn receive_granted<T:Send+Sync+'static>(original:&mut NativeDecodeControl<'_>,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{
 let demand=NativeOwnedDecodeControl::shared_birth_demand::<T>();
 if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original native construction grant does not admit all physical axes"));}
 original.charge(demand.capacity_bytes)?;let birth=SealedShared::<T>::prepare_birth(grant)?;let(value,mut progress)=birth.materialize(input.take().expect("admitted original native payload"),issuer);*output=Some(value);progress.copied_bytes=demand.copy_bytes;
 if !progress.fits(grant){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction exceeded its original physical grant").with_retained_progress(progress));}Ok(progress)
}
/// 🧷️ Construction borrows the original decoder and recipient without deriving retained authority from cumulative totals.
pub struct NativeOwnedDecodeControl<'operation,'decode>{original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut dyn NativeDecodeConstructionRecipient}
impl<'operation,'decode> NativeOwnedDecodeControl<'operation,'decode>{
 /// 🔗️ The required recipient belongs to the same owning operation that lends the original decode control.
 pub fn new(original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut dyn NativeDecodeConstructionRecipient)->Self{Self{original,recipient}}
 /// 📏️ Original payload and resulting source header movement accompany the exact sealed backing birth.
 pub fn shared_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<Option<T>>()+size_of::<Option<SealedShared<T>>>()+size_of::<SealedSharedBirth<T>>(),capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()}}
 /// 📦️ The original slot survives preflight refusal; born output remains in its recipient slot on receipt rejection.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>)->Result<RetainedCloneProgress,ValueError>{
  if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction requires one original input and an empty output slot"));}
  self.original.checkpoint()?;let demand=Self::shared_birth_demand::<T>();let grant=self.recipient.grant(demand)?;
  let progress=receive_granted(self.original,input,output,issuer,grant)?;
  self.recipient.accept(grant,progress).map_err(|error|error.with_retained_progress(progress))?;Ok(progress)
 }
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
