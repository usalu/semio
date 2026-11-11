//! 🏗️ Funded metadata moves, original effect steps and retirement steps shared by plugin job owners.
use semio_framework_job::{InteractiveJobCloseStep,OriginalTurnFailure,OriginalTurnPrimary,StepContext,close_original_effect_step};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,ValueError};
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_job::{OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalConstructionStatus,admit_original_construction};
use semio_framework_value_native_owned::native_decoding::construction::NativeConstructionScope;
use semio_framework_value::retirement::RetireOwned;
use semio_framework_value_native_owned::native_decoding::{OriginalEffectReply,OriginalEffectReplyComplete};
use std::mem::size_of;
/// 📏️ A fixed metadata move of one value copies exactly its bytes at depth one.
pub fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
/// 🧾️ Receipt of one fully performed demand.
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
/// 🪙️ True while the turn funds one exact demand and has not yet been asked to yield.
pub fn funds(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.should_yield()&&cx.retained_grant().maximum_items!=0&&cx.retained_work_can_enter(d)}
/// 🪙️ True while a retirement turn funds one exact demand.
pub fn funds_retirement(cx:&StepContext<'_>,d:RetirementDemand)->bool{!cx.retirement_should_yield()&&cx.retained_grant().maximum_items!=0&&cx.retained_work_can_enter(d)}
/// 💳️ Charges the receipt and one work unit of a metadata move that has been funded.
pub fn paid(cx:&mut StepContext<'_>,d:RetirementDemand)->Result<(),OriginalTurnFailure>{cx.consume_retained(receipt(d))?;cx.consume_fuel(1);Ok(())}
/// 🔀️ Moves one owned value when the turn funds it, otherwise leaves it in place.
pub fn moved(cx:&mut StepContext<'_>,d:RetirementDemand,mv:impl FnOnce())->Result<bool,OriginalTurnFailure>{if !funds(cx,d){return Ok(false)}mv();paid(cx,d)?;Ok(true)}
/// 🔀️ Runs one fallible funded metadata move and charges it only when it succeeds.
pub fn acted(cx:&mut StepContext<'_>,d:RetirementDemand,run:impl FnOnce()->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{if !funds(cx,d){return Ok(false)}run()?;paid(cx,d)?;Ok(true)}
/// 📤️ Produces one owned value into an empty destination when the turn funds the move.
pub fn produced<O>(cx:&mut StepContext<'_>,d:RetirementDemand,output:&mut Option<O>,produce:impl FnOnce()->Result<O,ValueError>)->Result<bool,OriginalTurnFailure>{if output.is_some(){return Err(ValueError::invariant("destination is occupied").into())}if !funds(cx,d){return Ok(false)}*output=Some(produce()?);paid(cx,d)?;Ok(true)}
/// 🧮️ Converts a retirement demand into the one-item construction demand of a born owner.
pub fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
/// 🎟️ Births one owner into the caller's output slot over the caller's resident admission; `Ok(true)` once born, settled and reset.
pub fn born<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,d:RetirementDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<O,ValueError>)->Result<bool,OriginalTurnFailure>{
 if output.is_some(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
 if failure.is_some(){return Err(ValueError::invariant("original construction retained a failed birth").into())}
 let status=admit_original_construction(cx,output,admission,construction(d),1,|native,_|{let result=construct(native).map(|owner|(owner,receipt(d)));RetainedWorkOutcome{work:u128::from(result.is_ok()),result}})?;
 match status{OriginalConstructionStatus::Ready=>admission.reset_step(cx),OriginalConstructionStatus::Pending=>Ok(false),OriginalConstructionStatus::Failed=>{if !admission.take_failure_into_step(failure,cx)?{return Ok(false)}Err(ValueError::invariant("original construction failed; its causes remain retained").into())}}
}
/// 🪙️ The exact one-item grant that funds one demand.
pub fn grant_of(d:RetirementDemand)->semio_framework_value::RetainedCloneGrant{semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth.max(1)}}
/// 🎟️ The exact grant of one turn that births one owner of demand `d` over its own output slot and settles it in that turn.
pub fn birth_grant<O>(d:RetirementDemand)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{
 let(complete,_)=semio_framework_job::original_construction_reserve::<O>(construction(d),1)?;let(items,copy,_)=semio_framework_job::original_construction_settle_overhead();
 Ok(semio_framework_value::RetainedCloneGrant{maximum_items:complete.items+items,maximum_copy_bytes:complete.copy_bytes+copy,maximum_capacity_bytes:complete.capacity_bytes,maximum_release_bytes:complete.release_bytes,maximum_depth:complete.depth})
}
/// 🗑️ Drops one terminal empty owner shell when the retirement turn funds it.
pub fn forgotten(cx:&mut StepContext<'_>,d:RetirementDemand,forget:impl FnOnce())->Result<bool,OriginalTurnFailure>{if !funds_retirement(cx,d){return Ok(false)}forget();paid(cx,d)?;Ok(true)}
/// 🪢️ Settles a plain close result against its effect.
pub fn settle(effect:&mut RetainedAllocationEffect<'_>,result:Result<RetainedCloneStep,ValueError>)->OriginalRetirementPoll{RetirementPollExit::settle(effect,result.map_err(Into::into))}
/// 🪆️ Begins or continues the retirement of one owned value.
pub fn begin<T:RetireOwned>(retirement:&mut RetainedCloneClose,value:&mut Option<T>,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 if !retirement.is_empty(){return retirement.step_granted(effect.grant().maximum_copy_bytes,effect)}
 let turn=retirement.begin_granted(value,effect);RetirementPollExit::settle(effect,turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into))
}
/// ♻️ Runs one funded original effect close step.
pub fn retired(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{close_original_effect_step(cx,demand,advance)}
/// ⏩️ Runs one funded original effect step forward and settles its poll once.
pub fn forward<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{close_original_effect_step(cx,demand,advance).map(drop)}
/// ⏩️ Runs one funded receiving effect step whose failures are original receiving primaries.
pub fn forward_receiving(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<(),OriginalTurnFailure>{close_original_effect_step(cx,demand,|effect|match advance(effect){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalTurnPrimary::Receiving),work:turn.work})}).map(drop)}
/// 📬️ Settles one reply-shaped action into a poll.
pub fn replied<E:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,run:impl for<'r> FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'r,RetainedCloneStep,E>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,E>,ValueError>)->RetainedWorkPoll<RetainedCloneStep,E>{
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|run(&mut *effect,reply).map(drop));let work=effect.performed_work();
 match(ran,slot.take()){(Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),(Ok(()),Some(result))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work}),(Ok(()),None)=>RetainedWorkPoll::Pending{work}}
}
/// 🧾️ A reply-shaped close that fails with a retirement failure.
pub type Retirement=RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>;
