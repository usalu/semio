//! ⏱️ Retained work outcomes cross owned cursors with their independent actual work.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::RetirementStep;
use semio_framework_value::{RetainedWorkOutcome, ValueError, ValueRefusalKind};
/// 🧭️ A retirement owner that was expected to exist but was omitted becomes an invariant refusal.
pub fn owned_step(turn: RetainedWorkOutcome<Option<RetainedCloneStep>>, message: &'static str) -> RetainedWorkOutcome {
    RetainedWorkOutcome { work: turn.work, result: turn.result.and_then(|step| step.ok_or_else(|| ValueError::literal(ValueRefusalKind::InvalidValue, message))) }
}
/// 🧭️ One owned close turn reports the cursor step for the work it spent.
pub fn cursor_step(turn: RetainedWorkOutcome, empty: bool) -> RetirementStep {
    match turn.result {
        Ok(RetainedCloneStep::Complete(progress)) if progress == RetainedCloneProgress::default() && empty => RetirementStep::Complete(turn.work),
        Ok(step) => RetirementStep::Progress(step.progress(), turn.work),
        Err(error) => RetirementStep::Failure(error, turn.work),
    }
}
/// 🧭️ A completed refusal-free turn without custody work.
pub fn idle_step(step: RetainedCloneStep) -> RetainedWorkOutcome {
    RetainedWorkOutcome { result: Ok(step), work: 0 }
}
/// 🧭️ A refusal that spent no work.
pub fn refused(error: ValueError) -> RetainedWorkOutcome {
    RetainedWorkOutcome { result: Err(error), work: 0 }
}
/// 🧭️ A retirement begin that found no owner lets the next owner try.
pub fn begun(turn: RetainedWorkOutcome<Option<RetainedCloneStep>>) -> Option<RetainedWorkOutcome> {
    match turn.result {
        Ok(Some(step)) => Some(RetainedWorkOutcome { result: Ok(step), work: turn.work }),
        Ok(None) => None,
        Err(error) => Some(RetainedWorkOutcome { result: Err(error), work: turn.work }),
    }
}
/// 🧭️ A single bounded construction action spends one unit of work.
pub fn costed<T, E>(result: Result<T, E>) -> RetainedWorkOutcome<T, E> {
    RetainedWorkOutcome { result, work: 1 }
}
