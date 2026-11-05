//! 🧾️ Actual StepContext supplies failure control through its existing source and same-caller loans.
use super::{StepContext,OriginalFailureContext,OriginalTurnFailure,ValueError,RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
impl OriginalFailureContext for StepContext<'_>{
 fn original_failure_identity(&self)->Result<(usize,Option<u64>),ValueError>{let parent=self.original_source_parent()?;parent.validate_borrowed_context(self)?;Ok((parent.source_identity()?,self.original_clock_binding()))}
 fn is_cancelled(&self)->bool{StepContext::is_cancelled(self)}
 fn should_yield(&self)->bool{StepContext::should_yield(self)}
 fn original_retirement_is_active(&self)->Result<bool,ValueError>{self.original_source_parent()?.validate_borrowed_context(self)?;Ok(self.original_retirement)}
 fn retirement_should_yield(&self)->bool{StepContext::retirement_should_yield(self)}
 fn retained_work_can_enter(&self,demand:RetirementDemand)->bool{StepContext::retained_work_can_enter(self,demand)}
 fn retained_grant(&self)->RetainedCloneGrant{StepContext::retained_grant(self)}
 fn original_total_retained_grant(&self)->RetainedCloneGrant{StepContext::original_total_retained_grant(self)}
 fn original_total_fuel_remaining(&self)->u64{StepContext::original_total_fuel_remaining(self)}
 fn fuel_remaining(&self)->u64{StepContext::fuel_remaining(self)}
 fn original_absolute_deadline_us(&self)->u64{StepContext::original_absolute_deadline_us(self)}
 fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{StepContext::consume_retained(self,progress)}
 fn consume_fuel(&mut self,work:u64){StepContext::consume_fuel(self,work)}
 fn with_retained_loan<R:Copy>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{StepContext::with_retained_loan(self,grant,run)}
 fn with_original_work_loan<R:Copy>(&mut self,work:u64,deadline:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{StepContext::with_original_work_loan(self,work,deadline,run)}
}
