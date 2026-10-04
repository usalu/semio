//! 🚚️ Store-owned bounded close handoff for retained-clone cursors.

use semio_framework_value::{
    ErasedSnapshotRetirement, SnapshotRetirementStep, ValueError, ValueRefusalKind,
    retained_clone::{RetainedClone, RetainedCloneCursor, admit_retained_clone_retirement},
};
use std::mem::ManuallyDrop;

pub(super) struct RetainedCloneCursorHandoff<T: RetainedClone> {
    cursor: ManuallyDrop<Option<T::Cursor>>,
    terminal: bool,
}

impl<T: RetainedClone> RetainedCloneCursorHandoff<T> {
    pub(super) fn new(mut cursor: T::Cursor) -> Self {
        let _ = cursor.begin_close();
        Self { cursor: ManuallyDrop::new(Some(cursor)), terminal: false }
    }

    fn cursor_mut(&mut self) -> Result<&mut T::Cursor, ValueError> {
        self.cursor.as_mut().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained clone cursor handoff lost its close owner"))
    }

    fn finish(&mut self) -> Result<(), ValueError> {
        let cursor = self.cursor.as_ref().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "retained clone cursor handoff completed twice"))?;
        if !cursor.terminal_is_empty() {
            return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "retained clone cursor handoff completed with a live owner"));
        }
        let cursor = self.cursor.take().expect("validated retained clone cursor handoff remains present");
        drop(cursor);
        self.terminal = true;
        Ok(())
    }
}

impl<T: RetainedClone> ErasedSnapshotRetirement for RetainedCloneCursorHandoff<T> {
    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        if self.terminal {
            return Ok(SnapshotRetirementStep::Complete);
        }
        let step = admit_retained_clone_retirement(self.cursor_mut()?.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained clone cursor handoff")?;
        if step == SnapshotRetirementStep::Complete {
            self.finish()?;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.cursor.is_none()
    }
}

impl<T: RetainedClone> Drop for RetainedCloneCursorHandoff<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "retained clone cursor handoff reached Drop before terminal-empty ownership");
        if self.terminal {
            unsafe { ManuallyDrop::drop(&mut self.cursor) };
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
