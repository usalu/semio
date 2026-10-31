//! ♻️ Original inline child retirement under explicit caller grants.
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,RetainedCloneGrant,RetainedWorkOutcome};
/// 🪆️ Observes an inline retained child's fallible demands with its logical parent depth.
pub fn retirement_owner_demands<O: ErasedSnapshotRetirement>(owner: &Option<O>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let Some(owner) = owner.as_ref() else { return Ok(Default::default()); };
    let mut demand = owner.next_demand(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement depth overflow"))?;
    Ok(demand)
}

/// 🧷️ Closes an inline owner under the original grant and removes it only after its exact terminal witness.
pub fn retirement_owner_close<O: ErasedSnapshotRetirement>(slot: &mut Option<O>, grant: RetainedCloneGrant) -> RetainedWorkOutcome {
    use semio_framework_value::retained_clone::{RetainedCloneStep, admit_retained_clone_close};
    let refuse = |error: ValueError, work: u64| RetainedWorkOutcome { result: Err(error), work };
    if slot.is_none() { return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 }; }
    if grant.maximum_items == 0 { return RetainedWorkOutcome { result: Ok(RetainedCloneStep::Progress(Default::default())), work: 0 }; }
    let depth = match retirement_owner_demands(slot, grant.maximum_copy_bytes) { Ok(demand) => demand.depth, Err(error) => return refuse(error, 0) };
    if grant.maximum_depth < depth { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement exceeds admitted depth"), 0); }
    let Some(owner) = slot.as_mut() else { return refuse(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "inline retirement owner vanished"), 0); };
    let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
    let RetainedWorkOutcome { result, work } = owner.close_step(child_grant);
    let step = match result { Ok(step) => step, Err(error) => return refuse(error, work) };
    let step = match admit_retained_clone_close(child_grant, step, owner.terminal_is_empty(), "inline retirement child") { Ok(step) => step, Err(error) => return refuse(error, work) };
    if matches!(step, RetainedCloneStep::Complete(_)) { drop(slot.take()); }
    RetainedWorkOutcome { result: Ok(step), work }
}
