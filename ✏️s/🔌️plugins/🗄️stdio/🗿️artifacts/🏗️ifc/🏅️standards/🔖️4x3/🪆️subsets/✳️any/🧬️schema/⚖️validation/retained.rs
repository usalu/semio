//! 🎟️ Shared retained semantic transitions debit only the initiating original turn.
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress},
    RetirementDemand, ValueError, ValueRefusalKind,
};

pub fn refusal(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}
pub fn fixed(copy_bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
}
pub fn capacity(count: usize, width: usize) -> Result<usize, ValueError> {
    count.checked_mul(width).ok_or_else(|| refusal("express.retained-capacity-overflow"))
}
pub fn fits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool {
    grant.maximum_items > 0 && demand.copy_bytes <= grant.maximum_copy_bytes && demand.capacity_bytes <= grant.maximum_capacity_bytes && demand.release_bytes <= grant.maximum_release_bytes && demand.depth <= grant.maximum_depth
}
pub fn admitted(cx: &StepContext<'_>, demand: RetirementDemand) -> Result<bool, OriginalTurnFailure> {
    if cx.is_cancelled() {
        return Err(ValueError::literal(ValueRefusalKind::Canceled, "express.retained-cancelled").into());
    }
    Ok(!cx.should_yield() && fits(cx.retained_grant(), demand))
}
pub fn debit(cx: &mut StepContext<'_>, demand: RetirementDemand) -> Result<(), OriginalTurnFailure> {
    cx.consume_retained(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, released_bytes: demand.release_bytes })?;
    cx.consume_fuel(1);
    Ok(())
}
/// ⏳️ A suspended close keeps its state and retries next turn as zero progress; only a settled failure is a refusal.
pub fn settled(poll: semio_framework_value::OriginalRetirementPoll) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
    match poll {
        semio_framework_value::RetainedWorkPoll::Pending { .. } => Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default())),
        semio_framework_value::RetainedWorkPoll::Complete(turn) => turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),
    }
}
