//! 🎟️ Close cursors borrow the same original clock, work and independently remaining retained currencies.
use crate::{InteractiveJobCloseStep,OriginalTurnFailure,RetainedCloneProgress,StepContext};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind};

/// 🪙️ See `README.md`.
pub fn close_original_retained_step(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 close_original_retained_turn(cx,demand,|grant|{let turn=advance(grant);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}})
}
fn context_refusal(cx:&StepContext<'_>)->Option<ValueError>{cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)).err()}
fn receive_original_work<T:Copy,E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<T,E>,effects:impl FnOnce(&T)->RetainedCloneProgress)->Result<T,OriginalTurnFailure>{
 let result:Result<T,crate::OriginalTurnPrimary>=turn.result.map_err(Into::into);let progress=match &result{Ok(value)=>effects(value),Err(primary)=>primary.retained_progress()};let receipt_refusal=cx.consume_retained(progress).err();cx.consume_fuel(turn.work);let context_refusal=context_refusal(cx);
 match result{
  Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary,receipt_refusal,context_refusal}),
  Ok(value)=>match(receipt_refusal,context_refusal){(None,None)=>Ok(value),(Some(primary),context_refusal)=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal}),(None,Some(primary))=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal:None})}
 }
}
/// 📊️ See `README.md`.
pub fn receive_original_retained_work<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<RetainedCloneStep,E>)->Result<RetainedCloneStep,OriginalTurnFailure>{receive_original_work(cx,turn,|step|step.progress())}
fn fold_original_close(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<InteractiveJobCloseStep,crate::OriginalTurnPrimary>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{receive_original_work(cx,turn,|step|step.progress())}

/// 🧳️ See `README.md`.
pub fn close_original_retained_turn(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,crate::OriginalTurnPrimary>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 let remaining=cx.retained_grant();if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let turn=advance(RetainedCloneGrant{maximum_items:1,..remaining});fold_original_close(cx,RetainedWorkOutcome{result:turn.result.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}),work:turn.work})
}

/// 🎫️ See `README.md`.
pub fn close_original_granted_step(cx:&mut StepContext<'_>,close:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<InteractiveJobCloseStep>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let turn=close(cx.retained_grant());fold_original_close(cx,RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})
}

/// 🪆️ See `README.md`.
pub fn close_original_nested_step(cx:&mut StepContext<'_>,close:impl FnOnce(&mut StepContext<'_>,RetainedCloneGrant)->Result<InteractiveJobCloseStep,OriginalTurnFailure>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let before=cx.retained_progress();let grant=cx.retained_grant();let step=close(cx,grant)?;
 let(after,report)=(cx.retained_progress(),step.progress());let owed=|reported:usize,initial:usize,current:usize|current.checked_sub(initial).and_then(|posted|reported.checked_sub(posted)).ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original nested close reported fewer effects than its nested callers posted"));
 let progress=RetainedCloneProgress{copied_items:owed(report.copied_items,before.copied_items,after.copied_items)?,copied_bytes:owed(report.copied_bytes,before.copied_bytes,after.copied_bytes)?,retained_capacity_bytes:owed(report.retained_capacity_bytes,before.retained_capacity_bytes,after.retained_capacity_bytes)?,released_bytes:owed(report.released_bytes,before.released_bytes,after.released_bytes)?};
 fold_original_close(cx,RetainedWorkOutcome{result:Ok(InteractiveJobCloseStep::Pending{progress}),work:0})?;Ok(step)
}


