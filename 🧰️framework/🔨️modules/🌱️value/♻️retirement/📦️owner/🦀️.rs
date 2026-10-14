//! ♻️ Original inline child retirement under explicit caller grants.
use semio_framework_value::{ErasedSnapshotRetirement,ValueError,RetainedCloneGrant,RetainedCloneStep};
/// 🪆️ Observes an inline retained child's fallible demands with its logical parent depth.
pub fn retirement_owner_demands<O: ErasedSnapshotRetirement>(owner: &Option<O>, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
    let Some(owner) = owner.as_ref() else { return Ok(Default::default()); };
    let mut demand = owner.next_demand(body)?;
    demand.depth = demand.depth.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement depth overflow"))?;
    Ok(demand)
}

/// 🧷️ Closes an inline owner under the original grant and removes it only after its exact terminal witness.
pub fn retirement_owner_close<O: ErasedSnapshotRetirement>(slot: &mut Option<O>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    use semio_framework_value::retained_clone::{RetainedCloneStep, admit_retained_clone_close};
    if slot.is_none() { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
    if grant.maximum_depth < retirement_owner_demands(slot, grant.maximum_copy_bytes)?.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "inline retirement exceeds admitted depth")); }
    let owner = slot.as_mut().unwrap();
    let child_grant = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
    let step = owner.close_step(child_grant)?;
    admit_retained_clone_close(child_grant, step, owner.terminal_is_empty(), "inline retirement child")?;
    if matches!(step, RetainedCloneStep::Complete(_)) { drop(slot.take()); }
    Ok(step)
}

