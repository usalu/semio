//! 🔗️ A leased shared allocation is the original authority of an MVD source and funds its admissions through the turn's own effect.
use semio_framework_job::{OriginalTurnFailure, StepContext, receive_original_retained_work};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneStep, RetainedWorkOutcome, ValueError, ValueRefusalKind};
/// 🔗️ A shared allocation leases its own payload as the original MVD clone source.
pub(super) use semio_framework_value::retirement::arc_custody::ArcLease as Lease;
/// 🪙️ Runs one admission step inside the turn's own original effect and settles its receipt and performed work once; `None` means the quote was not admitted and nothing ran.
pub(super) fn funded_turn(cx: &mut StepContext<'_>, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome<RetainedCloneStep, ValueError>) -> Result<Option<RetainedCloneStep>, OriginalTurnFailure> {
    let grant = cx.retained_grant();
    let Some((turn, performed)) = cx.with_original_effect(grant, |_, effect| run(effect))? else { return Ok(None) };
    let overrun = turn.work != performed;
    let step = receive_original_retained_work(cx, turn)?;
    if overrun { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "mvd admission work differs from its original effect").into()); }
    Ok(Some(step))
}
