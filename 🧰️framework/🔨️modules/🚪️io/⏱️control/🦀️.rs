//! ⏱️ Only the active original native direction observes a physical IO phase.
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, RetainedWorkOutcome, retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
#[path="🧾️failure/🦀️.rs"]
mod failure;
pub use failure::{NativeSnapshotFailure,NativeSnapshotFailureParts,NativeSnapshotCause,NativeSnapshotAllocationFailure,NativeSnapshotWork,native_snapshot_turn};
#[path="📦️octets/🦀️.rs"]
mod octets;
#[path="🧺️backing/🦀️.rs"]
mod backing;
#[path="🛫️snapshot/🦀️.rs"]mod snapshot;
pub use snapshot::{NativeSnapshotEncodeOwner,NativeSnapshotEncodeSettlement};
/// 🎫️ Holds an original physical quote and one reserved native action until the caller posts its effect.
#[must_use]
pub struct NativeSnapshotDecodeEffectQuote<'a,'control>{grant:RetainedCloneGrant,body:&'a mut NativeSnapshotBodyWallet,native:&'a mut NativeDecodeControl<'control>}
/// 🛫️ Holds one encoder physical quote and reserved native action.
#[must_use]
pub struct NativeSnapshotEncodeEffectQuote<'a,'control>{grant:RetainedCloneGrant,body:&'a mut NativeSnapshotBodyWallet,native:&'a mut NativeEncodeControl<'control>}
impl<'a,'control> NativeSnapshotDecodeEffectQuote<'a,'control>{
 /// 🛬️ Quotes the original decoder construction recipient and reserves one action before mutation.
 pub fn decode(body:&'a mut NativeSnapshotBodyWallet,native:&'a mut NativeDecodeControl<'control>,demand:RetainedCloneGrant)->Result<Self,NativeSnapshotFailure>{
  native.checkpoint()?;let grant=body.admit_constructed(native,demand)?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot effect requires original decoder work").into())}Ok(Self{grant,body,native})
 }
 /// 🧾️ Posts the actual effect after mutation, retaining the operation, work and physical refusals.
 pub fn post_decode(self,progress:RetainedCloneProgress,operation:Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let body=self.body;let native=self.native;let work=native.commit_work(1);let local=body.record_progress(progress);let receipt=if progress==RetainedCloneProgress::default(){Ok(())}else{native.submit_constructed_progress(self.grant,progress)};effect_outcome(operation,work,local,receipt,body.progress)
 }
 pub fn run_native(mut self,progress:&mut NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut NativeDecodeControl<'_>,&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if progress.performed||progress.physical!=RetainedCloneProgress::default()||progress.attempted_work.is_some()||progress.settlement.is_some()||unwind_failure.is_some(){let primary=ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot effect requires empty retained marker and failure slots");let result=match self.cancel_decode(){Ok(())=>NativeSnapshotFailure::from(primary),Err(work)=>NativeSnapshotFailure::primary_with_work(primary,work)};return RetainedWorkOutcome{work:0,result:Err(result)}}
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||effect(&mut *self.native,&mut progress.performed,&mut progress.physical)));let performed=progress.performed||progress.physical!=RetainedCloneProgress::default();
  match result{
   Ok(operation)=>if performed{self.post_decode(progress.physical,operation)}else{let canceled=self.cancel_decode();let result=match(operation,canceled){(Ok(()),Ok(()))=>Ok(()),(Err(primary),Ok(()))=>Err(primary.into()),(Ok(()),Err(work))=>Err(NativeSnapshotFailure::work_post(work)),(Err(primary),Err(work))=>Err(NativeSnapshotFailure::primary_with_work(primary,work))};RetainedWorkOutcome{work:0,result}},
   Err(payload)=>{*unwind_failure=if performed{self.post_decode(progress.physical,Ok(())).result.err()}else{self.cancel_decode().err().map(NativeSnapshotFailure::work_post)};std::panic::resume_unwind(payload)}
  }
 }
 /// ↩️ Cancels an unused decoder action without posting a physical effect.
 pub fn cancel_decode(self)->Result<(),ValueError>{self.native.cancel_work()}

}
impl<'a,'control> NativeSnapshotEncodeEffectQuote<'a,'control>{
 /// 🛫️ Quotes the original encoder construction recipient and reserves one action before mutation.
 pub fn encode(body:&'a mut NativeSnapshotBodyWallet,native:&'a mut NativeEncodeControl<'control>,demand:RetainedCloneGrant)->Result<Self,NativeSnapshotFailure>{
  native.checkpoint()?;let grant=body.admit_encoded(native,demand)?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot effect requires original encoder work").into())}Ok(Self{grant,body,native})
 }
 /// 📤️ Posts the actual encoder effect to the same original work and construction recipients.
 pub fn post_encode(self,progress:RetainedCloneProgress,operation:Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let body=self.body;let native=self.native;let work=native.commit_work(1);let local=body.record_progress(progress);let receipt=if progress==RetainedCloneProgress::default(){Ok(())}else{native.submit_constructed_progress(self.grant,progress)};effect_outcome(operation,work,local,receipt,body.progress)
 }
 /// 🧾️ Runs one quoted effect with the same original native control and retained unwind slots.
 pub fn run_native(mut self,progress:&mut NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut NativeEncodeControl<'_>,&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if progress.performed||progress.physical!=RetainedCloneProgress::default()||progress.attempted_work.is_some()||unwind_failure.is_some(){let primary=ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot effect requires empty retained marker and failure slots");let result=match self.cancel_encode(){Ok(())=>NativeSnapshotFailure::from(primary),Err(work)=>NativeSnapshotFailure::primary_with_work(primary,work)};return RetainedWorkOutcome{work:0,result:Err(result)}}
  let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||effect(&mut *self.native,&mut progress.performed,&mut progress.physical)));let performed=progress.performed||progress.physical!=RetainedCloneProgress::default();
  match result{
   Ok(operation)=>if performed{self.post_encode(progress.physical,operation)}else{let canceled=self.cancel_encode();let result=match(operation,canceled){(Ok(()),Ok(()))=>Ok(()),(Err(primary),Ok(()))=>Err(primary.into()),(Ok(()),Err(work))=>Err(NativeSnapshotFailure::work_post(work)),(Err(primary),Err(work))=>Err(NativeSnapshotFailure::primary_with_work(primary,work))};RetainedWorkOutcome{work:0,result}},
   Err(payload)=>{*unwind_failure=if performed{self.post_encode(progress.physical,Ok(())).result.err()}else{self.cancel_encode().err().map(NativeSnapshotFailure::work_post)};std::panic::resume_unwind(payload)}
  }
 }
 /// ↩️ Cancels an unused encoder action without posting a physical effect.
 pub fn cancel_encode(self)->Result<(),ValueError>{self.native.cancel_work()}
}
/// 🎟️ Holds one original decoder work reservation for a caller-retained effect.
#[must_use]
pub struct NativeSnapshotDecodeWorkQuote<'a,'control>{native:&'a mut NativeDecodeControl<'control>}
/// 🛫️ Holds one original encoder work reservation for a caller-retained effect.
#[must_use]
pub struct NativeSnapshotEncodeWorkQuote<'a,'control>{native:&'a mut NativeEncodeControl<'control>}
macro_rules! snapshot_work_quote{
 ($quote:ident,$native:ident,$message:literal)=>{
  impl<'a,'control> $quote<'a,'control>{
   /// 🚦️ Reserves one original native action before its selected effect.
   pub fn reserve(native:&'a mut $native<'control>)->Result<Self,NativeSnapshotFailure>{native.checkpoint()?;if !native.prepare_work(1)?{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,$message).into())}Ok(Self{native})}
   /// 🧾️ Runs one borrowed effect and keeps a refused work post in the caller's unwind slot.
   pub fn run(self,performed:&mut bool,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut bool)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
    if *performed||unwind_failure.is_some(){let primary=ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot work effect requires empty retained marker and failure slots");let failure=match self.native.cancel_work(){Ok(())=>NativeSnapshotFailure::from(primary),Err(work)=>NativeSnapshotFailure::primary_with_work(primary,work)};return RetainedWorkOutcome{work:0,result:Err(failure)}}
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||effect(performed)));
    match result{
     Ok(operation)=>{let post=if *performed{self.native.commit_work(1)}else{self.native.cancel_work()};let work=u64::from(*performed);let result=match(operation,post){(Ok(()),Ok(()))=>Ok(()),(Err(primary),Ok(()))=>Err(primary.into()),(Ok(()),Err(post))=>Err(NativeSnapshotFailure::work_post(post)),(Err(primary),Err(post))=>Err(NativeSnapshotFailure::primary_with_work(primary,post))};RetainedWorkOutcome{work,result}},
     Err(payload)=>{let post=if *performed{self.native.commit_work(1)}else{self.native.cancel_work()};*unwind_failure=post.err().map(NativeSnapshotFailure::work_post);std::panic::resume_unwind(payload)}
    }
   }
  }
 };
}
snapshot_work_quote!(NativeSnapshotDecodeWorkQuote,NativeDecodeControl,"snapshot decoder effect requires original work authority");
snapshot_work_quote!(NativeSnapshotEncodeWorkQuote,NativeEncodeControl,"snapshot encoder effect requires original work authority");
/// 🛫️ Retains original encoder allocation markers and independent refused outcomes.
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct NativeSnapshotEncodeAllocation { pub expected:Option<usize>, pub actual:Option<usize>, pub unwind:Option<semio_framework_value::native_encoding::NativeEncodeAllocationProgress>, pub outcome:Option<semio_framework_value::native_encoding::NativeEncodeAllocationOutcome>, pub effect_unwind:Option<semio_framework_value::native_encoding::NativeEncodeEffectFailure>, pub pending:Option<NativeSnapshotAllocationFailure>, pub collision:Option<NativeSnapshotAllocationFailure> }
impl NativeSnapshotEncodeAllocation{
 /// 🔭️ Borrows the canonical outcome status without moving an original allocation refusal.
 pub fn allocation_status(&self)->semio_framework_value::native_encoding::NativeEncodeAllocationOutcomeStatus{semio_framework_value::native_encoding::native_encode_allocation_outcome_status(&self.outcome)}
 /// 🧾️ Reports every retained original allocation or lower effect refusal.
 pub fn is_unsettled(&self)->bool{self.expected.is_some()||self.actual.is_some()||self.unwind.is_some()||self.outcome.is_some()||self.effect_unwind.is_some()||self.pending.is_some()||self.collision.is_some()}
 /// 📥️ Retains a refused allocation outcome without replacing an occupied original owner.
 pub fn capture(&mut self,outcome:semio_framework_value::native_encoding::NativeEncodeAllocationOutcome,expected:usize)->Result<bool,NativeSnapshotAllocationFailure>{let failed=!outcome.admitted||outcome.primary.is_some()||outcome.forwarded_quote.is_some()||outcome.scoped_quote.is_some()||outcome.extent_refusal.is_some()||outcome.forwarded_refusal.is_some()||outcome.scoped_refusal.is_some()||outcome.actual!=Some(expected)||self.actual!=outcome.actual;let outcome=NativeSnapshotAllocationFailure::Encode(outcome);if self.pending.is_some()||self.unwind.is_some()||self.outcome.is_some()||self.effect_unwind.is_some()||self.collision.is_some(){return Err(outcome)}if failed{self.expected=Some(expected);self.pending=Some(outcome);Ok(false)}else{self.expected=None;self.actual=None;Ok(true)}}
 /// 🧾️ Combines a completed native effect with its independent original allocation outcome.
 pub fn finish(&mut self,result:Result<(),NativeSnapshotFailure>)->Result<(),NativeSnapshotFailure>{match(result,self.pending.take()){(result,None)=>result,(Ok(()),Some(allocation))=>Err(NativeSnapshotFailure::from_allocation(allocation)),(Err(error),Some(allocation))=>match error.with_allocation(allocation){Ok(error)=>Err(error),Err((error,allocation))=>{self.pending=Some(allocation);Err(error)}}}}
}
/// 🛬️ Retains original decoder allocation markers and independent refused outcomes.
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct NativeSnapshotDecodeAllocation { pub expected:Option<usize>, pub actual:Option<usize>, pub unwind:Option<semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationProgress>, pub pending:Option<NativeSnapshotAllocationFailure>, pub collision:Option<NativeSnapshotAllocationFailure> }
impl NativeSnapshotDecodeAllocation{
 /// 🧾️ Reports every retained original allocation refusal.
 pub fn is_unsettled(&self)->bool{self.expected.is_some()||self.actual.is_some()||self.unwind.is_some()||self.pending.is_some()||self.collision.is_some()}
 /// 📥️ Retains a refused allocation outcome without replacing an occupied original owner.
 pub fn capture(&mut self,outcome:semio_framework_value_native_owned::native_decoding::NativeDecodeAllocationOutcome,expected:usize)->Result<bool,NativeSnapshotAllocationFailure>{let failed=!outcome.admitted||outcome.primary.is_some()||outcome.quote_refusal.is_some()||outcome.extent_refusal.is_some()||outcome.settlement_refusal.is_some()||outcome.actual!=Some(expected)||self.actual!=outcome.actual;let outcome=NativeSnapshotAllocationFailure::Decode(outcome);if self.pending.is_some()||self.unwind.is_some()||self.collision.is_some(){return Err(outcome)}if failed{self.expected=Some(expected);self.pending=Some(outcome);Ok(false)}else{self.expected=None;self.actual=None;Ok(true)}}
 /// 🧾️ Combines a completed native effect with its independent original allocation outcome.
 pub fn finish(&mut self,result:Result<(),NativeSnapshotFailure>)->Result<(),NativeSnapshotFailure>{match(result,self.pending.take()){(result,None)=>result,(Ok(()),Some(allocation))=>Err(NativeSnapshotFailure::from_allocation(allocation)),(Err(error),Some(allocation))=>match error.with_allocation(allocation){Ok(error)=>Err(error),Err((error,allocation))=>{self.pending=Some(allocation);Err(error)}}}}
}
/// 🧭️ Keeps allocation custody in the same original native direction as its owner.
#[derive(semio_framework_value::RetireOwned)]
pub enum NativeSnapshotAllocation { Encode(NativeSnapshotEncodeAllocation), Decode(NativeSnapshotDecodeAllocation) }
impl NativeSnapshotAllocation{
 /// 🧾️ Reports whether any original allocation receipt still requires custody.
 pub fn is_unsettled(&self)->bool{match self{Self::Encode(value)=>value.is_unsettled(),Self::Decode(value)=>value.is_unsettled()}}
 /// 🛫️ Lends only an actual original encoder allocation frame.
 pub fn encode_mut(&mut self)->Result<&mut NativeSnapshotEncodeAllocation,ValueError>{match self{Self::Encode(value)=>Ok(value),Self::Decode(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot allocation direction is not encode"))}}
 /// 🛬️ Lends only an actual original decoder allocation frame.
 pub fn decode_mut(&mut self)->Result<&mut NativeSnapshotDecodeAllocation,ValueError>{match self{Self::Decode(value)=>Ok(value),Self::Encode(_)=>Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot allocation direction is not decode"))}}
}
/// 🧭️ A caller-owned marker retains the actual selected action and physical prefix.
#[derive(Default,semio_framework_value::RetireOwned)]
pub struct NativeSnapshotEffectProgress{pub performed:bool,pub physical:RetainedCloneProgress,pub attempted_work:Option<u64>,pub settlement:Option<semio_framework_value::native_encoding::NativeEncodeEffectSettlement>}
impl NativeSnapshotEffectProgress{
 /// 🔭️ Borrows the canonical original encoder settlement status without moving its causes.
 pub fn encode_settlement_status(&self)->semio_framework_value::native_encoding::NativeEncodeEffectSettlementStatus{semio_framework_value::native_encoding::native_encode_effect_settlement_status(&self.settlement)}
 /// 🧹️ Resets a completed marker only after its original settlement leaves the caller slot.
 pub fn reset_completed(&mut self)->Result<(),ValueError>{if self.settlement.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"snapshot effect retains its original encoder settlement"))}self.performed=false;self.physical=RetainedCloneProgress::default();self.attempted_work=None;Ok(())}
}
macro_rules! run_snapshot_effect{
 ($quote:ident,$post:ident,$cancel:ident)=>{
  impl $quote<'_,'_>{
   /// 🧾️ Runs one borrowed effect, posting its recorded prefix before returning or resuming unwind.
   pub fn run(self,progress:&mut NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
    if progress.performed||progress.physical!=RetainedCloneProgress::default()||progress.attempted_work.is_some()||unwind_failure.is_some(){let primary=ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot effect requires empty retained marker and failure slots");let result=match self.$cancel(){Ok(())=>NativeSnapshotFailure::from(primary),Err(work)=>NativeSnapshotFailure::primary_with_work(primary,work)};return RetainedWorkOutcome{work:0,result:Err(result)}}
    let result=std::panic::catch_unwind(std::panic::AssertUnwindSafe(||effect(&mut progress.performed,&mut progress.physical)));let performed=progress.performed||progress.physical!=RetainedCloneProgress::default();
    match result{
     Ok(operation)=>if performed{self.$post(progress.physical,operation)}else{let canceled=self.$cancel();let result=match(operation,canceled){(Ok(()),Ok(()))=>Ok(()),(Err(primary),Ok(()))=>Err(primary.into()),(Ok(()),Err(work))=>Err(NativeSnapshotFailure::work_post(work)),(Err(primary),Err(work))=>Err(NativeSnapshotFailure::primary_with_work(primary,work))};RetainedWorkOutcome{work:0,result}},
     Err(payload)=>{*unwind_failure=if performed{self.$post(progress.physical,Ok(())).result.err()}else{self.$cancel().err().map(NativeSnapshotFailure::work_post)};std::panic::resume_unwind(payload)}
    }
   }
  }
 };
}
run_snapshot_effect!(NativeSnapshotDecodeEffectQuote,post_decode,cancel_decode);
run_snapshot_effect!(NativeSnapshotEncodeEffectQuote,post_encode,cancel_encode);
fn effect_outcome(operation:Result<(),ValueError>,work:Result<(),ValueError>,local:Result<(),ValueError>,receipt:Result<(),ValueError>,progress:RetainedCloneProgress)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
 let cause=operation.err().map(NativeSnapshotCause::Value);let work_refusal=work.err();let settlement_refusal=local.err();let receipt_refusal=receipt.err();let result=if cause.is_none()&&work_refusal.is_none()&&settlement_refusal.is_none()&&receipt_refusal.is_none(){Ok(())}else{Err(NativeSnapshotFailure::from_effect_parts(cause,work_refusal,settlement_refusal,receipt_refusal,progress))};RetainedWorkOutcome{result,work:1}
}
/// 📸️ Debits actual producer spans from one admitted receiving frame's original authority.
pub struct NativeSnapshotBodyWallet{grant:RetainedCloneGrant,progress:RetainedCloneProgress}
impl NativeSnapshotBodyWallet{
 /// 🎟️ Binds explicitly supplied original producer authority without deriving it from storage demands.
 pub fn new(grant:RetainedCloneGrant)->Self{Self{grant,progress:Default::default()}}
 /// 🧮️ Returns only currencies unspent by earlier accepted producer spans.
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 /// 🛂️ Checks all independently authored currencies before the producer can create an owned frontier.
 pub fn admit_frontier(&self,demand:RetainedCloneGrant)->Result<(),ValueError>{let remaining=self.remaining_grant();if demand.maximum_items>remaining.maximum_items{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"snapshot producer exceeds remaining work grant"))}if demand.maximum_depth>remaining.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"snapshot producer exceeds original parent depth"))}if demand.maximum_copy_bytes>remaining.maximum_copy_bytes||demand.maximum_capacity_bytes>remaining.maximum_capacity_bytes||demand.maximum_release_bytes>remaining.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer exceeds remaining physical grant"))}Ok(())}
 /// 🔐️ Records actual performed effects before refusing exceeded original authority.
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer receipt exceeds original grant").with_retained_progress(progress))}Ok(())}
 /// 🎟️ Quotes both the local body and the same original decoder recipient before an effect.
 pub fn admit_constructed(&self,native:&mut NativeDecodeControl<'_>,requested:RetainedCloneGrant)->Result<RetainedCloneGrant,NativeSnapshotFailure>{
  self.admit_frontier(requested)?;
  let demand=semio_framework_value::RetirementDemand{copy_bytes:requested.maximum_copy_bytes,capacity_bytes:requested.maximum_capacity_bytes,release_bytes:requested.maximum_release_bytes,depth:requested.maximum_depth};
  let grant=native.construction_grant(demand)?;
  if grant.maximum_items<requested.maximum_items||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot body exceeds original recipient quote").into())}Ok(grant)
 }
 /// 🧾️ Posts actual body effects through the already quoted original decoder recipient.
 pub fn record_constructed_progress(&mut self,native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),NativeSnapshotFailure>{self.finish_constructed(native,grant,progress,None)}
 fn finish_constructed(&mut self,native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,primary:Option<ValueError>)->Result<(),NativeSnapshotFailure>{
  let local=self.record_progress(progress);let submitted=if progress==RetainedCloneProgress::default(){Ok(())}else{native.submit_constructed_progress(grant,progress)};
  let mut failure=primary.map(NativeSnapshotFailure::from);
  if let Err(error)=local{failure=Some(match failure{Some(primary)=>primary.with_context_refusal(error),None=>NativeSnapshotFailure::context(error)});}
  if let Err(error)=submitted{failure=Some(match failure{Some(primary)=>primary.with_receipt_refusal(error),None=>NativeSnapshotFailure::receipt(error)});}
  match failure{Some(failure)=>Err(failure.with_retained_progress(self.progress)),None=>Ok(())}
 }
 /// 🧵️ Retains every copied prefix and submits the exact physical receipt even on refusal.
 pub fn copy_text_receiving(&mut self,native:&mut NativeDecodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),NativeSnapshotFailure>{
  if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot text receiving slot must be empty").into())}
  let grant=self.admit_constructed(native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_depth:depth,..Default::default()})?;
  let result=native.copy_text_into(text,output);let progress=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};
  self.finish_constructed(native,grant,progress,result.err())
 }
 /// 🪑️ Retains actual vector backing and submits its exact birth to the original recipient.
 pub fn allocate_vec_receiving<T>(&mut self,native:&mut NativeDecodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),NativeSnapshotFailure>{
  if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot vector receiving slot must be empty").into())}
  let bytes=count.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer vector extent overflow"))?;
  let grant=self.admit_constructed(native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_depth:depth,..Default::default()})?;
  match native.allocate_vec(count){Ok(value)=>{*output=value;self.finish_constructed(native,grant,RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:output.capacity()*size_of::<T>(),released_bytes:0},None)},Err(error)=>Err(error.into())}
 }
 /// 🛫️ Quotes the original encoder construction recipient and remaining body wallet.
 pub fn admit_encoded(&self,native:&mut NativeEncodeControl<'_>,requested:RetainedCloneGrant)->Result<RetainedCloneGrant,NativeSnapshotFailure>{
  self.admit_frontier(requested)?;
  let demand=semio_framework_value::RetirementDemand{copy_bytes:requested.maximum_copy_bytes,capacity_bytes:requested.maximum_capacity_bytes,release_bytes:requested.maximum_release_bytes,depth:requested.maximum_depth};
  let grant=native.construction_grant(demand)?;
  if grant.maximum_items<requested.maximum_items||!semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoder body exceeds original recipient quote").into())}Ok(grant)
 }
 /// 🧵️ Copies into the original encoded destination and posts its exact retained prefix.
 pub fn copy_text_encoded(&mut self,native:&mut NativeEncodeControl<'_>,text:&str,output:&mut String,marker:&mut NativeSnapshotEffectProgress,unwind:&mut Option<NativeSnapshotFailure>,allocation:&mut NativeSnapshotEncodeAllocation,depth:usize)->Result<(),NativeSnapshotFailure>{
  if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoded text slot must be empty").into())}
  if allocation.is_unsettled(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoded allocation state is unsettled").into())}
  native.scoped_stage(|native|{
   native.begin_stage(text.len())?;if text.is_empty(){return Ok(())}
   let layout=std::alloc::Layout::array::<u8>(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoded text extent overflow"))?;
   let quote=NativeSnapshotEncodeEffectQuote::encode(self,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<String>(),maximum_capacity_bytes:text.len(),maximum_depth:depth,..Default::default()})?;
   let turn=quote.run_native(marker,unwind,&mut |native,performed,physical|{let outcome=native.allocate_effect(text.len(),&mut allocation.actual,&mut allocation.unwind,&mut |actual|{*performed=true;*actual=Some(0);let pointer=unsafe{std::alloc::alloc(layout)};if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot encoded text allocation failed"))}*output=unsafe{String::from_raw_parts(pointer,0,text.len())};*actual=Some(text.len());*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<String>(),retained_capacity_bytes:text.len(),..Default::default()};Ok(())});match allocation.capture(outcome,text.len()){Ok(true)=>Ok(()),Ok(false)=>Ok(()),Err(outcome)=>{allocation.collision=Some(outcome);Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoded allocation custody is occupied"))}}});
   allocation.finish(turn.result).map_err(|error|error.with_retained_progress(self.progress()))?;marker.reset_completed()?;
   let mut position=0;while position<text.len(){let mut end=position.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}let bytes=end-position;
    let quote=NativeSnapshotEncodeEffectQuote::encode(self,native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes,maximum_depth:depth,..Default::default()})?;
    let turn=quote.run(marker,unwind,&mut |performed,physical|{output.push_str(&text[position..end]);*performed=true;*physical=RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()};Ok(())});turn.result?;marker.reset_completed()?;native.advance(bytes)?;position=end;
   }Ok(())
  })
 }
 /// 📊️ Reports accepted actual producer receipts independently of native allocation counters.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🛫️ Loans the original encoder while settling accepted spans into this same wallet.
 pub fn snapshot_encode<'owner,'control>(&'owner mut self,native:&'owner mut NativeEncodeControl<'control>)->NativeSnapshotEncodeOwner<'owner,'control>{NativeSnapshotEncodeOwner::borrowed(native,self.grant,&mut self.progress)}
 /// 🛬️ Loans the original decoder while preserving earlier accepted wallet receipts.
 pub fn snapshot_decode<'owner,'control>(&'owner mut self,native:&'owner mut NativeDecodeControl<'control>)->NativeSnapshotDecodeOwner<'owner,'control>{NativeSnapshotDecodeOwner::borrowed(native,self.grant,&mut self.progress)}
 /// 🧵️ Preserves an original UTF8 destination and settles every copied prefix before propagating refusal.
 pub fn copy_text_into(&mut self,native:&mut NativeDecodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot text receiving slot must be empty"))}self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_release_bytes:0,maximum_depth:depth})?;let result=native.copy_text_into(text,output);let performed=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(performed)?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 📦️ Keeps original intrinsic octet backing and settles accepted prefixes before propagating cancellation.
 pub fn copy_bytes_into(&mut self,native:&mut NativeDecodeControl<'_>,bytes:&[u8],output:&mut Vec<u8>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot octet receiving slot must be empty"))}self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:bytes.len(),maximum_capacity_bytes:bytes.len(),maximum_release_bytes:0,maximum_depth:depth})?;let result=native.copy_bytes_into(bytes,output);let performed=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(performed)?;result.map_err(|error|error.with_retained_progress(performed))}
 /// 🪑️ Admits and moves actual typed vector backing into its original receiving slot.
 pub fn allocate_vec_into<T>(&mut self,native:&mut NativeDecodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot vector receiving slot must be empty"))}let bytes=count.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot producer vector extent overflow"))?;self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:depth})?;*output=native.allocate_vec(count)?;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:output.capacity()*size_of::<T>(),released_bytes:0})}
 /// 🦜️ Keeps an import reconstruction's actual UTF8 prefix under its original encoder custody.
 pub fn copy_encode_text_into(&mut self,native:&mut NativeEncodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{
  if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoding text receiving slot must be empty"))}
  self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:text.len(),maximum_capacity_bytes:text.len(),maximum_release_bytes:0,maximum_depth:depth})?;
  let result:Result<(),ValueError>=native.scoped_stage(|native|{native.begin_stage(text.len())?;native.charge(text.len())?;output.try_reserve_exact(text.len()).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"snapshot encoding text allocation failed"))?;let mut copied=0;while copied<text.len(){let mut end=copied.saturating_add(65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[copied..end]);native.advance(end-copied)?;copied=end;}Ok(())});
  let progress=RetainedCloneProgress{copied_items:usize::from(result.is_ok()||output.capacity()>0),copied_bytes:output.len(),retained_capacity_bytes:output.capacity(),released_bytes:0};self.record_progress(progress)?;result.map_err(|error|error.with_retained_progress(progress))
 }
 /// ⏪️ Admits actual typed reconstruction backing through the original encoder allocation port.
 pub fn allocate_encode_vec_into<T>(&mut self,native:&mut NativeEncodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot encoding vector receiving slot must be empty"))}let bytes=count.checked_mul(size_of::<T>()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot encoding vector extent overflow"))?;self.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Vec<T>>(),maximum_capacity_bytes:bytes,maximum_release_bytes:0,maximum_depth:depth})?;*output=native.allocate_vec(count)?;self.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<T>>(),retained_capacity_bytes:output.capacity()*size_of::<T>(),released_bytes:0})}
}
struct SnapshotBodyScope<'a>{wallet:NativeSnapshotBodyWallet,progress:&'a mut RetainedCloneProgress}
impl Drop for SnapshotBodyScope<'_>{fn drop(&mut self){*self.progress=self.wallet.progress;}}
/// 💻️ Carries the original decoder and independently supplied physical ownership grant.
pub struct NativeSnapshotDecodeOwner<'owner,'control>{native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant,progress:RetainedCloneProgress,wallet:Option<&'owner mut RetainedCloneProgress>,pending:Option<&'owner mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>}
impl<'owner,'control> NativeSnapshotDecodeOwner<'owner,'control>{
 fn with_io<T,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut IoRunControl<'_,'control>)->RetainedWorkOutcome<T,E>)->RetainedWorkOutcome<T,NativeSnapshotFailure>{let mut run=IoRunControl::borrowed_decoding(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut());let turn=operation(&mut run);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}}
 /// 🫴️ Binds authentic caller authority without reconstructing any grant currency.
 pub fn new(native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant)->Self{Self{native,grant,progress:Default::default(),wallet:None,pending:None}}
 fn borrowed(native:&'owner mut NativeDecodeControl<'control>,grant:RetainedCloneGrant,wallet:&'owner mut RetainedCloneProgress)->Self{let progress=*wallet;Self{native,grant,progress,wallet:Some(wallet),pending:None}}
 pub fn remaining_grant(&self)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:self.grant.maximum_items.saturating_sub(self.progress.copied_items),maximum_copy_bytes:self.grant.maximum_copy_bytes.saturating_sub(self.progress.copied_bytes),maximum_capacity_bytes:self.grant.maximum_capacity_bytes.saturating_sub(self.progress.retained_capacity_bytes),maximum_release_bytes:self.grant.maximum_release_bytes.saturating_sub(self.progress.released_bytes),maximum_depth:self.grant.maximum_depth}}
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 pub fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{let admitted=self.progress.fits(self.grant)&&progress.fits(self.remaining_grant());self.progress=self.progress.checked_add(progress).map_err(|error|error.with_retained_progress(progress))?;if !admitted{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"snapshot decoding receipt exceeds original grant").with_retained_progress(progress))}Ok(())}
 /// 🎟️ Quotes the actual original recipient and remaining local wallet before a decoder effect.
 pub fn admit_constructed(&mut self,demand:RetainedCloneGrant)->Result<RetainedCloneGrant,NativeSnapshotFailure>{let wallet=NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress};wallet.admit_constructed(self.native,demand)}
 /// 🧾️ Publishes one actual local effect under the same previously quoted recipient grant.
 pub fn record_constructed_progress(&mut self,grant:RetainedCloneGrant,progress:RetainedCloneProgress)->Result<(),NativeSnapshotFailure>{
  let local=self.record_progress(progress);let submitted=self.native.submit_constructed_progress(grant,progress);
  match(local,submitted){(Ok(()),Ok(()))=>Ok(()),(Err(error),Ok(()))=>Err(NativeSnapshotFailure::context(error)),(Ok(()),Err(error))=>Err(NativeSnapshotFailure::receipt(error)),(Err(primary),Err(receipt))=>Err(NativeSnapshotFailure::context(primary).with_receipt_refusal(receipt))}
 }
 /// 🧾️ Runs one bounded original effect while retaining its progress in the caller's own frame.
 pub fn effect(&mut self,demand:RetainedCloneGrant,progress:&mut NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>,effect:&mut impl FnMut(&mut bool,&mut RetainedCloneProgress)->Result<(),ValueError>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  let mut body=SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};
  match NativeSnapshotDecodeEffectQuote::decode(&mut body.wallet,self.native,demand){Ok(quote)=>quote.run(progress,unwind_failure,effect),Err(error)=>RetainedWorkOutcome{work:0,result:Err(error)}}
 }
 /// 🪢️ Moves one existing owner between caller-retained slots under one original metadata action.
 pub fn move_existing<T>(&mut self,source:&mut Option<T>,output:&mut Option<T>,progress:&mut NativeSnapshotEffectProgress,unwind_failure:&mut Option<NativeSnapshotFailure>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if source.is_none()||output.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot owner transfer requires a source and empty destination").into())}}
  self.effect(RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()},progress,unwind_failure,&mut |performed,physical|{*output=source.take();*performed=true;*physical=RetainedCloneProgress{copied_items:1,..Default::default()};Ok(())})
 }
 /// 🥡️ Places one copy scalar in its caller slot before posting actual original work and physical receipt.
 pub fn receive_copy<T:Copy>(&mut self,output:&mut Option<T>,value:T)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if output.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot scalar output slot must be empty").into())}}
  let mut body=SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};
  let quote=match NativeSnapshotDecodeEffectQuote::decode(&mut body.wallet,self.native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_depth:1,..Default::default()}){Ok(quote)=>quote,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  *output=Some(value);quote.post_decode(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()},Ok(()))
 }
 /// 🪶️ Quotes and publishes one newly constructed absent optional payload.
 pub fn receive_none<T>(&mut self,output:&mut Option<Option<T>>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if output.is_some(){return RetainedWorkOutcome{work:0,result:Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"snapshot optional output slot must be empty").into())}}
  let mut body=SnapshotBodyScope{wallet:NativeSnapshotBodyWallet{grant:self.grant,progress:self.progress},progress:&mut self.progress};
  let quote=match NativeSnapshotDecodeEffectQuote::decode(&mut body.wallet,self.native,RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<Option<T>>(),maximum_depth:1,..Default::default()}){Ok(quote)=>quote,Err(error)=>return RetainedWorkOutcome{work:0,result:Err(error)}};
  *output=Some(None);quote.post_decode(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<T>>(),..Default::default()},Ok(()))
 }
 pub fn grant(&self)->RetainedCloneGrant{self.grant}
 pub fn native(&mut self)->&mut NativeDecodeControl<'control>{self.native}
 /// 🌳️ A scope borrows its actual cumulative wallet and returns every producer work unit and original cause.
 pub fn scoped_native<O,E:Into<NativeSnapshotFailure>>(&mut self,maximum:usize,observer:&mut dyn FnMut(semio_framework_value_native_owned::native_decoding::NativeDecodeProgress)->bool,operation:impl FnOnce(&mut NativeSnapshotDecodeOwner<'_,'_>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{
  let grant=self.grant;let wallet=&mut self.progress;let pending=self.pending.as_deref_mut();let mut work=0;
  let result=self.native.scoped_maximum(maximum,|native|native.scoped_observer(observer,|native|{let mut original=NativeSnapshotDecodeOwner::borrowed(native,grant,wallet);original.pending=pending;let turn=operation(&mut original);work=turn.work;turn.result.map_err(Into::into)}));
  RetainedWorkOutcome{result:result.map_err(|error:NativeSnapshotFailure|error.with_retained_progress(self.progress)),work}
 }
 pub fn scoped_stage<O,E:Into<NativeSnapshotFailure>>(&mut self,operation:impl FnOnce(&mut NativeSnapshotDecodeOwner<'_,'control>)->RetainedWorkOutcome<O,E>)->RetainedWorkOutcome<O,NativeSnapshotFailure>{
  let grant=self.grant;let wallet=&mut self.progress;let pending=self.pending.as_deref_mut();let mut work=0;
  let result=self.native.scoped_stage(|native|{let mut original=NativeSnapshotDecodeOwner::borrowed(native,grant,wallet);original.pending=pending;let turn=operation(&mut original);work=turn.work;turn.result.map_err(Into::into)});
  RetainedWorkOutcome{result:result.map_err(|error:NativeSnapshotFailure|error.with_retained_progress(self.progress)),work}
 }
 pub fn drive_cursor<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,cursor:&mut Option<T>,output:&mut Option<O>,advance:&mut impl FnMut(&mut T,&mut Option<O>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeDecodeControl<'_>,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{snapshot::drive_cursor(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),cursor,output,advance)}
 pub fn receive<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeDecodeControl<'_>,&mut NativeSnapshotBodyWallet)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{snapshot::receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),output,&mut |slot,result,marker,failure,native,body,_pending|operation(slot,result,marker,failure,native,body))}
 /// 🪆️ A nested receiver uses the same body wallet; child native receipts are never reposted.
 pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut NativeSnapshotDecodeOwner<'_,'control>)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  snapshot::receive(self.native,self.grant,&mut self.progress,self.pending.as_deref_mut(),output,&mut |slot,result,marker,failure,native,body,pending|{let mut original=NativeSnapshotDecodeOwner::borrowed(native,body.grant,&mut body.progress);original.pending=Some(pending);operation(slot,result,marker,failure,&mut original)})
 }
}
impl Drop for NativeSnapshotDecodeOwner<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}
/// 🧭️ Names the one active native receiving ledger.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum IoNativeDirection{Decode,Encode}
enum Owner<'a,'b>{Native{decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant},Decode{native:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant},Encode{native:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant}}
/// 🎛️ Borrows original authority; foreign phases bind only their selected native direction.
pub struct IoRunControl<'a,'b>{owner:Owner<'a,'b>,active:IoNativeDirection,progress:RetainedCloneProgress,wallet:Option<&'a mut RetainedCloneProgress>,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>}
impl<'a,'b> IoRunControl<'a,'b>{
 fn borrowed_native(decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>,active:IoNativeDirection)->Self{let progress=*wallet;Self{owner:Owner::Native{decode,encode,grant},active,progress,wallet:Some(wallet),pending}}
 fn borrowed_decoding(native:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>)->Self{let progress=*wallet;Self{owner:Owner::Decode{native,grant},active:IoNativeDirection::Decode,progress,wallet:Some(wallet),pending}}
 fn borrowed_encoding(native:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant,wallet:&'a mut RetainedCloneProgress,pending:Option<&'a mut Option<Box<dyn semio_framework_value::ErasedSnapshotRetirement>>>)->Self{let progress=*wallet;Self{owner:Owner::Encode{native,grant},active:IoNativeDirection::Encode,progress,wallet:Some(wallet),pending}}
 /// 🎞️ Admits a real child frame while retaining its pending ticket and all prior accepted physical spans.
 pub fn receive_nested<T:semio_framework_value::retirement::RetireOwned,O:semio_framework_value::retirement::RetireOwned,E:Into<NativeSnapshotFailure>>(&mut self,output:&mut Option<O>,operation:&mut impl FnMut(&mut Option<T>,&mut Option<O>,&mut NativeSnapshotEffectProgress,&mut Option<NativeSnapshotFailure>,&mut IoRunControl<'_,'b>)->RetainedWorkOutcome<(),E>)->RetainedWorkOutcome<(),NativeSnapshotFailure>{
  if let Owner::Native{decode,encode,grant}=&mut self.owner{return match self.active{
   IoNativeDirection::Decode=>{let mut original=NativeSnapshotDecodeOwner::borrowed(decode,*grant,&mut self.progress);original.pending=self.pending.as_deref_mut();original.receive_nested(output,&mut |slot,result,marker,failure,child|{let mut run=IoRunControl::borrowed_native(child.native,encode,child.grant,&mut child.progress,child.pending.as_deref_mut(),IoNativeDirection::Decode);operation(slot,result,marker,failure,&mut run)})},
   IoNativeDirection::Encode=>{let mut original=NativeSnapshotEncodeOwner::borrowed_pending(encode,*grant,&mut self.progress,self.pending.as_deref_mut());original.receive_nested(output,&mut |slot,result,marker,failure,child|child.with_io_companion(decode,|run|operation(slot,result,marker,failure,run)))},
  }}
  match self.active{
   IoNativeDirection::Decode=>match self.snapshot_decode(){Ok(mut owner)=>owner.receive_nested(output,&mut |slot,result,marker,failure,owner|owner.with_io(|run|operation(slot,result,marker,failure,run))),Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}},
   IoNativeDirection::Encode=>match self.snapshot_encode(){Ok(mut owner)=>owner.receive_nested(output,&mut |slot,result,marker,failure,owner|owner.with_io(|run|operation(slot,result,marker,failure,run))),Err(error)=>RetainedWorkOutcome{result:Err(error.into()),work:0}},
  }
 }
 /// 🎫️ Preserves both original native caller controls across explicit phase transitions.
 pub fn new(decode:&'a mut NativeDecodeControl<'b>,encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Native{decode,encode,grant},active:IoNativeDirection::Decode,progress:Default::default(),wallet:None,pending:None}}
 /// 🆕️ Binds only the current foreign decode phase without creating an inactive encoder.
 pub fn decoding(decode:&'a mut NativeDecodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Decode{native:decode,grant},active:IoNativeDirection::Decode,progress:Default::default(),wallet:None,pending:None}}
 /// 🪢️ Binds only the current foreign encode phase without creating an inactive decoder.
 pub fn encoding(encode:&'a mut NativeEncodeControl<'b>,grant:RetainedCloneGrant)->Self{Self{owner:Owner::Encode{native:encode,grant},active:IoNativeDirection::Encode,progress:Default::default(),wallet:None,pending:None}}
 /// 📒️ Reports the active original ledger for receiving phase completion.
 pub fn direction(&self)->IoNativeDirection{self.active}
 /// ⏳️ Reports accepted original physical spans, including those preceding a refused publication.
 pub fn progress(&self)->RetainedCloneProgress{self.progress}
 /// 🛑️ Observes only the active original lifetime and ownership ledger.
 pub fn checkpoint(&mut self)->Result<(),ValueError>{match (&mut self.owner,self.active){(Owner::Native{decode,..},IoNativeDirection::Decode)|(Owner::Decode{native:decode,..},IoNativeDirection::Decode)=>decode.checkpoint(),(Owner::Native{encode,..},IoNativeDirection::Encode)|(Owner::Encode{native:encode,..},IoNativeDirection::Encode)=>encode.checkpoint(),_=>Err(Self::opposite())}}
 /// 🧲️ Finishes an original native phase before selecting another available caller ledger.
 pub fn select(&mut self,direction:IoNativeDirection)->Result<(),ValueError>{if direction==self.active{return Ok(())}if !matches!(&self.owner,Owner::Native{..})||self.pending.as_deref().is_some_and(Option::is_some){return Err(Self::opposite())}self.checkpoint()?;self.active=direction;self.checkpoint()}
 /// 📬️ Reborrows the original decoder only after its receiving phase is selected.
 pub fn decode(&mut self)->Result<&mut NativeDecodeControl<'b>,ValueError>{self.select(IoNativeDirection::Decode)?;match &mut self.owner{Owner::Native{decode,..}|Owner::Decode{native:decode,..}=>Ok(decode),_=>Err(Self::opposite())}}
 /// 📩️ Reborrows the original encoder only after its receiving phase is selected.
 pub fn encode(&mut self)->Result<&mut NativeEncodeControl<'b>,ValueError>{self.select(IoNativeDirection::Encode)?;match &mut self.owner{Owner::Native{encode,..}|Owner::Encode{native:encode,..}=>Ok(encode),_=>Err(Self::opposite())}}
 /// 📷️ Reborrows the actual decoder beside its independent original ownership grant.
 pub fn snapshot_decode(&mut self)->Result<NativeSnapshotDecodeOwner<'_,'b>,ValueError>{self.select(IoNativeDirection::Decode)?;match &mut self.owner{Owner::Native{decode,grant,..}|Owner::Decode{native:decode,grant}=>{let mut owner=NativeSnapshotDecodeOwner::borrowed(decode,*grant,&mut self.progress);owner.pending=self.pending.as_deref_mut();Ok(owner)},_=>Err(Self::opposite())}}
 /// 🎚️ Reborrows an actual encoder with its original independent snapshot ownership grant.
 pub fn snapshot_encode(&mut self)->Result<NativeSnapshotEncodeOwner<'_,'b>,ValueError>{self.select(IoNativeDirection::Encode)?;match &mut self.owner{Owner::Native{encode,grant,..}|Owner::Encode{native:encode,grant}=>Ok(NativeSnapshotEncodeOwner::borrowed_pending(encode,*grant,&mut self.progress,self.pending.as_deref_mut())),_=>Err(Self::opposite())}}
 /// ✅️ Keeps every original IO currency already consumed by accepted producer spans.
 pub fn remaining_grant(&self)->RetainedCloneGrant{let grant=match &self.owner{Owner::Native{grant,..}|Owner::Decode{grant,..}|Owner::Encode{grant,..}=>*grant};NativeSnapshotBodyWallet{grant,progress:self.progress}.remaining_grant()}
 /// ♻️ Borrows the original decoder for funded cleanup without selecting or checkpointing another phase.
 pub fn with_native_retirement(&mut self,grant:RetainedCloneGrant,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, 'b>)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>)->Result<semio_framework_value::retained_clone::RetainedCloneStep,ValueError>{
  let original=match &self.owner{Owner::Native{grant,..}|Owner::Decode{grant,..}=>*grant,Owner::Encode{..}=>return Err(Self::opposite())};
  let mut wallet=NativeSnapshotBodyWallet{grant:original,progress:self.progress};wallet.admit_frontier(grant)?;
  let result=match &mut self.owner{Owner::Native{decode,..}|Owner::Decode{native:decode,..}=>decode.with_retirement(grant,operation),Owner::Encode{..}=>unreachable!()};
  let performed=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};let settled=wallet.record_progress(performed);self.progress=wallet.progress();settled.map_err(|error|error.with_retained_progress(performed))?;result
 }
 fn opposite()->ValueError{ValueError::literal(ValueRefusalKind::UnsupportedOwner,"IO foreign receiving phase requires completion before the opposite direction begins")}
}
impl Drop for IoRunControl<'_, '_>{fn drop(&mut self){if let Some(wallet)=self.wallet.as_deref_mut(){*wallet=self.progress;}}}
#[cfg(test)]
#[path="🧪️tests/effects.rs"]
mod effect_tests;
#[cfg(test)]
#[path="🧪️tests/nested-io.rs"]
mod nested_io_tests;
#[cfg(test)]
#[path="♻️retirement/🧪️tests/🦀️.rs"]
mod retirement_tests;
