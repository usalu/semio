//! 📦️ An exact prepaid return frame retains independent original callback and final-qualification causes.
use crate::OriginalTurnFailure;
use semio_framework_job_payload::OriginalOutcomeDescriptor;
use semio_framework_value_retained_clone::{RetirementDemand,RetainedCloneGrant,RetainedCloneStep,RetainedWorkOutcome,ValueError,retirement::controlled::ControlledRetirement};
#[derive(Debug,semio_framework_value_derive::RetireOwned)]
#[value(crate="::semio_framework_value_retained_clone")]
#[value(send_only)]
pub struct OriginalReturnPayload{pub callback:Option<OriginalTurnFailure>,pub qualification:Option<OriginalTurnFailure>,pub receipt_qualification:Option<OriginalTurnFailure>}
pub struct OriginalReturnFrame<A>{pub owner:ControlledRetirement<OriginalReturnPayload>,pub descriptor:Option<OriginalOutcomeDescriptor<A>>}
impl<A> OriginalReturnFrame<A>{
 pub fn empty()->Self{Self{owner:ControlledRetirement::retain(OriginalReturnPayload{callback:None,qualification:None,receipt_qualification:None}),descriptor:None}}
 pub fn terminal_is_empty(&self)->bool{self.owner.terminal_is_empty()&&self.descriptor.is_none()}
 pub fn demand(&self,body:usize)->Result<RetirementDemand,ValueError>{if !self.owner.terminal_is_empty(){return self.owner.next_demand(body)}Ok(RetirementDemand{depth:self.descriptor.as_ref().map_or(0,|descriptor|if descriptor.is_acknowledged(){1}else{2}),..Default::default()})}
 pub fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome{if !self.owner.terminal_is_empty(){let result=self.owner.step(grant);return RetainedWorkOutcome{result,work:self.owner.last_turn_work()}}let Some(descriptor)=self.descriptor.as_mut()else{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}};let depth=if descriptor.is_acknowledged(){1}else{2};if grant.maximum_items==0||grant.maximum_depth<depth{return RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if !descriptor.is_acknowledged(){let turn=descriptor.acknowledge(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant});return RetainedWorkOutcome{result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work}}self.descriptor.take();RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(semio_framework_value_retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
}
