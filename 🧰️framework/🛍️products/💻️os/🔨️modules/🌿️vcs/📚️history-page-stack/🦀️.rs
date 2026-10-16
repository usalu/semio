//! 📚️ History page stack: the paged, bounded-allocation stack that stores one history catalog and its serialization.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

/// 📄 Slots in one history page. One page is the largest contiguous slot allocation.
pub const ARTIFACT_HISTORY_PAGE_SLOTS: usize = 64;
/// 📏️ The largest physical release one history catalog close step performs: a directory chunk or one value page.
pub const ARTIFACT_HISTORY_RELEASE_PAGE_BYTES: usize = 4096;
/// 📄 Directory pages addressed by one chunk. The chunk's pointer array, count and link fill exactly one release page,
/// and the chunk is never reallocated.
pub const ARTIFACT_HISTORY_DIRECTORY_PAGES: usize = (ARTIFACT_HISTORY_RELEASE_PAGE_BYTES - 2 * size_of::<usize>()) / size_of::<usize>();
/// 📄 Page size of every artifact history ledger. The name is the page, not a session ceiling.
pub const ARTIFACT_HISTORY_LEDGER_CAPACITY: usize = ARTIFACT_HISTORY_PAGE_SLOTS;
pub(super) const ARTIFACT_HISTORY_CONTIGUOUS_CEILING: usize = 64 * 1024;

/// 📄 Dense append/pop catalog paged so a full page opens another fixed page instead of growing one buffer.
pub struct HistoryPageStack<T> {
    pub(super) head: Option<Box<HistoryPageChunk<T>>>,
    detached: Option<Box<HistoryValuePage<T>>>,
    pub(super) pages: u32,
    pub(super) len: usize,
}

pub(super) struct HistoryPageChunk<T> {
    pub(super) pages: [Option<Box<HistoryValuePage<T>>>; ARTIFACT_HISTORY_DIRECTORY_PAGES],
    pub(super) count: u16,
    pub(super) next: Option<Box<HistoryPageChunk<T>>>,
}

const _: () = assert!(size_of::<HistoryPageChunk<()>>() <= ARTIFACT_HISTORY_RELEASE_PAGE_BYTES);

pub(super) struct HistoryValuePage<T> {
    pub(super) slots: Box<[Option<T>]>,
}

impl<T> HistoryPageChunk<T> {
    pub(super) fn blank() -> Self {
        Self { pages: std::array::from_fn(|_| None), count: 0, next: None }
    }
}

fn history_value_page<T>() -> Box<[Option<T>]> {
    let bytes = size_of::<Option<T>>().saturating_mul(ARTIFACT_HISTORY_PAGE_SLOTS);
    assert!(bytes <= ARTIFACT_HISTORY_CONTIGUOUS_CEILING, "one history catalog page must stay under the 64 KiB contiguous ceiling");
    let mut slots = Vec::new();
    slots.try_reserve_exact(ARTIFACT_HISTORY_PAGE_SLOTS).expect("history catalog page allocation");
    slots.resize_with(ARTIFACT_HISTORY_PAGE_SLOTS, || None);
    slots.into_boxed_slice()
}

fn history_place_page<T>(head: &mut Option<Box<HistoryPageChunk<T>>>, page_index: usize, page: Box<HistoryValuePage<T>>) {
    if head.is_none() {
        *head = Some(Box::new(HistoryPageChunk::blank()));
    }
    let chunk = history_descend_chunk(head.as_mut().expect("history catalog chunk"), page_index / ARTIFACT_HISTORY_DIRECTORY_PAGES);
    let offset = page_index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
    chunk.pages[offset] = Some(page);
    chunk.count = chunk.count.saturating_add(1);
}

fn history_descend_chunk<T>(chunk: &mut HistoryPageChunk<T>, remaining: usize) -> &mut HistoryPageChunk<T> {
    if remaining == 0 {
        return chunk;
    }
    if chunk.next.is_none() {
        chunk.next = Some(Box::new(HistoryPageChunk::blank()));
    }
    history_descend_chunk(chunk.next.as_mut().expect("history catalog chunk link"), remaining - 1)
}

fn history_page<'a, T>(head: &'a HistoryPageChunk<T>, page_index: usize) -> &'a HistoryValuePage<T> {
    let chunk = history_chunk_ref(head, page_index / ARTIFACT_HISTORY_DIRECTORY_PAGES);
    chunk.pages[page_index % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_ref().expect("history catalog page")
}

fn history_page_mut<'a, T>(head: &'a mut HistoryPageChunk<T>, page_index: usize) -> &'a mut HistoryValuePage<T> {
    let chunk = history_descend_chunk(head, page_index / ARTIFACT_HISTORY_DIRECTORY_PAGES);
    chunk.pages[page_index % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_mut().expect("history catalog page")
}

fn history_chunk_ref<T>(chunk: &HistoryPageChunk<T>, remaining: usize) -> &HistoryPageChunk<T> {
    if remaining == 0 { chunk } else { history_chunk_ref(chunk.next.as_ref().expect("history catalog chunk link"), remaining - 1) }
}

impl<T> HistoryPageStack<T> {
    /// 🪶️ Creates an empty retained catalog without allocating resident pages.
    pub fn empty() -> Self { Self { head: None, detached: None, pages: 0, len: 0 } }
    /// 🧮 Quotes the exact original first stack page and its directory.
    pub fn initial_page_birth_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.pages != 0 { return Ok(Default::default()); }
        let capacity_bytes = size_of::<Option<T>>().checked_mul(ARTIFACT_HISTORY_PAGE_SLOTS)
            .and_then(|bytes| bytes.checked_add(size_of::<HistoryValuePage<T>>()))
            .and_then(|bytes| bytes.checked_add(size_of::<HistoryPageChunk<T>>()))
            .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history stack original page extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    /// 🎟️ Admits the first original stack page before installing any physical backing.
    pub fn admit_initial_page(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, ValueError> {
        if self.pages != 0 { return Ok(Default::default()); }
        let demand = self.initial_page_birth_demand()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history stack birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        *self = Self::try_new().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history stack original page allocation refused"))?;
        let progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original history stack first page")?;
        Ok(progress)
    }


    /// 🎟️ Reports all backing allocations required by exactly the next pushed entry.
    pub fn next_push_allocation_bytes(&self) -> usize {
        if self.len < self.capacity() { return 0; }
        size_of::<Option<T>>().saturating_mul(ARTIFACT_HISTORY_PAGE_SLOTS)
            .saturating_add(size_of::<HistoryValuePage<T>>())
            .saturating_add(if self.pages as usize % ARTIFACT_HISTORY_DIRECTORY_PAGES == 0 { size_of::<HistoryPageChunk<T>>() } else { 0 })
    }

    pub fn new() -> Self {
        Self::try_new().expect("history catalog first page")
    }

    pub fn try_new() -> Result<Self, &'static str> {
        let mut stack = Self::empty();
        stack.open_page().map_err(|_| "history catalog page allocation failed")?;
        Ok(stack)
    }

    pub(super) fn open_page(&mut self) -> Result<(), ()> {
        if self.pages == u32::MAX {
            return Err(());
        }
        let page = Box::new(HistoryValuePage { slots: history_value_page() });
        history_place_page(&mut self.head, self.pages as usize, page);
        self.pages += 1;
        Ok(())
    }

    fn value_page(&self, page: usize) -> &HistoryValuePage<T> {
        history_page(self.head.as_ref().expect("history catalog owns its first page"), page)
    }

    fn value_page_mut(&mut self, page: usize) -> &mut HistoryValuePage<T> {
        history_page_mut(self.head.as_mut().expect("history catalog owns its first page"), page)
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn capacity(&self) -> usize {
        self.pages as usize * ARTIFACT_HISTORY_PAGE_SLOTS
    }

    /// ➕ True while another entry can open a fresh page or fill an allocated slot.
    pub fn admits_one(&self) -> bool {
        self.len < self.capacity() || self.pages < u32::MAX
    }

    pub fn push(&mut self, value: T) {
        if self.try_push(value).is_err() { panic!("history catalog admits the entry"); }
    }

    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if let Some(page) = self.detached.take() {
            history_place_page(&mut self.head, self.pages as usize - 1, page);
        }
        if self.len == self.capacity() && self.open_page().is_err() {
            return Err(value);
        }
        let index = self.len;
        self.value_page_mut(index / ARTIFACT_HISTORY_PAGE_SLOTS).slots[index % ARTIFACT_HISTORY_PAGE_SLOTS] = Some(value);
        self.len += 1;
        Ok(())
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let index = self.len;
        self.value_page_mut(index / ARTIFACT_HISTORY_PAGE_SLOTS).slots[index % ARTIFACT_HISTORY_PAGE_SLOTS].take()
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "history catalog index outside the live order");
        let removed = self.value_page_mut(index / ARTIFACT_HISTORY_PAGE_SLOTS).slots[index % ARTIFACT_HISTORY_PAGE_SLOTS].take().expect("occupied history catalog slot");
        for cursor in index + 1..self.len {
            let next = self.value_page_mut(cursor / ARTIFACT_HISTORY_PAGE_SLOTS).slots[cursor % ARTIFACT_HISTORY_PAGE_SLOTS].take().expect("occupied history catalog slot");
            let previous = cursor - 1;
            self.value_page_mut(previous / ARTIFACT_HISTORY_PAGE_SLOTS).slots[previous % ARTIFACT_HISTORY_PAGE_SLOTS] = Some(next);
        }
        self.len -= 1;
        removed
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        (index < self.len).then(|| self.value_page(index / ARTIFACT_HISTORY_PAGE_SLOTS).slots[index % ARTIFACT_HISTORY_PAGE_SLOTS].as_ref().expect("occupied history catalog slot"))
    }

    pub fn last(&self) -> Option<&T> {
        self.len.checked_sub(1).and_then(|index| self.get(index))
    }

    /// 📏️ Borrows the exact extent the next empty-catalog release step frees: the last value page, or the directory
    /// chunk that page alone still occupies. Either fits one [`ARTIFACT_HISTORY_RELEASE_PAGE_BYTES`] release page.
    pub fn next_empty_page_release_byte_demand(&self) -> Option<usize> {
        if self.len != 0 { return None; }
        if self.pages == 0 { return Some(0); }
        if self.detached.is_none() && (self.pages as usize - 1) % ARTIFACT_HISTORY_DIRECTORY_PAGES == 0 { return Some(size_of::<HistoryPageChunk<T>>()); }
        size_of::<Option<T>>().checked_mul(ARTIFACT_HISTORY_PAGE_SLOTS)?.checked_add(size_of::<HistoryValuePage<T>>())
    }

    /// ♻️ Releases one physical extent of an empty catalog: a chunk whose only remaining page is first detached, so the
    /// chunk and that page leave in separate steps; callers retire entries before releasing resident storage.
    pub fn release_empty_page(&mut self) -> bool {
        if self.len != 0 || self.pages == 0 { return false; }
        if let Some(page) = self.detached.take() {
            drop(page);
            self.pages -= 1;
            return true;
        }
        let page = self.pages as usize - 1;
        let chunk_index = page / ARTIFACT_HISTORY_DIRECTORY_PAGES;
        let chunk = history_descend_chunk(self.head.as_mut().expect("resident empty catalog"), chunk_index);
        let taken = chunk.pages[page % ARTIFACT_HISTORY_DIRECTORY_PAGES].take();
        if page % ARTIFACT_HISTORY_DIRECTORY_PAGES != 0 {
            chunk.count -= 1;
            drop(taken);
            self.pages -= 1;
            return true;
        }
        self.detached = taken;
        if chunk_index == 0 { drop(self.head.take()); }
        else { drop(history_descend_chunk(self.head.as_mut().expect("resident catalog prefix"), chunk_index - 1).next.take()); }
        true
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq,
    {
        self.iter().any(|item| item == value)
    }

    pub fn iter(&self) -> HistoryPageIter<'_, T> {
        HistoryPageIter { stack: self, index: 0, end: self.len }
    }

    pub fn extend<I: IntoIterator<Item = T>>(&mut self, values: I) {
        for value in values {
            self.push(value);
        }
    }

    pub fn starts_with(&self, prefix: &[T]) -> bool
    where
        T: PartialEq,
    {
        prefix.len() <= self.len && self.iter().zip(prefix.iter()).all(|(item, expected)| item == expected)
    }

    pub fn is_prefix_of_slice(&self, full: &[T]) -> bool
    where
        T: PartialEq,
    {
        self.len <= full.len() && self.iter().zip(full.iter()).all(|(item, expected)| item == expected)
    }

    pub fn eq_slice(&self, other: &[T]) -> bool
    where
        T: PartialEq,
    {
        self.len == other.len() && self.iter().zip(other.iter()).all(|(item, expected)| item == expected)
    }

    pub fn to_vec(&self) -> Vec<T>
    where
        T: Clone,
    {
        self.iter().cloned().collect()
    }

    pub fn cloned_from(&self, from: usize) -> Vec<T>
    where
        T: Clone,
    {
        self.iter_from(from).cloned().collect()
    }

    /// ⏩️ The entries from `from` on, seeking there in O(1).
    pub fn iter_from(&self, from: usize) -> HistoryPageIter<'_, T> {
        HistoryPageIter { stack: self, index: from.min(self.len), end: self.len }
    }
}

impl<T> Default for HistoryPageStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> Clone for HistoryPageStack<T> {
    fn clone(&self) -> Self {
        let mut cloned = Self::new();
        while cloned.pages < self.pages {
            cloned.open_page().expect("cloned history catalog page");
        }
        for item in self.iter() {
            cloned.push(item.clone());
        }
        cloned
    }
}

impl<T: PartialEq> PartialEq for HistoryPageStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.len == other.len && self.iter().zip(other.iter()).all(|(left, right)| left == right)
    }
}

impl<T: Eq> Eq for HistoryPageStack<T> {}

impl<T: PartialEq> PartialEq<[T]> for HistoryPageStack<T> {
    fn eq(&self, other: &[T]) -> bool {
        self.eq_slice(other)
    }
}

impl<T: PartialEq, const N: usize> PartialEq<[T; N]> for HistoryPageStack<T> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.eq_slice(other)
    }
}

impl<T: PartialEq> PartialEq<Vec<T>> for HistoryPageStack<T> {
    fn eq(&self, other: &Vec<T>) -> bool {
        self.eq_slice(other)
    }
}

impl<T: PartialEq> PartialEq<HistoryPageStack<T>> for Vec<T> {
    fn eq(&self, other: &HistoryPageStack<T>) -> bool {
        other.eq_slice(self)
    }
}

impl<T: std::hash::Hash> std::hash::Hash for HistoryPageStack<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.len.hash(state);
        for item in self.iter() {
            item.hash(state);
        }
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for HistoryPageStack<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T> FromIterator<T> for HistoryPageStack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self {
        let mut stack = Self::new();
        stack.extend(values);
        stack
    }
}

impl<T> From<Vec<T>> for HistoryPageStack<T> {
    fn from(values: Vec<T>) -> Self {
        let mut stack = Self::new();
        stack.extend(values);
        stack
    }
}

impl<T> std::ops::Index<usize> for HistoryPageStack<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("history catalog index outside the live order")
    }
}

impl<'a, T> IntoIterator for &'a HistoryPageStack<T> {
    type Item = &'a T;
    type IntoIter = HistoryPageIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct HistoryPageIter<'a, T> {
    stack: &'a HistoryPageStack<T>,
    pub(super) index: usize,
    pub(super) end: usize,
}

impl<'a, T> Iterator for HistoryPageIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }
        let item = self.stack.get(self.index)?;
        self.index += 1;
        Some(item)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.end.saturating_sub(self.index);
        (remaining, Some(remaining))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.index = self.index.saturating_add(n).min(self.end);
        self.next()
    }
}

impl<T> DoubleEndedIterator for HistoryPageIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index >= self.end {
            return None;
        }
        self.end -= 1;
        self.stack.get(self.end)
    }
}

impl<T> std::iter::FusedIterator for HistoryPageIter<'_ , T> {}

impl<T> ExactSizeIterator for HistoryPageIter<'_, T> {}

impl<T: ToValue> ToValue for HistoryPageStack<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for HistoryPageStack<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let values = Vec::<T>::from_value(value)?;
        let mut stack = Self::try_new().map_err(|reason|ValueError::new(protocol::value::ValueRefusalKind::AllocationFailed,reason))?;
        for value in values {
            stack.try_push(value).map_err(|_| ValueError::new(protocol::value::ValueRefusalKind::OwnershipLimit,"history catalog address space exhausted"))?;
        }
        Ok(stack)
    }
}

/// 📭 Empty when the live order is empty. Resident pages are not part of the wire.
pub fn history_id_stack_is_empty(stack: &HistoryPageStack<String>) -> bool {
    stack.is_empty()
}

#[cfg(test)]
impl<T: serde::Serialize> serde::Serialize for HistoryPageStack<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(self.iter())
    }
}

#[cfg(test)]
impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for HistoryPageStack<T> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let values = Vec::<T>::deserialize(deserializer)?;
        Ok(values.into())
    }
}
