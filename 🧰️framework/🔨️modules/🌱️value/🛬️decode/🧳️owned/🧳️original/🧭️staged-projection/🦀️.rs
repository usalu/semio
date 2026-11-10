//! 🧭️ One source admission's lifetime-bound projection keeps its resident frame and unsettled action across turns and cancellation.
use crate::{RetainedAllocationEffect,RetainedWorkPoll,RetirementDemand,ValueError,retirement::RetireOwned,retained_clone::{OriginalEffectAction,OriginalProjectionFrame,OriginalProjectionFramePhase,RetainedCloneProgress,RetainedCloneSourceAdmission,RetainedCloneStep}};
/// 🧳️ The frame and action of an admission of `I` into its own sealed source.
pub struct StagedProjection<I:RetireOwned+Sync+'static>{frame:Option<OriginalProjectionFrame<I,I>>,action:Option<OriginalEffectAction<RetainedCloneStep,ValueError>>}
impl<I:RetireOwned+Sync+'static>Default for StagedProjection<I>{fn default()->Self{Self{frame:None,action:None}}}
type Admission<I>=RetainedCloneSourceAdmission<I,I>;
impl<I:RetireOwned+Sync+'static>StagedProjection<I>{
 pub fn terminal_is_empty(&self)->bool{self.frame.is_none()&&self.action.is_none()}
 /// 📏️ Quotes the open projection stage or the recorded refusal handback; none when the plain admission advances.
 pub fn quote(&self,admission:&Admission<I>)->Result<Option<(RetirementDemand,u64)>,ValueError>{
  if let Some(stage)=admission.projection_stage(&self.frame)?{return admission.projection_stage_quote(&self.frame,stage).map(Some)}
  Ok((admission.projection_result()==Some(false)).then(||(admission.take_projection_failure_demand(),1)))
 }
 /// 🪡️ Runs one stage or hands back the recorded refusal as the admission's error; an unsettled producer stays Pending.
 pub fn advance(&mut self,admission:&mut Admission<I>,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<RetainedCloneProgress,ValueError>{
  match admission.projection_stage(&self.frame){
   Ok(Some(stage))=>match admission.project_staged(&mut self.frame,&mut self.action,stage,effect){RetainedWorkPoll::Pending{work}=>RetainedWorkPoll::Pending{work},RetainedWorkPoll::Complete(turn)=>RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:turn.result.map(|step|step.progress()),work:turn.work})},
   Ok(None)=>{let mut refusal=None;let turn=admission.take_projection_failure(&mut refusal,effect);RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:match(turn.result,refusal){(Ok(step),Some(error))=>Err(error.with_retained_progress(step.progress())),(Ok(_),None)=>Err(ValueError::invariant("original projection refusal is absent")),(Err(error),_)=>Err(error)},work:turn.work})},
   Err(error)=>RetainedWorkPoll::Complete(crate::RetainedWorkOutcome{result:Err(error),work:0}),
  }
 }
 /// 📏️ Quotes the cancellation action that precedes the return of the admission's inputs; none once nothing is projected.
 pub fn close_quote(&self,admission:&Admission<I>)->Result<Option<(RetirementDemand,u64)>,ValueError>{
  if let Some(frame)=self.frame.as_ref(){return if frame.phase()?==OriginalProjectionFramePhase::ReturnOwner{Ok(Some((admission.return_projection_owner_demand(frame)?,1)))}else{let stage=admission.projection_stage(&self.frame)?.ok_or_else(||ValueError::invariant("open projection frame has no stage"))?;admission.projection_stage_quote(&self.frame,stage).map(Some)}}
  Ok(match admission.projection_result(){Some(true)=>Some((admission.discard_projection_demand(),1)),Some(false)=>Some((admission.take_projection_failure_demand(),1)),None=>None})
 }
 /// ♻️ Settles the open projection, then discards its answer; quiet once the admission may return its inputs.
 pub fn close(&mut self,admission:&mut Admission<I>,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{
  if let Some(frame)=self.frame.as_mut(){
   if frame.phase()?==OriginalProjectionFramePhase::ReturnOwner{return Ok(admission.return_projection_owner(frame,effect).result?.progress())}
   let stage=admission.projection_stage(&self.frame)?.ok_or_else(||ValueError::invariant("open projection frame has no stage"))?;
   return match admission.project_staged(&mut self.frame,&mut self.action,stage,effect){RetainedWorkPoll::Pending{..}=>Ok(Default::default()),RetainedWorkPoll::Complete(turn)=>Ok(turn.result?.progress())}
  }
  match admission.projection_result(){Some(true)=>Ok(admission.discard_projection(effect).result?.progress()),Some(false)=>{let mut refusal=None;let progress=admission.take_projection_failure(&mut refusal,effect).result?.progress();drop(refusal);Ok(progress)},None=>Ok(Default::default())}
 }
}
