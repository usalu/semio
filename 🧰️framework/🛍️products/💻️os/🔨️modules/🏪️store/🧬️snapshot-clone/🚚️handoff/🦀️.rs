//! 🚚️ Store-owned bounded close handoff for retained-clone cursors.

use semio_framework_value::{
    ErasedSnapshotRetirement, ValueError, ValueRefusalKind,
    retained_clone::{RetainedClone, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneStep, admit_retained_clone_close},
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
    fn cursor(&self) -> Result<&T::Cursor, ValueError> {
        self.cursor.as_ref().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "retained clone cursor handoff lost its close owner"))
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
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal {
            return Ok(RetainedCloneStep::Complete(Default::default()));
        }
        let cursor = self.cursor_mut()?;
        let step = cursor.close_step(grant)?;
        let step = admit_retained_clone_close(grant, step, cursor.terminal_is_empty(), "retained clone cursor handoff")?;
        if matches!(step, RetainedCloneStep::Complete(_)) {
            self.finish()?;
        }
        Ok(step)
    }

    fn terminal_is_empty(&self) -> bool {
        self.terminal && self.cursor.is_none()
    }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.next_close_copy_byte_demand() } }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.next_close_capacity_byte_demand(body) } }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.next_close_release_byte_demand() } }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { if self.terminal { Ok(0) } else { self.cursor()?.next_close_depth_demand() } }
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
