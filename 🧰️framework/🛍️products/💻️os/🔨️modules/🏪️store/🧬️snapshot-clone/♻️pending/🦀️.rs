//! 🧳️ Original inline values transfer only through copy admission before typed retirement.
use semio_framework_value::{RetirementDemand, ValueError, retirement::{RetireOwned, controlled::ControlledRetirement}, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

pub(super) fn demands<T: RetireOwned>(value: &Option<T>, owner: &Option<ControlledRetirement<T>>, body: usize) -> Result<RetirementDemand, ValueError> {
    if let Some(owner) = owner.as_ref() {
        return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(body)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()? });
    }
    Ok(if value.is_some() { RetirementDemand { copy_bytes: size_of::<T>(), depth: 1, ..Default::default() } } else { Default::default() })
}

pub(super) fn close<T: RetireOwned>(value: &mut Option<T>, owner: &mut Option<ControlledRetirement<T>>, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<Option<RetainedCloneStep>> {
    let mut work=0;
    let result=(||{
    if value.is_none() && owner.is_none() { return Ok(None); }
    let empty = RetainedCloneProgress::default();
    if grant.maximum_items == 0 { return Ok(Some(RetainedCloneStep::Progress(empty))); }
    if let Some(cursor) = owner.as_mut() {
        let result=cursor.step(grant);work=cursor.last_turn_work();let step=result?;
        if cursor.terminal_is_empty() { *owner = None; }
        return Ok(Some(RetainedCloneStep::Progress(step.progress())));
    }
    let demand = demands(value, owner, grant.maximum_copy_bytes)?;
    if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original inline retirement transfer exceeds granted depth")); }
    if grant.maximum_copy_bytes < demand.copy_bytes { return Ok(Some(RetainedCloneStep::Progress(empty))); }
    work=1;
    match ControlledRetirement::new(value.take().unwrap()) {
        Ok(cursor) => *owner = Some(cursor),
        Err((error, original)) => { *value = Some(original); return Err(error); }
    }
    Ok(Some(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..empty })))
    })();
    semio_framework_value::RetainedWorkOutcome{result,work}
}
