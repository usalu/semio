//! 🪞️ One resident source admission keeps its staged projection frame and action across turns, cancellation included.
use semio_framework_job::{OriginalTurnFailure,StepContext};
use semio_framework_value::{OriginalRetirementFailure,RetainedAllocationEffect,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::{OriginalEffectAction,OriginalProjectionFrame,OriginalProjectionFramePhase,OriginalSourceProjection,RetainedCloneSource,RetainedCloneSourceAdmission},retirement::RetireOwned};
use semio_plugin_job::original_steps::{funded,retired_poll};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"surface source admission changed its projection frame")}
fn complete<E>(result:Result<RetainedCloneStep,E>,work:u128)->RetainedWorkPoll<RetainedCloneStep,E>{RetainedWorkPoll::Complete(RetainedWorkOutcome{result,work})}
pub(super) struct SourceAdmission<T:Sync+'static,A:OriginalSourceProjection<T>,B:RetireOwned>{admission:Option<RetainedCloneSourceAdmission<T,A,B>>,frame:Option<OriginalProjectionFrame<T,A>>,action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>}
impl<T:Sync+'static,A:OriginalSourceProjection<T>,B:RetireOwned> Default for SourceAdmission<T,A,B>{fn default()->Self{Self{admission:None,frame:None,action:None}}}
impl<T:Sync+'static,A:OriginalSourceProjection<T>,B:RetireOwned> SourceAdmission<T,A,B>{
 pub(super) fn is_none(&self)->bool{self.admission.is_none()}
 pub(super) fn set(&mut self,admission:RetainedCloneSourceAdmission<T,A,B>){self.admission=Some(admission)}
 pub(super) fn terminal_is_empty(&self)->bool{self.admission.is_none()&&self.frame.is_none()&&self.action.is_none()}
 /// 📏️ Quotes the next funded admission or projection stage.
 pub(super) fn quote(&self)->Result<RetirementDemand,ValueError>{
  let admission=self.admission.as_ref().ok_or_else(invalid)?;
  if let Some(stage)=admission.projection_stage(&self.frame)?{return admission.projection_stage_quote(&self.frame,stage).map(|(demand,_)|demand)}
  if admission.projection_result()==Some(false){return Ok(admission.take_projection_failure_demand())}
  admission.next_demand()
 }
 /// 🪡️ Runs one admission or projection stage and publishes the source into `output` once ready.
 pub(super) fn drive(&mut self,cx:&mut StepContext<'_>,output:&mut Option<RetainedCloneSource<T,A>>)->Result<(),OriginalTurnFailure>{
  let demand=self.quote()?;
  funded(cx,demand,|effect|{
   let Some(admission)=self.admission.as_mut()else{return complete(Err(invalid()),0)};
   match admission.projection_stage(&self.frame){Err(error)=>return complete(Err(error),0),Ok(Some(stage))=>return admission.project_staged(&mut self.frame,&mut self.action,stage,effect),Ok(None)=>{}}
   if admission.projection_result()==Some(false){let mut refusal=None;let turn=admission.take_projection_failure(&mut refusal,effect);return complete(match turn.result{Ok(_)=>Err(refusal.unwrap_or_else(invalid)),Err(error)=>Err(error)},turn.work)}
   let turn=if admission.phase()=="ready"{admission.take_output(output,effect)}else{admission.advance(effect)};
   if admission.terminal_is_empty(){self.admission=None}
   RetainedWorkPoll::Complete(turn)
  })?;Ok(())
 }
 /// 📏️ Quotes the cancellation action that precedes the return of the admission's inputs.
 pub(super) fn close_quote(&self)->Result<RetirementDemand,ValueError>{
  let admission=self.admission.as_ref().ok_or_else(invalid)?;
  if let Some(frame)=self.frame.as_ref(){return if frame.phase()?==OriginalProjectionFramePhase::ReturnOwner{admission.return_projection_owner_demand(frame)}else{let stage=admission.projection_stage(&self.frame)?.ok_or_else(invalid)?;admission.projection_stage_quote(&self.frame,stage).map(|(demand,_)|demand)}}
  match admission.projection_result(){Some(true)=>return Ok(admission.discard_projection_demand()),Some(false)=>return Ok(admission.take_projection_failure_demand()),None=>{}}
  if admission.phase()=="ready"{admission.next_demand()}else{admission.next_return_demand()}
 }
 /// ♻️ Settles the open projection, then publishes or returns the admission's inputs into `output` or `inputs`.
 pub(super) fn close(&mut self,cx:&mut StepContext<'_>,output:&mut Option<RetainedCloneSource<T,A>>,inputs:&mut Option<(A,B)>)->Result<bool,OriginalTurnFailure>{
  let demand=self.close_quote()?;
  retired_poll(cx,demand,|effect|self.close_effect(effect,output,inputs))?;Ok(self.terminal_is_empty())
 }
 fn close_effect(&mut self,effect:&mut RetainedAllocationEffect<'_>,output:&mut Option<RetainedCloneSource<T,A>>,inputs:&mut Option<(A,B)>)->RetainedWorkPoll<RetainedCloneStep,OriginalRetirementFailure>{
  let Some(admission)=self.admission.as_mut()else{return complete(Err(invalid().into()),0)};
  if let Some(frame)=self.frame.as_mut(){
   let phase=match frame.phase(){Ok(phase)=>phase,Err(error)=>return complete(Err(error.into()),0)};
   if phase==OriginalProjectionFramePhase::ReturnOwner{let turn=admission.return_projection_owner(frame,effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
   let stage=match admission.projection_stage(&self.frame){Ok(Some(stage))=>stage,Ok(None)=>return complete(Err(invalid().into()),0),Err(error)=>return complete(Err(error.into()),0)};
   return match admission.project_staged(&mut self.frame,&mut self.action,stage,effect){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
  }
  match admission.projection_result(){
   Some(true)=>{let turn=admission.discard_projection(effect);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
   Some(false)=>{let mut refusal=None;let turn=admission.take_projection_failure(&mut refusal,effect);drop(refusal);return RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})}
   None=>{}
  }
  let turn=if admission.phase()=="ready"{admission.take_output(output,effect)}else{admission.return_inputs(inputs,effect)};
  if admission.terminal_is_empty(){self.admission=None}
  RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map_err(Into::into),work:turn.work})
 }
}
