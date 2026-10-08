//! 📦️ Whole physical retirement of empty resident Store metadata backings.
use super::*;

fn extent<T>(capacity: usize) -> usize { capacity.checked_mul(std::mem::size_of::<T>()).expect("resident metadata backing layout") }

fn denied() -> SnapshotRetirementStep { SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 } }

fn released(bytes: usize) -> SnapshotRetirementStep { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes } }

pub(super) fn vec_demand<T>(owner: &Vec<T>) -> usize { extent::<T>(owner.capacity()) }

pub(super) fn deque_demand<T>(owner: &VecDeque<T>) -> usize { extent::<T>(owner.capacity()) }

pub(super) fn close_vec<T>(owner: &mut Vec<T>, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident vector backing still holds typed owners")); }
    let demand = vec_demand(owner);
    if demand == 0 { return Ok(SnapshotRetirementStep::Complete); }
    if items == 0 || bytes < demand { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand))
}

pub(super) fn close_deque<T>(owner: &mut VecDeque<T>, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident deque backing still holds typed owners")); }
    let demand = deque_demand(owner);
    if demand == 0 { return Ok(SnapshotRetirementStep::Complete); }
    if items == 0 || bytes < demand { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand))
}

pub(super) fn close_history<T>(owner: &mut crate::os_vcs::HistoryPageStack<T>, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
    let demand = owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history backing still holds typed owners"))?;
    if demand == 0 { return Ok(SnapshotRetirementStep::Complete); }
    if items == 0 || bytes < demand { return Ok(denied()); }
    if !owner.release_empty_page() { return Err(ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history page changed after its exact demand")); }
    Ok(released(demand))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
