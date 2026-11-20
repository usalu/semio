//! 📦️ Separately funded mutation pages keep large edits atomic and physically releasable.

use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
use std::collections::VecDeque;
use std::mem::{size_of, take};

const PAGE_ITEMS: usize = 64;

/// 📥️ Single-owner input pages in the order a batch prepares and publishes its mutations.
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactMutationPages<T> {
    pages: Vec<VecDeque<T>>,
    capacity: usize,
    written: usize,
    read: usize,
}

impl<T> ArtifactMutationPages<T> {
    pub fn needs_pages(capacity: usize) -> bool { capacity > PAGE_ITEMS }

    /// 🔢️ Counts the independently admitted data pages for an edit.
    pub fn page_count(capacity: usize) -> usize { capacity.div_ceil(PAGE_ITEMS) }

    /// 📏️ Quotes the separately retained page directory.
    pub fn directory_demand(capacity: usize) -> RetirementDemand {
        RetirementDemand { capacity_bytes: Self::page_count(capacity) * size_of::<VecDeque<T>>(), depth: 1, ..Default::default() }
    }

    /// 🏗️ Reserves the original page directory; the writer admits data pages individually.
    pub fn new(capacity: usize) -> Result<Self, ValueError> {
        let mut pages = Vec::new();
        pages.try_reserve_exact(Self::page_count(capacity)).map_err(Self::allocation_error)?;
        Ok(Self { pages, capacity, written: 0, read: 0 })
    }

    /// 🧾️ Reports the original page-directory allocation.
    pub fn directory_bytes(&self) -> usize { self.pages.capacity() * size_of::<VecDeque<T>>() }

    fn allocation_error(_: std::collections::TryReserveError) -> ValueError {
        ValueError::literal(ValueRefusalKind::AllocationFailed, "artifact mutation page allocation was refused")
    }

    /// 📐️ Quotes the next data page when the next chunk would exceed the resident slots.
    pub fn next_page_demand(&self) -> Option<RetirementDemand> {
        let reserved = self.pages.len() * PAGE_ITEMS;
        if reserved >= self.capacity || reserved >= self.written.saturating_add(PAGE_ITEMS) { return None }
        let capacity_bytes = (self.capacity - reserved).min(PAGE_ITEMS) * size_of::<T>();
        Some(RetirementDemand { copy_bytes: size_of::<VecDeque<T>>(), capacity_bytes, depth: 1, ..Default::default() })
    }

    /// 🪙️ Adds one quoted data page under the caller's grant before writing its items.
    pub fn admit_page(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let Some(demand) = self.next_page_demand() else { return Ok(Default::default()) };
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant, demand) { return Ok(Default::default()) }
        let items = (self.capacity - self.pages.len() * PAGE_ITEMS).min(PAGE_ITEMS);
        let mut page = VecDeque::new();
        page.try_reserve_exact(items).map_err(Self::allocation_error)?;
        let retained_capacity_bytes = page.capacity() * size_of::<T>();
        self.pages.push(page);
        Ok(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes, ..Default::default() })
    }

    /// ➕️ Moves an original item into its prepaid slot.
    pub fn push(&mut self, item: T) -> Result<(), T> {
        if self.written == self.capacity { return Err(item) }
        let Some(page) = self.pages.get_mut(self.written / PAGE_ITEMS) else { return Err(item) };
        page.push_back(item);
        self.written += 1;
        Ok(())
    }

    pub fn len(&self) -> usize { self.written - self.read }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn iter(&self) -> impl Iterator<Item = &T> { self.pages.iter().flat_map(|page| page.iter()) }
    pub fn into_mutations(self) -> impl Iterator<Item = T> { self.pages.into_iter().flatten() }

    pub(super) fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len() { return None }
        let position = self.read + index;
        let page = position / PAGE_ITEMS;
        let consumed = if page == self.read / PAGE_ITEMS { self.read % PAGE_ITEMS } else { 0 };
        self.pages[page].get(position % PAGE_ITEMS - consumed)
    }

    pub(super) fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() { return None }
        let value = self.pages[self.read / PAGE_ITEMS].pop_front()?;
        self.read += 1;
        Some(value)
    }

    pub(super) fn restore_front(&mut self, item: T) {
        self.read -= 1;
        self.pages[self.read / PAGE_ITEMS].push_front(item);
    }

    pub(super) fn empty_release_bytes(&self) -> usize {
        debug_assert!(self.is_empty());
        self.pages.last().map_or(self.directory_bytes(), |page| page.capacity() * size_of::<T>())
    }

    pub(super) fn backing_is_empty(&self) -> bool {
        self.pages.is_empty() && self.pages.capacity() == 0
    }

    pub(super) fn release_empty_backing(&mut self) {
        debug_assert!(self.is_empty());
        if self.pages.pop().is_none() { drop(take(&mut self.pages)) }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
