//! ♻️ Exact ownership retirement for an original diagnostic ledger and its context text.
use semio_framework_replication_conflict::{EditMessages, MutationReplayOutcome};
use semio_framework_replication_mutation::MutationMessage;
use crate::MutationMessageRetirement;
use semio_framework_value::{ErasedSnapshotRetirement, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};
use std::mem::{ManuallyDrop, size_of};

/// 🧳️ The original strings and vector scaffolds remain owned until individually released.
pub struct MutationMessageLedgerRetirement {
    context: ManuallyDrop<[Option<String>; 2]>,
    messages: ManuallyDrop<Vec<MutationMessage>>,
    active: Option<MutationMessageRetirement>,
    transfer_closed: bool,
}

impl MutationMessageLedgerRetirement {
    /// 🫙️ An allocation-free terminal shell admits future displaced original custody.
    pub fn empty() -> Self { Self { context: ManuallyDrop::new([None, None]), messages: ManuallyDrop::new(Vec::new()), active: None, transfer_closed: true } }
    /// 👁️ Borrows original diagnostics while their publication or refusal still owns them.
    pub fn retained_messages(&self) -> &[MutationMessage] { &self.messages }
    pub(crate) fn retained_messages_mut(&mut self) -> &mut [MutationMessage] { &mut self.messages }
    pub(crate) fn retained_message_rows(&self)->&Vec<MutationMessage>{&self.messages}
    pub(crate) fn retained_message_rows_mut(&mut self)->&mut Vec<MutationMessage>{&mut self.messages}

    /// 📏️ Moving the original vector frame preserves its payload allocations without copying rows.
    pub fn transfer_copy_byte_demand(&self) -> usize { size_of::<Vec<MutationMessage>>() }

    /// 📤️ Transfers the original bundle once under the caller's unspent physical grant.
    pub fn take_messages_controlled(&mut self, grant: RetainedCloneGrant) -> Result<Option<(Vec<MutationMessage>, RetainedCloneProgress)>, ValueError> {
        if self.transfer_closed || self.active.is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "diagnostic transfer follows closure or prior transfer")); }
        let copy = self.transfer_copy_byte_demand();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        self.transfer_closed = true;
        Ok(Some((std::mem::take(&mut *self.messages), RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })))
    }

    /// 🧾️ Transfers one actual edit context and its original vector as a fully owned ledger record.
    pub fn take_edit_messages_controlled(&mut self, grant: RetainedCloneGrant) -> Result<Option<(EditMessages, RetainedCloneProgress)>, ValueError> {
        if self.transfer_closed || self.active.is_some() || self.context[1].is_some() || self.context[0].as_ref().is_none_or(String::is_empty) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "edit diagnostic transfer lacks an original live edit context")); }
        let copy = size_of::<EditMessages>();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copy { return Ok(None); }
        self.transfer_closed = true;
        let edit_id = self.context[0].take().expect("admitted original edit context");
        let messages = std::mem::take(&mut *self.messages);
        Ok(Some((EditMessages { edit_id, messages }, RetainedCloneProgress { copied_items: 1, copied_bytes: copy, ..Default::default() })))
    }

    pub fn new(context: String, messages: Vec<MutationMessage>) -> Self { Self { context: ManuallyDrop::new([Some(context),None]), messages: ManuallyDrop::new(messages), active: None, transfer_closed: false } }

    pub fn from_replay_outcome(outcome: MutationReplayOutcome) -> Self { Self { context: ManuallyDrop::new([Some(outcome.mutation_id.0),Some(outcome.edit_id)]), messages: ManuallyDrop::new(outcome.messages), active: None, transfer_closed: false } }

    pub fn next_copy_byte_demand(&self) -> usize {
        if self.context.iter().any(Option::is_some) { return 0; }
        self.active.as_ref().map_or_else(|| usize::from(!self.messages.is_empty()) * size_of::<MutationMessage>(), MutationMessageRetirement::next_copy_byte_demand)
    }

    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }

    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(context) = self.context.iter().find_map(Option::as_ref) { return Ok(context.capacity()); }
        Ok(self.active.as_ref().map_or_else(|| if self.messages.is_empty() { self.messages.capacity() * size_of::<MutationMessage>() } else { 0 }, MutationMessageRetirement::next_release_byte_demand))
    }

    pub fn next_depth_demand(&self) -> Result<usize, ValueError> {
        if self.terminal_is_empty() { return Ok(0); }
        self.active.as_ref().map_or(Ok(1), |active| active.next_depth_demand().map(|depth| depth + 1))
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if grant.maximum_items == 0 || grant.maximum_depth < self.next_depth_demand()? || grant.maximum_copy_bytes < self.next_copy_byte_demand() || grant.maximum_release_bytes < self.next_release_byte_demand()? { return Ok(RetainedCloneStep::Progress(idle)); }
        self.transfer_closed = true;
        let progress = if let Some(index) = self.context.iter().position(Option::is_some) {
            let released_bytes = self.context[index].as_ref().expect("selected context remains owned").capacity();
            drop(self.context[index].take());
            RetainedCloneProgress { copied_items: 1, released_bytes, ..idle }
        } else if let Some(active) = self.active.as_mut() {
            let progress = active.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant });
            if !progress.fits(grant) { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "diagnostic row retirement exceeded its independent grant")); }
            if active.terminal_is_empty() { self.active = None; }
            progress
        } else if let Some(message) = self.messages.pop() {
            self.active = Some(MutationMessageRetirement::new(message));
            RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<MutationMessage>(), ..idle }
        } else {
            let released_bytes = self.messages.capacity() * size_of::<MutationMessage>();
            drop(std::mem::take(&mut *self.messages));
            RetainedCloneProgress { copied_items: 1, released_bytes, ..idle }
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    pub fn terminal_is_empty(&self) -> bool { self.context.iter().all(Option::is_none) && self.active.is_none() && self.messages.is_empty() && self.messages.capacity() == 0 }
}

impl ErasedSnapshotRetirement for MutationMessageLedgerRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { Ok(semio_framework_value::RetirementDemand { copy_bytes: Self::next_copy_byte_demand(self), capacity_bytes: Self::next_capacity_byte_demand(self,body)?, release_bytes: Self::next_release_byte_demand(self)?, depth: Self::next_depth_demand(self)? }) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(Self::next_copy_byte_demand(self)) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self, body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Self::next_depth_demand(self) }
}

impl Drop for MutationMessageLedgerRetirement {
    fn drop(&mut self) { let empty = self.terminal_is_empty(); assert!(empty || std::thread::panicking(), "diagnostic ledger reached Drop before exact physical ownership retirement"); if empty { unsafe { ManuallyDrop::drop(&mut self.context); ManuallyDrop::drop(&mut self.messages); } } }
}

/// 🗃️ Each original edit ledger and the containing vector retain independent physical ownership.
pub struct EditMessageLedgerRetirement {
    entries: ManuallyDrop<Vec<EditMessages>>,
    active: Option<MutationMessageLedgerRetirement>,
}

impl EditMessageLedgerRetirement {
    /// 🔎️ Borrows the original untransferred rows without allocating or changing close custody.
    pub fn retained_entries(&self) -> &[EditMessages] { &self.entries }
    pub fn new(entries: Vec<EditMessages>) -> Self { Self { entries: ManuallyDrop::new(entries), active: None } }
    pub fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.active.as_ref().map_or_else(|| usize::from(!self.entries.is_empty()) * size_of::<EditMessages>(), MutationMessageLedgerRetirement::next_copy_byte_demand)) }
    pub fn next_capacity_byte_demand(&self, _: usize) -> Result<usize, ValueError> { Ok(0) }
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> { self.active.as_ref().map_or_else(|| Ok(if self.entries.is_empty() { self.entries.capacity() * size_of::<EditMessages>() } else { 0 }), MutationMessageLedgerRetirement::next_release_byte_demand) }
    pub fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.terminal_is_empty() { return Ok(0); } self.active.as_ref().map_or(Ok(1), |active| active.next_depth_demand().map(|depth| depth + 1)) }
    pub fn terminal_is_empty(&self) -> bool { self.active.is_none() && self.entries.is_empty() && self.entries.capacity() == 0 }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let idle=RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(idle)); }
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < self.next_copy_byte_demand()? || grant.maximum_release_bytes < self.next_release_byte_demand()? || grant.maximum_depth < self.next_depth_demand()? { return Ok(RetainedCloneStep::Progress(idle)); }
        let progress=if let Some(active)=self.active.as_mut() {
            let step=active.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant })?;
            if !step.progress().fits(grant) || matches!(step,RetainedCloneStep::Complete(_)) && !active.terminal_is_empty() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"nested diagnostic ledger violated its exact close authority")); }
            if active.terminal_is_empty() { self.active=None; }
            step.progress()
        } else if let Some(entry)=self.entries.pop() {
            self.active=Some(MutationMessageLedgerRetirement::new(entry.edit_id,entry.messages));
            RetainedCloneProgress { copied_items: 1, copied_bytes: size_of::<EditMessages>(), ..idle }
        } else {
            let released_bytes=self.entries.capacity()*size_of::<EditMessages>();
            drop(std::mem::take(&mut *self.entries));
            RetainedCloneProgress { copied_items: 1, released_bytes, ..idle }
        };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}

impl ErasedSnapshotRetirement for EditMessageLedgerRetirement {
    fn next_demand(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> { if let Some(active)=self.active.as_ref() { let mut demand=active.next_demand(body)?; demand.depth+=1; return Ok(demand); } Ok(semio_framework_value::RetirementDemand { copy_bytes: usize::from(!self.entries.is_empty())*size_of::<EditMessages>(), release_bytes: if self.entries.is_empty(){self.entries.capacity()*size_of::<EditMessages>()}else{0}, depth: usize::from(!self.terminal_is_empty()), ..Default::default() }) }
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { Self::close_step(self,grant) }
    fn terminal_is_empty(&self) -> bool { Self::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Self::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { Self::next_capacity_byte_demand(self,body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Self::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { Self::next_depth_demand(self) }
}

impl Drop for EditMessageLedgerRetirement {
    fn drop(&mut self) { let empty=self.terminal_is_empty(); assert!(empty || std::thread::panicking(),"nested edit diagnostics reached Drop before exact physical ownership retirement"); if empty { unsafe { ManuallyDrop::drop(&mut self.entries); } } }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
