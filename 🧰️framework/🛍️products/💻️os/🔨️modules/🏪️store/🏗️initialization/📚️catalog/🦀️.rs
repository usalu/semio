//! ♻️ One admitted empty initialization catalog page per retained close turn.

use super::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_vcs::HistoryPageStack;
use semio_framework_value::{ValueError, ValueRefusalKind};

fn demand<T>(owner: &HistoryPageStack<T>) -> Result<usize, ValueError> {
    owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "initialization catalog retained an occupied page during empty close"))
}

impl ArtifactStoreInitializationOwnerCatalog {
    /// 📏️ Borrows the next resident native page and its last directory scaffold without allocation.
    pub fn next_release_byte_demand(&self) -> Result<usize, ValueError> {
        if self.applied_edit_ids.capacity() != 0 { return demand(&self.applied_edit_ids); }
        if self.redo_edit_ids.capacity() != 0 { return demand(&self.redo_edit_ids); }
        if self.cursor_applied_edit_ids.capacity() != 0 { return demand(&self.cursor_applied_edit_ids); }
        if self.cursor_redo_edit_ids.capacity() != 0 { return demand(&self.cursor_redo_edit_ids); }
        if self.applied_revision.capacity() != 0 { return demand(&self.applied_revision); }
        demand(&self.redo_revision)
    }

    /// 📏️ Borrows the original native page release with no copy or allocation work.
    pub fn close_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_release_byte_demand()?, depth: usize::from(!self.terminal_is_empty()), ..Default::default() })
    }

    /// ⛽️ Releases the one complete empty page only after its exact physical extent is granted.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let pending = RetainedCloneStep::Progress(Default::default());
        if grant.maximum_items == 0 { return Ok(pending); }
        let bytes = self.next_release_byte_demand()?;
        if grant.maximum_release_bytes < bytes { return Ok(pending); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "initialization catalog page requires admitted depth")); }
        let released = if self.applied_edit_ids.capacity() != 0 { self.applied_edit_ids.release_empty_page() }
        else if self.redo_edit_ids.capacity() != 0 { self.redo_edit_ids.release_empty_page() }
        else if self.cursor_applied_edit_ids.capacity() != 0 { self.cursor_applied_edit_ids.release_empty_page() }
        else if self.cursor_redo_edit_ids.capacity() != 0 { self.cursor_redo_edit_ids.release_empty_page() }
        else if self.applied_revision.capacity() != 0 { self.applied_revision.release_empty_page() }
        else { self.redo_revision.release_empty_page() };
        if !released { return Err(ValueError::new(ValueRefusalKind::InvariantViolated, "initialization catalog lost its admitted empty page")); }
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

    /// 🫙️ Confirms all six native page roots are empty before the catalog object may be dropped.
    pub fn terminal_is_empty(&self) -> bool { self.admitted_items() == 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
