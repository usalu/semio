//! ⚠️ Failure-service crossings retain their own exact receipt and preissued result destination.
use super::*;
use semio_framework_actor::ActorFailureCallKind;
use semio_plugin_guest_contract::original_actor_context::ColdJobAction;

fn failure_crossing(kind:ActorFailureCallKind)->Result<(ColdJobAction,OwnedSemioExport),OriginalTurnFailure>{
 let action=match kind{
  ActorFailureCallKind::PublishActorFailure=>ColdJobAction::PublishFailure,
  ActorFailureCallKind::ReadActorFailure=>ColdJobAction::ReadFailure,
  ActorFailureCallKind::BorrowActorFailure=>ColdJobAction::BorrowFailureChunk,
  ActorFailureCallKind::AcknowledgeActorFailureChunk=>ColdJobAction::AcknowledgeFailureChunk,
  ActorFailureCallKind::AcknowledgeActorFailure=>ColdJobAction::AcknowledgeFailure,
  _=>return Err(invalid("failure service selected a semantic export")),
 };
 let export=OwnedSemioExport::from_actor_failure_call_kind(kind).ok_or_else(||invalid("failure call is missing its owned export"))?;
 Ok((action,export))
}

#[cfg(test)]
#[test]
fn failure_crossings_use_the_named_abi_exports(){
 let law:serde_json::Value=serde_json::from_str(include_str!("failure-crossing.json")).unwrap();
 for row in law["crossings"].as_array().unwrap(){
  let kind=ActorFailureCallKind::from_tag(row["kind"].as_u64().unwrap()).unwrap();
  let(action,export)=failure_crossing(kind).unwrap();
  assert_eq!(action.label(),row["action"].as_str().unwrap());
  assert_eq!(export.core_name(),row["coreExport"].as_str().unwrap());
 }
 for tag in law["refused"].as_array().unwrap(){
  let kind=ActorFailureCallKind::from_tag(tag.as_u64().unwrap()).unwrap();
  assert!(failure_crossing(kind).is_err());
 }
}
pub(super) fn failure_fields(cx:&mut StepContext<'_>,parts:&[usize])->Result<bool,OriginalTurnFailure>{let bytes=parts.iter().try_fold(0usize,|sum,value|sum.checked_add(*value)).ok_or_else(||invalid("failure control extent overflow"))?;let p=RetainedCloneProgress{copied_items:parts.len(),copied_bytes:bytes,..Default::default()};if cx.retirement_should_yield()||cx.retained_grant().maximum_depth==0||!p.fits(cx.retained_grant())||cx.fuel_remaining()<parts.len()as u64{return Ok(false)}cx.consume_retained(p)?;cx.consume_fuel(parts.len()as u64);Ok(true)}
impl<T:NativeReceive<DslValue>+RetireOwned+Sync,R:GuestPollFailureReceiving> GuestPoll<T,R>{
 pub(super) fn failure_terminal_is_empty(&self)->bool{self.actor_failure.is_none()&&self.failure_fetch.terminal_is_empty()&&self.failure_plan.is_none()&&self.failure_budget.is_none()&&self.failure_received.is_none()&&self.failure_begin_place.terminal_is_empty()&&self.failure_born.is_none()&&!self.failure_active&&!self.failure_releasing&&self.failure_pointer==0&&self.failure_length==0&&self.failure_recipient_accepted.is_none()&&self.failure_ack_frame.is_none()&&self.failure_ack_lower.is_none()&&self.failure_ack_host.is_none()&&self.failure_acknowledged.is_none()}
 pub(super) fn prepare_failure_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if self.failure_born.is_some(){if self.failure_begin_place.is_admitted(){self.failure_begin_place.settle_return(cx)?;return Ok(false)}if !failure_fields(cx,&[std::mem::size_of::<bool>(),std::mem::size_of_val(&self.failure_born)])?{return Ok(false)}self.scalar_started=self.failure_born.take().unwrap();return Ok(false)}if !self.failure_active||!cx.original_retirement_active(){return Err(invalid("failure service requires its actual retained cleanup context"))}self.failure_fetch.status(cx)?;if self.scalar_progress.is_some(){return Ok(false)}if self.return_place.is_admitted(){return Ok(false)}if let Some(export)=runtime.original_scalar_export(instance,cx)?{if !self.failure_releasing&&self.failure_plan.as_ref().is_some_and(|plan|plan.scoped){let (_,expected)=failure_crossing(self.failure_plan.as_ref().unwrap().kind)?;if export!=expected{return Err(invalid("failure service changed its actual Ready export"))}if !self.frozen{if !failure_fields(cx,&[std::mem::size_of::<bool>()])?{return Ok(false)}self.frozen=true;}}return Ok(true)}if runtime.original_scalar_is_pending(instance)?{if !runtime.prepare_original_retirement_step(instance,cx)?{return Ok(false)}let address=(&mut self.scalar_progress as*mut _)as usize;let bytes=std::mem::size_of_val(&self.scalar_progress);if !self.return_place.admit_sized(cx,address,bytes)?{return Ok(false)}if !self.return_place.capture_is_preissued()&&!self.return_place.preissue_capture(cx,address,bytes)?{return Ok(false)}}Ok(true)}
 pub(super) fn begin_failure_scalar_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !self.failure_active||!cx.original_retirement_active()||self.failure_releasing{return Err(invalid("failure admission requires its retained service frontier"))}if self.failure_born.is_some(){return Ok(false)}if self.scalar_started{return Ok(true)}let plan=self.failure_plan.as_ref().ok_or_else(||invalid("failure crossing has no retained plan"))?;let reference=self.failure_fetch.original_reference().ok_or_else(||invalid("failure crossing lost its opaque producer reference"))?;let actual=reference.reference_for(self.failure_fetch.recipient_permit().ok_or_else(||invalid("failure crossing has no actual recipient"))?.producer(),cx)?;let(action,expected)=failure_crossing(plan.kind)?;let budget=if plan.scoped{self.failure_budget.ok_or_else(||invalid("failure crossing lacks its held issued scope"))?}else{OriginalJobBudget::borrow_original(semio_framework_actor::RetainedTurnInput::borrow_original(cx)?,cx)?};let call=semio_plugin_host_post_turn_relay::ColdJobCallPlan::new(action,0,Some(actual),usize::try_from(plan.prefix[2]).map_err(|_|invalid("failure offset exceeds host extent"))?,usize::try_from(plan.prefix[3]).map_err(|_|invalid("failure page exceeds host extent"))?,budget)?;if call.export!=expected{return Err(invalid("failure scalar plan changed its typed export"))}let address=(&mut self.failure_born as*mut Option<bool>)as usize;let extent=std::mem::size_of_val(&self.failure_born);if !self.failure_begin_place.admit_sized(cx,address,extent)?{return Ok(false)}if !self.failure_begin_place.preissue_capture(cx,address,extent)?{return Ok(false)}let born=runtime.begin_original_scalar_step(instance,call.export,call.arguments(),cx);self.failure_born=Some(born.as_ref().copied().unwrap_or(false));born.map(|_|false)}

 pub(super) fn drive_failure_scalar_step(&mut self,runtime:&GuestRuntimes,instance:&mut GuestInstance,execution:Option<OriginalGuestExecutionBudget>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !self.failure_active||!self.scalar_started||!self.failure_begin_place.terminal_is_empty()||self.failure_born.is_some(){return Err(invalid("failure execution lacks its observed actual admission"))}if self.scalar_progress.is_some()||self.failure_received.is_some(){return Ok(false)}if runtime.original_scalar_export(instance,cx)?.is_some(){if execution.is_some(){return Err(invalid("Ready failure result cannot consume VM credit"))}if !self.failure_releasing&&self.failure_plan.as_ref().is_some_and(|plan|plan.scoped){return Ok(false)}return runtime.take_original_scalar_into_step(instance,&mut self.failure_received,cx)}if !runtime.original_scalar_is_pending(instance)?{return Err(invalid("failure execution lost its actual scalar frontier"))}if !self.return_place.capture_is_preissued(){return Err(invalid("failure execution lacks its preissued actual result destination"))}self.return_place.qualify_sized(cx,(&mut self.scalar_progress as*mut _)as usize,std::mem::size_of_val(&self.scalar_progress))?;let progress=runtime.advance_original_scalar_step(instance,execution,cx)?;self.scalar_progress=Some(progress);Ok(false)}

 pub(super) fn select_failure_plan_step(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !cx.original_retirement_active(){return Err(invalid("failure selection requires its actual cleanup Source"))}let Some(plan)=self.failure_fetch.plan_step(cx)?else{return Ok(false)};if let Some(current)=self.failure_plan.as_ref(){return if *current==plan{Ok(true)}else{Err(invalid("failure plan changed before its actual receipt returned"))}}if !failure_fields(cx,&[std::mem::size_of_val(&self.failure_plan),std::mem::size_of::<bool>()])?{return Ok(false)}self.failure_plan=Some(plan);self.failure_active=true;Ok(true)}
 pub(super) fn receive_failure_budget_step(&mut self,budget:OriginalJobBudget,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(cx)?;if !self.failure_active||!self.failure_plan.as_ref().is_some_and(|plan|plan.scoped)||self.failure_budget.is_some(){return Err(invalid("failure budget requires its unissued actual scoped plan"))}require_core_wire_reserve(budget,184)?;if !failure_fields(cx,&[std::mem::size_of_val(&self.failure_budget)])?{return Ok(false)}self.failure_budget=Some(budget);Ok(true)}

}
