//! ♻️ Retirement of the version-graph owners: history pages, history keys, group visibility and authors release
//! through the neutral `RetireOwned` contract beside the types they own, so every layer above the version graph
//! (store, sync, composition) retires them without owning their protocol.
use semio_framework_value::artifact_retire_struct;
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use semio_framework_value::retirement::{RetirementStep, controlled::ControlledRetirement, record_retirement_work};
use semio_framework_value::RetainedWorkPoll;
use std::mem::ManuallyDrop;

fn pay(effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->Result<u128,RetirementStep>{record_retirement_work(effect,1).map_err(|(error,work)|RetirementStep::Failure(error.into(),work))}

#[doc(hidden)]
pub struct HistoryPageRetirement<T: RetireOwned> {
    original: ManuallyDrop<super::HistoryPageStack<T>>,
    active: ManuallyDrop<Option<ControlledRetirement<T>>>,
}

impl<T: RetireOwned> RetirementCursor for HistoryPageRetirement<T> {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{if let Some(active)=self.active.as_ref(){let mut demand=active.next_demand(body)?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::DepthLimit,"history entry depth overflow"))?;return Ok(demand);}Ok(semio_framework_value::RetirementDemand {release_bytes:if self.original.is_empty(){self.original.next_empty_page_release_byte_demand().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"history backing extent unavailable"))?}else{0},depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetirementStep {
        let grant = effect.grant();
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        match self.next_depth_demand() {
            Err(error) => return RetirementStep::Failure(error.into(), 0),
            Ok(depth) if grant.maximum_depth < depth => return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "Store history page exceeds admitted depth").into(), 0),
            _ => {}
        }
        if let Some(active) = self.active.as_mut() {
            if active.terminal_is_empty() {
                let work = match pay(effect) { Ok(work) => work, Err(step) => return step };
                drop(self.active.take());
                return RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }, work);
            }
            return match active.step(&mut effect.nested(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })) {
                RetainedWorkPoll::Pending { work } => RetirementStep::Pending(work),
                RetainedWorkPoll::Complete(turn) => match turn.result {
                    Ok(RetainedCloneStep::Progress(progress) | RetainedCloneStep::Complete(progress)) => RetirementStep::Progress(progress, turn.work),
                    Err(failure) => RetirementStep::Failure(failure, turn.work),
                },
            };
        }
        if self.original.len() != 0 {
            let work = match pay(effect) { Ok(work) => work, Err(step) => return step };
            let Some(value) = self.original.pop() else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "history page lost its counted entry").into(), work); };
            return match ControlledRetirement::new(value) {
                Ok(active) => { *self.active = Some(active); RetirementStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }, work) }
                Err((error, value)) => match self.original.try_push(value) {
                    Ok(()) => RetirementStep::Failure(error.into(), work),
                    Err(lost) => { std::mem::forget(lost); RetirementStep::Failure(error.into(), work) }
                },
            };
        }
        let Some(release) = self.original.next_empty_page_release_byte_demand() else { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "empty history page lost its release extent").into(), 0); };
        if release == 0 { return RetirementStep::Complete(0); }
        if grant.maximum_release_bytes < release { return RetirementStep::BudgetExhausted; }
        if let Err(step) = pay(effect) { return step; }
        if !self.original.release_empty_page() { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::InvariantViolated, "history page changed after exact release inquiry").into(), 0); }
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
    type RetirementCursor = HistoryPageRetirement<T>;
    fn retirement_seed(self) -> Self::RetirementCursor { HistoryPageRetirement { original: ManuallyDrop::new(self), active: ManuallyDrop::new(None) } }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<HistoryPageRetirement<T>>()) }
    fn controlled_retirement_supported() -> bool { T::controlled_retirement_supported() }
}

artifact_retire_struct!(super::ArtifactHistoryKey { index, generation });
#[doc(hidden)]
pub struct GroupVisibilityRetirement {
    original: Option<super::ArtifactGroupVisibility>,
    remaining: usize,
}

impl RetirementCursor for GroupVisibilityRetirement {
    fn next_demand(&self, body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{Ok(semio_framework_value::RetirementDemand {copy_bytes:usize::from(self.remaining!=0),depth:usize::from(!self.terminal_is_empty()),..Default::default()})}
    fn close_step(&mut self, effect: &mut semio_framework_value::RetainedAllocationEffect<'_>) -> RetirementStep {
        let grant = effect.grant();
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        if !self.terminal_is_empty() && grant.maximum_depth == 0 { return RetirementStep::Failure(ValueError::literal(ValueRefusalKind::DepthLimit, "group visibility requires its original owner depth").into(), 0); }
        if self.remaining != 0 {
            let bytes = self.remaining.min(grant.maximum_copy_bytes);
            if bytes == 0 { return RetirementStep::BudgetExhausted; }
            if let Err(step) = pay(effect) { return step; }
            self.remaining -= bytes;
            return RetirementStep::ProcessedBytes(bytes);
        }
        let work = match pay(effect) { Ok(work) => work, Err(step) => return step };
        drop(self.original.take());
        RetirementStep::Complete(work)
    }
    fn terminal_is_empty(&self) -> bool { self.original.is_none() && self.remaining == 0 }
    fn next_work_byte_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(self.remaining != 0)) }
    fn next_birth_bytes(&self, _: usize) -> Option<usize> { Some(0) }
    fn next_close_byte_demand(&self) -> Option<usize> { Some(0) }
    fn terminal_release_bytes(&self) -> Option<usize> { self.terminal_is_empty().then_some(size_of::<Self>()) }
}

impl RetireOwned for super::ArtifactGroupVisibility {
    type RetirementCursor = GroupVisibilityRetirement;
    fn retirement_seed(self) -> Self::RetirementCursor { GroupVisibilityRetirement { original: Some(self), remaining: size_of::<Self>() } }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<GroupVisibilityRetirement>()) }
    fn controlled_retirement_supported() -> bool { true }
    fn retirement_element_copy_bytes() -> usize { size_of::<Self>() }
}

artifact_retire_struct!(super::Change { id, edit_ids, description, saved_at });
artifact_retire_struct!(super::CompositionPin { child_ref, checkpoint_id });
artifact_retire_struct!(super::Checkpoint { id, change_ids, parent_id, authors, message, timestamp, composition_pins });
artifact_retire_struct!(super::Alternative { id, name, checkpoint_ids });
