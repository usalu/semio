//! 🪙️ Details cursors pay their work through the turn's own original effect and lease the host's shared snapshot.
use super::Ifc2x3Snapshot;
use semio_framework_job::{OriginalTurnFailure,StepContext,receive_original_retained_work};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,ValueError,ValueRefusalKind,retained_clone::OriginalEffectReplyComplete,retirement::arc_custody::ArcLease};

/// 🔗️ The host's shared snapshot leases its own payload as the original clone source.
pub(super)type SnapshotLease=ArcLease<Ifc2x3Snapshot>;
/// 🪙️ Runs one admission step inside the turn's own original effect and settles its receipt and performed work once; `None` means the quote was not admitted and nothing ran.
pub(super)fn funded_turn(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneStep,ValueError>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{
 let grant=cx.retained_grant();
 let Some((turn,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(None)};
 let overrun=turn.work!=performed;
 let step=receive_original_retained_work(cx,turn)?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC2x3 admission work differs from its original effect").into());}
 Ok(Some(step))
}
/// 🧾️ Runs one quoted step inside the turn's own original effect and charges the turn exactly the work it performed; `None` means the quote was not admitted.
pub(super)fn effected<R>(cx:&mut StepContext<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Result<Option<R>,OriginalTurnFailure>{
 let grant=cx.retained_grant();
 let Some((output,performed))=cx.with_original_effect(grant,|_,effect|run(effect))? else{return Ok(None)};
 cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 Ok(Some(output))
}
/// ⏳️ A suspended close keeps its state and retries next turn as zero progress; only a settled failure is a refusal.
pub(super)fn settled(poll:OriginalRetirementPoll)->Result<RetainedCloneStep,ValueError>{
 match poll{RetainedWorkPoll::Pending{..}=>Ok(RetainedCloneStep::Progress(Default::default())),RetainedWorkPoll::Complete(turn)=>turn.result.map_err(OriginalRetirementFailure::into_value_error)}
}
/// 🫴️ Runs one quoted effect-shaped step inside one original effect; the turn is charged exactly the work it performed.
pub(super)fn quoted(cx:&mut StepContext<'_>,d:semio_framework_value::RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<bool,OriginalTurnFailure>{
 if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d){return Ok(false)}
 let grant=cx.retained_grant();let Some((result,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(false)};
 let progress=match &result{Ok(step)=>step.progress(),Err(error)=>error.retained_progress()};
 cx.consume_retained(progress)?;cx.consume_fuel(u64::try_from(performed).unwrap_or(u64::MAX));
 result?;Ok(true)
}
/// 🧾️ Folds a typed native receiving failure into its cause with the receipt it retained.
pub(super)fn receiving_fault<E:semio_framework_pack_json::native_decoding::NativeReceivingEffectError>(error:E)->ValueError{let progress=error.receiving_progress();error.cause().cloned().unwrap_or_else(||ValueError::invariant("native receiving failed without a typed cause")).with_retained_progress(progress)}
/// 🫴️ One native receiving turn of a polled cursor under the context's receiving scope.
pub(super)fn receive<E:semio_framework_pack_json::native_decoding::NativeReceivingEffectError>(cx:&mut StepContext<'_>,d:semio_framework_value::RetirementDemand,run:impl FnOnce(&mut semio_framework_value::NativeDecodeControl<'_>,u64,semio_framework_value::RetainedCloneGrant)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{
 if cx.should_yield()||!semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),d){return Ok(())}
 let maximum=cx.fuel_remaining();let mut out=None;let mut polled=None;
 cx.with_native_receiving(&mut out,|native,grant|{polled=Some(run(native,maximum,grant));Ok(())})?;
 match polled{Some(RetainedWorkPoll::Complete(turn))=>{semio_framework_job::receive_original_receiving_work(cx,RetainedWorkOutcome{result:turn.result.map_err(receiving_fault),work:turn.work})?;Ok(())},Some(RetainedWorkPoll::Pending{work})=>{cx.consume_fuel(u64::try_from(work).unwrap_or(u64::MAX));Ok(())},None=>Ok(())}
}
/// ♻️ Pays the receiver close start once, then closes it through its reply-shaped retirement cursor.
pub(super)fn receiver_close<P:semio_framework_pack_json::native_decoding::owned::NativeReceive<semio_framework_value::DslValue>>(receiver:&mut semio_framework_value_native_receiving_original::NativeReceivingOriginal<semio_framework_value::DslValue,P>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 use semio_framework_value::retained_clone::OriginalEffectReply;
 use semio_framework_value::retirement::{RetirementCursor,RetirementStep};
 let mut slot=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut slot).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop)).map(|()|None)}else{let mut stepped=None;OriginalEffectReply::borrow(&mut stepped).and_then(|reply|RetirementCursor::close_into(receiver,body,effect,reply).map(drop)).map(|()|stepped.take())};
 let work=effect.performed_work();
 match(ran,slot.take()){
  (Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(_),Some(settled))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled,work}),
  (Ok(Some(settled)),None)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(ValueError::invariant("native receiver close produced a foreign child step").into())}),work}),
  (Ok(None),None)=>RetainedWorkPoll::Pending{work},
 }
}
/// 🪢️ Runs one reply-shaped callee and returns its recorded result; `None` means the callee suspended.
pub(super)fn replied<E>(effect:&mut RetainedAllocationEffect<'_>,action:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,semio_framework_value::retained_clone::OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->Result<Option<Result<RetainedCloneStep,E>>,ValueError>{
 let mut slot=None;let reply=semio_framework_value::retained_clone::OriginalEffectReply::borrow(&mut slot)?;action(&mut *effect,reply)?;Ok(slot.take())
}
