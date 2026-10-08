//! 📦️ Whole physical retirement of empty resident Store metadata backings.
use super::*;

fn extent<T>(capacity: usize) -> usize { capacity.checked_mul(std::mem::size_of::<T>()).expect("resident metadata backing layout") }

fn denied() -> RetainedCloneStep { RetainedCloneStep::Progress(Default::default()) }

fn released(bytes: usize, terminal: bool) -> RetainedCloneStep {
    let progress = RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() };
    if terminal { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }
}

fn admit_release(grant: RetainedCloneGrant, demand: usize) -> Result<bool, ValueError> {
    if grant.maximum_items == 0 || grant.maximum_release_bytes < demand { return Ok(false); }
    if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "resident backing requires original frame depth")); }
    Ok(true)
}

pub(super) fn vec_demand<T>(owner: &Vec<T>) -> usize { extent::<T>(owner.capacity()) }

pub(super) fn deque_demand<T>(owner: &VecDeque<T>) -> usize { extent::<T>(owner.capacity()) }

pub(super) fn close_vec<T>(owner: &mut Vec<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident vector backing still holds typed owners")); }
    let demand = vec_demand(owner);
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand, true))
}

pub(super) fn close_deque<T>(owner: &mut VecDeque<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    if !owner.is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident deque backing still holds typed owners")); }
    let demand = deque_demand(owner);
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    drop(std::mem::take(owner));
    Ok(released(demand, true))
}

pub(super) fn close_history<T>(owner: &mut crate::os_vcs::HistoryPageStack<T>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let demand = owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history backing still holds typed owners"))?;
    if demand == 0 { return Ok(RetainedCloneStep::Complete(Default::default())); }
    if !admit_release(grant, demand)? { return Ok(denied()); }
    if !owner.release_empty_page() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "resident history page changed after its exact demand")); }
    Ok(released(demand, owner.capacity() == 0))
}

/// 🧵️ Retains the original native vector allocation across ordered prefix publication.
pub(super) struct SourceIterator<T: semio_framework_value::retirement::RetireOwned> {
    source: std::mem::ManuallyDrop<std::vec::IntoIter<T>>,
    source_bytes: usize,
    pending: std::mem::ManuallyDrop<Option<T>>,
    active: Option<semio_framework_value::ControlledRetirement<T>>,
    terminal: bool,
}

impl<T: semio_framework_value::retirement::RetireOwned> SourceIterator<T> {
    pub(super) fn new(source: Vec<T>) -> Self {
        let source_bytes = extent::<T>(source.capacity());
        Self { source: std::mem::ManuallyDrop::new(source.into_iter()), source_bytes, pending: std::mem::ManuallyDrop::new(None), active: None, terminal: false }
    }

    pub(super) fn as_slice(&self) -> &[T] { self.source.as_slice() }

    pub(super) fn demands(&self, copy: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        use semio_framework_value::RetirementDemand;
        if self.terminal { return Ok(Default::default()); }
        if let Some(owner) = self.active.as_ref() {
            return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand()?, capacity_bytes: owner.next_capacity_byte_demand(copy)?, release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "source iterator child depth overflow"))? });
        }
        if self.pending.is_some() || self.source.len() != 0 {
            if !T::controlled_retirement_supported() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner, "original source iterator element has no controlled retirement declaration")); }
            return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<T>(), depth: 1, ..Default::default() });
        }
        Ok(RetirementDemand { release_bytes: self.source_bytes, depth: 1, ..Default::default() })
    }
}

impl<T: semio_framework_value::retirement::RetireOwned> Iterator for SourceIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<T> {
        assert!(!self.terminal && self.active.is_none() && self.pending.is_none(), "original source publication cannot race controlled retirement");
        self.source.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) { self.source.size_hint() }
}

impl<T: semio_framework_value::retirement::RetireOwned> ExactSizeIterator for SourceIterator<T> {}

impl<T: semio_framework_value::retirement::RetireOwned> ErasedSnapshotRetirement for SourceIterator<T> {
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.copy_bytes) }
    fn next_capacity_byte_demand(&self, copy: usize) -> Result<usize, ValueError> { self.demands(copy).map(|demand| demand.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.demands(0).map(|demand| demand.depth) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if grant.maximum_items == 0 { return Ok(denied()); }
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "original source iterator depth refused")); }
        if grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(denied()); }
        if let Some(owner) = self.active.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            let step = owner.step(child)?;
            let terminal = owner.terminal_is_empty();
            semio_framework_value::retained_clone::admit_retained_clone_close(child, step, terminal, "original source iterator child")?;
            if terminal { drop(self.active.take()); }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.pending.is_none() { *self.pending = self.source.next(); }
        if let Some(value) = self.pending.take() {
            match semio_framework_value::ControlledRetirement::new(value) {
                Ok(owner) => { self.active = Some(owner); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })); }
                Err((error, value)) => { *self.pending = Some(value); return Err(error); }
            }
        }
        drop(std::mem::replace(&mut *self.source, Vec::new().into_iter()));
        self.source_bytes = 0;
        self.terminal = true;
        Ok(released(demand.release_bytes, true))
    }
    fn terminal_is_empty(&self) -> bool { self.terminal && self.source_bytes == 0 && self.source.len() == 0 && self.pending.is_none() && self.active.is_none() }
}

impl<T: semio_framework_value::retirement::RetireOwned> Drop for SourceIterator<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original source iterator reached Drop before physical backing retirement");
        if self.terminal_is_empty() { unsafe { std::mem::ManuallyDrop::drop(&mut self.source); } }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
