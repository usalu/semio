//! 🛬️ Source-sealed native Job decoder and work issuer custody.
use super::{OriginalCallerSourceLoan,OriginalTurnFailure,StepContext};
use semio_framework_job_receiving::native_call_context;
use semio_framework_value_retained_clone::{ErasedSnapshotRetirement,ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeProgress,NativeDecodeRetirementRecipient,NativeDecodeConstructionRecipient,NativeDetachedRetirementDecodeContinuation};
/// 🧳️ One job owns the same decoder continuation and original retirement recipient until terminal closure.
pub struct OriginalJobDecodeOwner{source:std::mem::ManuallyDrop<Option<OriginalCallerSourceLoan>>,work_issuer:std::mem::ManuallyDrop<Option<semio_framework_value_native_owned::native_decoding::NativeDecodeWorkIssuer>>,construction_issuer:semio_framework_value_native_owned::native_decoding::NativeConstructionRecipientIssuer,receipt:Option<NativeDetachedRetirementDecodeContinuation>,recipient:NativeDecodeRetirementRecipient,closing:bool}
impl OriginalJobDecodeOwner{
 /// 🪑️ Creates one inline original recipient without allocating or granting decode capacity.
 pub fn empty()->Self{Self{source:std::mem::ManuallyDrop::new(None),work_issuer:std::mem::ManuallyDrop::new(None),construction_issuer:semio_framework_value_native_owned::native_decoding::NativeConstructionRecipientIssuer::new(),receipt:None,recipient:NativeDecodeRetirementRecipient::new(),closing:false}}
 /// 📏️ Quotes the fixed original work issuer header before its paid birth.
 pub fn work_issuer_birth_copy_bytes()->usize{std::mem::size_of::<semio_framework_value_native_owned::native_decoding::NativeDecodeWorkIssuer>()}
 /// 👁️ Reports the actual retained original source loan.
 pub fn source_loan_admitted(&self)->bool{self.source.is_some()}
 /// 🪪️ Reports the admitted original work issuer.
 pub fn work_issuer_admitted(&self)->bool{self.work_issuer.is_some()}
 /// 🎟️ Admits the actual source loan, then its work issuer, in separate original turns.
 pub fn admit_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
  if self.closing{return Err(ValueError::invariant("closing Job decoder cannot admit a source").into())}
  if self.source.is_none(){cx.original_source_parent()?.admit_loan(&mut self.source,cx)?;return Ok(false)}
  self.source.as_ref().expect("original Job source was observed").validate_context(cx)?;
  if self.work_issuer.is_none(){let bytes=Self::work_issuer_birth_copy_bytes();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||cx.fuel_remaining()==0||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return Ok(false)}*self.work_issuer=Some(semio_framework_value_native_owned::native_decoding::NativeDecodeWorkIssuer::new());let receipt=cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()});cx.consume_fuel(1);receipt?;return Ok(false)}
  Ok(true)
 }
 pub(crate) fn validate_source(&self,cx:&StepContext<'_>,retirement:bool)->Result<(),OriginalTurnFailure>{let source=self.source.as_ref().ok_or_else(||ValueError::invariant("original Job decoder has no source loan"))?;if retirement{source.validate_retirement_context(cx)}else{source.validate_context(cx)}?;if self.work_issuer.is_none(){return Err(ValueError::invariant("original Job decoder has no work issuer").into())}Ok(())}
 /// 🧵️ Reborrows the same consuming decoder, exact recipient and caller construction authority for one original turn.
 pub(crate) fn receive<T,E:From<ValueError>>(&mut self,grant:RetainedCloneGrant,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,construction:&mut dyn NativeDecodeConstructionRecipient,work:&mut dyn semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient,observed:&mut u64,operation:impl FnOnce(&mut NativeDecodeControl<'_>)->Result<T,E>)->Result<T,semio_framework_value_native_owned::native_decoding::NativeDecodeCallFailure<T,E>>{
  if self.closing{return Err(native_call_context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job decoder is closing")))}if grant.maximum_items==0||grant.maximum_depth==0{return Err(native_call_context(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job receiving requires one original caller turn")))}
  let mut continuation=if let Some(receipt)=self.receipt.take(){match receipt.bind(&mut self.recipient){Ok(bound)=>bound,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(native_call_context(error))}}}else{let receipt=NativeDecodeControl::new_retained(callback).pause().map_err(native_call_context)?;receipt.with_retirement_recipient(&mut self.recipient).map_err(|(error,_)|native_call_context(error))?};
  let mut construction_loan=self.construction_issuer.loan(construction);let mut work_loan=self.work_issuer.as_mut().expect("validated original work issuer").loan(work);let attempted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||continuation.decode(callback,|control|{control.admit_turn_capacity(grant.maximum_capacity_bytes).map_err(E::from)?;control.checkpoint().map_err(E::from)?;control.with_work_recipient(&mut work_loan,|control|control.with_construction_recipient(&mut construction_loan,operation))})));let(detached,_)=continuation.detach();let after=detached.actual_work();self.receipt=Some(detached);*observed=after.checked_sub(before).expect("original decoder work is monotonic");match attempted{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}
 }
 /// ♻️ Loans only retirement charging for already born custody under the original grant, deadline and observer.
 pub(crate) fn retire<T,E:From<ValueError>>(&mut self,grant:RetainedCloneGrant,callback:&mut dyn FnMut(NativeDecodeProgress)->bool,deadline:&mut dyn FnMut()->bool,work:&mut dyn semio_framework_value_native_owned::native_decoding::NativeDecodeWorkRecipient,observed:&mut u64,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>)->Result<T,E>)->Result<T,semio_framework_value_native_owned::native_decoding::NativeDecodeCallFailure<T,E>>{
  if !deadline(){return Err(native_call_context(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job retirement deadline reached")))}if grant.maximum_items==0||grant.maximum_depth==0{return Err(native_call_context(ValueError::literal(ValueRefusalKind::WorkLimit,"original Job retirement requires its original caller turn")))}
  let receipt=self.receipt.take().ok_or_else(||native_call_context(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job retirement requires its existing decoder receipt")))?;let mut continuation=match receipt.bind(&mut self.recipient){Ok(bound)=>bound,Err((error,receipt))=>{self.receipt=Some(receipt);return Err(native_call_context(error))}};
  let mut work_loan=self.work_issuer.as_mut().expect("validated original work issuer").loan(work);let attempted=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||continuation.retire(grant,callback,&mut work_loan,operation)));let(detached,_)=continuation.detach();let after=detached.actual_work();self.receipt=Some(detached);*observed=after.checked_sub(before).expect("original decoder work is monotonic");match attempted{Ok(result)=>result,Err(payload)=>std::panic::resume_unwind(payload)}
 }
 /// 🪪️ Borrows only an existing original decoder receipt and its exact retirement recipient.
 pub fn receiving_identity(&self)->Option<(u64,u64)>{self.receipt.as_ref().map(NativeDetachedRetirementDecodeContinuation::identity)}
 /// 📊️ Reads cumulative original receiving allocation without granting another credit.
 pub fn owned_bytes(&self)->usize{self.receipt.as_ref().map_or(0,NativeDetachedRetirementDecodeContinuation::owned_bytes)}
 /// 📏️ Quotes actual recipient closure before separately removing its scalar continuation.
 pub fn retirement_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.receipt.as_ref().is_some_and(|receipt|receipt.has_pending_work()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Job decoder retains unresolved work reservation"))}
  if self.terminal_is_empty(){return Ok(Default::default());}
  if !self.closing{return Ok(RetirementDemand{depth:1,..Default::default()});}
  if !self.recipient.terminal_is_empty(){return self.recipient.next_demand(body);}
  Ok(RetirementDemand{depth:usize::from(self.receipt.is_some()),..Default::default()})
 }
 /// 📪️ Closes original returned construction ownership before separate continuation metadata removal.
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value_retained_clone::OriginalRetirementOutcome{
  if self.terminal_is_empty(){return semio_framework_value_retained_clone::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0};}
  let demand=match self.retirement_demands(grant.maximum_copy_bytes){Ok(demand)=>demand,Err(error)=>return semio_framework_value_retained_clone::RetainedWorkOutcome{result:Err(error.into()),work:0}};
  if !semio_framework_value_retained_clone::retained_clone::retained_clone_grant_admits(grant,demand){return semio_framework_value_retained_clone::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}
  if !self.closing{self.closing=true;return semio_framework_value_retained_clone::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}
  if !self.recipient.terminal_is_empty(){return self.recipient.close_step(grant);}
  self.receipt=None;
  semio_framework_value_retained_clone::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}
 }
 /// 🫙️ Every original decoder recipient has closed and its consuming receipt has been removed.
 pub fn terminal_is_empty(&self)->bool{self.receipt.is_none()&&self.recipient.terminal_is_empty()&&self.work_issuer.is_none()&&self.source.is_none()}
}
impl Drop for OriginalJobDecodeOwner{fn drop(&mut self){assert!(self.terminal_is_empty()||std::thread::panicking(),"original Job decoder requires funded recipient and scalar closure");}}
#[cfg(test)]
#[path="../../🛬️receiving/🧪️tests/🔬️custody/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path="../../🛬️receiving/♻️close/🧪️tests/🦀️.rs"]
mod close_work_tests;

#[cfg(test)]
#[global_allocator]
static RECEIVING_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
