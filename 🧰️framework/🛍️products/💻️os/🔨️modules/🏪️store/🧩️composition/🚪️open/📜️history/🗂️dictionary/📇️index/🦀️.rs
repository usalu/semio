//! 📇️ Private bounded range index; offsets alone convey no input-reading authority.
//! The retained record owner validates UTF-8/EOF and supplies ranges from its own witness.

use std::mem::ManuallyDrop;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum DictionaryIndexClose {
    Complete,
    Pending { released_items: usize, released_bytes: usize },
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
    close_offset: usize,
}

impl RetainedDictionaryIndex {
    pub(super) fn new(verified_end: u64, maximum_entries: usize, maximum_bytes: u64) -> Result<Self, DictionaryIndexError> {
        if maximum_entries > PAGE_ENTRIES * MAXIMUM_PAGES || maximum_bytes > 1_048_576 {
            return Err(DictionaryIndexError::Capacity);
        }
        Ok(Self { pages: ManuallyDrop::new(std::array::from_fn(|_| None)), allocated: 0, visible: 0, dictionary_bytes: 0, maximum_entries, maximum_bytes, verified_end, last_end: 0, delta: None, diagnostic: None, closing: false, close_offset: 0 })
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

    pub(super) fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> DictionaryIndexClose {
        self.closing = true;
        self.delta = None;
        self.visible = 0;
        if self.allocated == 0 {
            return DictionaryIndexClose::Complete;
        }
        if maximum_items == 0 || maximum_bytes == 0 {
            return DictionaryIndexClose::Pending { released_items: 0, released_bytes: 0 };
        }
        let count = maximum_bytes.min(PAGE_BYTES - self.close_offset);
        let page = self.pages[self.allocated - 1].as_mut().expect("dense private range pages");
        for offset in self.close_offset..self.close_offset + count {
            let entry = &mut page[offset / 16];
            let byte = offset % 16;
            let field = if byte < 8 { &mut entry.offset } else { &mut entry.length };
            *field &= !(255u64 << ((byte % 8) * 8));
        }
        self.close_offset += count;
        let released_items = if self.close_offset == PAGE_BYTES {
            self.allocated -= 1;
            self.pages[self.allocated].take();
            self.close_offset = 0;
            1
        } else {
            0
        };
        DictionaryIndexClose::Pending { released_items, released_bytes: count }
    }

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.closing && self.allocated == 0 && self.delta.is_none()
    }
}

impl Drop for RetainedDictionaryIndex {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "dictionary range pages require bounded retirement");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn retire(index: &mut RetainedDictionaryIndex, grant: usize) -> usize {
        let mut bytes = 0;
        let mut pages = 0;
        let expected = index.allocated_pages();
        for _ in 0..4096 {
            match index.close_step(1, grant) {
                DictionaryIndexClose::Complete => {
                    assert_eq!(pages, expected);
                    assert!(index.terminal_is_empty());
                    return bytes;
                }
                DictionaryIndexClose::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= grant);
                    pages += released_items;
                    bytes += released_bytes;
                    assert_eq!(index.lookup(0), Err(DictionaryIndexError::State));
                }
            }
        }
        panic!("bounded range retirement failed to converge");
    }

    #[test]
    fn retained_dictionary_range_index_never_publishes_partial_delta_and_retires_two_pages() {
        assert_eq!(std::mem::size_of::<[DictionaryRange; PAGE_ENTRIES]>(), PAGE_BYTES);
        for grant in [1, 7, 4096] {
            let mut index = RetainedDictionaryIndex::new(10_000, 8192, 1048576).unwrap();
            index.begin_delta(0, 65).unwrap();
            for entry in 0..65 {
                index.append(DictionaryRange { offset: 32 + entry, length: 1 }).unwrap();
                assert_eq!(index.visible_entries(), 0);
                assert_eq!(index.lookup(entry as usize), Err(DictionaryIndexError::Malformed));
            }
            assert_eq!(index.allocated_pages(), 2);
            index.publish_delta().unwrap();
            assert_eq!(index.visible_entries(), 65);
            assert_eq!(index.lookup(64), Ok(DictionaryRange { offset: 96, length: 1 }));
            assert_eq!(index.dictionary_bytes(), 65);
            index.begin_delta(65, 2).unwrap();
            index.append(DictionaryRange { offset: 100, length: 1 }).unwrap();
            index.reject_record();
            assert_eq!(index.visible_entries(), 65);
            assert_eq!(index.lookup(0), Err(DictionaryIndexError::Malformed));
            assert_eq!(retire(&mut index, grant), 2048);
        }
    }

    #[test]
    fn retained_dictionary_range_index_caps_all_deltas_and_retains_late_rejections() {
        for grant in [1, 7, 4096] {
            let mut index = RetainedDictionaryIndex::new(100, 3, 4).unwrap();
            index.begin_delta(0, 1).unwrap();
            index.append(DictionaryRange { offset: 32, length: 3 }).unwrap();
            index.publish_delta().unwrap();
            index.begin_delta(1, 2).unwrap();
            index.append(DictionaryRange { offset: 35, length: 1 }).unwrap();
            assert_eq!(index.append(DictionaryRange { offset: 36, length: 1 }), Err(DictionaryIndexError::Capacity));
            assert_eq!(index.visible_entries(), 1);
            assert_eq!(index.dictionary_bytes(), 4);
            assert_eq!(retire(&mut index, grant), 1024);
            for range in [DictionaryRange { offset: 99, length: 2 }, DictionaryRange { offset: u64::MAX, length: 1 }] {
                let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
                index.begin_delta(0, 1).unwrap();
                assert_eq!(index.append(range), Err(DictionaryIndexError::Malformed));
                assert_eq!(retire(&mut index, grant), 0);
            }
            let mut index = RetainedDictionaryIndex::new(100, 1, 1048576).unwrap();
            assert_eq!(index.begin_delta(1, 1), Err(DictionaryIndexError::Malformed));
            assert_eq!(retire(&mut index, grant), 0);
        }
    }
}
