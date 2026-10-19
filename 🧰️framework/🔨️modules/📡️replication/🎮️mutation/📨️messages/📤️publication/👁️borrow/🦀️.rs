//! 👁️ Builds two diagnostic owners from an immutable committed source through exact per-row copying.
use super::*;

pub struct MutationMessageBorrowedPublication {
    source: Option<(usize, usize)>,
    rows: ManuallyDrop<Vec<MutationMessage>>,
    active: Option<MessageCopyCursor>,
    publication: Option<MutationMessagePublication>,
    retirement: Option<MutationMessageLedgerRetirement>,
    index: usize,
    phase: u8,
    closing: bool,
}

impl Default for MutationMessageBorrowedPublication { fn default() -> Self { Self { source: None, rows: ManuallyDrop::new(Vec::new()), active: None, publication: None, retirement: None, index: 0, phase: 0, closing: false } } }

impl MutationMessageBorrowedPublication {
    fn validate(&self, source: &[MutationMessage]) -> Result<(), ValueError> { if self.source.is_some_and(|identity| identity != (source.as_ptr() as usize, source.len())) { Err(fault("borrowed diagnostic source changed its original identity")) } else { Ok(()) } }
    pub fn ready(&self) -> bool { !self.closing && self.phase == 3 && self.publication.as_ref().is_some_and(MutationMessagePublication::ready) }
    pub fn demands(&self, source: &[MutationMessage]) -> Result<RetirementDemand, ValueError> {
        self.validate(source)?;
        if self.closing { return self.close_demands(); }
        if self.phase == 0 { return Ok(RetirementDemand { capacity_bytes: source.len().checked_mul(size_of::<MutationMessage>()).ok_or_else(|| fault("borrowed diagnostic row layout overflow"))?, depth: 1, ..Default::default() }); }
        if let Some(owner) = self.publication.as_ref() { let mut demand = owner.demands()?; demand.depth += 1; return Ok(demand); }
        if let Some(cursor) = self.active.as_ref() {
            if cursor.ready() { return Ok(RetirementDemand { copy_bytes: size_of::<MutationMessage>(), depth: 2, ..Default::default() }); }
            if self.phase == 2 { return Ok(RetirementDemand { copy_bytes: cursor.next_close_copy_byte_demand()?, release_bytes: cursor.next_close_release_byte_demand()?, depth: cursor.next_close_depth_demand()?.saturating_add(1), ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: cursor.next_copy_byte_demand(&source[self.index])?, capacity_bytes: cursor.next_capacity_byte_demand(&source[self.index])?, depth: 2, ..Default::default() });
        }
        Ok(RetirementDemand { copy_bytes: if self.index == source.len() { size_of::<MutationMessagePublication>() } else { size_of::<MessageCopyCursor>() }, depth: 1, ..Default::default() })
    }
    pub fn advance(&mut self, source: &[MutationMessage], grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        self.validate(source)?;
        if self.ready() { return Ok(RetainedCloneStep::Complete(idle)); }
        if self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.demands(source)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        self.source = Some((source.as_ptr() as usize, source.len()));
        if self.phase == 0 { self.rows.try_reserve_exact(source.len()).map_err(|_| fault("borrowed diagnostic row allocation failed"))?; self.phase = 1; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: self.rows.capacity() * size_of::<MutationMessage>(), ..idle })); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(owner) = self.publication.as_mut() { let step = owner.advance(child)?; if owner.ready() { self.phase = 3; } return Ok(if self.ready() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) }); }
        if let Some(cursor) = self.active.as_mut() {
            if cursor.ready() { self.rows.push(cursor.take().ok_or_else(|| fault("borrowed diagnostic ready row disappeared"))?); cursor.begin_close(); self.phase = 2; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle })); }
            if self.phase == 2 { let step = cursor.close_step(child)?; if cursor.terminal_is_empty() { self.active = None; self.index += 1; self.phase = 1; } return Ok(RetainedCloneStep::Progress(step.progress())); }
            return cursor.advance(&source[self.index], child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        if self.index == source.len() { self.publication = Some(MutationMessagePublication::new(std::mem::take(&mut *self.rows))); }
        else { self.active = Some(MessageCopyCursor::new()); }
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle }))
    }
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Option<(MutationMessagePublication, RetainedCloneProgress)> {
        let copy = size_of::<MutationMessagePublication>();
        if !self.ready() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return None; }
        self.closing = true; self.source = None;
        Some((self.publication.take().expect("admitted borrowed diagnostic publication"), RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() }))
    }
    pub fn begin_close(&mut self) { self.closing = true; if let Some(cursor) = self.active.as_mut() { cursor.cancel(); cursor.begin_close(); } if let Some(owner) = self.publication.as_mut() { owner.begin_close(); } }
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.active.is_none() && self.publication.is_none() && self.retirement.is_none() && self.rows.capacity() == 0 }
    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(cursor) = self.active.as_ref() { return Ok(RetirementDemand { copy_bytes: cursor.next_close_copy_byte_demand()?, release_bytes: cursor.next_close_release_byte_demand()?, depth: cursor.next_close_depth_demand()?.saturating_add(1), ..Default::default() }); }
        if let Some(owner) = self.publication.as_ref() { let mut demand = owner.close_demands()?; demand.depth += 1; return Ok(demand); }
        if let Some(owner) = self.retirement.as_ref() { return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand(), release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.saturating_add(1), ..Default::default() }); }
        Ok(RetirementDemand { copy_bytes: usize::from(self.rows.capacity() != 0) * size_of::<MutationMessageLedgerRetirement>(), depth: 1, ..Default::default() })
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if !self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(cursor) = self.active.as_mut() { let step = cursor.close_step(child)?; if cursor.terminal_is_empty() { self.active = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.publication.as_mut() { let step = owner.close_step(child)?; if owner.terminal_is_empty() { self.publication = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.retirement.as_mut() { let step = owner.close_step(child)?; if owner.terminal_is_empty() { self.retirement = None; } return Ok(RetainedCloneStep::Progress(step.progress())); }
        if self.rows.capacity() != 0 { self.retirement = Some(MutationMessageLedgerRetirement::new(String::new(), std::mem::take(&mut *self.rows))); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        self.source = None; Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..idle }))
    }
}

impl ErasedSnapshotRetirement for MutationMessageBorrowedPublication {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> { self.close_demands() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.depth) }
}
impl Drop for MutationMessageBorrowedPublication { fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "borrowed diagnostic publication requires terminal close"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.rows); } } } }
