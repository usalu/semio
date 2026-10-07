//! 📨️ Cooperatively copies one admitted diagnostic without unbounded target-vector growth.

use crate::os_spr::MutationMessage;
use super::super::{ArtifactStoreMessageLedgerRetirement, ErasedSnapshotRetirement, SnapshotRetirementStep};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;

/// 🎟️ Payload copying and retained allocation require independent byte permissions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MessageCopyGrant {
    pub maximum_items: usize,
    pub maximum_copy_bytes: usize,
    pub maximum_capacity_bytes: usize,
}

/// 🧾️ Every target segment, including an empty segment, consumes structural work.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct MessageCopyProgress {
    pub items: usize,
    pub copied_bytes: usize,
    pub retained_capacity_bytes: usize,
}

/// ⏯️ Completion transfers an explicitly owned diagnostic through `take`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageCopyStep {
    Blocked,
    Pending(MessageCopyProgress),
    Complete(MessageCopyProgress),
}

/// 📋️ The caller keeps the borrowed source at one immutable address until this cursor closes.
pub struct MessageCopyCursor {
    source: Option<usize>,
    output: Option<MutationMessage>,
    active: Option<Box<dyn ErasedSnapshotRetirement>>,
    segment: usize,
    phase: u8,
    allocated: bool,
    cancelled: bool,
    closing: bool,
}

impl Default for MessageCopyCursor {
    fn default() -> Self { Self::new() }
}

impl MessageCopyCursor {
    pub fn new() -> Self {
        Self { source: None, output: None, active: None, segment: 0, phase: 0, allocated: false, cancelled: false, closing: false }
    }

    pub fn advance(&mut self, source: &MutationMessage, grant: MessageCopyGrant) -> Result<MessageCopyStep, ValueError> {
        if self.cancelled || self.closing || grant.maximum_items == 0 { return Ok(MessageCopyStep::Blocked); }
        let address = std::ptr::from_ref(source) as usize;
        if self.source.is_some_and(|expected| expected != address) {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message copy source owner changed"));
        }
        self.source = Some(address);
        if self.phase == 4 { return Ok(MessageCopyStep::Complete(MessageCopyProgress::default())); }
        if self.phase == 0 {
            let capacity = source.target.len().checked_mul(size_of::<String>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "message target capacity overflow"))?;
            if capacity > grant.maximum_capacity_bytes {
                return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "message target scaffold exceeds admitted capacity"));
            }
            let mut targets = Vec::new();
            targets.try_reserve_exact(source.target.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "message target allocation failed"))?;
            let actual = targets.capacity().checked_mul(size_of::<String>()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "message target actual capacity overflow"))?;
            if actual > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message target allocator exceeded capacity")); }
            self.output = Some(MutationMessage { level: source.level, code: semio_framework_diagnostic::FaultCode(String::new()), message: String::new(), target: targets, op_index: source.op_index });
            self.phase = 1;
            return Ok(MessageCopyStep::Pending(MessageCopyProgress { items: 1, copied_bytes: 0, retained_capacity_bytes: actual }));
        }
        let output = self.output.as_mut().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "message copy lost its output owner"))?;
        if self.phase == 3 && self.segment == source.target.len() {
            self.phase = 4;
            return Ok(MessageCopyStep::Complete(MessageCopyProgress { items: 1, ..Default::default() }));
        }
        let (input, destination) = match self.phase {
            1 => (source.code.0.as_str(), &mut output.code.0),
            2 => (source.message.as_str(), &mut output.message),
            3 => {
                if output.target.len() == self.segment {
                    output.target.push(String::new());
                    return Ok(MessageCopyStep::Pending(MessageCopyProgress { items: 1, ..Default::default() }));
                }
                (source.target[self.segment].as_str(), &mut output.target[self.segment])
            }
            _ => return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message copy phase is invalid")),
        };
        if !self.allocated {
            if input.len() > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "message string exceeds admitted capacity")); }
            destination.try_reserve_exact(input.len()).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "message string allocation failed"))?;
            if destination.capacity() > grant.maximum_capacity_bytes { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message string allocator exceeded capacity")); }
            self.allocated = true;
            return Ok(MessageCopyStep::Pending(MessageCopyProgress { items: 1, copied_bytes: 0, retained_capacity_bytes: destination.capacity() }));
        }
        let start = destination.len();
        if start > input.len() || !input.is_char_boundary(start) { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message copy source contents changed")); }
        if start == input.len() {
            self.allocated = false;
            if self.phase == 3 { self.segment += 1; } else { self.phase += 1; }
            return Ok(MessageCopyStep::Pending(MessageCopyProgress { items: 1, ..Default::default() }));
        }
        let mut end = start.saturating_add(grant.maximum_copy_bytes).min(input.len());
        while end > start && !input.is_char_boundary(end) { end -= 1; }
        if end == start { return Ok(MessageCopyStep::Blocked); }
        destination.push_str(&input[start..end]);
        Ok(MessageCopyStep::Pending(MessageCopyProgress { items: 1, copied_bytes: end - start, retained_capacity_bytes: 0 }))
    }

    pub fn take(&mut self) -> Option<MutationMessage> {
        if self.phase != 4 || self.cancelled || self.closing { return None; }
        self.output.take()
    }

    pub fn cancel(&mut self) { self.cancelled = true; }

    pub fn begin_close(&mut self) { self.closing = true; }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if !self.closing || maximum_items == 0 { return Ok(SnapshotRetirementStep::Blocked); }
        if let Some(active) = self.active.as_mut() {
            let step = active.close_step(maximum_items.min(1), maximum_bytes)?;
            if step == SnapshotRetirementStep::Complete {
                if !active.terminal_is_empty() { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "message copy retirement has live owners")); }
                self.active = None;
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if let Some(output) = self.output.take() {
            self.active = Some(Box::new(ArtifactStoreMessageLedgerRetirement::new(String::new(), vec![output])));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.source.is_none() && self.output.is_none() && self.active.is_none() }

    pub fn into_retirement(mut self) -> Box<dyn ErasedSnapshotRetirement> {
        self.cancel();
        self.begin_close();
        Box::new(self)
    }

    pub(super) fn close_cold(&mut self) {
        self.cancel();
        self.begin_close();
        while self.close_step(1, usize::MAX).expect("cold message copy retirement") != SnapshotRetirementStep::Complete {}
    }
}

impl ErasedSnapshotRetirement for MessageCopyCursor {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> { MessageCopyCursor::close_step(self, maximum_items, maximum_bytes) }
    fn terminal_is_empty(&self) -> bool { MessageCopyCursor::terminal_is_empty(self) }
}

impl Drop for MessageCopyCursor {
    fn drop(&mut self) { assert!(self.terminal_is_empty() || std::thread::panicking(), "message copy cursor dropped before terminal-empty ownership"); }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
