//! 🏭️ Retained native construction requires original physical grants and retains born output before receipt acceptance.
use super::NativeDecodeControl;
use crate::{RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress},shared_issuance::{SealedShared,SharedIssuer}};
#[path="🧾️result/🦀️.rs"]
mod result;
pub use result::{NativeConstructionStepResult,NativeConstructionStepOutcome,NativeConstructionStepFailure,OriginalReceivingPrimary};
/// 🫴️ The owning native operation supplies grants and accepts actual receipts through this mandatory borrowed port.
pub trait NativeDecodeConstructionRecipient:Send{
 fn grant(&mut self,demand:RetirementDemand)->Result<RetainedCloneGrant,ValueError>;
 fn accept(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),ValueError>;
}
struct Scope<'owner,'call>{control:NativeDecodeControl<'call>,maximum:&'owner mut usize,scope_maximum:&'owner mut Option<usize>,owned:&'owner mut usize,completed:&'owner mut usize,total:&'owner mut usize,started:&'owner mut bool,stage:&'owner mut u64,depth:&'owner mut usize,work_actual:&'owner mut u64,work_reserved:&'owner mut u64}
impl Drop for Scope<'_, '_>{fn drop(&mut self){*self.maximum=self.control.maximum_bytes;*self.scope_maximum=self.control.scope_maximum;*self.owned=self.control.owned_bytes;*self.completed=self.control.completed;*self.total=self.control.total;*self.started=self.control.started;*self.stage=self.control.stage;*self.depth=self.control.depth;*self.work_actual=self.control.work.actual;*self.work_reserved=self.control.work.reserved;}}
pub(super) fn run<T,E:From<ValueError>>(original:&mut NativeDecodeControl<'_>,recipient:&mut dyn NativeDecodeConstructionRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
 if original.construction.is_some(){return Err(E::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction cannot substitute its original receiving authority")));}
 let control=NativeDecodeControl{work:super::work::Work{actual:original.work.actual,baseline:original.work.baseline,reserved:original.work.reserved,recipient:original.work.recipient.as_mut().map(|p|&mut **p as &mut dyn super::NativeDecodeWorkRecipient)},ledger:original.ledger,scope_maximum:original.scope_maximum,construction:Some(recipient),receiving:original.receiving,maximum_bytes:original.maximum_bytes,owned_bytes:original.owned_bytes,completed:original.completed,total:original.total,started:original.started,stage:original.stage,depth:original.depth,callback:&mut *original.callback,retirement:original.retirement.as_deref_mut(),allocation:match &mut original.allocation{super::allocation::Binding::Local=>super::allocation::Binding::Local,super::allocation::Binding::Forwarded(port)=>super::allocation::Binding::Forwarded(&mut **port)}};
 let mut scope=Scope{control,maximum:&mut original.maximum_bytes,scope_maximum:&mut original.scope_maximum,owned:&mut original.owned_bytes,completed:&mut original.completed,total:&mut original.total,started:&mut original.started,stage:&mut original.stage,depth:&mut original.depth,work_actual:&mut original.work.actual,work_reserved:&mut original.work.reserved};operation(&mut scope.control)
}
impl NativeDecodeControl<'_>{
 /// 🛬️ One original physical recipient funds and records a bounded typed construction transition.
 pub fn receive_construction_step(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->crate::RetainedWorkOutcome)->crate::RetainedWorkOutcome<crate::retained_clone::RetainedCloneStep,NativeConstructionStepFailure<ValueError>>{
  self.receive_construction_step_typed(demand,|control,grant|{let turn=operation(control,grant);let progress=match &turn.result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};NativeConstructionStepResult{result:turn.result,progress,work:turn.work}}).into_step_outcome()
 }
 /// 🧬️ Typed output or primary failure remains resident even when its physical recipient refuses the same effects.
 pub fn receive_construction_step_typed<T,E:From<ValueError>>(&mut self,demand:RetirementDemand,operation:impl FnOnce(&mut Self,RetainedCloneGrant)->NativeConstructionStepResult<T,E>)->NativeConstructionStepOutcome<T,E>{
  let admission=self.checkpoint().and_then(|()|self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native construction step requires its original recipient"))?.grant(demand));
  let grant=match admission{Ok(grant)=>grant,Err(error)=>return NativeConstructionStepOutcome{result:Some(Err(error.into())),receipt_refusal:None,progress:Default::default(),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return NativeConstructionStepOutcome{result:None,receipt_refusal:None,progress:Default::default(),work:0}}
  let NativeConstructionStepResult{result,progress,work}=operation(self,grant);
  let receipt_refusal=self.construction.as_deref_mut().expect("original scoped construction recipient").accept(grant,progress).err().map(|error|error.with_retained_progress(progress));
  NativeConstructionStepOutcome{result:Some(result),receipt_refusal,progress,work}
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
 /// 🏗️ Lends one original construction recipient through this receiving hop without changing its cumulative decoder receipt.
 pub fn with_construction_recipient<T,E:From<ValueError>>(&mut self,recipient:&mut dyn NativeDecodeConstructionRecipient,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{run(self,recipient,operation)}
 /// 🧷️ Admits sealed field ownership only under this hop's explicitly supplied original construction recipient.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  self.receive_allocation(input,output,NativeOwnedDecodeControl::shared_birth_demand::<T>(),|value,grant|SealedShared::admit(value,issuer,grant))
 }
 /// 🖼️ Original receiving allocates an ordinary immutable Arc without replacing its recipient.
 pub fn receive_arc<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<std::sync::Arc<T>>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  self.receive_allocation(input,output,NativeOwnedDecodeControl::arc_birth_demand::<T>(),crate::retirement::arc_custody::admit_shared_allocation)
 }
 fn receive_allocation<T,R>(&mut self,input:&mut Option<T>,output:&mut Option<R>,demand:RetirementDemand,allocate:impl FnOnce(T,RetainedCloneGrant)->crate::RetainedWorkOutcome<(R,RetainedCloneProgress),(ValueError,T)>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let admission=(||{if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction requires one original input and an empty output slot"))}self.checkpoint()?;self.construction.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"native shared field requires its original construction recipient"))?.grant(demand)})();
  let grant=match admission{Ok(grant)=>grant,Err(error)=>return crate::RetainedWorkOutcome{result:Err(OriginalReceivingPrimary{primary:None,quote_refusal:Some(error),allocation_refusal:None,progress:Default::default()}),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,demand){return crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let turn=receive_allocated(self,input,output,demand,grant,allocate);let progress=match &turn.result{Ok(progress)=>*progress,Err(failure)=>failure.progress};let refusal=self.construction.as_deref_mut().unwrap().accept(grant,progress).err();fold_shared_receipt(turn,refusal)
 }

}
fn receive_granted<T:Send+Sync+'static>(original:&mut NativeDecodeControl<'_>,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>,grant:RetainedCloneGrant)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
 receive_allocated(original,input,output,NativeOwnedDecodeControl::shared_birth_demand::<T>(),grant,|value,grant|SealedShared::admit(value,issuer,grant))
}
fn receive_allocated<T,R>(original:&mut NativeDecodeControl<'_>,input:&mut Option<T>,output:&mut Option<R>,demand:RetirementDemand,grant:RetainedCloneGrant,allocate:impl FnOnce(T,RetainedCloneGrant)->crate::RetainedWorkOutcome<(R,RetainedCloneProgress),(ValueError,T)>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
 let bytes=demand.capacity_bytes;if original.owned_bytes.checked_add(bytes).is_none_or(|next|next>original.maximum_bytes){return crate::RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"native decoding ownership exceeds caller limit").into()),work:0}}let turn=allocate(match input.take(){Some(v)=>v,None=>return crate::RetainedWorkOutcome{result:Err(ValueError::invariant("input absent").into()),work:0}},grant);let result=match turn.result{Ok((owner,progress))=>{*output=Some(owner);let before=original.owned_bytes;original.owned_bytes=before.checked_add(progress.retained_capacity_bytes).unwrap();let refusal=if let super::allocation::Binding::Forwarded(port)=&mut original.allocation{port(super::NativeDecodeAllocation{bytes:progress.retained_capacity_bytes,owned_bytes:before,next_owned_bytes:original.owned_bytes,maximum_bytes:original.maximum_bytes}).err()}else{None};match refusal{None=>Ok(progress),Some(error)=>Err(OriginalReceivingPrimary{primary:None,quote_refusal:None,allocation_refusal:Some(error.with_retained_progress(progress)),progress})}},Err((error,value))=>{*input=Some(value);Err(error.into())}};crate::RetainedWorkOutcome{result,work:turn.work}
}
fn fold_shared_receipt(turn:crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>,refusal:Option<ValueError>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let result=match(turn.result,refusal){(result,None)=>result,(Ok(progress),Some(error))=>Err(OriginalReceivingPrimary{primary:None,quote_refusal:Some(error.with_retained_progress(progress)),allocation_refusal:None,progress}),(Err(mut failure),Some(error))=>{failure.quote_refusal=Some(error.with_retained_progress(failure.progress));Err(failure)}};crate::RetainedWorkOutcome{result,work:turn.work}}
/// 🎛️ Construction borrows the original decoder and recipient without deriving retained authority from cumulative totals.
pub struct NativeOwnedDecodeControl<'operation,'decode>{original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut dyn NativeDecodeConstructionRecipient}
impl<'operation,'decode> NativeOwnedDecodeControl<'operation,'decode>{
 /// 🔗️ The required recipient belongs to the same owning operation that lends the original decode control.
 pub fn new(original:&'operation mut NativeDecodeControl<'decode>,recipient:&'operation mut dyn NativeDecodeConstructionRecipient)->Self{Self{original,recipient}}
 /// 📏️ Original payload and resulting source header movement accompany the exact sealed backing birth.
 pub fn shared_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{capacity_bytes:SealedShared::<T>::birth_bytes(),depth:1,..Default::default()}}
 /// 📐️ An ordinary shared backing quotes the actual standard Arc layout.
 pub fn arc_birth_demand<T:Send+Sync+'static>()->RetirementDemand{RetirementDemand{capacity_bytes:crate::retirement::arc_custody::shared_allocation_bytes::<T>(),depth:1,..Default::default()}}
 /// 📦️ The exact original payload or born owner survives every native primary and receipt refusal.
 pub fn receive_shared<T:Send+Sync+'static>(&mut self,input:&mut Option<T>,output:&mut Option<SealedShared<T>>,issuer:SharedIssuer<T>)->crate::RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
  let admission=(||{if input.is_none()||output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native construction requires one original input and an empty output slot"))}self.original.checkpoint()?;self.recipient.grant(Self::shared_birth_demand::<T>())})();let grant=match admission{Ok(grant)=>grant,Err(error)=>return crate::RetainedWorkOutcome{result:Err(OriginalReceivingPrimary{primary:None,quote_refusal:Some(error),allocation_refusal:None,progress:Default::default()}),work:0}};
  if !crate::retained_clone::retained_clone_grant_admits(grant,Self::shared_birth_demand::<T>()){return crate::RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let turn=receive_granted(self.original,input,output,issuer,grant);let progress=match &turn.result{Ok(progress)=>*progress,Err(failure)=>failure.progress};let refusal=self.recipient.accept(grant,progress).err();fold_shared_receipt(turn,refusal)
 }

}

#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod shared_work_tests;

#[path="📦️backing/🦀️.rs"]mod backing;
