//! 🧺 Bounds each retained queue allocation independently of its logical byte budget.

use std::collections::TryReserveError;

const SLOTS_PER_PAGE: usize = 64;
type Chunk = Vec<u8>;
type Page = Vec<Option<Chunk>>;

#[derive(Debug)]
pub(super) struct ArtifactOutputPages {
    pages: Vec<Page>,
    slots: usize,
    head: usize,
    len: usize,
}

impl ArtifactOutputPages {
    pub(super) fn unavailable(slots: usize) -> Self {
        Self { pages: Vec::new(), slots, head: 0, len: 0 }
    }

    pub(super) fn try_new(slots: usize) -> Result<Self, TryReserveError> {
        let mut queue = Self::unavailable(slots);
        let pages = slots.div_ceil(SLOTS_PER_PAGE);
        queue.pages.try_reserve_exact(pages)?;
        queue.pages.resize_with(pages, Vec::new);
        Ok(queue)
    }

    pub(super) fn table_bytes(slots: usize) -> usize {
        slots.div_ceil(SLOTS_PER_PAGE) * size_of::<Page>()
    }

    pub(super) fn inline_slot_bytes(slots: usize) -> usize {
        if slots <= SLOTS_PER_PAGE { slots * size_of::<Option<Chunk>>() } else { 0 }
    }

    pub(super) fn table_capacity_bytes(&self) -> usize {
        self.pages.capacity() * size_of::<Page>()
    }

    fn tail(&self) -> usize {
        (self.head + self.len) % self.slots
    }

    pub(super) fn can_push(&self) -> bool {
        !self.pages.is_empty() && self.len < self.slots
    }

    #[cfg(test)]
    pub(super) fn backing_unadmit(&mut self) {
        assert_eq!(self.len, 0);
        self.pages.clear();
    }

    pub(super) fn tail_page_bytes(&self) -> usize {
        let index = self.tail() / SLOTS_PER_PAGE;
        if self.pages[index].is_empty() {
            (self.slots - index * SLOTS_PER_PAGE).min(SLOTS_PER_PAGE) * size_of::<Option<Chunk>>()
        } else {
            0
        }
    }

    pub(super) fn reserve_tail(&mut self) -> Result<usize, TryReserveError> {
        let page = self.tail() / SLOTS_PER_PAGE;
        let capacity = (self.slots - page * SLOTS_PER_PAGE).min(SLOTS_PER_PAGE);
        let slots = &mut self.pages[page];
        if !slots.is_empty() {
            return Ok(0);
        }
        slots.try_reserve_exact(capacity)?;
        slots.resize_with(capacity, || None);
        Ok(slots.capacity() * size_of::<Option<Chunk>>())
    }

    pub(super) fn len(&self) -> usize { self.len }

    fn slot(&mut self, slot: usize) -> Option<&mut Option<Chunk>> {
        self.pages.get_mut(slot / SLOTS_PER_PAGE)?.get_mut(slot % SLOTS_PER_PAGE)
    }

    #[cfg(test)]
    pub(super) fn first(&self) -> Option<&Chunk> {
        self.pages.get(self.head / SLOTS_PER_PAGE)?.get(self.head % SLOTS_PER_PAGE)?.as_ref()
    }

    #[cfg(test)]
    pub(super) fn last(&self) -> Option<&Chunk> {
        let tail = (self.head + self.len.checked_sub(1)?) % self.slots;
        self.pages.get(tail / SLOTS_PER_PAGE)?.get(tail % SLOTS_PER_PAGE)?.as_ref()
    }

    pub(super) fn push(&mut self, value: Chunk) -> Result<(), Chunk> {
        if !self.can_push() { return Err(value); }
        let Some(slot @ None) = self.slot(self.tail()) else { return Err(value); };
        *slot = Some(value);
        self.len += 1;
        Ok(())
    }

    pub(super) fn pop(&mut self) -> Option<Chunk> {
        if self.len == 0 { return None; }
        let chunk = self.slot(self.head)?.take()?;
        self.head = (self.head + 1) % self.slots;
        self.len -= 1;
        Some(chunk)
    }

    pub(super) fn pages(&self) -> &Vec<Page> { &self.pages }
    pub(super) fn into_pages(self) -> Vec<Page> { self.pages }

    pub(super) fn drained_release(&self) -> Option<(usize, u64)> {
        if self.len != 0 || self.pages.len() > 16 { return None; }
        let mut bytes = self.table_capacity_bytes();
        let mut work = 1;
        for page in &self.pages {
            bytes = bytes.checked_add(page.capacity().checked_mul(size_of::<Option<Chunk>>())?)?;
            work += 1 + page.len() as u64;
        }
        Some((bytes, work))
    }
}
