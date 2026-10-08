//! 📦️ Retains original diagnostics in native pages until one final edit selection.
use crate::{MutationMessage, MutationReplayOutcome};
use semio_framework_value::{list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}, ValueError};
use std::mem::{size_of, ManuallyDrop};

pub struct ReplayMessageAccumulator {
    rows: ManuallyDrop<PagedList<MutationMessage, {usize::MAX}>>,
    active: Option<MutationMessageRetirement>,
}

impl Default for ReplayMessageAccumulator {
    fn default() -> Self { Self { rows: ManuallyDrop::new(PagedList::default()), active: None } }
}

impl ReplayMessageAccumulator {
    pub fn rows(&self) -> &PagedList<MutationMessage, {usize::MAX}> { &self.rows }
    pub fn rows_mut(&mut self) -> &mut PagedList<MutationMessage, {usize::MAX}> { &mut self.rows }
    pub fn next_capacity_byte_demand(&self) -> Result<usize, ValueError> { if self.rows.has_reserved_slot() { Ok(0) } else { self.rows.next_allocation_bytes().map_err(ValueError::from) } }
    pub fn append(&mut self, source: &mut Option<MutationMessage>, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let idle = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || source.is_none() { return Ok(idle); }
        if !self.rows.has_reserved_slot() {
            let progress = self.rows.reserve_one(grant.maximum_capacity_bytes).map_err(|error| ValueError::from(error.refusal()))?;
            return Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), retained_capacity_bytes: progress.allocated_bytes, ..idle });
        }
        let progress = self.rows.place_reserved(source, grant.maximum_copy_bytes).map_err(ValueError::from)?;
        Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), copied_bytes: progress.placed_bytes, ..idle })
    }
    pub fn next_close_copy_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| usize::from(!self.rows.is_empty()) * size_of::<MutationMessage>(), MutationMessageRetirement::next_copy_byte_demand) }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or_else(|| if self.rows.is_empty() { self.rows.next_release_allocation_bytes().map_err(ValueError::from) } else { Ok(0) }, |active| Ok(active.next_release_byte_demand())) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let idle = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(idle); }
        if let Some(active) = self.active.as_mut() {
            let progress = active.close_step(grant);
            if active.terminal_is_empty() { self.active = None; }
            return Ok(progress);
        }
        if !self.rows.is_empty() {
            if grant.maximum_copy_bytes < size_of::<MutationMessage>() { return Ok(idle); }
            self.active = Some(MutationMessageRetirement::new(self.rows.pop().expect("retained message row")));
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle });
        }
        let progress = self.rows.release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
        Ok(RetainedCloneProgress { copied_items: usize::from(progress.progressed), released_bytes: progress.released_allocation_bytes, ..idle })
    }
    pub fn terminal_is_empty(&self) -> bool { self.rows.terminal_is_empty() && self.active.is_none() }
}

impl Drop for ReplayMessageAccumulator {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "replay original messages require terminal page retirement"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.rows); } } }
}

pub struct MutationMessageRetirement {
    strings: [Option<String>; 2],
    targets: ManuallyDrop<Vec<String>>,
    active: Option<String>,
    terminal: bool,
}

impl MutationMessageRetirement {
    pub fn new(message: MutationMessage) -> Self { Self { strings: [Some(message.code.0), Some(message.message)], targets: ManuallyDrop::new(message.target), active: None, terminal: false } }
    pub(super) fn body(&self) -> Option<&str> { self.strings[1].as_deref() }
    pub fn next_copy_byte_demand(&self) -> usize { usize::from(self.active.is_none() && (self.strings.iter().any(Option::is_some) || !self.targets.is_empty())) * size_of::<String>() }
    pub fn next_release_byte_demand(&self) -> usize { self.active.as_ref().map_or_else(|| if self.next_copy_byte_demand() == 0 { self.targets.capacity() * size_of::<String>() } else { 0 }, String::capacity) }
    pub fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal)) }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneProgress {
        let idle = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || self.terminal { return idle; }
        if let Some(active) = self.active.as_ref() {
            let released = active.capacity();
            if grant.maximum_release_bytes < released { return idle; }
            drop(self.active.take());
            return RetainedCloneProgress { copied_items: 1, released_bytes: released, ..idle };
        }
        let copied = self.next_copy_byte_demand();
        if copied > 0 {
            if grant.maximum_copy_bytes < copied { return idle; }
            self.active = self.strings.iter_mut().find_map(Option::take).or_else(|| self.targets.pop());
            return RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..idle };
        }
        let released = self.targets.capacity() * size_of::<String>();
        if grant.maximum_release_bytes < released { return idle; }
        drop(std::mem::take(&mut *self.targets));
        self.terminal = true;
        RetainedCloneProgress { copied_items: 1, released_bytes: released, ..idle }
    }
    pub fn terminal_is_empty(&self) -> bool { self.terminal && self.active.is_none() && self.strings.iter().all(Option::is_none) && self.targets.is_empty() && self.targets.capacity() == 0 }
}

impl Drop for MutationMessageRetirement {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "original diagnostic row requires paid physical retirement"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.targets); } } }
}

pub struct OperationMessageDrain {
    outcome: ManuallyDrop<Option<MutationReplayOutcome>>,
    pending: Option<MutationMessage>,
    index: usize,
    original_len: usize,
}

impl OperationMessageDrain {
    pub fn new(outcome: MutationReplayOutcome) -> Self { let original_len = outcome.messages.len(); Self { outcome: ManuallyDrop::new(Some(outcome)), pending: None, index: 0, original_len } }
    pub fn next_capacity_byte_demand(&self, ledger: &ReplayMessageAccumulator) -> Result<usize, ValueError> { if self.pending.is_some() { ledger.next_capacity_byte_demand() } else { Ok(0) } }
    pub fn next_copy_byte_demand(&self, ledger: &ReplayMessageAccumulator) -> usize { if self.pending.is_some() && !ledger.rows.has_reserved_slot() { 0 } else { usize::from(self.outcome.as_ref().is_some_and(|outcome| self.index < self.original_len || !outcome.messages.is_empty()) || self.pending.is_some()) * size_of::<MutationMessage>() } }
    pub fn next_release_byte_demand(&self) -> usize { self.outcome.as_ref().map_or(0, |outcome| if self.index == self.original_len && outcome.messages.is_empty() && self.pending.is_none() { outcome.messages.capacity() * size_of::<MutationMessage>() } else { 0 }) }
    pub fn step(&mut self, ledger: &mut ReplayMessageAccumulator, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let idle = RetainedCloneProgress::default();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || self.outcome.is_none() { return Ok(idle); }
        if self.pending.is_some() { return ledger.append(&mut self.pending, grant); }
        let outcome = self.outcome.as_mut().expect("operation owns original diagnostic vector");
        if self.index < self.original_len {
            if grant.maximum_copy_bytes < size_of::<MutationMessage>() { return Ok(idle); }
            let empty = MutationMessage { level: outcome.messages[self.index].level, code: String::new().into(), message: String::new(), target: Vec::new(), op_index: None };
            let mut message = std::mem::replace(&mut outcome.messages[self.index], empty);
            message.op_index = Some(outcome.op_index);
            outcome.worst = outcome.worst.max(Some(message.level));
            self.pending = Some(message);
            self.index += 1;
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle });
        }
        if !outcome.messages.is_empty() {
            if grant.maximum_copy_bytes < size_of::<MutationMessage>() { return Ok(idle); }
            drop(outcome.messages.pop());
            return Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle });
        }
        let released = outcome.messages.capacity() * size_of::<MutationMessage>();
        if grant.maximum_release_bytes < released { return Ok(idle); }
        drop(std::mem::take(&mut outcome.messages));
        Ok(RetainedCloneProgress { copied_items: usize::from(released > 0), released_bytes: released, ..idle })
    }
    pub fn is_finished(&self) -> bool { self.pending.is_none() && self.index == self.original_len && self.outcome.as_ref().is_some_and(|outcome| outcome.messages.is_empty() && outcome.messages.capacity() == 0) }
    pub fn take(&mut self) -> Option<MutationReplayOutcome> { if self.is_finished() { self.outcome.take() } else { None } }
    pub fn into_parts(mut self) -> (MutationReplayOutcome, Option<MutationMessage>) { (self.outcome.take().expect("original operation diagnostic owner"), self.pending.take()) }
}

impl Drop for OperationMessageDrain {
    fn drop(&mut self) { assert!(std::thread::panicking() || (self.outcome.is_none() && self.pending.is_none()), "operation diagnostic ownership must transfer before drop"); if self.outcome.is_none() { unsafe { ManuallyDrop::drop(&mut self.outcome); } } }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
