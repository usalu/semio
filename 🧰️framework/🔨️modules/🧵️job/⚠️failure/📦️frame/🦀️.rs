//! 📦️ An exact prepaid return frame retains independent original callback and final-qualification causes.
use crate::OriginalTurnFailure;
use semio_framework_job_payload::payload_builder::OriginalOutcomeDescriptor;
use semio_framework_value_retained_clone::{OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetainedWorkPoll,RetirementDemand,ValueError,retirement::controlled::ControlledRetirement};
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalReturnPayload{pub callback:Option<OriginalTurnFailure>,pub qualification:Option<OriginalTurnFailure>,pub receipt_qualification:Option<OriginalTurnFailure>}
pub struct OriginalReturnFrame<A>{pub owner:ControlledRetirement<OriginalReturnPayload>,pub descriptor:Option<OriginalOutcomeDescriptor<A>>}
impl<A> OriginalReturnFrame<A>{
 pub fn empty()->Self{Self{owner:ControlledRetirement::retain(OriginalReturnPayload{callback:None,qualification:None,receipt_qualification:None}),descriptor:None}}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.descriptor.is_none()}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.owner.terminal_is_empty(){return self.owner.next_demand(body)}Ok(RetirementDemand{depth:self.descriptor.as_ref().map_or(0,|descriptor|if descriptor.is_acknowledged(){1}else{2}),..Default::default()})}
 pub fn close_step(&mut self,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll{
  if !self.owner.terminal_is_empty(){return self.owner.step(effect)}
  let grant=effect.grant();let done=|result:Result<RetainedCloneStep,ValueError>,work:u128|RetainedWorkPoll::Complete(RetainedWorkOutcome{result:result.map_err(OriginalRetirementFailure::from),work});
  let Some(descriptor)=self.descriptor.as_mut()else{return done(Ok(RetainedCloneStep::Complete(Default::default())),0)};
  let depth=if descriptor.is_acknowledged(){1}else{2};
  if grant.maximum_items==0||grant.maximum_depth<depth{return done(Ok(RetainedCloneStep::Progress(Default::default())),0)}
  if !descriptor.is_acknowledged(){let turn=descriptor.acknowledge(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});return done(turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),turn.work)}
  self.descriptor.take();done(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()})),1)
 }
}
