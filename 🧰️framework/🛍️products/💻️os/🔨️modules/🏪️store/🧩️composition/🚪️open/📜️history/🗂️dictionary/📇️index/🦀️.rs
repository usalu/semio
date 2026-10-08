//! 📇️ Private bounded range index; offsets alone convey no input-reading authority.
//! The retained record owner validates UTF-8/EOF and supplies ranges from its own witness.

use std::mem::ManuallyDrop;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

const PAGE_ENTRIES: usize = 64;
const MAXIMUM_PAGES: usize = 128;
const PAGE_BYTES: usize = 1024;

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub(super) struct DictionaryRange {
    pub offset: u64,
    pub length: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DictionaryIndexError {
    Malformed,
    Capacity,
    State,
}

#[derive(Clone, Copy)]
struct Delta {
    remaining: usize,
    staged: usize,
}

/// 🧱️ At most one fixed page is allocated by append; failed deltas remain private until close.
pub(super) struct RetainedDictionaryIndex {
    pages: ManuallyDrop<[Option<Box<[DictionaryRange; PAGE_ENTRIES]>>; MAXIMUM_PAGES]>,
    allocated: usize,
    visible: usize,
    dictionary_bytes: u64,
    maximum_entries: usize,
    maximum_bytes: u64,
    verified_end: u64,
    last_end: u64,
    delta: Option<Delta>,
    diagnostic: Option<DictionaryIndexError>,
    closing: bool,
}

impl RetainedDictionaryIndex {
    pub(super) fn new(verified_end: u64, maximum_entries: usize, maximum_bytes: u64) -> Result<Self, DictionaryIndexError> {
        if maximum_entries > PAGE_ENTRIES * MAXIMUM_PAGES || maximum_bytes > 1_048_576 {
            return Err(DictionaryIndexError::Capacity);
        }
        Ok(Self { pages: ManuallyDrop::new(std::array::from_fn(|_| None)), allocated: 0, visible: 0, dictionary_bytes: 0, maximum_entries, maximum_bytes, verified_end, last_end: 0, delta: None, diagnostic: None, closing: false })
    }

    fn reject<T>(&mut self, diagnostic: DictionaryIndexError) -> Result<T, DictionaryIndexError> {
        Err(*self.diagnostic.get_or_insert(diagnostic))
    }
    fn check(&self) -> Result<(), DictionaryIndexError> {
        if self.closing {
            return Err(DictionaryIndexError::State);
        }
        self.diagnostic.map_or(Ok(()), Err)
    }

    pub(super) fn begin_delta(&mut self, base: u64, count: u64) -> Result<(), DictionaryIndexError> {
        self.check()?;
        if self.delta.is_some() {
            return self.reject(DictionaryIndexError::State);
        }
        if base != self.visible as u64 {
            return self.reject(DictionaryIndexError::Malformed);
        }
        if count > (self.maximum_entries - self.visible) as u64 {
            return self.reject(DictionaryIndexError::Capacity);
        }
        self.delta = Some(Delta { remaining: count as usize, staged: 0 });
        Ok(())
    }

    pub(super) fn append(&mut self, range: DictionaryRange) -> Result<(), DictionaryIndexError> {
        self.check()?;
        let Some(delta) = self.delta else {
            return self.reject(DictionaryIndexError::State);
        };
        if delta.remaining == 0 {
            return self.reject(DictionaryIndexError::Malformed);
        }
        let Some(end) = range.offset.checked_add(range.length) else {
            return self.reject(DictionaryIndexError::Malformed);
        };
        if range.offset < self.last_end || end > self.verified_end {
            return self.reject(DictionaryIndexError::Malformed);
        }
        if range.length > self.maximum_bytes - self.dictionary_bytes {
            return self.reject(DictionaryIndexError::Capacity);
        }
        let index = self.visible + delta.staged;
        let page = index / PAGE_ENTRIES;
        if self.pages[page].is_none() {
            self.pages[page] = Some(Box::new([DictionaryRange::default(); PAGE_ENTRIES]));
            self.allocated += 1;
        }
        self.pages[page].as_mut().expect("admitted fixed page")[index % PAGE_ENTRIES] = range;
        self.dictionary_bytes += range.length;
        self.last_end = end;
        self.delta = Some(Delta { remaining: delta.remaining - 1, staged: delta.staged + 1 });
        Ok(())
    }

    pub(super) fn publish_delta(&mut self) -> Result<(), DictionaryIndexError> {
        self.check()?;
        let Some(delta) = self.delta else {
            return self.reject(DictionaryIndexError::State);
        };
        if delta.remaining != 0 {
            return self.reject(DictionaryIndexError::Malformed);
        }
        self.visible += delta.staged;
        self.delta = None;
        Ok(())
    }

    pub(super) fn reject_record(&mut self) {
        self.diagnostic.get_or_insert(DictionaryIndexError::Malformed);
    }
    pub(super) fn visible_entries(&self) -> usize {
        self.visible
    }
    pub(super) fn allocated_pages(&self) -> usize {
        self.allocated
    }
    pub(super) fn dictionary_bytes(&self) -> u64 {
        self.dictionary_bytes
    }

    pub(super) fn lookup(&self, index: usize) -> Result<DictionaryRange, DictionaryIndexError> {
        self.check()?;
        if index >= self.visible {
            return Err(DictionaryIndexError::Malformed);
        }
        Ok(self.pages[index / PAGE_ENTRIES].as_ref().ok_or(DictionaryIndexError::State)?[index % PAGE_ENTRIES])
    }

    pub(super) fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(0) }
    pub(super) fn next_capacity_byte_demand(&self, _body: usize) -> Result<usize, ValueError> { Ok(0) }
    pub(super) fn next_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(if self.allocated == 0 { 0 } else { PAGE_BYTES }) }
    pub(super) fn next_depth_demand(&self) -> Result<usize, ValueError> { Ok(usize::from(!self.terminal_is_empty())) }

    pub(super) fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        let empty = RetainedCloneProgress::default();
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit, "dictionary backing requires admitted depth")); }
        if self.allocated != 0 && grant.maximum_release_bytes < PAGE_BYTES { return Ok(RetainedCloneStep::Progress(empty)); }
        self.closing = true;
        self.delta = None;
        self.visible = 0;
        if self.allocated == 0 { return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..empty })); }
        self.allocated -= 1;
        drop(self.pages[self.allocated].take());
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: PAGE_BYTES, ..empty }))
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.closing && self.allocated == 0 && self.delta.is_none()
    }
}

impl semio_framework_value::ErasedSnapshotRetirement for RetainedDictionaryIndex {
    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> { RetainedDictionaryIndex::close_step(self, grant) }
    fn terminal_is_empty(&self) -> bool { RetainedDictionaryIndex::terminal_is_empty(self) }
    fn next_copy_byte_demand(&self) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_copy_byte_demand(self) }
    fn next_capacity_byte_demand(&self, body: usize) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_capacity_byte_demand(self, body) }
    fn next_release_byte_demand(&self) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_release_byte_demand(self) }
    fn next_depth_demand(&self) -> Result<usize, ValueError> { RetainedDictionaryIndex::next_depth_demand(self) }
}

impl Drop for RetainedDictionaryIndex {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary range pages require bounded retirement");
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
