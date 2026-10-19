//! 🎟️ Close cursors borrow the same original clock, work and independently remaining retained currencies.
use crate::{default_now_us,root_cancel_token,CancelToken,Generation,InteractiveJobCloseStep,OperationId,OriginalTurnFailure,RetainedCloneProgress,StepBudget,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};

/// 🪙️ One quoted original cursor action records its actual physical effects and CPU exactly once.
pub fn close_original_retained_step(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 let remaining=cx.retained_grant();if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let grant=RetainedCloneGrant{maximum_items:1,..remaining};cx.consume_fuel(1);let step=advance(grant).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;let progress=step.progress();cx.consume_retained(progress)?;Ok(InteractiveJobCloseStep::Pending{progress})
}

/// 🎫️ One grant-shaped close body runs once over the caller's remaining retained grant; its actual effects or its whole refusal are debited to that same context.
pub fn close_original_granted_step(cx:&mut StepContext<'_>,close:impl FnOnce(RetainedCloneGrant)->Result<InteractiveJobCloseStep,ValueError>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 cx.consume_fuel(1);match close(cx.retained_grant()){Ok(step)=>{cx.consume_retained(step.progress())?;Ok(step)},Err(error)=>Err(OriginalTurnFailure::debit_original(error,cx))}
}

/// 🪆️ A close body that closes nested original jobs over loaned contexts debits only the part of its reported step the nested callers have not already posted.
pub fn close_original_nested_step(cx:&mut StepContext<'_>,close:impl FnOnce(&mut StepContext<'_>,RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let before=cx.retained_progress();let grant=cx.retained_grant();cx.consume_fuel(1);let step=close(cx,grant)?;
 let(after,report)=(cx.retained_progress(),step.progress());let owed=|reported:usize,initial:usize,current:usize|current.checked_sub(initial).and_then(|posted|reported.checked_sub(posted)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original nested close reported fewer effects than its nested callers posted"));
 cx.consume_retained(RetainedCloneProgress{copied_items:owed(report.copied_items,before.copied_items,after.copied_items)?,copied_bytes:owed(report.copied_bytes,before.copied_bytes,after.copied_bytes)?,retained_capacity_bytes:owed(report.retained_capacity_bytes,before.retained_capacity_bytes,after.retained_capacity_bytes)?,released_bytes:owed(report.released_bytes,before.released_bytes,after.released_bytes)?})?;Ok(step)
}

/// 🧪️ A throwaway caller whose one cancellation root serves every close turn it lends; single-owner laws use it where no host caller exists, and allocation oracles build it before they observe.
pub struct SelfFundedCloseCaller{cancel:CancelToken,sequence:u64}
impl SelfFundedCloseCaller{
 pub fn new()->Self{Self{cancel:root_cancel_token(),sequence:0}}
 /// 🎟️ Lends one context whose whole retained authority is `grant`, with unbounded fuel and deadline, and returns the body's outcome whole.
 pub fn turn<T>(&mut self,grant:RetainedCloneGrant,close:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{
  let mut receipt=RetainedCloneProgress::default();
  let mut cx=StepContext::new(OperationId(1),Generation(1),StepBudget::new(u64::MAX,u64::MAX,grant),self.cancel.clone(),default_now_us,&mut self.sequence,&mut receipt);
  close(&mut cx)
 }
}
impl Default for SelfFundedCloseCaller{fn default()->Self{Self::new()}}

/// 🧪️ Drives one close turn over a fresh [`SelfFundedCloseCaller`].
pub fn close_with_self_funded_context<T>(grant:RetainedCloneGrant,close:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{SelfFundedCloseCaller::new().turn(grant,close)}
