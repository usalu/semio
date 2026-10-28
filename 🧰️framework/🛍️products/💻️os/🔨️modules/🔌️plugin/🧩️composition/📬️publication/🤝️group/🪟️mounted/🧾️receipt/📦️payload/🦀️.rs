//! 📦️ Causal receipt octets enter actual canonical pages and retire one bounded payload or physical backing event.
use super::*;
pub type MountedCausalPayload = semio_framework_value::paged::PagedBytes<{ isize::MAX as usize }>;
pub fn payload_demands(bytes: &MountedCausalPayload) -> Result<RetirementDemand, ValueError> {
    let owner = bytes.retained_bytes();
    if !bytes.is_empty() {
        return Ok(RetirementDemand { copy_bytes: bytes.len().min(64), depth: owner.next_get_depth_demand(bytes.len() - 1).map_err(ValueError::from)?, ..Default::default() });
    }
    if owner.terminal_is_empty() {
        return Ok(Default::default());
    }
    Ok(RetirementDemand { release_bytes: owner.next_release_allocation_bytes().map_err(ValueError::from)?, depth: owner.next_release_depth_demand().map_err(ValueError::from)?, ..Default::default() })
}
pub fn payload_close(bytes: &mut MountedCausalPayload, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
    let d = payload_demands(bytes)?;
    if !plugin_grant_funds(d, grant) {
        return Ok(RetainedCloneStep::Progress(Default::default()));
    }
    if bytes.retained_bytes().terminal_is_empty() {
        return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
    }
    let p = if !bytes.is_empty() {
        let count = d.copy_bytes;
        for _ in 0..count {
            bytes.retained_bytes_mut().pop();
        }
        RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() }
    } else {
        let p = bytes.retained_bytes_mut().release_empty_page(grant.maximum_release_bytes).map_err(ValueError::from)?;
        RetainedCloneProgress { copied_items: usize::from(p.progressed), released_bytes: p.released_allocation_bytes, ..Default::default() }
    };
    Ok(if bytes.retained_bytes().terminal_is_empty() { RetainedCloneStep::Complete(p) } else { RetainedCloneStep::Progress(p) })
}
pub struct MountedReceiptPayload {
    pub bytes: ManuallyDrop<MountedCausalPayload>,
    closed: bool,
    closing: bool,
}
impl MountedReceiptPayload {
    pub fn new() -> Self {
        Self { bytes: ManuallyDrop::new(Default::default()), closed: false, closing: false }
    }
    pub fn ready_for(&self, _additional: usize) -> bool {
        !self.closed && !self.closing && self.bytes.retained_bytes().has_reserved_slot()
    }
    pub fn next_capacity_byte_demand(&self, additional: usize) -> Result<usize, ValueError> {
        if self.closed || self.closing {
            return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical receipt recipient is closed"));
        }
        if self.bytes.len().checked_add(additional).is_none_or(|len| len > MOUNTED_RECEIPT_MAXIMUM_BYTES) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "canonical receipt payload exceeds its original byte authority"));
        }
        Ok(self.bytes.byte_backing_demand()?.capacity_bytes)
    }
    pub fn reserve_step(&mut self, additional: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.next_capacity_byte_demand(additional)?;
        if self.ready_for(additional) {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        Ok(RetainedCloneStep::Progress(self.bytes.reserve_byte_backing(grant)?))
    }
    pub fn append_step(&mut self, source: &[u8], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        self.next_capacity_byte_demand(source.len())?;
        if grant.maximum_items == 0 || grant.maximum_depth == 0 {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let count = source.len().min(64).min(grant.maximum_copy_bytes).min(self.bytes.retained_bytes().capacity() - self.bytes.len());
        for byte in &source[..count] {
            self.bytes.push_reserved_byte(*byte)?;
        }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: usize::from(count > 0), copied_bytes: count, ..Default::default() }))
    }
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Option<MountedCausalPayload> {
        if self.closed || self.closing || grant.maximum_items == 0 {
            return None;
        }
        self.closed = true;
        Some(std::mem::take(&mut *self.bytes))
    }
    pub fn retirement_demands(&self) -> Result<RetirementDemand, ValueError> {
        let mut d = payload_demands(&self.bytes)?;
        if !self.terminal_is_empty() {
            d.depth = d.depth.max(1);
        }
        Ok(d)
    }
    pub fn next_close_byte_demand(&self) -> usize {
        self.retirement_demands().expect("canonical receipt retains valid page layout").release_bytes
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        let d = self.retirement_demands().expect("canonical receipt retains valid page layout");
        if !plugin_grant_funds(d, grant) {
            return RetainedCloneStep::Progress(Default::default());
        }
        self.closing = true;
        let step = payload_close(&mut self.bytes, grant).expect("quoted original canonical receipt page remains retained");
        if self.bytes.retained_bytes().terminal_is_empty() {
            self.closed = true;
        }
        if step.progress() == Default::default() { RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }) } else { step }
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.closed && self.bytes.retained_bytes().terminal_is_empty()
    }
}
impl Drop for MountedReceiptPayload {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "canonical receipt recipient retains every original page until handoff or paid closure")
    }
}
