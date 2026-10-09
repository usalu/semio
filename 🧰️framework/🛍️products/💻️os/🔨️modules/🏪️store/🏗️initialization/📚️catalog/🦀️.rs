//! ♻️ One admitted empty initialization catalog page per retained close turn.

use super::ArtifactStoreInitializationOwnerCatalog;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use crate::os_vcs::HistoryPageStack;
use semio_framework_value::{ValueError, ValueRefusalKind};

fn demand<T>(owner: &HistoryPageStack<T>) -> Result<usize, ValueError> {
    owner.next_empty_page_release_byte_demand().ok_or_else(|| ValueError::new(ValueRefusalKind::InvariantViolated, "initialization catalog retained an occupied page during empty close"))
}

impl ArtifactStoreInitializationOwnerCatalog {
    /// 🪶️ Retains the six original page roots without creating any native backing.
    pub fn empty() -> Self {
        Self { applied_edit_ids: HistoryPageStack::empty(), redo_edit_ids: HistoryPageStack::empty(), cursor_applied_edit_ids: HistoryPageStack::empty(), cursor_redo_edit_ids: HistoryPageStack::empty(), applied_revision: HistoryPageStack::empty(), redo_revision: HistoryPageStack::empty() }
    }

    /// 📏️ Quotes the next original page, page descriptor and native directory before allocation.
    pub fn admission_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        let capacity_bytes = if self.applied_edit_ids.capacity() == 0 { self.applied_edit_ids.next_push_allocation_bytes() }
        else if self.redo_edit_ids.capacity() == 0 { self.redo_edit_ids.next_push_allocation_bytes() }
        else if self.cursor_applied_edit_ids.capacity() == 0 { self.cursor_applied_edit_ids.next_push_allocation_bytes() }
        else if self.cursor_redo_edit_ids.capacity() == 0 { self.cursor_redo_edit_ids.next_push_allocation_bytes() }
        else if self.applied_revision.capacity() == 0 { self.applied_revision.next_push_allocation_bytes() }
        else if self.redo_revision.capacity() == 0 { self.redo_revision.next_push_allocation_bytes() } else { 0 };
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: usize::from(!self.admission_is_complete()), ..Default::default() })
    }

    /// 🎟️ Admits exactly one empty native lane while retaining every previously born page.
    pub fn admit_next(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        if self.admission_is_complete() { return Ok(Default::default()); }
        let demand = self.admission_demands()?;
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "initialization catalog birth requires admitted depth")); }
        macro_rules! lane {
            ($field:ident) => {
                if self.$field.capacity() == 0 {
                    self.$field = HistoryPageStack::try_new().map_err(|reason| ValueError::literal(ValueRefusalKind::AllocationFailed, reason))?;
                    let progress = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
                    semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original initialization catalog page birth")?;
                    return Ok(progress);
                }
            };
        }
        lane!(applied_edit_ids); lane!(redo_edit_ids); lane!(cursor_applied_edit_ids); lane!(cursor_redo_edit_ids); lane!(applied_revision); lane!(redo_revision);
        Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "initialization catalog admission lost its original lane"))
    }

    /// 🏁 Requires all six actual original lane backings before runtime assembly.
    pub fn admission_is_complete(&self) -> bool {
        self.applied_edit_ids.capacity() != 0 && self.redo_edit_ids.capacity() != 0 && self.cursor_applied_edit_ids.capacity() != 0 && self.cursor_redo_edit_ids.capacity() != 0 && self.applied_revision.capacity() != 0 && self.redo_revision.capacity() != 0
    }

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
