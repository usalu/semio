//! 🎟️ Close cursors borrow the same original clock, work and independently remaining retained currencies.
use crate::{InteractiveJobCloseStep,OriginalTurnFailure,RetainedCloneProgress,StepContext};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,RetainedWorkOutcome,ValueError,ValueRefusalKind};

/// 🪙️ One quoted original cursor action records its actual physical effects and independent CPU once.
pub fn close_original_retained_step<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 close_original_retained_turn(cx,demand,|grant|{let turn=advance(grant);RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work}})
}
/// ⛽️ Admits an original selected close work quote before allowing the cursor to move. A quote larger than this turn's remaining fuel waits for a turn whose fuel is untouched and is then admitted whole (the overdraft is charged); a demand larger than the operation's whole remaining grant is a typed `WorkLimit` refusal (a narrowed turn that cannot fit yields and retries wider), never zero-work Pending forever.
pub fn close_original_quoted_retained_step<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,work:u64,advance:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,E>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 let remaining=cx.retained_grant();if cx.retirement_should_yield(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 if remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.original_total_retained_grant(),demand){return Err(OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::WorkLimit,"original close demand exceeds the whole grant of its turn and can never fit")))}
  return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})
 }
 if cx.fuel_remaining()<work&&!cx.fuel_turn_untouched(){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let turn=advance(RetainedCloneGrant{maximum_items:1,..remaining});let excess=turn.work>u128::from(work);
 let result=fold_original_close(cx,RetainedWorkOutcome{result:turn.result.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}).map_err(Into::into),work:turn.work});
 if !excess{return result}let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original close exceeded its admitted work quote");
 Err(match result{Ok(step)=>OriginalTurnFailure{effects:step.progress(),primary:refusal().into(),receipt_refusal:None,context_refusal:None,work_refusal:None},Err(mut failure)=>{failure.work_refusal=Some(refusal());failure}})
}
fn context_refusal(cx:&StepContext<'_>)->Option<ValueError>{cx.original_source_parent().and_then(|parent|parent.validate_borrowed_context(cx)).err()}
fn receive_original_work<T:Copy,E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<T,E>,effects:impl FnOnce(&T)->RetainedCloneProgress)->Result<T,OriginalTurnFailure>{
 let result:Result<T,crate::OriginalTurnPrimary>=turn.result.map_err(Into::into);let progress=match &result{Ok(value)=>effects(value),Err(primary)=>primary.retained_progress()};let receipt_refusal=cx.consume_retained(progress).err();cx.consume_work(turn.work);let context_refusal=context_refusal(cx);
 match result{
  Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary,receipt_refusal,context_refusal,work_refusal:None}),
  Ok(value)=>match(receipt_refusal,context_refusal){(None,None)=>Ok(value),(Some(primary),context_refusal)=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal,work_refusal:None}),(None,Some(primary))=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal:None,work_refusal:None})}
 }
}
/// 📊️ See `README.md`.
pub fn receive_original_retained_work<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<RetainedCloneStep,E>)->Result<RetainedCloneStep,OriginalTurnFailure>{receive_original_work(cx,turn,|step|step.progress())}
/// 🧾️ Posts a native-receiving turn against its actual issued grant, retaining every refusal and CPU witness.
pub fn receive_original_receiving_work<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<RetainedCloneStep,E>)->Result<RetainedCloneStep,OriginalTurnFailure>{let result:Result<_,crate::OriginalTurnPrimary>=turn.result.map_err(Into::into);let progress=match &result{Ok(step)=>step.progress(),Err(primary)=>primary.retained_progress()};let receipt_refusal=match cx.receiving_issued_grant(){Some(grant)=>cx.consume_receiving_retained(grant,progress).err(),None=>cx.consume_retained(progress).err()};cx.consume_work(turn.work);let context_refusal=context_refusal(cx);match result{Err(primary)=>Err(OriginalTurnFailure{effects:progress,primary,receipt_refusal,context_refusal,work_refusal:None}),Ok(step)=>match(receipt_refusal,context_refusal){(None,None)=>Ok(step),(Some(primary),context_refusal)=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal,work_refusal:None}),(None,Some(primary))=>Err(OriginalTurnFailure{effects:progress,primary:primary.into(),receipt_refusal:None,context_refusal:None,work_refusal:None})}}}
fn fold_original_close(cx:&mut StepContext<'_>,turn:RetainedWorkOutcome<InteractiveJobCloseStep,crate::OriginalTurnPrimary>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{receive_original_work(cx,turn,|step|step.progress())}

/// 🧳️ See `README.md`.
fn close_original_admission(cx:&mut StepContext<'_>,demand:RetirementDemand)->Result<Option<RetainedCloneGrant>,OriginalTurnFailure>{
 cx.original_source_parent()?.validate_borrowed_context(cx)?;
 let remaining=cx.retained_grant();if cx.retirement_should_yield()||remaining.maximum_items==0{return Ok(None)}
 if remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{
  if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.original_total_retained_grant(),demand){return Err(OriginalTurnFailure::from(ValueError::literal(ValueRefusalKind::WorkLimit,"original close demand exceeds the whole grant of its turn and can never fit")))}
  return Ok(None)
 }
 Ok(Some(RetainedCloneGrant{maximum_items:1,..remaining}))
}
pub fn close_original_retained_turn(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneStep,crate::OriginalTurnPrimary>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 let Some(grant)=close_original_admission(cx,demand)?else{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})};
 let turn=advance(grant);fold_original_close(cx,RetainedWorkOutcome{result:turn.result.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}),work:turn.work})
}
/// 🪙️ An effect-shaped close runs inside the context's own performed-work effect and settles the converted poll once.
pub fn close_original_effect_step(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkPoll<RetainedCloneStep,semio_framework_value_control_contract::OriginalRetirementFailure>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 use semio_framework_value_control_contract::RetainedWorkPoll;
 let Some(grant)=close_original_admission(cx,demand)?else{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})};
 let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|advance(effect))?else{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})};
 let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
 let overrun=performed>turn.work;
 let result=fold_original_close(cx,RetainedWorkOutcome{result:turn.result.map(|step|InteractiveJobCloseStep::Pending{progress:step.progress()}).map_err(crate::OriginalTurnPrimary::from),work:turn.work});
 if !overrun{return result}let refusal=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect close performed more work than it settled");
 Err(match result{Ok(step)=>OriginalTurnFailure{effects:step.progress(),primary:refusal().into(),receipt_refusal:None,context_refusal:None,work_refusal:None},Err(mut failure)=>{failure.work_refusal=Some(refusal());failure}})
}

/// 🎫️ A grant-shaped close returns its full actual CPU witness, including denial and failure.
pub fn close_original_granted_step<E:Into<crate::OriginalTurnPrimary>>(cx:&mut StepContext<'_>,close:impl FnOnce(RetainedCloneGrant)->RetainedWorkOutcome<InteractiveJobCloseStep,E>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
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

