//! ⏱️ One admitted operation ledger remains owned across bounded step contexts.
use super::{JobPayloadOperationLedger, StepContext, StepBudget, CancelToken, ClockStride, OperationId, Generation, InteractiveJobCloseStep};
use std::{mem::ManuallyDrop, sync::Arc};

pub struct StepContextOwner {
    ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
    closing: bool,
}

impl StepContextOwner {
    pub fn birth_bytes() -> usize {
        std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<JobPayloadOperationLedger>()).expect("operation ledger layout").0.pad_to_align().size()
    }

    pub fn new(operation: OperationId, generation: Generation, maximum_items: usize, maximum_bytes: usize) -> Option<Self> {
        if maximum_items == 0 || maximum_bytes < Self::birth_bytes() { return None; }
        Some(Self { ledger: ManuallyDrop::new(Some(Arc::new(JobPayloadOperationLedger::new(operation, generation)))), closing: false })
    }

    pub fn context<'a>(&self, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64) -> Option<StepContext<'a>> {
        if self.closing { return None; }
        let ledger = self.ledger.as_ref()?;
        Some(StepContext::with_payload_ledger(ledger.operation, ledger.generation, budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::clone(ledger)))
    }

    pub fn next_close_byte_demand(&self) -> usize {
        if self.ledger.is_some() { Self::birth_bytes() } else { 0 }
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> InteractiveJobCloseStep {
        let Some(ledger) = self.ledger.as_ref() else { return InteractiveJobCloseStep::Complete; };
        if maximum_items == 0 || maximum_bytes < Self::birth_bytes() { return InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 }; }
        self.closing = true;
        if !ledger.terminal_is_empty() { return InteractiveJobCloseStep::Blocked; }
        match Arc::try_unwrap(self.ledger.take().unwrap()) {
            Ok(ledger) => {
                debug_assert!(ledger.terminal_is_empty());
                InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: Self::birth_bytes() }
            }
            Err(ledger) => {
                *self.ledger = Some(ledger);
                InteractiveJobCloseStep::Blocked
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool { self.ledger.is_none() }
}

impl Drop for StepContextOwner {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "step context owner dropped before its original ledger was returned"); }
}
