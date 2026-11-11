//! 🎛️ One shared driver maps every selected schema action to its exact reply-shaped or control-shaped call.
use super::*;
use super::walk::NativeSchemaMatchSelection as S;
use engine::source::key_frame::{settlement_of,Settle};
use semio_framework_value_control_contract::{OriginalConstructionAcceptStatus,OriginalConstructionCompletion,OriginalWorkActionIdentity,OriginalWorkActionProof,OriginalWorkBorrow};
use semio_framework_value_native_decoding::native_decoding::{NativeReceivingFailure,OriginalEffectReply,OriginalEffectReplyComplete};
use semio_framework_value_native_receiver_protocol::{native_relay,NativeAdvanceSelection};

type Poll=engine::RetainedWorkPoll<RetainedCloneStep,NativeReceivingFailure>;
type Reply<'r,F>=OriginalEffectReply<'r,RetainedCloneStep,F>;
type Complete<'r,F>=OriginalEffectReplyComplete<'r,RetainedCloneStep,F>;
fn idle(work:u128)->Poll{engine::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work})}
fn failed(error:ValueError)->Poll{engine::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work:0})}


fn finish_partial(work:&mut OriginalWorkBorrow<'_>,action:Option<OriginalWorkActionIdentity>,actual:u128,proof:&mut Option<OriginalWorkActionProof>)->Result<(),ValueError>{let mut partial=None;if !work.settle_partial_into(action,&mut partial)?{return Err(ValueError::invariant("schema acknowledgement Work settlement pending"))}let(_,_,_,admitted,refusal)=partial.take().ok_or_else(invalid)?.into_parts();if !admitted||refusal.is_some(){return Err(ValueError::invariant("schema acknowledgement Work was refused"))}if !work.commit(actual,action,proof)?{return Err(ValueError::invariant("schema acknowledgement Work commit pending"))}Ok(())}
impl<I:NativeValueSource+?Sized> NativeRecordMatcher<I>{
 fn settlement(&self)->Result<Settle,ValueError>{settlement_of(self.key_backing.get().ok_or_else(invalid)?).ok_or_else(invalid)}
 fn local_work(&self,selection:S,body:usize)->Result<u64,ValueError>{Ok(match selection{S::Complete|S::KeyPost=>0,S::KeyBirth=>self.key_birth_work_demand()?,S::PendingSlot=>self.pending_slot_work_demand()?,S::CompareLeaf=>self.compare_leaf_work_demand(body)?,S::CaptureFrame=>self.capture_frame_work_demand()?,S::ApplyFrame=>self.apply_frame_work_demand()?,S::PopFrame=>self.pop_frame_work_demand()?,S::FinishPopped=>self.finish_popped_work_demand()?,S::Finish=>2,_=>1})}
 /// 📐️ Quotes only the selected action of the resident matcher.
 pub fn demands(&self,input:RetainedCloneRef<'_,I>,body:usize,_maximum:u64)->Result<RetirementDemand,ValueError>{Ok(match self.selection()?{S::Complete|S::KeyPost|S::CheckpointRecovery=>Default::default(),S::KeyAcknowledgement=>self.key_backing.get().ok_or_else(invalid)?.acknowledgement().acknowledgement_demand()?,S::KeyBirth=>self.key_birth_demand()?,S::KeyStart=>self.key_start_demand()?,S::KeyAction=>self.key_action_quote(input)?.0,S::KeyMetadata(operation)=>{let action=self.key_metadata_demand(operation)?;let post=OriginalConstructionCompletion::demand();RetirementDemand{copy_bytes:action.copy_bytes.checked_add(post.copy_bytes.checked_mul(2).ok_or_else(invalid)?).ok_or_else(invalid)?,capacity_bytes:action.capacity_bytes.checked_add(post.capacity_bytes.checked_mul(2).ok_or_else(invalid)?).ok_or_else(invalid)?,release_bytes:action.release_bytes.checked_add(post.release_bytes.checked_mul(2).ok_or_else(invalid)?).ok_or_else(invalid)?,depth:action.depth.max(post.depth),..action}},S::KeySettlement=>match self.settlement()?{Settle::Outcome=>self.key_action_result_demand()?,Settle::Request=>self.key_clear_request_demand()},S::KeyError=>self.key_error_demand()?,S::KeyBind=>self.key_bind_demand()?,S::PendingSlot=>self.pending_slot_demand()?,S::CompareLeaf=>self.compare_leaf_demand(body)?,S::CaptureFrame=>self.capture_frame_demand()?,S::ApplyFrame=>self.apply_frame_demand()?,S::PopFrame=>self.pop_frame_demand()?,S::FinishPopped=>self.finish_popped_demand()?,S::Finish=>self.finish_match_demand()?})}
 /// ⏱️ Quotes the Work of the selected nested original Source action.
 pub fn nested_work_demand(&self,input:RetainedCloneRef<'_,I>)->Result<u64,ValueError>{match self.selection()?{S::KeyAction=>Ok(self.key_action_quote(input)?.1),S::KeyAcknowledgement|S::CheckpointRecovery|S::KeyMetadata(_)=>Ok(1),_=>Err(invalid())}}
 /// 🧭️ Selects the resident original Source action or one separately funded local action.
 pub fn select_advance(&self,_input:RetainedCloneRef<'_,I>,body:usize,maximum:u64)->Result<NativeAdvanceSelection,ValueError>{let selection=self.selection()?;if matches!(selection,S::KeyAction|S::KeyAcknowledgement|S::CheckpointRecovery|S::KeyMetadata(_)){return Ok(NativeAdvanceSelection::Nested)}let work=self.local_work(selection,body)?;if work>maximum{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema matcher action exceeds its original work ceiling"))}Ok(NativeAdvanceSelection::Local{work})}
 /// 🪢️ Polls only the resident original key Source action or its acknowledgement.
 pub fn advance_nested(&mut self,input:RetainedCloneRef<'_,I>,control:&mut NativeDecodeControl<'_>,witness:engine::OriginalSourceTurnWitness<'_>,body:usize,maximum:u64,grant:RetainedCloneGrant)->Poll{
  match self.selection(){
   Ok(S::KeyAcknowledgement|S::KeyMetadata(_)) if !self.demands(input,body,maximum).is_ok_and(|demand|semio_framework_value_control_contract::retained_clone_grant_admits(grant,demand))=>idle(0),
   Ok(S::KeyAction|S::CheckpointRecovery)=>match self.key_action_advance(input,control,witness,body,maximum,grant){Err(error)=>failed(error),Ok(NativeSchemaKeyPoll::Denied)=>idle(0),Ok(NativeSchemaKeyPoll::Pending{work})=>engine::RetainedWorkPoll::Pending{work},Ok(NativeSchemaKeyPoll::Ready{work,progress})=>engine::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work})},
   Ok(S::KeyAcknowledgement)=>self.advance_acknowledgement(control),
   Ok(S::KeyMetadata(operation))=>{let before=control.actual_work();let advanced=self.advance_metadata(control,operation);let work=control.actual_work().saturating_sub(before);match advanced{Ok(progress)=>engine::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(progress)),work}),Err(error)=>engine::RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error.into()),work})}},
   Ok(_)=>failed(invalid()),
   Err(error)=>failed(error),
  }
 }
 /// 🧾️ Settles the resident acknowledgement within one call so a finalizing step never reports zero progress and zero work; work already reported by an earlier pending poll stays charged there.
 fn advance_acknowledgement(&mut self,control:&mut NativeDecodeControl<'_>)->Poll{
  use semio_framework_value_native_decoding::native_decoding::NativeReceivingEffectState as State;
  let mut total=0u128;
  loop{
   match self.key_advance_acknowledgement(control){
    Err(error)=>return failed(error),
    Ok(State::Pending{work:0})=>return engine::RetainedWorkPoll::Pending{work:total},
    Ok(State::Pending{work})=>total=total.saturating_add(work),
    Ok(State::Ready{work})=>return idle(total.saturating_add(work)),
   }
  }
 }
 /// 📬️ Records exactly one selected local action into the borrowed original reply.
 pub fn advance_local<'r>(&mut self,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut engine::RetainedAllocationEffect<'_>,reply:Reply<'r,OriginalReceivingPrimary>)->Result<Complete<'r,OriginalReceivingPrimary>,ValueError>{
  let selection=self.selection()?;let reply=reply.progress();
  match selection{
   S::Complete=>Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default())))),
   S::KeyPost=>Ok(reply.pending()),
   S::KeyAction|S::KeyAcknowledgement|S::CheckpointRecovery|S::KeyMetadata(_)=>Err(invalid()),
   S::KeyError=>self.advance_key_error(effect,reply),
   _=>native_relay(reply,|reply|self.advance_selected(selection,input,body,effect,reply)),
  }
 }
 fn advance_selected<'r>(&mut self,selection:S,input:RetainedCloneRef<'_,I>,body:usize,effect:&mut engine::RetainedAllocationEffect<'_>,reply:Reply<'r,ValueError>)->Result<Complete<'r,ValueError>,ValueError>{
  match selection{
   S::KeyBirth=>self.key_birth_into(effect,reply),
   S::KeyStart=>self.key_start_into(input,effect,reply),
   S::KeySettlement=>match self.settlement()?{Settle::Outcome=>self.key_take_action_result_into(effect,reply),Settle::Request=>self.key_clear_request_into(effect,reply)},
   S::KeyBind=>self.key_bind_into(effect,reply),
   S::PendingSlot=>self.pending_slot_into(effect,reply),
   S::CompareLeaf=>self.compare_leaf_into(body,effect,reply),
   S::CaptureFrame=>self.capture_frame_into(effect,reply),
   S::ApplyFrame=>self.apply_frame_into(effect,reply),
   S::PopFrame=>self.pop_frame_into(effect,reply),
   S::FinishPopped=>self.finish_popped_into(effect,reply),
   S::Finish=>self.finish_match_into(effect,reply),
   _=>Err(invalid()),
  }
 }
 fn advance_key_error<'r>(&mut self,effect:&mut engine::RetainedAllocationEffect<'_>,reply:Reply<'r,OriginalReceivingPrimary>)->Result<Complete<'r,OriginalReceivingPrimary>,ValueError>{
  let mut error=None;let mut slot=None;drop(self.key_error_into(&mut error,effect,OriginalEffectReply::borrow(&mut slot)?)?);
  Ok(match(error,slot.take()){(Some(error),Some(Ok(step)))=>reply.complete(Err(error.with_retained_progress(step.progress()).into())),(Some(error),_)=>reply.complete(Err(error.into())),(None,Some(Ok(step)))=>reply.complete(Ok(RetainedCloneStep::Progress(step.progress()))),(None,Some(Err(error)))=>reply.complete(Err(error.into())),(None,None)=>reply.pending()})
 }
 /// 🧾️ Performs one acknowledgement metadata action and authenticates its Source Post under the actual Work and construction authorities.
 pub fn advance_metadata(&mut self,control:&mut NativeDecodeControl<'_>,operation:NativeSchemaKeyMetadata)->Result<RetainedCloneProgress,ValueError>{
  let demand=self.key_metadata_demand(operation)?;
  control.with_original_work_and_construction(|work,physical|{
   let mut total=RetainedCloneProgress::default();
   let(mut original,mut original_work,mut receipt,mut claim,mut original_proof,mut marker)=(None,None,None,None,None,None);
   physical.begin(demand,&mut original,work)?;physical.quote(original.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(original.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;
   if work.prepare(1,&mut original_work)?!=Some(true){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema acknowledgement action lacks original Work"))}
   let progress={let mut scope=work.performed_scope(original_work,&mut receipt,&mut claim)?;let mut effect=scope.construction_effect(physical,original.as_mut().ok_or_else(invalid)?,1)?;let binding=semio_framework_value_native_decoding::NativeReceivingConstructionBinding::from_effect(&effect)?;let mut result=None;drop(self.key_metadata_into(operation,binding,&mut effect,OriginalEffectReply::borrow(&mut result)?)?);result.take().ok_or_else(invalid)??.progress()};
   let _=grant;
   let actual=claim.take().ok_or_else(invalid)??;finish_partial(work,original_work,actual,&mut original_proof)?;
   total=total.checked_add(progress)?;physical.post(original.as_mut().ok_or_else(invalid)?,progress,None)?;if physical.recover_accept(original.as_mut().ok_or_else(invalid)?)?!=Some(OriginalConstructionAcceptStatus::Accepted){return Err(ValueError::invariant("schema acknowledgement action was not accepted"))}
   work.finish_retained(&mut original_proof,&mut original_work)?;
   let(mut completion,mut payout,mut action,mut receipt,mut claim,mut proof)=(None,None,None,None,None,None);
   physical.begin(OriginalConstructionCompletion::demand(),&mut payout,work)?;physical.quote(payout.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(payout.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;if work.prepare(1,&mut action)?!=Some(true){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema acknowledgement Post lacks original Work"))}
   let progress={let witness=physical.witness(payout.as_ref().ok_or_else(invalid)?)?;let mut scope=work.performed_scope(action,&mut receipt,&mut claim)?;let mut effect=scope.effect(grant,1,&mut marker)?;physical.stage_completion(&mut original,&mut completion,&witness,&mut effect).result?.ok_or_else(invalid)?};
   let actual=claim.take().ok_or_else(invalid)??;finish_partial(work,action,actual,&mut proof)?;
   total=total.checked_add(progress)?;physical.post(payout.as_mut().ok_or_else(invalid)?,progress,None)?;if physical.recover_accept(payout.as_mut().ok_or_else(invalid)?)?!=Some(OriginalConstructionAcceptStatus::Accepted){return Err(ValueError::invariant("schema acknowledgement Post was not accepted"))}
   if !physical.publish_completion(completion.as_mut().ok_or_else(invalid)?,payout.as_mut().ok_or_else(invalid)?,work,proof.as_ref().ok_or_else(invalid)?)?{return Err(ValueError::invariant("schema acknowledgement Post was not published"))}
   let accepted=semio_framework_value_native_decoding::OriginalConstructionAcceptedProof::authenticate(physical,completion.as_ref().ok_or_else(invalid)?,work,original_proof.as_ref().ok_or_else(invalid)?)?.ok_or_else(invalid)?;if !self.key_accept_post(&accepted)?{return Err(ValueError::invariant("schema acknowledgement Post was not awaited"))}
   physical.finish_accepted(&mut payout)?;work.finish(&mut proof,&mut action)?;drop(original_proof.take());
   let(mut payout,mut action,mut receipt,mut claim,mut proof)=(None,None,None,None,None);
   physical.begin(completion.as_ref().ok_or_else(invalid)?.release_demand(),&mut payout,work)?;physical.quote(payout.as_mut().ok_or_else(invalid)?)?;let grant=physical.recover_grant(payout.as_mut().ok_or_else(invalid)?)?.ok_or_else(invalid)??;if work.prepare(1,&mut action)?!=Some(true){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"schema acknowledgement release lacks original Work"))}
   let progress={let mut scope=work.performed_scope(action,&mut receipt,&mut claim)?;let mut effect=scope.effect(grant,1,&mut marker)?;physical.retire_completion(&mut completion,payout.as_ref().ok_or_else(invalid)?,&mut effect).result?.ok_or_else(invalid)?};
   let actual=claim.take().ok_or_else(invalid)??;finish_partial(work,action,actual,&mut proof)?;
   total=total.checked_add(progress)?;physical.post(payout.as_mut().ok_or_else(invalid)?,progress,None)?;if physical.recover_accept(payout.as_mut().ok_or_else(invalid)?)?!=Some(OriginalConstructionAcceptStatus::Accepted){return Err(ValueError::invariant("schema acknowledgement release was not accepted"))}
   physical.finish_accepted(&mut payout)?;work.finish(&mut proof,&mut action)?;
   if completion.is_some(){return Err(ValueError::invariant("schema acknowledgement completion remains"))}
   total.copied_items=total.copied_items.min(1);Ok(total)
  })
 }
 /// ⏳️ Reports only a begun original key or acknowledgement action that remains unresolved.
 pub fn advance_pending(&self)->bool{self.key_backing.get().is_some_and(|frame|{let acknowledgement=frame.acknowledgement();frame.pending()||frame.checkpoint_unresolved()||acknowledgement.semantic_pending()||acknowledgement.acknowledgement_pending()||acknowledgement.source_post_pending()})}
}
