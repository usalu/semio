//! 🎟️ Failure ownership borrows actual first-party caller control without depending on a concrete context.
use super::{OriginalTurnFailure,ValueError};
use semio_framework_value_retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetirementDemand};
/// 🧭️ Implementations preserve their genuine source, remaining stock and original receiving context.
pub trait OriginalFailureContext{
 fn original_failure_identity(&self)->Result<(usize,Option<u64>),ValueError>;
 fn is_cancelled(&self)->bool;
 fn should_yield(&self)->bool;
 fn original_retirement_is_active(&self)->Result<bool,ValueError>;
 fn retirement_should_yield(&self)->bool;
 fn retained_work_can_enter(&self,demand:RetirementDemand)->bool;
 fn retained_grant(&self)->RetainedCloneGrant;
 fn original_total_retained_grant(&self)->RetainedCloneGrant;
 fn original_total_fuel_remaining(&self)->u64;
 fn fuel_remaining(&self)->u64;
 fn original_absolute_deadline_us(&self)->u64;
 fn consume_retained(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>;
 fn consume_fuel(&mut self,work:u64);
 fn with_retained_loan<R:Copy>(&mut self,grant:RetainedCloneGrant,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>;
 fn with_original_work_loan<R:Copy>(&mut self,work:u64,deadline:u64,run:impl FnOnce(&mut Self)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>;
}
