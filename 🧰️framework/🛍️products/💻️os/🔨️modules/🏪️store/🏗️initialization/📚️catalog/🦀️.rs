//! ♻️ One admitted empty initialization catalog page per retained close turn.

use super::{ArtifactStoreInitializationOwnerCatalog, SnapshotRetirementStep};
use crate::os_vcs::HistoryPageStack;
use semio_framework_value::{ValueError, ValueRefusalKind};

fn demand<T>(owner: &HistoryPageStack<T>) -> Result<usize, ValueError> {
    owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "initialization catalog retained an occupied page during empty close"))
}

impl ArtifactStoreInitializationOwnerCatalog {
    /// 📏️ Borrows the next resident native page and its last directory scaffold without allocation.
    pub fn next_close_byte_demand(&self) -> Result<usize, ValueError> {
        if self.applied_edit_ids.capacity() != 0 { return demand(&self.applied_edit_ids); }
        if self.redo_edit_ids.capacity() != 0 { return demand(&self.redo_edit_ids); }
        if self.cursor_applied_edit_ids.capacity() != 0 { return demand(&self.cursor_applied_edit_ids); }
        if self.cursor_redo_edit_ids.capacity() != 0 { return demand(&self.cursor_redo_edit_ids); }
        if self.applied_revision.capacity() != 0 { return demand(&self.applied_revision); }
        demand(&self.redo_revision)
    }

    /// ⛽️ Releases the one complete empty page only after its exact physical extent is granted.
    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, ValueError> {
        let pending = SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 };
        if maximum_items == 0 { return Ok(pending); }
        let bytes = self.next_close_byte_demand()?;
        if bytes == 0 { return Ok(SnapshotRetirementStep::Complete); }
        if maximum_bytes < bytes { return Ok(pending); }
        let released = if self.applied_edit_ids.capacity() != 0 { self.applied_edit_ids.release_empty_page() }
        else if self.redo_edit_ids.capacity() != 0 { self.redo_edit_ids.release_empty_page() }
        else if self.cursor_applied_edit_ids.capacity() != 0 { self.cursor_applied_edit_ids.release_empty_page() }
        else if self.cursor_redo_edit_ids.capacity() != 0 { self.cursor_redo_edit_ids.release_empty_page() }
        else if self.applied_revision.capacity() != 0 { self.applied_revision.release_empty_page() }
        else { self.redo_revision.release_empty_page() };
        if !released { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "initialization catalog lost its admitted empty page")); }
        Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: bytes })
    }

    /// 🫙️ Confirms all six native page roots are empty before the catalog object may be dropped.
    pub fn terminal_is_empty(&self) -> bool { self.admitted_items() == 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
