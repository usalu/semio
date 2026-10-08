//! 🔗️ Ordered adjacency owns each original text and backing allocation through exact controlled closure.
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
type Row = (String, Vec<String>);
#[derive(Debug, Default)]
#[cfg_attr(test, derive(Clone, PartialEq, Eq))]
pub(super) struct LinkRoot(Vec<Row>);
impl LinkRoot {
    pub(super) fn get(&self, source: &str) -> Option<&Vec<String>> { self.0.binary_search_by(|row| row.0.as_str().cmp(source)).ok().map(|index| &self.0[index].1) }
    pub(super) fn get_mut(&mut self, source: &str) -> Option<&mut Vec<String>> { self.0.binary_search_by(|row| row.0.as_str().cmp(source)).ok().map(|index| &mut self.0[index].1) }
    pub(super) fn capacity(&self) -> usize { self.0.capacity() }
    pub(super) fn try_reserve(&mut self, count: usize) -> Result<(), std::collections::TryReserveError> { self.0.try_reserve_exact(count) }
    pub(super) fn insert(&mut self, source: String, mut targets: Vec<String>) {
        targets.sort(); targets.dedup();
        match self.0.binary_search_by(|row| row.0.cmp(&source)) { Ok(index) => self.0[index].1 = targets, Err(index) => self.0.insert(index, (source, targets)) }
    }
    pub(super) fn remove(&mut self, source: &str) -> Option<Vec<String>> { self.0.binary_search_by(|row| row.0.as_str().cmp(source)).ok().map(|index| self.0.remove(index).1) }
    pub(super) fn add_target(&mut self, source: &str, target: &str) {
        if let Some(targets) = self.get_mut(source) { if let Err(index) = targets.binary_search_by(|value| value.as_str().cmp(target)) { targets.insert(index, target.into()); } }
        else { self.insert(source.into(), vec![target.into()]); }
    }
    pub(super) fn copy_demand(&self) -> usize {
        self.0.last().map_or(0, |row| if !row.1.is_empty() { std::mem::size_of::<String>() } else if row.1.capacity() == 0 && row.0.capacity() == 0 { std::mem::size_of::<Row>() } else { 0 })
    }
    pub(super) fn release_demand(&self) -> usize {
        self.0.last().map_or(self.0.capacity() * std::mem::size_of::<Row>(), |row| if let Some(target) = row.1.last() { target.capacity() } else if row.1.capacity() != 0 { row.1.capacity() * std::mem::size_of::<String>() } else { row.0.capacity() })
    }
    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> RetainedCloneStep {
        let empty = RetainedCloneProgress::default();
        if self.0.capacity() == 0 { return RetainedCloneStep::Complete(empty); }
        let copied_bytes = self.copy_demand(); let released_bytes = self.release_demand();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 || grant.maximum_copy_bytes < copied_bytes || grant.maximum_release_bytes < released_bytes { return RetainedCloneStep::Progress(empty); }
        if let Some(row) = self.0.last_mut() {
            if !row.1.is_empty() { drop(row.1.pop()); }
            else if row.1.capacity() != 0 { drop(std::mem::take(&mut row.1)); }
            else if row.0.capacity() != 0 { drop(std::mem::take(&mut row.0)); }
            else { self.0.pop(); }
        } else { drop(std::mem::take(&mut self.0)); }
        RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes, released_bytes, ..empty })
    }
}
