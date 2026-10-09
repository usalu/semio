//! 🗂️ Original ordered window addresses in individually releasable semantic pages.

use std::borrow::Borrow;
use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, list::PagedList, retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep}};

pub(crate) struct WindowRegistry<K, V> { entries: PagedList<(K, V), {usize::MAX}> }

impl<K, V> Drop for WindowRegistry<K, V> {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.entries.terminal_is_empty(), "original window registry reached Drop before every address and page retired"); }
}

impl<K: Ord + Borrow<str>, V> WindowRegistry<K, V> {
    pub(crate) fn new() -> Self {
        Self { entries: PagedList::with_payload_page_bytes(4096.max(std::mem::size_of::<(K, V)>())).expect("registry page admits its original inline entry") }
    }
    fn position(&self, key: &str) -> Result<usize, usize> {
        let (mut low, mut high) = (0, self.entries.len());
        while low < high {
            let middle = low + (high - low) / 2;
            match self.entries.get(middle).unwrap().0.borrow().cmp(key) {
                std::cmp::Ordering::Less => low = middle + 1,
                std::cmp::Ordering::Greater => high = middle,
                std::cmp::Ordering::Equal => return Ok(middle),
            }
        }
        Err(low)
    }
    pub(crate) fn contains_key(&self, key: &str) -> bool { self.position(key).is_ok() }
    pub(crate) fn get(&self, key: &str) -> Option<&V> { self.position(key).ok().and_then(|index| self.entries.get(index)).map(|entry| &entry.1) }
    pub(crate) fn get_mut(&mut self, key: &str) -> Option<&mut V> { let index = self.position(key).ok()?; self.entries.get_mut(index).map(|entry| &mut entry.1) }
    pub(crate) fn len(&self) -> usize { self.entries.len() }
    pub(crate) fn is_empty(&self) -> bool { self.entries.is_empty() }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.entries.terminal_is_empty() }
    pub(crate) fn iter(&self) -> impl Iterator<Item = (&K, &V)> { self.entries.iter().map(|(key, value)| (key, value)) }
    pub(crate) fn values(&self) -> impl Iterator<Item = &V> { self.entries.iter().map(|entry| &entry.1) }
    pub(crate) fn values_mut(&mut self) -> impl Iterator<Item = &mut V> { self.entries.iter_mut().map(|entry| &mut entry.1) }
    pub(crate) fn get_index_mut(&mut self, index: usize) -> Option<&mut V> { self.entries.get_mut(index).map(|entry| &mut entry.1) }
    pub(crate) fn get_index(&self, index: usize) -> Option<&V> { self.entries.get(index).map(|entry| &entry.1) }
    pub(crate) fn last(&self) -> Option<(&K, &V)> { self.entries.last().map(|(key, value)| (key, value)) }
    /// 🌱️ Cold registration funds each original page from its own published allocation quote.
    pub(crate) fn insert(&mut self, key: K, value: V) -> Option<V> {
        let index = match self.position(key.borrow()) {
            Ok(index) => return Some(std::mem::replace(&mut self.entries.get_mut(index).unwrap().1, value)),
            Err(index) => index,
        };
        if !self.entries.has_reserved_slot() {
            let bytes = self.entries.next_allocation_bytes().expect("cold registry original page quote");
            self.entries.reserve_one(bytes).expect("cold registry original page allocation");
        }
        self.entries.push_reserved((key, value)).unwrap_or_else(|_| panic!("reserved original registry entry remains admitted"));
        for current in (index..self.entries.len() - 1).rev() { self.entries.swap(current, current + 1); }
        None
    }
    pub(crate) fn pop_demand(&self) -> Result<RetirementDemand, ValueError> {
        if self.is_empty() { return self.backing_demand(); }
        Ok(RetirementDemand { copy_bytes: std::mem::size_of::<(K, V)>(), depth: self.entries.next_pop_depth_demand().map_err(page_error)?, ..Default::default() })
    }
    pub(crate) fn pop_original(&mut self, grant: RetainedCloneGrant) -> Result<Option<((K, V), RetainedCloneProgress)>, ValueError> {
        if self.is_empty() { return Ok(None); }
        let demand = self.pop_demand()?;
        if !fits(grant, demand) { return Ok(None); }
        Ok(self.entries.pop().map(|entry| (entry, RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })))
    }
    pub(crate) fn backing_demand(&self) -> Result<RetirementDemand, ValueError> {
        if !self.is_empty() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "registry backing still retains original entries")); }
        Ok(RetirementDemand { release_bytes: self.entries.next_release_allocation_bytes().map_err(page_error)?, depth: self.entries.next_release_depth_demand().map_err(page_error)?, ..Default::default() })
    }
    pub(crate) fn close_backing_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        let demand = self.backing_demand()?;
        if !fits(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let step = self.entries.release_empty_page(grant.maximum_release_bytes).map_err(page_error)?;
        let progress = RetainedCloneProgress { copied_items: usize::from(step.progressed), released_bytes: step.released_allocation_bytes, ..Default::default() };
        Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }
}

fn page_error(error: semio_framework_value::list::PagedListError) -> ValueError { ValueError::literal(ValueRefusalKind::OwnershipLimit, error.reason) }
pub(crate) fn fits(grant: RetainedCloneGrant, demand: RetirementDemand) -> bool { grant.maximum_items > 0 && grant.maximum_copy_bytes >= demand.copy_bytes && grant.maximum_capacity_bytes >= demand.capacity_bytes && grant.maximum_release_bytes >= demand.release_bytes && grant.maximum_depth >= demand.depth }

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
