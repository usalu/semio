//! ⏱️ One admitted operation ledger remains owned across bounded step contexts.
use super::{JobPayloadOperationLedger, StepContext, StepBudget, CancelToken, ClockStride, OperationId, Generation, InteractiveJobCloseStep,RetainedCloneGrant,RetainedCloneProgress,ValueError,ValueRefusalKind};
use std::{mem::ManuallyDrop, sync::Arc};

pub struct StepContextOwner {
    ledger: ManuallyDrop<Option<Arc<JobPayloadOperationLedger>>>,
    closing: bool,
}

impl StepContextOwner {
    pub fn birth_bytes() -> usize {
        std::alloc::Layout::new::<[usize; 2]>().extend(std::alloc::Layout::new::<JobPayloadOperationLedger>()).expect("operation ledger layout").0.pad_to_align().size()
    }

    pub fn new(operation:OperationId,generation:Generation,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),ValueError>{
        if grant.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"context ledger birth requires one admitted item"));}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"context ledger birth requires original depth"));}
        if grant.maximum_capacity_bytes<Self::birth_bytes(){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"context ledger birth exceeds original capacity"));}
        Ok((Self{ledger:ManuallyDrop::new(Some(Arc::new(JobPayloadOperationLedger::new(operation,generation)))),closing:false},RetainedCloneProgress{copied_items:1,retained_capacity_bytes:Self::birth_bytes(),..Default::default()}))
    }

    pub fn context<'a>(&self, budget: StepBudget, cancel: CancelToken, now_us: fn() -> Option<u64>, preview_sequence: &'a mut u64,retained_progress:&'a mut RetainedCloneProgress) -> Option<StepContext<'a>> {
        if self.closing { return None; }
        let ledger = self.ledger.as_ref()?;
        Some(StepContext::with_payload_ledger(ledger.operation, ledger.generation, budget, cancel, now_us, ClockStride::new(), preview_sequence, Arc::clone(ledger),retained_progress))
    }

    pub fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(0)}
    pub fn next_close_capacity_byte_demand(&self,_maximum_copy_bytes:usize)->Result<usize,ValueError>{Ok(0)}
    pub fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(if self.ledger.is_some(){Self::birth_bytes()}else{0})}
    pub fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(usize::from(self.ledger.is_some()))}

    pub fn close_step(&mut self, grant:RetainedCloneGrant) -> InteractiveJobCloseStep {
        let Some(ledger) = self.ledger.as_ref() else { return InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress::default()}; };
        if grant.maximum_items==0||grant.maximum_release_bytes<Self::birth_bytes()||grant.maximum_depth==0 { return InteractiveJobCloseStep::Pending{progress:RetainedCloneProgress::default()}; }
        self.closing = true;
        if !ledger.terminal_is_empty() { return InteractiveJobCloseStep::Blocked; }
        match Arc::try_unwrap(self.ledger.take().unwrap()) {
            Ok(ledger) => {
                debug_assert!(ledger.terminal_is_empty());
                InteractiveJobCloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,released_bytes:Self::birth_bytes(),..RetainedCloneProgress::default()}}
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
