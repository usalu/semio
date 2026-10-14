//! 🎟️ Close cursors borrow the same original clock, work and independently remaining retained currencies.
use crate::{InteractiveJobCloseStep,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneStep,RetirementDemand,ValueError};

/// 🪙️ One quoted original cursor action records its actual physical effects and CPU exactly once.
pub fn close_original_retained_step(cx:&mut StepContext<'_>,demand:RetirementDemand,advance:impl FnOnce(RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>)->Result<InteractiveJobCloseStep,OriginalTurnFailure>{
 let remaining=cx.retained_grant();if cx.retirement_should_yield()||remaining.maximum_items==0||remaining.maximum_copy_bytes<demand.copy_bytes||remaining.maximum_capacity_bytes<demand.capacity_bytes||remaining.maximum_release_bytes<demand.release_bytes||remaining.maximum_depth<demand.depth{return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
 let grant=RetainedCloneGrant{maximum_items:1,..remaining};cx.consume_fuel(1);let step=advance(grant).map_err(|error|OriginalTurnFailure::debit_original(error,cx))?;let progress=step.progress();cx.consume_retained(progress)?;Ok(InteractiveJobCloseStep::Pending{progress})
}

