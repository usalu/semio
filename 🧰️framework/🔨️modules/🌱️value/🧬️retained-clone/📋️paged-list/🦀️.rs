//! 📋️ Retained cloning and retirement for the first-party fixed-page value list.

use super::{close_retained_binding, RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
use crate::value::list::PagedList;
use crate::{
    SnapshotRetirementStep,
    retirement::{RetireOwned, RetirementCursor, RetirementStep},
};
use std::{
    mem::{ManuallyDrop, size_of},
    ops::{Deref, DerefMut},
};

pub struct PagedListRetirement<T: RetireOwned, const N: usize> {
    owner: ManuallyDrop<PagedList<T, N>>,
    released: bool,
}

impl<T: RetireOwned, const N: usize> RetirementCursor for PagedListRetirement<T, N> {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> RetirementStep {
        if grant.maximum_items == 0 { return RetirementStep::BudgetExhausted; }
        let maximum_bytes = if self.owner.is_empty() { grant.maximum_release_bytes } else { grant.maximum_copy_bytes };
        if self.released {
            return RetirementStep::Complete;
        }
        let width = size_of::<T>();
        if width != 0 && !std::mem::needs_drop::<T>() && !self.owner.is_empty() {
            let count = (maximum_bytes / width).min(self.owner.len());
            if count != 0 {
                for _ in 0..count {
                    self.owner.pop();
                }
                return RetirementStep::ProcessedBytes(count * width);
            }
        }
        if let Some(value) = self.owner.pop() {
            return RetirementStep::Child(value.retirement());
        }
        if self.owner.terminal_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.owner) };
            self.released = true;
            return RetirementStep::Complete;
        }
        let Ok(required) = self.owner.next_release_allocation_bytes() else { return RetirementStep::BudgetExhausted; };
        if required > maximum_bytes {
            return RetirementStep::BudgetExhausted;
        }
        match self.owner.release_empty_page(maximum_bytes) {
            Ok(progress) if progress.progressed => RetirementStep::Bytes(progress.released_allocation_bytes),
            _ => RetirementStep::BudgetExhausted,
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.released
    }

    fn next_work_byte_demand(&self) -> usize { if !self.released && !self.owner.is_empty() && !std::mem::needs_drop::<T>() { size_of::<T>() } else { 0 } }

    fn next_close_byte_demand(&self) -> Option<usize> {
        if self.released || !self.owner.is_empty() || self.owner.terminal_is_empty() {
            None
        } else {
            self.owner.next_release_allocation_bytes().ok()
        }
    }
    fn next_birth_bytes(&self, maximum_bytes: usize) -> Option<usize> {
        if self.released || self.owner.is_empty() || (!std::mem::needs_drop::<T>() && size_of::<T>() != 0 && maximum_bytes >= size_of::<T>()) { Some(0) }
        else { self.owner.get(self.owner.len() - 1)?.retirement_birth_bytes() }
    }
    fn terminal_release_bytes(&self) -> Option<usize> { Some(size_of::<Self>()) }
}

impl<T: RetireOwned, const N: usize> Drop for PagedListRetirement<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.released, "paged list retired before terminal-empty");
    }
}

impl<T: RetireOwned, const N: usize> RetireOwned for PagedList<T, N> {
    fn retirement(self) -> Box<dyn RetirementCursor> {
        Box::new(PagedListRetirement { owner: ManuallyDrop::new(self), released: false })
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { Some(size_of::<PagedListRetirement<T, N>>()) }
    fn controlled_retirement_supported() -> bool { true }
}

#[doc(hidden)]
pub struct PagedListCursorState<T: RetainedClone, const N: usize> {
    values: PagedList<T, N>,
    child: Option<T::Cursor>,
    child_value: Option<T>,
    index: usize,
    source: Option<RetainedCloneBinding>,
    phase: u8,
    output: Option<PagedList<T, N>>,
    closing: bool,
    close: RetainedCloneClose,
}

pub struct PagedListCursor<T: RetainedClone, const N: usize> {
    state: ManuallyDrop<PagedListCursorState<T, N>>,
}

impl<T: RetainedClone, const N: usize> Deref for PagedListCursor<T, N> {
    type Target = PagedListCursorState<T, N>;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl<T: RetainedClone, const N: usize> DerefMut for PagedListCursor<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.state
    }
}

impl<T: RetainedClone, const N: usize> Default for PagedListCursor<T, N> {
    fn default() -> Self {
        Self { state: ManuallyDrop::new(PagedListCursorState { values: PagedList::default(), child: None, child_value: None, index: 0, source: None, phase: 0, output: None, closing: false, close: RetainedCloneClose::default() }) }
    }
}

impl<T: RetainedClone, const N: usize> PagedListCursor<T, N> {
    fn owner_state_is_empty(&self) -> bool {
        self.values.terminal_is_empty() && self.child.is_none() && self.child_value.is_none() && self.output.is_none() && self.close.is_empty() && self.source.is_none()
    }
}

impl<T: RetainedClone, const N: usize> Drop for PagedListCursor<T, N> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.owner_state_is_empty(), "retained paged list cursor abandoned before bounded terminal close");
        if self.owner_state_is_empty() {
            unsafe { ManuallyDrop::drop(&mut self.state) };
        }
    }
}

impl<T: RetainedClone, const N: usize> RetainedCloneCursor<PagedList<T, N>> for PagedListCursor<T, N> {
    fn advance(&mut self, source: RetainedCloneRef<'_, PagedList<T, N>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained paged list clone cursor is closing"));
        }
        if grant.maximum_items == 0 && grant.maximum_copy_bytes == 0 && grant.maximum_capacity_bytes == 0 && grant.maximum_release_bytes == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        source.bind(&mut self.source)?;
        let source_value = source.get();
        if source_value.len() > N {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained paged list source exceeds its declared logical capacity"));
        }
        if self.phase == 1 {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        if self.phase == 2 {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained paged list clone cursor is spent"));
        }
        let mut used = RetainedCloneProgress::default();
        loop {
            let remaining = RetainedCloneGrant {
                maximum_items: grant.maximum_items.saturating_sub(used.copied_items),
                maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),
                maximum_capacity_bytes: grant.maximum_capacity_bytes.saturating_sub(used.retained_capacity_bytes),
                maximum_depth: grant.maximum_depth, maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(used.released_bytes) };
            if self.values.capacity() < source_value.len() {
                if remaining.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                let Some(required) = self.values.next_capacity_allocation_bytes(source_value.len())? else {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained paged list capacity state is inconsistent"));
                };
                if required > remaining.maximum_capacity_bytes {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                let progress = self.values.reserve_capacity_one(source_value.len(), remaining.maximum_capacity_bytes).map_err(|error| crate::ValueError::from(error.refusal()))?;
                if !progress.progressed {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                let progress = admit_retained_clone_progress(remaining, RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: progress.allocated_bytes, ..Default::default() }, "retained paged list capacity")?;
                used = used.checked_add(progress)?;
                continue;
            }
            let state = &mut *self.state;
            if let Some(child) = state.child.as_mut() {
                if state.child_value.is_some() {
                    if !child.terminal_is_empty() {
                        if remaining.maximum_items == 0 {
                            return Ok(RetainedCloneStep::Progress(used));
                        }
                        let step = child.close_granted(remaining)?;
                        let progress = super::admit_retained_clone_close(remaining, step, child.terminal_is_empty(), "retained paged list child scaffold close")?.progress();
                        if progress == RetainedCloneProgress::default() { return Ok(RetainedCloneStep::Progress(used)); }
                        used = used.checked_add(progress)?;
                        continue;
                    }
                    if remaining.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    let progress = state.values.adopt_reserved(&mut state.child_value)?;
                    if !progress.progressed {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    state.child = None;
                    state.index += 1;
                    let progress = admit_retained_clone_progress(remaining, RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: 0, released_bytes: 0 }, "retained paged list owner adoption")?;
                    used = used.checked_add(progress)?;
                    continue;
                }
            }
            if self.index == source_value.len() {
                if remaining.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(used));
                }
                self.output = Some(std::mem::take(&mut self.values));
                self.phase = 1;
                used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                return Ok(RetainedCloneStep::Complete(used));
            }
            if remaining.maximum_items == 0 && remaining.maximum_copy_bytes == 0 && remaining.maximum_capacity_bytes == 0 && remaining.maximum_release_bytes == 0 {
                return Ok(RetainedCloneStep::Progress(used));
            }
            let index = self.index;
            let state = &mut *self.state;
            let child = state.child.get_or_insert_with(T::retained_clone_cursor);
            match child.advance(source.project(index + 1, |values| values.get(index).expect("retained paged list source ordinal exists")), remaining)? {
                RetainedCloneStep::Progress(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained paged list child")?;
                    if progress == RetainedCloneProgress::default() {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    used = used.checked_add(progress)?;
                }
                RetainedCloneStep::Complete(progress) => {
                    let progress = admit_retained_clone_progress(remaining, progress, "retained paged list child")?;
                    used = used.checked_add(progress)?;
                    if used.copied_items == grant.maximum_items {
                        return Ok(RetainedCloneStep::Progress(used));
                    }
                    state.child_value = Some(child.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained paged list child completed without an owner"))?);
                    let _ = child.begin_close();
                    used = used.checked_add(RetainedCloneProgress { copied_items: 1, ..Default::default() })?;
                }
            }
        }
    }

    fn take(&mut self) -> Option<PagedList<T, N>> {
        if self.phase != 1 {
            return None;
        }
        self.phase = 2;
        self.output.take()
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, crate::ValueError> {
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if child.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(child.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained paged list child close")?;
            if step != SnapshotRetirementStep::Complete {
                return Ok(step);
            }
            if !child.terminal_is_empty() {
                return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained paged list child close completed with a live owner"));
            }
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            state.child = None;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if !state.close.is_empty() {
            let step = state.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = state.close.begin_option(&mut state.child_value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = state.close.begin_option(&mut state.output, maximum_items)? {
            return Ok(step);
        }
        if !state.values.terminal_is_empty() {
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            state.close.begin(std::mem::take(&mut state.values))?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        state.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "paged list must begin close before granted retirement")); }
        let state = &mut *self.state;
        if let Some(child) = state.child.as_mut() {
            if child.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            if !child.terminal_is_empty() { let step = child.close_granted(grant)?; return Ok(RetainedCloneStep::Progress(super::admit_retained_clone_close(grant, step, child.terminal_is_empty(), "retained child close")?.progress())); }
            state.child = None;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        }
        if !state.close.is_empty() { return state.close.step_granted(grant); }
        if let Some(step) = state.close.begin_granted(&mut state.child_value, grant)? { return Ok(step); }
        if let Some(step) = state.close.begin_granted(&mut state.output, grant)? { return Ok(step); }
        if !state.values.terminal_is_empty() {
            if let Some(step) = state.close.begin_default_granted(&mut state.values, grant)? { return Ok(step); }
        }
        close_retained_binding(&mut state.source, grant)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child) = self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_copy_byte_demand() }; }
        self.close.next_copy_byte_demand()
    }
    fn next_close_capacity_byte_demand(&self, maximum_release_bytes: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_capacity_byte_demand(maximum_release_bytes) }; }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(maximum_release_bytes); }
        if self.child_value.is_some() { return self.close.next_owner_capacity_byte_demand::<T>(true,maximum_release_bytes); }
        self.close.next_owner_capacity_byte_demand::<PagedList<T,N>>(self.output.is_some()||!self.values.terminal_is_empty(),maximum_release_bytes)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if let Some(child)=self.child.as_ref() { return if child.terminal_is_empty() { Ok(0) } else { child.next_close_release_byte_demand() }; }
        self.close.next_release_byte_demand()
    }
    fn terminal_is_empty(&self) -> bool { self.closing && self.owner_state_is_empty() }
}

impl<T: RetainedClone, const N: usize> RetainedClone for PagedList<T, N> {
    type Cursor = PagedListCursor<T, N>;

    fn retained_clone_cursor() -> Self::Cursor {
        PagedListCursor::default()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
