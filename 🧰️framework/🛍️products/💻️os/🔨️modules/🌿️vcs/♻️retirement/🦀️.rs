//! ♻️ Retirement of the version-graph owners: history pages, history keys, group visibility and authors release
//! through the neutral `RetireOwned` contract beside the types they own, so every layer above the version graph
//! (store, sync, composition) retires them without owning their protocol.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetirementStep, controlled::ControlledRetirement};
use std::mem::ManuallyDrop;

struct HistoryPageRetirement<T: RetireOwned> {
    original: ManuallyDrop<super::HistoryPageStack<T>>,
    active: ManuallyDrop<Option<ControlledRetirement<T>>>,
}

impl<T: RetireOwned> RetirementCursor for HistoryPageRetirement<T> {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.active.as_ref(){let mut demand=active.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"history entry depth overflow"))?;return Ok(demand);}Ok(semio_framework_value::RetirementDemand {release_bytes:if self.original.is_empty(){self.original.next_empty_page_release_byte_demand().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"history backing extent unavailable"))?}else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        match self.next_depth_demand() {
            Err(error) => return RetirementStep::Failure(error),
            Ok(depth) if grant.maximum_depth < depth => return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "Store history page exceeds admitted depth")),
            _ => {}
        }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                drop(self.active.take());
                return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
            }
            return match active.step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }) {
                Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => RetirementStep::Progress(progress),
                Err(error) => RetirementStep::Failure(error),
            };
        }
        if let Some(value) = self.original.pop() {
            *self.active = Some(ControlledRetirement::new(value).unwrap_or_else(|_| unreachable!("history entry declares controlled retirement")));
            return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() });
        }
        let Some(release) = self.original.next_empty_page_release_byte_demand() else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "empty history page lost its release extent")); };
        if release == 0 { return RetirementStep::Complete; }
        if grant.maximum_release_bytes < release { return RetirementStep::BudgetExhausted; }
        if !self.original.release_empty_page() { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "history page changed after exact release inquiry")); }
        RetirementStep::Bytes(release)
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_empty() && self.original.capacity() == 0 && self.active.is_none() }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or(Ok(0), ControlledRetirement::next_copy_byte_demand) }
    fn next_birth_bytes(&self, copy: usize) -> Option<usize> { self.active.as_ref().map_or(Some(0), |active| active.next_capacity_byte_demand(copy).ok()) }
    fn next_close_byte_demand(&self) -> Option<usize> { self.active.as_ref().map_or_else(|| if self.original.is_empty() { self.original.next_empty_page_release_byte_demand() } else { Some(0) }, |active| active.next_release_byte_demand().ok()) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or(Ok(usize::from(!self.terminal_is_empty())), |active| active.next_depth_demand()?.checked_add(1).ok_or_else(|| ValueError::literal(ValueRefusalKind::DepthLimit, "history entry depth overflow"))) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}

impl<T: RetireOwned> Drop for HistoryPageRetirement<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "original Store history pages must reach physical terminal ownership");
        if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.original); ManuallyDrop::drop(&mut self.active); } }
    }
}

impl<T: RetireOwned> RetireOwned for super::HistoryPageStack<T> {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(HistoryPageRetirement { original: ManuallyDrop::new(self), active: ManuallyDrop::new(None) }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<HistoryPageRetirement<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

artifact_retire_struct!(super::ArtifactHistoryKey { index, generation });
struct GroupVisibilityRetirement {
    original: Option<super::ArtifactGroupVisibility>,
    remaining: usize,
}

impl RetirementCursor for GroupVisibilityRetirement {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand {copy_bytes:usize::from(self.remaining!=0),depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        if !self.terminal_is_empty() && grant.maximum_depth == 0 { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "group visibility requires its original owner depth")); }
        if self.remaining != 0 {
            let bytes = self.remaining.min(grant.maximum_copy_bytes);
            if bytes == 0 { return RetirementStep::BudgetExhausted; }
            self.remaining -= bytes;
            return RetirementStep::ProcessedBytes(bytes);
        }
        drop(self.original.take());
        RetirementStep::Complete
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_none() && self.remaining == 0 }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.remaining != 0)) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}

impl RetireOwned for super::ArtifactGroupVisibility {
    fn retirement(self) -> Box<dyn RetirementCursor> { Box::new(GroupVisibilityRetirement { original: Some(self), remaining: size_of::<Self>() }) }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<GroupVisibilityRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
    fn retirement_element_copy_bytes() -> usize { size_of::<Self>() }
}

artifact_retire_struct!(super::Change { id, edit_ids, description, saved_at });
artifact_retire_struct!(super::CompositionPin { child_ref, checkpoint_id });
artifact_retire_struct!(super::Checkpoint { id, change_ids, parent_id, authors, message, timestamp, composition_pins });
artifact_retire_struct!(super::Alternative { id, name, checkpoint_ids });
