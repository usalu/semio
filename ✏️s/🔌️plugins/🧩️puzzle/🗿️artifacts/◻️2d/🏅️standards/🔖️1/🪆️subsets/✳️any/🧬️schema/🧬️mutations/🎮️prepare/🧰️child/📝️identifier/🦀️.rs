//! 📝️ Builds one literal identifier in resident storage, borrowing each original UTF-8 chunk.

use crate::standards::v1::subsets::any::schema::preparation::native_preparation_child::pay;
use semio_framework_value::native_decoding::NativeOwnedStorage;
use semio_framework_value::paged::{PagedUtf8, PagedUtf8AppendCursor};
use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep, retained_clone_grant_admits};
use semio_framework_value::{ErasedSnapshotRetirement, RetainedAllocationEffect, RetainedWorkPoll, RetirementDemand, ValueError};
use semio_framework_value_persistent::list::PagedList;

pub type Text = PagedUtf8<{ usize::MAX }>;

pub struct IdentifierCopy {
    output: NativeOwnedStorage<Text>,
    append: Option<PagedUtf8AppendCursor>,
    chunk: usize,
    append_closing: bool,
}

impl Default for IdentifierCopy {
    fn default() -> Self {
        Self {
            output: NativeOwnedStorage::empty(), append: None, chunk: 0, append_closing: false,
        }
    }
}

impl IdentifierCopy {
    pub fn advance_demand(&self, source: &Text, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.output.reserved() { return Ok(NativeOwnedStorage::<Text>::reserve_demand()); }
        let Some(output) = self.output.get() else { return Ok(NativeOwnedStorage::<Text>::seed_demand()); };
        let Some(chunk) = source.retained_chunks().get(self.chunk) else { return Ok(Default::default()); };
        match self.append.as_ref() {
            None => Ok(copy(size_of::<Option<PagedUtf8AppendCursor>>())),
            Some(append) if append.complete => self.append_close_demand(body),
            Some(append) => append.next_advance_demand(chunk, output, body),
        }
    }

    pub fn advance(&mut self, source: &Text, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.advance_demand(source, body)?;
        if !retained_clone_grant_admits(effect.grant(), demand) { return Ok(pending()); }
        if !self.output.reserved() { return step(self.output.reserve_empty(effect)); }
        if !self.output.initialized() {
            return step(self.output.initialize_seed(effect, || {
                let mut output = Text::default();
                output.chunks = PagedList::with_page_items(16);
                Ok(output)
            }));
        }
        let Some(chunk) = source.retained_chunks().get(self.chunk) else {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        };
        let Some(append) = self.append.as_mut() else {
            if !pay(effect)? { return Ok(pending()); }
            self.append = Some(PagedUtf8AppendCursor::default());
            return Ok(paid(demand));
        };
        if append.complete { return self.close_append(body, effect, true); }
        let output = self.output.get_mut().ok_or_else(|| ValueError::invariant("identifier output backing is absent"))?;
        append.advance_funded(chunk, output, body, effect).result.map(|step| RetainedCloneStep::Progress(step.progress()))
    }

    pub fn take(&mut self, output: &mut Option<Text>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        self.output.take_into(output, effect).result
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.append.is_none() && self.output.terminal_is_empty()
    }

    fn append_close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        let append = self.append.as_ref().ok_or_else(|| ValueError::invariant("identifier chunk cursor is absent"))?;
        if append.terminal_is_empty() { return Ok(copy(size_of::<Option<PagedUtf8AppendCursor>>() + size_of::<usize>() + size_of::<bool>())); }
        if !self.append_closing { return Ok(copy(3 * size_of::<bool>())); }
        let demand = append.next_demand(body)?;
        if demand.copy_bytes == 0 && demand.capacity_bytes == 0 && demand.release_bytes == 0 {
            return Ok(copy(2 * size_of::<Option<(usize, usize)>>() + size_of::<Option<&str>>()));
        }
        Ok(demand)
    }

    fn close_append(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>, advance: bool) -> Result<RetainedCloneStep, ValueError> {
        let demand = self.append_close_demand(body)?;
        if !retained_clone_grant_admits(effect.grant(), demand) { return Ok(pending()); }
        let append = self.append.as_mut().ok_or_else(|| ValueError::invariant("identifier chunk cursor is absent"))?;
        if append.terminal_is_empty() {
            if !pay(effect)? { return Ok(pending()); }
            self.append = None;
            self.append_closing = false;
            self.chunk += usize::from(advance);
            return Ok(paid(demand));
        }
        if !self.append_closing {
            if !pay(effect)? { return Ok(pending()); }
            append.begin_close();
            self.append_closing = true;
            return Ok(paid(demand));
        }
        let native_demand = append.next_demand(body)?;
        let work = append.next_computational_work(body)?;
        if effect.remaining_work_units()? < work { return Ok(pending()); }
        let before = effect.performed_boundary();
        let poll = append.close_step(body, effect);
        let RetainedWorkPoll::Complete(turn) = poll else { return Ok(pending()); };
        let observed = effect.performed_since(before)?;
        if turn.work > observed {
            let remainder = u64::try_from(turn.work - observed).map_err(|_| ValueError::invariant("identifier close work exceeds its quote"))?;
            if !effect.performed(remainder)? { return Err(ValueError::invariant("identifier close exceeded its original work")); }
        }
        let step = turn.result.map_err(|failure| failure.into_value_error())?;
        if native_demand.copy_bytes == 0 && native_demand.capacity_bytes == 0 && native_demand.release_bytes == 0
            && matches!(step, RetainedCloneStep::Complete(_))
        {
            Ok(paid(demand))
        } else {
            Ok(RetainedCloneStep::Progress(step.progress()))
        }
    }

    pub fn close_demand(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.append.is_some() { return self.append_close_demand(body); }
        if self.output.initialized() { return Ok(NativeOwnedStorage::<Text>::move_demand()); }
        Ok(self.output.release_demand())
    }

    pub fn close(&mut self, unused: &mut Option<Text>, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> {
        let body = effect.grant().maximum_copy_bytes;
        if self.append.is_some() { return self.close_append(body, effect, false); }
        if self.output.initialized() { return self.output.take_into(unused, effect).result; }
        self.output.release(effect).result
    }

    #[cfg(feature = "testkit")]
    pub fn current_chunk_body_bytes(&self) -> usize { self.append.as_ref().map_or(0, |append| append.position) }
}

fn step(poll: RetainedWorkPoll<RetainedCloneStep, ValueError>) -> Result<RetainedCloneStep, ValueError> {
    match poll {
        RetainedWorkPoll::Complete(turn) => turn.result.map(|step| RetainedCloneStep::Progress(step.progress())),
        RetainedWorkPoll::Pending { .. } => Ok(pending()),
    }
}

fn copy(bytes: usize) -> RetirementDemand {
    RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() }
}

fn paid(demand: RetirementDemand) -> RetainedCloneStep {
    RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
}

fn pending() -> RetainedCloneStep { RetainedCloneStep::Progress(Default::default()) }
