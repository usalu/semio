//! 🪢️ Concatenates original diagnostic bundles and their funded copies without cloning payloads.
use super::*;

struct Input { original: ManuallyDrop<Vec<MutationMessage>>, copied: ManuallyDrop<Vec<MutationMessage>>, index: usize }

/// 🧳️ Every source, moved row, destination and unused vector scaffold stays in exact custody.
pub struct MutationMessagePublicationJoin {
    source: Option<MutationMessagePublicationBatch>,
    original: ManuallyDrop<Vec<MutationMessage>>,
    copied: ManuallyDrop<Vec<MutationMessage>>,
    input: Option<Input>,
    retiring: Option<MutationMessageLedgerRetirement>,
    count: usize,
    source_index: usize,
    phase: u8,
    closing: bool,
}

impl MutationMessagePublicationJoin {
    pub fn new(source: MutationMessagePublicationBatch, count: usize) -> Self { Self { source: Some(source), original: ManuallyDrop::new(Vec::new()), copied: ManuallyDrop::new(Vec::new()), input: None, retiring: None, count, source_index: 0, phase: 0, closing: false } }
    pub fn ready(&self) -> bool { !self.closing && self.phase == 9 && self.source.is_none() && self.input.is_none() }

    /// 📏️ Quotes the next allocation, original row move or scaffold release independently.
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.ready() { return Ok(Default::default()); }
        let base = RetirementDemand { depth: 1, ..Default::default() };
        if self.phase < 2 { return Ok(RetirementDemand { capacity_bytes: self.count.checked_mul(size_of::<MutationMessage>()).ok_or_else(|| fault("joined diagnostic vector extent overflow"))?, ..base }); }
        if self.phase == 2 {
            let source = self.source.as_ref().ok_or_else(|| fault("diagnostic join source is absent"))?;
            return Ok(if self.source_index == source.items.len() { base } else { RetirementDemand { copy_bytes: 2 * size_of::<Vec<MutationMessage>>(), ..base } });
        }
        if self.phase == 8 {
            let mut demand = self.source.as_ref().ok_or_else(|| fault("diagnostic join close source is absent"))?.close_demands()?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| fault("diagnostic join source depth overflow"))?;
            return Ok(demand);
        }
        let input = self.input.as_ref().ok_or_else(|| fault("diagnostic join original input is absent"))?;
        let rows = if self.phase == 3 || self.phase == 5 || self.phase == 6 { &*input.original } else { &*input.copied };
        if self.phase == 3 || self.phase == 4 { return Ok(RetirementDemand { copy_bytes: usize::from(input.index < rows.len()) * size_of::<MutationMessage>(), ..base }); }
        if rows.is_empty() { return Ok(RetirementDemand { release_bytes: rows.capacity() * size_of::<MutationMessage>(), ..base }); }
        Ok(RetirementDemand { copy_bytes: size_of::<MutationMessage>(), ..base })
    }

    pub fn advance(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.ready() { return Ok(RetainedCloneStep::Complete(idle)); }
        if self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.demands()?;
        if !fits(demand, grant) { return Ok(RetainedCloneStep::Progress(idle)); }
        let mut receipt = RetainedCloneProgress { copied_items: 1, ..idle };
        match self.phase {
            0 | 1 => {
                let target = if self.phase == 0 { &mut *self.original } else { &mut *self.copied };
                target.try_reserve_exact(self.count).map_err(|_| fault("joined diagnostic vector allocation failed"))?;
                receipt.retained_capacity_bytes = target.capacity() * size_of::<MutationMessage>();
                self.phase += 1;
            }
            2 => {
                let source = self.source.as_mut().ok_or_else(|| fault("diagnostic join source is absent"))?;
                if self.source_index == source.items.len() {
                    if self.original.len() != self.count || self.copied.len() != self.count { return Err(fault("diagnostic join disagrees with its captured row census")); }
                    source.begin_close(); self.phase = 8;
                } else {
                    let (original, copied, progress) = source.items[self.source_index].take(grant)?.ok_or_else(|| fault("admitted diagnostic input transfer blocked"))?;
                    self.input = Some(Input { original: ManuallyDrop::new(original), copied: ManuallyDrop::new(copied), index: 0 });
                    receipt = progress; self.phase = 3;
                }
            }
            3 | 4 => {
                let input = self.input.as_mut().ok_or_else(|| fault("diagnostic join input is absent"))?;
                let (rows, target) = if self.phase == 3 { (&mut *input.original, &mut *self.original) } else { (&mut *input.copied, &mut *self.copied) };
                if input.index == rows.len() { input.index = 0; self.phase += 1; }
                else {
                    if target.len() == target.capacity() { return Err(fault("diagnostic row exceeds captured destination capacity")); }
                    let row = std::mem::replace(&mut rows[input.index], empty_row());
                    target.push(row); input.index += 1; receipt.copied_bytes = size_of::<MutationMessage>();
                }
            }
            5 | 6 | 7 => {
                let input = self.input.as_mut().ok_or_else(|| fault("diagnostic join source scaffold is absent"))?;
                let rows = if self.phase == 5 { &mut *input.original } else { &mut *input.copied };
                if rows.pop().is_some() { receipt.copied_bytes = size_of::<MutationMessage>(); }
                else {
                    receipt.released_bytes = rows.capacity() * size_of::<MutationMessage>(); drop(std::mem::take(rows));
                    if self.phase == 5 { self.phase = 7; } else { self.input = None; self.source_index += 1; self.phase = 2; }
                }
            }
            8 => {
                let source = self.source.as_mut().ok_or_else(|| fault("diagnostic join closing source is absent"))?;
                let step = source.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
                receipt = step.progress();
                if source.terminal_is_empty() { self.source = None; self.phase = 9; }
            }
            _ => return Err(fault("diagnostic join already completed")),
        }
        Ok(if self.ready() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) })
    }

    /// 📤️ Transfers the already joined vectors as one ready publication owner.
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<Option<(MutationMessagePublication, RetainedCloneProgress)>, ValueError> {
        if self.closing { return Err(fault("diagnostic join transfer follows closure")); }
        let copy = size_of::<MutationMessagePublication>();
        if !self.ready() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        let owner = MutationMessagePublication::from_copied_rows(std::mem::take(&mut *self.original), std::mem::take(&mut *self.copied));
        self.closing = true;
        Ok(Some((owner, RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })))
    }

    pub fn begin_close(&mut self) { self.closing = true; if let Some(source) = self.source.as_mut() { source.begin_close(); } }
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.input.is_none() && self.retiring.is_none() && self.original.capacity() == 0 && self.copied.capacity() == 0 }

    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.retiring.as_ref() { return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand(), release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.saturating_add(1), ..Default::default() }); }
        let input_owns = self.input.as_ref().is_some_and(|input| input.original.capacity() != 0 || input.copied.capacity() != 0);
        if input_owns || self.original.capacity() != 0 || self.copied.capacity() != 0 { return Ok(RetirementDemand { copy_bytes: size_of::<MutationMessageLedgerRetirement>(), depth: 1, ..Default::default() }); }
        if self.input.is_some() { return Ok(RetirementDemand { copy_bytes: size_of::<Input>(), depth: 1, ..Default::default() }); }
        if let Some(source) = self.source.as_ref() {
            let mut demand = source.close_demands()?; demand.depth = demand.depth.checked_add(1).ok_or_else(|| fault("diagnostic join close depth overflow"))?; return Ok(demand);
        }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if !self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.close_demands()?;
        if !fits(demand, grant) { return Ok(RetainedCloneStep::Progress(idle)); }
        if let Some(owner) = self.retiring.as_mut() {
            let step = owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
            if owner.terminal_is_empty() { self.retiring = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let rows = if let Some(input) = self.input.as_mut().filter(|input| input.original.capacity() != 0 || input.copied.capacity() != 0) { Some(if input.original.capacity() != 0 { &mut *input.original } else { &mut *input.copied }) }
            else if self.original.capacity() != 0 { Some(&mut *self.original) } else if self.copied.capacity() != 0 { Some(&mut *self.copied) } else { None };
        if let Some(rows) = rows { self.retiring = Some(MutationMessageLedgerRetirement::new(String::new(), std::mem::take(rows))); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        if self.input.is_some() { self.input = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        if let Some(source) = self.source.as_mut() {
            let step = source.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
            if source.terminal_is_empty() { self.source = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(idle))
    }
}

fn fits(demand: RetirementDemand, grant: RetainedCloneGrant) -> bool { grant.maximum_items != 0 && grant.maximum_depth >= demand.depth && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes }
fn empty_row() -> MutationMessage { MutationMessage { level: semio_framework_diagnostic::Severity::Info, code: String::new().into(), message: String::new(), target: Vec::new(), op_index: None } }

impl ErasedSnapshotRetirement for MutationMessagePublicationJoin {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> { self.close_demands() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.depth) }
}

impl Drop for MutationMessagePublicationJoin {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "diagnostic join requires exact terminal closure"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.original); ManuallyDrop::drop(&mut self.copied); } } }
}
