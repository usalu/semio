//! 🏗️ Funded metadata moves, one-unit original construction and retirement steps shared by plugin job owners.
use semio_framework_job::{InteractiveJobCloseStep,OriginalConstructionAdmission,OriginalConstructionDemand,OriginalConstructionFailure,OriginalConstructionStatus,OriginalNativeReceivingOutcome,OriginalTurnFailure,OriginalTurnPrimary,StepContext,admit_original_construction,close_original_effect_step,receive_original_receiving_work};
use semio_framework_value::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,RetirementPollExit,OriginalReceivingPrimary,retirement::{RetireOwned,RetirementCursor,RetirementStep},ValueError,ValueRefusalKind,retained_clone::{OriginalEffectReply,RetainedCloneClose,RetainedCloneSourceAdmission,retained_clone_grant_admits}};
use std::mem::size_of;
use semio_framework_value_native_owned::{NativeReceive,native_decoding::{NativeReceivingFailure,construction::NativeConstructionScope}};
use semio_framework_value_native_receiving_original::NativeReceivingOriginal;
pub fn fixed<T>()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<T>(),depth:1,..Default::default()}}
pub fn construction(d:RetirementDemand)->OriginalConstructionDemand{OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth}}
pub fn receipt(d:RetirementDemand)->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}}
fn actual<O>(result:&Result<(O,RetainedCloneProgress),ValueError>,actual:&mut Option<usize>){if let Ok((_,progress))=result{if progress.retained_capacity_bytes!=0&&actual.is_none(){*actual=Some(progress.retained_capacity_bytes)}}}
/// 🏛️ The caller's resident admission keeps a partly funded construction alive across turns; `Ok(false)` means the next funded turn resumes it.
pub fn built<O>(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce()->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{semio_framework_job::admit_original_construction_settled(cx,admission,output,demand,1,|_,reported|{let result=construct();actual(&result,reported);RetainedWorkOutcome{result,work:1}})}
pub fn built_with<O>(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,output:&mut Option<O>,demand:OriginalConstructionDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<(O,RetainedCloneProgress),ValueError>)->Result<bool,OriginalTurnFailure>{semio_framework_job::admit_original_construction_settled(cx,admission,output,demand,1,|scope,reported|{let result=construct(scope);actual(&result,reported);RetainedWorkOutcome{result,work:1}})}
/// 🧱️ A one-unit state change settles through the resident admission and its resident marker, so a pending turn never repeats the change.
pub fn unit(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission,marker:&mut Option<()>,demand:RetirementDemand,apply:impl FnOnce()->Result<(),ValueError>)->Result<bool,OriginalTurnFailure>{
 let settled=built(cx,admission,marker,construction(demand),||apply().map(|()|((),receipt(demand))))?;if settled{marker.take();}Ok(settled)
}
/// 🪦️ Retires the resident admission's settled control metadata before its owner reaches terminal closure.
pub fn unbuilt(cx:&mut StepContext<'_>,admission:&mut OriginalConstructionAdmission)->Result<bool,OriginalTurnFailure>{if admission.terminal_is_empty(){return Ok(true)}admission.close_step(cx)?;Ok(admission.terminal_is_empty())}
/// 🏁️ Moves a settled admission into its constructed owner so the owner retires it, leaving the caller's slot empty.
pub fn adopted<T>(settled:bool,admission:&mut OriginalConstructionAdmission,output:&mut Option<Box<T>>,resident:impl FnOnce(&mut T)->&mut OriginalConstructionAdmission)->bool{if settled&&!admission.terminal_is_empty(){if let Some(owner)=output.as_mut(){std::mem::swap(resident(&mut **owner),admission)}}settled}
pub fn retired(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{semio_framework_job::close_original_retained_effect_step(cx,demand,|effect|advance(effect).map_err(OriginalRetirementFailure::from))}
pub fn retired_poll(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{semio_framework_job::close_original_effect_step(cx,demand,advance)}
pub fn begun(cx:&mut StepContext<'_>,demand:RetirementDemand,begin:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneStep>>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{retired_poll(cx,demand,|effect|{let turn=begin(effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(OriginalRetirementFailure::from),work:turn.work})})}
pub fn funded<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{
 let grant=cx.retained_grant();if cx.should_yield()||!retained_clone_grant_admits(grant,demand){return Ok(None)}
 let Some((poll,performed))=cx.with_original_effect(grant,|_,effect|run(effect))?else{return Ok(None)};
 let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
 let overrun=performed>turn.work;let step=semio_framework_job::receive_original_retained_work(cx,turn)?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect performed more work than it settled").into())}Ok(Some(step))
}
fn scope_refusal(scope:&OriginalNativeReceivingOutcome)->Result<(),OriginalTurnFailure>{let cause=scope.call.as_ref().err().and_then(|call|call.operation.clone().or_else(||call.context.clone()).or_else(||call.receipt_refusal.clone())).or_else(||scope.source_refusal.clone());match cause{Some(error)=>Err(error.with_retained_progress(scope.retained_progress()).into()),None=>Ok(())}}
/// 🫴️ Runs one native-bound forward step inside the decoder's persistent receiving scope, the one Work scope that outlives a turn, so a cursor keeps the Work binding its birth recorded.
pub fn scoped<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,run:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<Option<RetainedCloneStep>,OriginalTurnFailure>{
 if cx.should_yield()||!retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(None)}
 let units=receiving_units(cx.fuel_remaining());let(mut scope,mut ran)=(None,None);
 if !cx.with_native_receiving_quoted(&mut scope,{let(grant,work)=receiving_quote(demand,units);semio_framework_value::invocation_extent(grant,work)?},|control,_|{ran=control.with_allocated_effect(demand,units,|_,effect|{let poll=run(effect);(poll,effect.performed_work())})?;Ok(())})?{return Ok(None)}
 scope_refusal(&scope.ok_or_else(||ValueError::invariant("original receiving scope vanished"))?)?;
 let Some((poll,performed))=ran else{return Ok(None)};
 let turn=match poll{RetainedWorkPoll::Complete(turn)=>turn,RetainedWorkPoll::Pending{work}=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work}};
 let overrun=performed>turn.work;let step=receive_original_receiving_work(cx,turn)?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect performed more work than it settled").into())}Ok(Some(step))
}
/// ♻️ Retires one native-bound owner inside the decoder's persistent retirement scope, the same Work scope its forward steps ran in.
pub fn retired_native(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll)->Result<(),OriginalTurnFailure>{
 if cx.retirement_should_yield()||!retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(())}
 let units=receiving_units(cx.fuel_remaining());let(mut scope,mut ran)=(None,None);
 cx.with_native_retirement_receiving(&mut scope,{let(grant,work)=StepContext::native_retirement_quote(demand,units);semio_framework_value::invocation_extent(grant,work)?},|native,_|{ran=native.with_original_effect(demand,units,|effect|{let poll=advance(effect);(poll,effect.performed_work())})?;Ok(())})?;
 if let Some(scope)=&scope{scope_refusal(scope)?}
 let Some((poll,performed))=ran else{return Ok(())};
 let turn=match RetirementPollExit::settled(poll){Ok(turn)=>turn,Err(RetirementPollExit::Pending(work))=>RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work},Err(RetirementPollExit::Failed(failure))=>RetainedWorkOutcome{result:Err(failure),work:0}};
 let overrun=performed>turn.work;receive_original_receiving_work(cx,turn)?;
 if overrun{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original effect performed more work than it settled").into())}Ok(())
}
/// 🧭️ The next physical demand of an owner's admission into its own sealed Source: its native identity projection when selected, else its plain step.
pub fn admission_demand<T:RetireOwned+Sync+'static>(admission:&RetainedCloneSourceAdmission<T,T>)->Result<RetirementDemand,ValueError>{if admission.phase()=="projectionStart"{admission.owned_projection_demand()}else{admission.next_demand()}}
/// 🪡️ Runs the native identity projection or else the plain admission step.
pub fn admission_step<T:RetireOwned+Sync+'static>(admission:&mut RetainedCloneSourceAdmission<T,T>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,ValueError>{RetainedWorkPoll::Complete(if admission.phase()=="projectionStart"{admission.project_owned(effect)}else{admission.advance(effect)})}
/// ♻️ The demand of cancelling an admission: clearing its non-owning identity projection first, then the handback of its inputs.
pub fn admission_return_demand<T:RetireOwned+Sync+'static>(admission:&RetainedCloneSourceAdmission<T,T>)->Result<RetirementDemand,ValueError>{if admission.projection_result()==Some(true){Ok(admission.discard_projection_demand())}else{admission.next_return_demand()}}
/// ♻️ Clears the identity projection of a cancelled admission; `None` once only the handback of its inputs remains.
pub fn admission_settle<T:RetireOwned+Sync+'static>(admission:&mut RetainedCloneSourceAdmission<T,T>,effect:&mut RetainedAllocationEffect<'_>)->Result<Option<RetainedCloneStep>,ValueError>{if admission.projection_result()!=Some(true){return Ok(None)}admission.discard_projection(effect).result.map(Some)}
pub fn replied<E
:From<ValueError>>(effect:&mut RetainedAllocationEffect<'_>,run:impl FnOnce(&mut RetainedAllocationEffect<'_>,OriginalEffectReply<'_,RetainedCloneStep,E>)->Result<(),ValueError>)->RetainedWorkPoll<RetainedCloneStep,E>{
 let mut slot=None;let ran=OriginalEffectReply::borrow(&mut slot).and_then(|reply|run(effect,reply));let work=effect.performed_work();
 let result=match(ran,slot.take()){(Err(error),_)=>Err(error.into()),(Ok(()),Some(result))=>result,(Ok(()),None)=>Err(ValueError::invariant("original reply did not settle").into())};
 RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})
}
pub fn receiving(poll:RetainedWorkPoll<RetainedCloneStep,OriginalReceivingPrimary>)->RetainedWorkPoll<RetainedCloneStep,OriginalTurnPrimary>{match poll{RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalTurnPrimary::Receiving),work:turn.work})}}
pub fn receiving_primary(failure:NativeReceivingFailure)->OriginalReceivingPrimary{match failure{NativeReceivingFailure::Primary(primary)=>primary,NativeReceivingFailure::ValueEffect(error)=>error.receiving,NativeReceivingFailure::RetirementEffect(error)=>error.receiving,NativeReceivingFailure::ReceivingEffect(error)=>error.receiving}}
pub fn receiver_close<I:Sync+'static,T:NativeReceive<I>,A:RetireOwned+Sync>(receiver:&mut NativeReceivingOriginal<I,T,A>,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
 let mut slot=None;
 let ran=if !receiver.is_closing(){OriginalEffectReply::borrow(&mut slot).and_then(|reply|receiver.begin_close_into(effect,reply).map(drop)).map(|()|None)}else{let mut stepped=None;OriginalEffectReply::borrow(&mut stepped).and_then(|reply|RetirementCursor::close_into(receiver,body,effect,reply).map(drop)).map(|()|stepped.take())};
 let work=effect.performed_work();
 match(ran,slot.take()){
  (Err(error),_)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work}),
  (Ok(_),Some(settled))=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled,work}),
  (Ok(Some(settled)),None)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:settled.and_then(|step|match step{RetirementStep::Progress(progress,_)=>Ok(RetainedCloneStep::Progress(progress)),RetirementStep::Failure(failure,_)=>Err(failure),RetirementStep::Complete(_)|RetirementStep::Pending(_)|RetirementStep::BudgetExhausted|RetirementStep::Advanced=>Ok(RetainedCloneStep::Progress(Default::default())),_=>Err(ValueError::invariant("original receiver close changed its step").into())}),work}),
  (Ok(None),None)=>RetainedWorkPoll::Pending{work},
 }
}
/// 🪙️ The work one receiving scope is quoted: half of the turn's fuel, the framework's own unquoted scope rule, so the held scope never starves the turn that drives it.
pub fn receiving_units(fuel:u64)->u64{fuel.div_ceil(2)}
/// 🧮️ The construction actions one nested record level acknowledges per step: its original action, its Post and its release.
pub const ACKNOWLEDGEMENT_ACTIONS:usize=3;
/// 🪙️ The scope quote of one receiving step: its exact physical demand as a single action and the work its receiver may spend.
pub fn receiving_quote(demand:RetirementDemand,units:u64)->(semio_framework_value::RetainedCloneGrant,u64){(semio_framework_value::RetainedCloneGrant{maximum_items:ACKNOWLEDGEMENT_ACTIONS*demand.depth.max(1),maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth},units)}
/// 🔀️ The smallest demand that funds both of two alternative steps of one turn.
pub fn widened(a:RetirementDemand,b:RetirementDemand)->RetirementDemand{RetirementDemand{copy_bytes:a.copy_bytes.max(b.copy_bytes),capacity_bytes:a.capacity_bytes.max(b.capacity_bytes),release_bytes:a.release_bytes.max(b.release_bytes),depth:a.depth.max(b.depth)}}
/// 📐️ The wallet every test caller of this crate is born with. Its capacity, release and depth are not the bound under test: the bound is each turn's grant, derived from the owner's own quote by [`step_grant`].
#[cfg(test)]
pub const STOCK:semio_framework_value::RetainedCloneGrant=semio_framework_value::RetainedCloneGrant{maximum_items:1<<30,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<29,maximum_release_bytes:1<<29,maximum_depth:4096};
/// 🎫️ The grant one turn needs for a step of `demand` that settles an `O` through its resident admission (`()` for a state change or a funded effect step): the framework's settled quote plus the origin refresh the turn pays first.
pub fn settled_grant<O>(demand:RetirementDemand)->Result<semio_framework_value::RetainedCloneGrant,ValueError>{semio_framework_job::original_construction_settled_turn_quote::<O>(construction(demand),1).map(|(grant,_)|grant)}
/// 🧮️ The grant of one state-change or funded step of `demand`.
#[cfg(test)]
pub fn step_grant(demand:RetirementDemand)->semio_framework_value::RetainedCloneGrant{settled_grant::<()>(demand).expect("bounded construction quote")}
/// 🧮️ The grant one turn of a `size`-byte owner's boxed settled construction is funded with.
#[cfg(test)]
pub fn admission_grant(size:usize)->semio_framework_value::RetainedCloneGrant{settled_grant::<Box<u8>>(RetirementDemand{copy_bytes:size,capacity_bytes:size,depth:1,..Default::default()}).expect("bounded construction quote")}
/// 🏁️ Closes the decoder in the quoted teardown steps, then the caller's Source root.
#[cfg(test)]
pub fn finished(mut caller:semio_framework_job::OriginalJobFixtureCaller,decoder:&mut semio_framework_job::OriginalJobDecodeOwner){
 for _ in 0..semio_framework_job::ORIGINAL_DECODER_TEARDOWN_STEPS{let(quote,_)=semio_framework_job::original_decoder_teardown_quote(decoder).expect("bounded teardown quote");caller.turn(quote,|cx|{decoder.close_on_source(cx)?;Ok(())}).expect("decoder teardown turn");}
 assert!(decoder.terminal_is_empty(),"decoder teardown exceeded its quoted steps");let source=step_grant(caller.retirement_demands().expect("Source closure quote"));caller.retire(source);
}

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
/// 🎟️ Births one owner into the caller's output slot over the caller's resident admission; `Ok(true)` once born, settled and reset.
pub fn born<O>(cx:&mut StepContext<'_>,output:&mut Option<O>,admission:&mut OriginalConstructionAdmission,failure:&mut Option<OriginalConstructionFailure>,d:RetirementDemand,construct:impl FnOnce(&mut NativeConstructionScope<'_,'_>)->Result<O,ValueError>)->Result<bool,OriginalTurnFailure>{
 if output.is_some()&&!admission.settling(){return if admission.terminal_is_empty(){Ok(true)}else{admission.reset_step(cx)}}
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
/// ⏩️ Runs one funded original effect step forward and settles its poll once.
pub fn forward<E:Into<OriginalTurnPrimary>>(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,E>)->Result<(),OriginalTurnFailure>{close_original_effect_step(cx,demand,advance).map(drop)}
/// ⏩️ Runs one funded receiving effect step whose failures are original receiving primaries.
pub fn forward_receiving(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneStep,semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary>)->Result<(),OriginalTurnFailure>{close_original_effect_step(cx,demand,|effect|match advance(effect){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(OriginalTurnPrimary::Receiving),work:turn.work})}).map(drop)}
/// 🧾️ A reply-shaped close that fails with a retirement failure.
pub type Retirement=RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>;
