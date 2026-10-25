//! 📤️ Original diagnostics and a separately funded report copy remain owned until transfer or cancellation.
use crate::{FinalMessageSelection, MessageCopyCursor, MutationMessage, MutationMessageLedgerRetirement};
use semio_framework_value::{ErasedSnapshotRetirement, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::{ManuallyDrop, size_of};

/// 🧳️ Two independent downstream owners are prepared one original row and string scalar at a time.
pub struct MutationMessagePublication {
    selection: [Option<FinalMessageSelection>;2],
    selection_context:Option<usize>,
    selection_index:usize,
    original: MutationMessageLedgerRetirement,
    copied: ManuallyDrop<Vec<MutationMessage>>,
    copy_retirement: Option<MutationMessageLedgerRetirement>,
    active: Option<MessageCopyCursor>,
    index: usize,
    reserved: bool,
    closing: bool,
    transferred: bool,
}

impl MutationMessagePublication {
    fn from_copied_rows(original: Vec<MutationMessage>, copied: Vec<MutationMessage>) -> Self {
        assert_eq!(original.len(), copied.len(), "joined diagnostic owners preserve row correspondence");
        let index = original.len();
        Self { selection:Default::default(),selection_context:None,selection_index:0,original: MutationMessageLedgerRetirement::new(String::new(), original), copied: ManuallyDrop::new(copied), copy_retirement: None, active: None, index, reserved: true, closing: false, transferred: false }
    }

    /// 🌱️ Captures the original vector without traversing or copying its rows.
    pub fn new(messages: Vec<MutationMessage>) -> Self {
        Self { selection:Default::default(),selection_context:None,selection_index:0,original: MutationMessageLedgerRetirement::new(String::new(), messages), copied: ManuallyDrop::new(Vec::new()), copy_retirement: None, active: None, index: 0, reserved: false, closing: false, transferred: false }
    }

    /// 👁️ Keeps the original diagnostic identity, target and severity available during preparation.
    pub fn original_messages(&self) -> &[MutationMessage] { self.original.retained_messages() }
    pub fn copied_messages(&self) -> &[MutationMessage] { &self.copied }
    pub fn ready(&self) -> bool { !self.closing && !self.transferred && self.selection.iter().all(Option::is_none) && self.reserved && self.active.is_none() && self.index == self.original_messages().len() }

    /// ✂️ Selects both independent owners with the same stable policy before publishing actual edit context.
    pub fn selection_ready(&self,edit_id_bytes:usize)->bool{self.ready()&&self.selection_index==2&&self.selection_context==Some(edit_id_bytes)}
    pub fn selection_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.selection_index==2{return Ok(Default::default());}
        let Some(cursor)=self.selection[self.selection_index].as_ref() else{return Ok(RetirementDemand{copy_bytes:2*size_of::<FinalMessageSelection>()+size_of::<Option<usize>>(),depth:1,..Default::default()});};
        if cursor.is_finished(){return Ok(RetirementDemand{copy_bytes:2*size_of::<Vec<MutationMessage>>()+size_of::<FinalMessageSelection>()+size_of::<usize>(),depth:1,..Default::default()});}
        let source=if self.selection_index==0{self.original.retained_message_rows()}else{&self.copied};
        Ok(RetirementDemand{copy_bytes:cursor.next_copy_byte_demand(source),capacity_bytes:cursor.next_capacity_byte_demand(),release_bytes:cursor.next_release_byte_demand(source)?,depth:2})
    }
    pub fn select(&mut self,edit_id_bytes:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let idle=RetainedCloneProgress::default();
        if self.closing||self.transferred{return Err(fault("diagnostic selection follows transfer or closure"));}
        if self.selection_context.is_some_and(|bytes|bytes!=edit_id_bytes){return Err(fault("diagnostic selection edit extent changed"));}
        if self.selection_ready(edit_id_bytes){return Ok(RetainedCloneStep::Complete(idle));}
        if self.selection_context.is_none()&&!self.ready(){return Err(fault("diagnostic selection requires independent ready owners"));}
        let demand=self.selection_demands()?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes{return Ok(RetainedCloneStep::Progress(idle));}
        if self.selection_context.is_none(){let count=self.original_messages().len();self.selection=[Some(FinalMessageSelection::new(edit_id_bytes,count)),Some(FinalMessageSelection::new(edit_id_bytes,count))];self.selection_context=Some(edit_id_bytes);return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}));}
        let cursor=self.selection[self.selection_index].as_mut().ok_or_else(||fault("diagnostic selection original cursor is absent"))?;
        if cursor.is_finished(){let rows=cursor.take().ok_or_else(||fault("diagnostic selected rows are absent"))?;if self.selection_index==0{*self.original.retained_message_rows_mut()=rows;self.index=self.original_messages().len();}else{*self.copied=rows;}self.selection[self.selection_index]=None;self.selection_index+=1;let receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle};return Ok(if self.selection_ready(edit_id_bytes){RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)});}
        let source=if self.selection_index==0{self.original.retained_message_rows_mut()}else{&mut self.copied};
        Ok(RetainedCloneStep::Progress(cursor.step(source,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?))
    }

    /// 🪪️ Rebinds one original and copied row to the actual batch or transaction operation offset.
    pub fn stamp_row(&mut self, index: usize, offset: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if !self.ready() { return Err(fault("diagnostic stamping requires a ready original owner")); }
        let original = self.original_messages().get(index).ok_or_else(|| fault("diagnostic stamp row is absent"))?;
        let copied = self.copied.get(index).ok_or_else(|| fault("diagnostic report stamp row is absent"))?;
        if original.op_index != copied.op_index { return Err(fault("diagnostic original and report indices disagree")); }
        let operation = u32::try_from(offset).ok().and_then(|offset| offset.checked_add(original.op_index.unwrap_or(0))).ok_or_else(|| fault("diagnostic operation index overflow"))?;
        let copy = 2 * size_of::<Option<u32>>();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(Default::default()); }
        self.original.retained_messages_mut()[index].op_index = Some(operation);
        self.copied[index].op_index = Some(operation);
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })
    }

    /// 📏️ Quotes one exact next physical action without allocating or changing source custody.
    pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
        if self.closing || self.transferred || self.ready() { return Ok(Default::default()); }
        if !self.reserved {
            let capacity = self.original_messages().len().checked_mul(size_of::<MutationMessage>()).ok_or_else(|| fault("diagnostic report vector extent overflow"))?;
            return Ok(RetirementDemand { capacity_bytes: capacity, depth: 1, ..Default::default() });
        }
        if let Some(active) = self.active.as_ref() {
            if active.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<Option<MessageCopyCursor>>(), depth: 1, ..Default::default() }); }
            if self.copied.len() > self.index {
                return Ok(RetirementDemand { copy_bytes: active.next_close_copy_byte_demand()?, capacity_bytes: active.next_close_capacity_byte_demand(0)?, release_bytes: active.next_close_release_byte_demand()?, depth: active.next_close_depth_demand()?.saturating_add(1) });
            }
            let source = &self.original_messages()[self.index];
            return Ok(RetirementDemand { copy_bytes: if active.ready() { size_of::<MutationMessage>() } else { active.next_copy_byte_demand(source)? }, capacity_bytes: active.next_capacity_byte_demand(source)?, depth: 2, ..Default::default() });
        }
        Ok(RetirementDemand { copy_bytes: size_of::<MessageCopyCursor>(), depth: 1, ..Default::default() })
    }

    /// 🪡️ Copies one admitted string scalar or moves one complete row under the original turn grant.
    pub fn advance(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.ready() { return Ok(RetainedCloneStep::Complete(idle)); }
        if self.closing || self.transferred { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        if !self.reserved {
            self.copied.try_reserve_exact(self.original.retained_messages().len()).map_err(|_| fault("diagnostic report vector allocation failed"))?;
            self.reserved = true;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: self.copied.capacity() * size_of::<MutationMessage>(), ..idle }));
        }
        if self.active.as_ref().is_some_and(MessageCopyCursor::terminal_is_empty) {
            self.active = None;
            self.index += 1;
            let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle };
            return Ok(if self.ready() { RetainedCloneStep::Complete(receipt) } else { RetainedCloneStep::Progress(receipt) });
        }
        if let Some(active) = self.active.as_mut() {
            let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
            if self.copied.len() > self.index { return active.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
            if let Some(row) = active.take() {
                self.copied.push(row);
                active.begin_close();
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle }));
            }
            return active.advance(&self.original.retained_messages()[self.index], child).map(|step| RetainedCloneStep::Progress(step.progress()));
        }
        self.active = Some(MessageCopyCursor::new());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle }))
    }

    /// 📦️ Moves two independently owned vectors only after the report copy is complete.
    pub fn take(&mut self, grant: RetainedCloneGrant) -> Result<Option<(Vec<MutationMessage>, Vec<MutationMessage>, RetainedCloneProgress)>, ValueError> {
        if self.closing || self.transferred { return Err(fault("diagnostic publication follows closure or prior transfer")); }
        let copy = 2 * size_of::<Vec<MutationMessage>>();
        if !self.ready() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        let (original, _) = self.original.take_messages_controlled(RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes - size_of::<Vec<MutationMessage>>(), ..grant })?.ok_or_else(|| fault("admitted original diagnostic transfer blocked"))?;
        self.transferred = true;
        Ok(Some((original, std::mem::take(&mut *self.copied), RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })))
    }

    /// 🛑️ Closure preserves every partially copied and original payload until its exact grants release it.
    pub fn begin_close(&mut self) { self.closing = true; for cursor in self.selection.iter_mut().flatten(){cursor.begin_close();} if let Some(active) = self.active.as_mut() { active.cancel(); active.begin_close(); } }

    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(cursor)=self.selection.iter().flatten().next(){return Ok(if cursor.terminal_is_empty(){RetirementDemand{copy_bytes:size_of::<FinalMessageSelection>(),depth:1,..Default::default()}}else{RetirementDemand{copy_bytes:cursor.next_close_copy_byte_demand(),release_bytes:cursor.next_close_release_byte_demand(),depth:2,..Default::default()}});}
        if let Some(active) = self.active.as_ref() {
            if active.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<Option<MessageCopyCursor>>(), depth: 1, ..Default::default() }); }
            return Ok(RetirementDemand { copy_bytes: active.next_close_copy_byte_demand()?, capacity_bytes: active.next_close_capacity_byte_demand(0)?, release_bytes: active.next_close_release_byte_demand()?, depth: active.next_close_depth_demand()?.saturating_add(1) });
        }
        if let Some(owner) = self.copy_retirement.as_ref() { return Ok(RetirementDemand { copy_bytes: owner.next_copy_byte_demand(), release_bytes: owner.next_release_byte_demand()?, depth: owner.next_depth_demand()?.saturating_add(1), ..Default::default() }); }
        if self.copied.capacity() != 0 { return Ok(RetirementDemand { copy_bytes: size_of::<MutationMessageLedgerRetirement>(), depth: 1, ..Default::default() }); }
        Ok(RetirementDemand { copy_bytes: self.original.next_copy_byte_demand(), release_bytes: self.original.next_release_byte_demand()?, depth: self.original.next_depth_demand()?.saturating_add(1), ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if !self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        let child = RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant };
        if let Some(index)=self.selection.iter().position(Option::is_some){let cursor=self.selection[index].as_mut().unwrap();if cursor.terminal_is_empty(){self.selection[index]=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..idle}));}return Ok(RetainedCloneStep::Progress(cursor.close_step(child)));}
        if self.active.as_ref().is_some_and(MessageCopyCursor::terminal_is_empty) { self.active = None; return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        if let Some(active) = self.active.as_mut() { return active.close_step(child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(owner) = self.copy_retirement.as_mut() {
            let step = owner.close_step(child)?;
            if owner.terminal_is_empty() { self.copy_retirement = None; }
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        if self.copied.capacity() != 0 { self.copy_retirement = Some(MutationMessageLedgerRetirement::new(String::new(), std::mem::take(&mut *self.copied))); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        self.original.close_step(child).map(|step| if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.selection.iter().all(Option::is_none) && self.active.is_none() && self.copy_retirement.is_none() && self.copied.capacity() == 0 && self.original.terminal_is_empty() }
}

fn fault(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvariantViolated, message) }

impl ErasedSnapshotRetirement for MutationMessagePublication {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> { self.close_demands() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.depth) }
}

impl Drop for MutationMessagePublication {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "diagnostic publication requires exact terminal closure"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.copied); } } }
}

/// 🧺️ A batch retains independently prepared diagnostic owners without flattening or cloning rows.
pub struct MutationMessagePublicationBatch { items: ManuallyDrop<Vec<MutationMessagePublication>>, closing: bool }

impl Default for MutationMessagePublicationBatch { fn default() -> Self { Self { items: ManuallyDrop::new(Vec::new()), closing: false } } }

impl MutationMessagePublicationBatch {
    /// ⏮️ Admits the batch's final owner slots before the first original publication moves.
    pub fn reserve(&mut self, count: usize, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.closing { return Err(fault("diagnostic batch reservation follows closure")); }
        if self.items.capacity() >= count { return Ok(Default::default()); }
        if !self.items.is_empty() { return Err(fault("diagnostic batch reservation follows transfer")); }
        let capacity = count.checked_mul(size_of::<MutationMessagePublication>()).ok_or_else(|| fault("diagnostic batch owner extent overflow"))?;
        let release = self.items.capacity() * size_of::<MutationMessagePublication>();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_capacity_bytes < capacity || grant.maximum_release_bytes < release { return Ok(Default::default()); }
        self.items.try_reserve_exact(count).map_err(|_| fault("diagnostic batch owner allocation failed"))?;
        Ok(RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: self.items.capacity() * size_of::<MutationMessagePublication>(), released_bytes: release, ..Default::default() })
    }

    /// 🎟️ Moves one ready original owner into its already admitted batch slot.
    pub fn append(&mut self, owner: MutationMessagePublication, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, MutationMessagePublication> {
        let copy = size_of::<MutationMessagePublication>();
        if self.closing || !owner.ready() || self.items.len() == self.items.capacity() || grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Err(owner); }
        self.items.push(owner);
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })
    }

    pub fn items(&self) -> &[MutationMessagePublication] { &self.items }
    pub fn rows(&self) -> impl Iterator<Item = &MutationMessage> { self.items.iter().flat_map(|item| item.original_messages().iter()) }
    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn terminal_is_empty(&self) -> bool { self.closing && self.items.is_empty() && self.items.capacity() == 0 }

    pub fn close_demands(&self) -> Result<RetirementDemand, ValueError> {
        if let Some(owner) = self.items.last() {
            if owner.terminal_is_empty() { return Ok(RetirementDemand { copy_bytes: size_of::<MutationMessagePublication>(), depth: 1, ..Default::default() }); }
            let mut demand = owner.close_demands()?;
            demand.depth = demand.depth.checked_add(1).ok_or_else(|| fault("diagnostic batch child depth overflow"))?;
            return Ok(demand);
        }
        Ok(RetirementDemand { release_bytes: self.items.capacity() * size_of::<MutationMessagePublication>(), depth: 1, ..Default::default() })
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if !self.closing { return Ok(RetainedCloneStep::Progress(idle)); }
        let demand = self.close_demands()?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || grant.maximum_copy_bytes < demand.copy_bytes || grant.maximum_capacity_bytes < demand.capacity_bytes || grant.maximum_release_bytes < demand.release_bytes { return Ok(RetainedCloneStep::Progress(idle)); }
        if self.items.last().is_some_and(MutationMessagePublication::terminal_is_empty) { self.items.pop(); return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..idle })); }
        if let Some(owner) = self.items.last_mut() { owner.begin_close(); return owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }).map(|step| RetainedCloneStep::Progress(step.progress())); }
        drop(std::mem::take(&mut *self.items));
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..idle }))
    }
}

impl ErasedSnapshotRetirement for MutationMessagePublicationBatch {
    fn next_demand(&self, _: usize) -> Result<RetirementDemand, ValueError> { self.close_demands() }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.copy_bytes) }
    fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(self.close_demands()?.capacity_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.release_bytes) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(self.close_demands()?.depth) }
}

impl Drop for MutationMessagePublicationBatch {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "diagnostic batch requires exact terminal closure"); if self.terminal_is_empty() { unsafe { ManuallyDrop::drop(&mut self.items); } } }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

#[path = "🪢️join/🦀️.rs"]
mod join;
pub use join::MutationMessagePublicationJoin;

#[path = "👁️borrow/🦀️.rs"]
mod borrowed;
pub use borrowed::MutationMessageBorrowedPublication;
