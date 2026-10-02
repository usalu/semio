//! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_progress, admit_retained_clone_retirement};
use crate::{SnapshotRetirementStep, retirement::RetireOwned};
use serde::{Serialize, Serializer, ser::SerializeMap};
use std::{cmp::Ordering, mem::size_of, sync::Arc};

pub const RETAINED_ORDERED_MAP_PAGE_CAPACITY: usize = 16;

/// 🗺️ Stores strictly ordered entries in fixed-capacity native pages with resumable directory growth.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RetainedOrderedMap<K, V> {
    pages: Vec<Vec<(K, V)>>,
    len: usize,
}

impl<K, V> Default for RetainedOrderedMap<K, V> {
    fn default() -> Self {
        Self { pages: Vec::new(), len: 0 }
    }
}

impl<K, V> RetainedOrderedMap<K, V> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn page_count(&self) -> usize {
        self.pages.len()
    }

    pub fn get_index(&self, ordinal: usize) -> Option<(&K, &V)> {
        if ordinal >= self.len {
            return None;
        }
        let page = self.pages.get(ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        let (key, value) = page.get(ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY)?;
        Some((key, value))
    }

    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.pages.iter().flat_map(|page| page.iter().map(|(key, value)| (key, value)))
    }

    #[cfg(test)]
    pub(crate) fn from_sorted_entries_for_test(entries: Vec<(K, V)>) -> Result<Self, String>
    where
        K: Ord,
    {
        if entries.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
            return Err("retained ordered-map fixture entries are not strictly ordered".into());
        }
        let len = entries.len();
        let page_count = len.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        let mut pages = Vec::with_capacity(page_count.checked_add(1).ok_or("retained ordered-map fixture page count overflow")?);
        let mut entries = entries.into_iter();
        for _ in 0..page_count {
            let mut page = Vec::with_capacity(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
            page.extend(entries.by_ref().take(RETAINED_ORDERED_MAP_PAGE_CAPACITY));
            pages.push(page);
        }
        Ok(Self { pages, len })
    }
}

impl<K: Serialize, V: Serialize> Serialize for RetainedOrderedMap<K, V> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(self.len))?;
        for (key, value) in self.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}

impl<K: RetireOwned, V: RetireOwned> RetireOwned for RetainedOrderedMap<K, V> {
    fn retirement(self) -> Box<dyn crate::retirement::RetirementCursor> {
        self.pages.retirement()
    }
}

pub struct RetainedOrderedMapCloneCursor<K: RetainedClone, V: RetainedClone> {
    pages: Vec<Vec<(K, V)>>,
    page_output: Option<Vec<(K, V)>>,
    key: Option<K>,
    value: Option<V>,
    key_cursor: K::Cursor,
    value_cursor: V::Cursor,
    page: usize,
    entry: usize,
    source_page_len: usize,
    phase: u8,
    source: Option<RetainedCloneBinding>,
    output: Option<RetainedOrderedMap<K, V>>,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<K: RetainedClone, V: RetainedClone> Default for RetainedOrderedMapCloneCursor<K, V> {
    fn default() -> Self {
        Self {
            pages: Vec::new(),
            page_output: None,
            key: None,
            value: None,
            key_cursor: K::retained_clone_cursor(),
            value_cursor: V::retained_clone_cursor(),
            page: 0,
            entry: 0,
            source_page_len: 0,
            phase: 0,
            source: None,
            output: None,
            spent: false,
            closing: false,
            close: RetainedCloneClose::default(),
        }
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedOrderedMapCloneCursor<K, V> {
    fn progress_from_close(step: SnapshotRetirementStep, terminal_is_empty: bool, grant: RetainedCloneGrant, label: &str) -> Result<RetainedCloneStep, String> {
        match admit_retained_clone_retirement(step, grant.maximum_items, grant.maximum_copy_bytes, &format!("retained ordered-map {label} scaffold close"))? {
            SnapshotRetirementStep::Pending { released_items, released_bytes } => Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: released_items, copied_bytes: released_bytes, retained_capacity_bytes: 0 })),
            SnapshotRetirementStep::Blocked => Err(format!("retained ordered-map {label} scaffold close blocked")),
            SnapshotRetirementStep::Complete if terminal_is_empty => Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default())),
            SnapshotRetirementStep::Complete => Err(format!("retained ordered-map {label} scaffold completed with a live owner")),
        }
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V> {
    fn advance(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, String> {
        if self.closing {
            return Err("retained ordered-map clone cursor is closing".into());
        }
        if self.spent {
            return Err("retained ordered-map clone cursor is spent".into());
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        source.bind(&mut self.source)?;
        let source_value = source.get();
        match self.phase {
            0 => {
                let planned_pages = source_value.pages.len().checked_add(1).ok_or("retained ordered-map directory page count overflow")?;
                let planned_capacity = planned_pages.checked_mul(size_of::<Vec<(K, V)>>()).ok_or("retained ordered-map directory capacity overflow")?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned_capacity };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.pages.try_reserve_exact(planned_pages).map_err(|_| "retained ordered-map directory allocation failed")?;
                let actual = self.pages.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or("retained ordered-map directory actual capacity overflow")?;
                if actual > grant.maximum_capacity_bytes {
                    return Err("retained ordered-map directory allocator exceeded its admitted capacity".into());
                }
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            1 => {
                if let Some(page) = self.page_output.take() {
                    if self.entry != self.source_page_len {
                        self.page_output = Some(page);
                    } else {
                        if grant.maximum_items == 0 {
                            self.page_output = Some(page);
                            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                        }
                        self.pages.push(page);
                        self.page += 1;
                        self.entry = 0;
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                    }
                }
                if self.page == source_value.pages.len() {
                    if grant.maximum_items == 0 {
                        return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                    }
                    self.output = Some(RetainedOrderedMap { pages: std::mem::take(&mut self.pages), len: source_value.len });
                    self.phase = 7;
                    return Ok(RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
                }
                if self.page_output.is_some() {
                    self.phase = 2;
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let source_page = source_value.pages.get(self.page).ok_or("retained ordered-map source page is missing")?;
                if source_page.is_empty() || source_page.len() > RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err("retained ordered-map source page violates its fixed capacity".into());
                }
                let planned = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map page capacity overflow")?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut page = Vec::new();
                page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| "retained ordered-map page allocation failed")?;
                let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map page actual capacity overflow")?;
                if actual > grant.maximum_capacity_bytes {
                    return Err("retained ordered-map page allocator exceeded its admitted capacity".into());
                }
                self.source_page_len = source_page.len();
                self.page_output = Some(page);
                self.phase = 2;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { retained_capacity_bytes: actual, ..progress }))
            }
            2 => {
                let ordinal = self.page * RETAINED_ORDERED_MAP_PAGE_CAPACITY + self.entry;
                match self.key_cursor.advance(source.project(ordinal * 2 + 1, |map| &map.pages[self.page][self.entry].0), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained ordered-map key")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained ordered-map key")?;
                        self.key = Some(self.key_cursor.take().ok_or("retained ordered-map key completed without an owner")?);
                        let _ = self.key_cursor.begin_close();
                        self.phase = 3;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            3 => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = self.key_cursor.close_step(grant.maximum_items, grant.maximum_copy_bytes)?;
                    return Self::progress_from_close(step, self.key_cursor.terminal_is_empty(), grant, "key");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.key_cursor = K::retained_clone_cursor();
                self.phase = 4;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            4 => {
                let ordinal = self.page * RETAINED_ORDERED_MAP_PAGE_CAPACITY + self.entry;
                match self.value_cursor.advance(source.project(ordinal * 2 + 2, |map| &map.pages[self.page][self.entry].1), grant)? {
                    RetainedCloneStep::Progress(progress) => Ok(RetainedCloneStep::Progress(admit_retained_clone_progress(grant, progress, "retained ordered-map value")?)),
                    RetainedCloneStep::Complete(progress) => {
                        let progress = admit_retained_clone_progress(grant, progress, "retained ordered-map value")?;
                        self.value = Some(self.value_cursor.take().ok_or("retained ordered-map value completed without an owner")?);
                        let _ = self.value_cursor.begin_close();
                        self.phase = 5;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            5 => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = self.value_cursor.close_step(grant.maximum_items, grant.maximum_copy_bytes)?;
                    return Self::progress_from_close(step, self.value_cursor.terminal_is_empty(), grant, "value");
                }
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.value_cursor = V::retained_clone_cursor();
                self.phase = 6;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            6 => {
                if grant.maximum_items == 0 {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.page_output.as_mut().ok_or("retained ordered-map output page is missing")?.push((self.key.take().ok_or("retained ordered-map key owner is missing")?, self.value.take().ok_or("retained ordered-map value owner is missing")?));
                self.entry += 1;
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            7 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err("retained ordered-map clone state is invalid".into()),
        }
    }

    fn take(&mut self) -> Option<RetainedOrderedMap<K, V>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(self.key_cursor.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained ordered-map key close")?;
            if step == SnapshotRetirementStep::Complete {
                if !self.key_cursor.terminal_is_empty() {
                    return Err("retained ordered-map key cursor completed close with a live owner".into());
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() {
                if maximum_items == 0 {
                    return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            let step = admit_retained_clone_retirement(self.value_cursor.close_step(maximum_items, maximum_bytes)?, maximum_items, maximum_bytes, "retained ordered-map value close")?;
            if step == SnapshotRetirementStep::Complete {
                if !self.value_cursor.terminal_is_empty() {
                    return Err("retained ordered-map value cursor completed close with a live owner".into());
                }
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            return Ok(step);
        }
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.key, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.page_output, maximum_items)? {
            return Ok(step);
        }
        if !self.pages.is_empty() || self.pages.capacity() != 0 {
            if maximum_items == 0 {
                return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            }
            self.close.begin(std::mem::take(&mut self.pages))?;
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        self.source = None;
        Ok(SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.pages.is_empty()
            && self.page_output.is_none()
            && self.key.is_none()
            && self.value.is_none()
            && self.output.is_none()
            && self.key_cursor.terminal_is_empty()
            && self.value_cursor.terminal_is_empty()
            && self.close.is_empty()
            && self.source.is_none()
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedClone for RetainedOrderedMap<K, V> {
    type Cursor = RetainedOrderedMapCloneCursor<K, V>;
    fn retained_clone_cursor() -> Self::Cursor {
        RetainedOrderedMapCloneCursor::default()
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdGrant {
    pub maximum_items: usize,
    pub maximum_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BoundedOrdProgress {
    pub compared_items: usize,
    pub compared_bytes: usize,
}

impl BoundedOrdProgress {
    pub fn fits(self, grant: BoundedOrdGrant) -> bool {
        self.compared_items <= grant.maximum_items && self.compared_bytes <= grant.maximum_bytes
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoundedOrdStep {
    Progress(BoundedOrdProgress),
    Complete { ordering: Ordering, progress: BoundedOrdProgress },
}

pub trait BoundedOrd: Ord + Send + Sync + Sized + 'static {
    type Cursor: BoundedOrdCursor<Self>;
    fn bounded_ord_cursor() -> Self::Cursor;
}

pub trait BoundedOrdCursor<T: BoundedOrd>: Send {
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, String>;
}

pub struct ScalarBoundedOrdCursor<T> {
    complete: Option<Ordering>,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Default for ScalarBoundedOrdCursor<T> {
    fn default() -> Self {
        Self { complete: None, left: None, right: None, marker: std::marker::PhantomData }
    }
}

impl<T: BoundedOrd + Copy> BoundedOrdCursor<T> for ScalarBoundedOrdCursor<T> {
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, String> {
        if let Some(ordering) = self.complete {
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress::default() });
        }
        left.bind(&mut self.left)?;
        right.bind(&mut self.right)?;
        let progress = BoundedOrdProgress { compared_items: 1, compared_bytes: size_of::<T>() };
        if !progress.fits(grant) {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        let ordering = left.get().cmp(right.get());
        self.complete = Some(ordering);
        Ok(BoundedOrdStep::Complete { ordering, progress })
    }
}

macro_rules! bounded_ord_scalar {
    ($($type:ty),+ $(,)?) => {$ (
        impl BoundedOrd for $type {
            type Cursor = ScalarBoundedOrdCursor<Self>;
            fn bounded_ord_cursor() -> Self::Cursor { ScalarBoundedOrdCursor::default() }
        }
    )+ };
}

bounded_ord_scalar!(bool, char, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize);

#[derive(Default)]
pub struct StringBoundedOrdCursor {
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    offset: usize,
    complete: Option<Ordering>,
}

impl BoundedOrdCursor<String> for StringBoundedOrdCursor {
    fn compare(&mut self, left: RetainedCloneRef<'_, String>, right: RetainedCloneRef<'_, String>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, String> {
        if let Some(ordering) = self.complete {
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress::default() });
        }
        let first = self.left.is_none();
        left.bind(&mut self.left)?;
        right.bind(&mut self.right)?;
        if first {
            if grant.maximum_items == 0 {
                self.left = None;
                self.right = None;
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        let left = left.get().as_bytes();
        let right = right.get().as_bytes();
        let common = left.len().min(right.len());
        if self.offset == common {
            if grant.maximum_items == 0 {
                return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
            }
            let ordering = left.len().cmp(&right.len());
            self.complete = Some(ordering);
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 1, compared_bytes: 0 } });
        }
        if grant.maximum_bytes == 0 {
            return Ok(BoundedOrdStep::Progress(BoundedOrdProgress::default()));
        }
        let end = common.min(self.offset.saturating_add(grant.maximum_bytes));
        let start = self.offset;
        while self.offset < end {
            let ordering = left[self.offset].cmp(&right[self.offset]);
            self.offset += 1;
            if ordering != Ordering::Equal {
                self.complete = Some(ordering);
                return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start } });
            }
        }
        Ok(BoundedOrdStep::Progress(BoundedOrdProgress { compared_items: 0, compared_bytes: self.offset - start }))
    }
}

impl BoundedOrd for String {
    type Cursor = StringBoundedOrdCursor;
    fn bounded_ord_cursor() -> Self::Cursor {
        StringBoundedOrdCursor::default()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapLookup {
    Found(usize),
    Missing(usize),
}

pub struct RetainedOrderedMapLookupCursor<K: BoundedOrd> {
    low: usize,
    high: usize,
    middle: usize,
    comparison: Option<K::Cursor>,
    pending: Option<Ordering>,
    source: Option<RetainedCloneBinding>,
    complete: Option<RetainedOrderedMapLookup>,
}

impl<K: BoundedOrd> Default for RetainedOrderedMapLookupCursor<K> {
    fn default() -> Self {
        Self { low: 0, high: 0, middle: 0, comparison: None, pending: None, source: None, complete: None }
    }
}

impl<K: BoundedOrd> RetainedOrderedMapLookupCursor<K> {
    pub fn advance<V>(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, target: RetainedCloneRef<'_, K>, grant: BoundedOrdGrant) -> Result<(Option<RetainedOrderedMapLookup>, BoundedOrdProgress), String> {
        if let Some(result) = self.complete {
            return Ok((Some(result), BoundedOrdProgress::default()));
        }
        let first = self.source.is_none();
        source.bind(&mut self.source)?;
        let source_value = source.get();
        if first {
            if grant.maximum_items == 0 {
                self.source = None;
                return Ok((None, BoundedOrdProgress::default()));
            }
            self.high = source_value.len();
            return Ok((None, BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        if let Some(ordering) = self.pending.take() {
            if grant.maximum_items == 0 {
                self.pending = Some(ordering);
                return Ok((None, BoundedOrdProgress::default()));
            }
            match ordering {
                Ordering::Less => self.low = self.middle + 1,
                Ordering::Greater => self.high = self.middle,
                Ordering::Equal => unreachable!(),
            }
            self.comparison = None;
            return Ok((None, BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        if self.low == self.high {
            if grant.maximum_items == 0 {
                return Ok((None, BoundedOrdProgress::default()));
            }
            let result = RetainedOrderedMapLookup::Missing(self.low);
            self.complete = Some(result);
            return Ok((Some(result), BoundedOrdProgress { compared_items: 1, compared_bytes: 0 }));
        }
        self.middle = self.low + (self.high - self.low) / 2;
        let comparison = self.comparison.get_or_insert_with(K::bounded_ord_cursor);
        match comparison.compare(source.project(self.middle + 1, |map| map.get_index(self.middle).expect("validated retained ordered-map ordinal").0), target, grant)? {
            BoundedOrdStep::Progress(progress) => Ok((None, progress)),
            BoundedOrdStep::Complete { ordering: Ordering::Equal, progress } => {
                let result = RetainedOrderedMapLookup::Found(self.middle);
                self.complete = Some(result);
                Ok((Some(result), progress))
            }
            BoundedOrdStep::Complete { ordering, progress } => {
                self.pending = Some(ordering);
                Ok((None, progress))
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapInsertStep {
    Progress(RetainedOrderedMapInsertProgress),
    Complete { ordinal: usize, progress: RetainedOrderedMapInsertProgress },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertGrant {
    pub comparison: BoundedOrdGrant,
    pub maximum_moved_items: usize,
    pub maximum_moved_bytes: usize,
    pub maximum_capacity_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertProgress {
    pub comparison: BoundedOrdProgress,
    pub moved_items: usize,
    pub moved_bytes: usize,
    pub retained_capacity_bytes: usize,
}

impl RetainedOrderedMapInsertProgress {
    pub fn fits(self, grant: RetainedOrderedMapInsertGrant) -> bool {
        self.comparison.fits(grant.comparison) && self.moved_items <= grant.maximum_moved_items && self.moved_bytes <= grant.maximum_moved_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
    }
}

/// 🧯 Owns one exclusive unpublished workspace through completion or bounded cancellation.
pub struct RetainedOrderedMapInsertCursor<K: BoundedOrd + RetireOwned, V: RetireOwned> {
    map: Option<RetainedOrderedMap<K, V>>,
    output: Option<RetainedOrderedMap<K, V>>,
    key: Option<K>,
    value: Option<V>,
    lookup: RetainedOrderedMapLookupCursor<K>,
    lookup_lease: Arc<super::RetainedCloneLeaseOwner>,
    target: Option<usize>,
    shift_page: Option<usize>,
    old_directory: Option<Vec<Vec<(K, V)>>>,
    new_directory: Option<Vec<Vec<(K, V)>>>,
    directory_index: usize,
    phase: u8,
    spent: bool,
    closing: bool,
    close: RetainedCloneClose,
}

impl<K: BoundedOrd + RetireOwned, V: RetireOwned> RetainedOrderedMapInsertCursor<K, V> {
    pub fn new(map: RetainedOrderedMap<K, V>, key: K, value: V) -> Self {
        Self {
            map: Some(map),
            output: None,
            key: Some(key),
            value: Some(value),
            lookup: Default::default(),
            lookup_lease: super::retained_clone_exclusive_lease(),
            target: None,
            shift_page: None,
            old_directory: None,
            new_directory: None,
            directory_index: 0,
            phase: 0,
            spent: false,
            closing: false,
            close: Default::default(),
        }
    }

    pub fn advance(&mut self, grant: RetainedOrderedMapInsertGrant) -> Result<RetainedOrderedMapInsertStep, String> {
        if self.closing {
            return Err("retained ordered-map insertion cursor is closing".into());
        }
        if self.spent {
            return Err("retained ordered-map insertion cursor is spent".into());
        }
        match self.phase {
            0 => {
                let map = self.map.as_ref().ok_or("retained ordered-map insertion workspace is missing")?;
                let key = self.key.as_ref().ok_or("retained ordered-map insertion key is missing")?;
                let map_ref = super::retained_clone_exclusive_ref(map, &self.lookup_lease, 1);
                let key_ref = super::retained_clone_exclusive_ref(key, &self.lookup_lease, 2);
                let (result, comparison) = self.lookup.advance(map_ref, key_ref, grant.comparison)?;
                let progress = RetainedOrderedMapInsertProgress { comparison, ..Default::default() };
                match result {
                    None => Ok(RetainedOrderedMapInsertStep::Progress(progress)),
                    Some(RetainedOrderedMapLookup::Found(_)) => {
                        self.phase = 255;
                        Err("retained ordered-map insertion refused a duplicate key".into())
                    }
                    Some(RetainedOrderedMapLookup::Missing(ordinal)) => {
                        self.target = Some(ordinal);
                        self.phase = 1;
                        Ok(RetainedOrderedMapInsertStep::Progress(progress))
                    }
                }
            }
            1 => {
                let map = self.map.as_mut().ok_or("retained ordered-map insertion workspace is missing")?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        self.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map insertion page capacity overflow")?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| "retained ordered-map insertion page allocation failed")?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map insertion page actual capacity overflow")?;
                    if actual > grant.maximum_capacity_bytes {
                        return Err("retained ordered-map insertion page allocator exceeded its admitted capacity".into());
                    }
                    map.pages.push(page);
                    self.shift_page = map.pages.len().checked_sub(1);
                    self.phase = 2;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }));
                }
                if grant.maximum_moved_items == 0 {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                self.shift_page = map.pages.len().checked_sub(1);
                self.phase = 2;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()))
            }
            2 => {
                let map = self.map.as_mut().ok_or("retained ordered-map insertion workspace is missing")?;
                let ordinal = self.target.ok_or("retained ordered-map insertion target is missing")?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = self.shift_page.ok_or("retained ordered-map insertion shift page is missing")?;
                if shift_page == target_page {
                    self.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or("retained ordered-map insertion previous page is missing")?;
                let current = right.first_mut().ok_or("retained ordered-map insertion current page is missing")?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err("retained ordered-map insertion shift page has no admitted slot".into());
                }
                let moved_items = current.len().checked_add(1).ok_or("retained ordered-map insertion move count overflow")?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map insertion shift overflow")?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let entry = previous.pop().ok_or("retained ordered-map insertion previous page is empty")?;
                current.insert(0, entry);
                self.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = self.map.as_mut().ok_or("retained ordered-map insertion workspace is missing")?;
                let ordinal = self.target.ok_or("retained ordered-map insertion target is missing")?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or("retained ordered-map insertion target page is missing")?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err("retained ordered-map insertion target page has no admitted slot".into());
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or("retained ordered-map insertion move count overflow")?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or("retained ordered-map insertion target shift overflow")?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or("retained ordered-map insertion length overflow")?;
                page.insert(entry_index, (self.key.take().ok_or("retained ordered-map insertion key owner is missing")?, self.value.take().ok_or("retained ordered-map insertion value owner is missing")?));
                map.len = new_len;
                self.output = self.map.take();
                self.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete { ordinal: self.target.ok_or("retained ordered-map insertion target is missing")?, progress: RetainedOrderedMapInsertProgress::default() }),
            10 => {
                let map = self.map.as_ref().ok_or("retained ordered-map insertion workspace is missing")?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or("retained ordered-map directory growth overflow")?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or("retained ordered-map directory capacity overflow")?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                directory.try_reserve_exact(planned).map_err(|_| "retained ordered-map directory growth allocation failed")?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or("retained ordered-map directory actual capacity overflow")?;
                if actual > grant.maximum_capacity_bytes {
                    return Err("retained ordered-map directory allocator exceeded its admitted capacity".into());
                }
                self.new_directory = Some(directory);
                self.phase = 11;
                Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress { retained_capacity_bytes: actual, ..progress }))
            }
            11 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let map = self.map.as_mut().ok_or("retained ordered-map insertion workspace is missing")?;
                self.old_directory = Some(std::mem::take(&mut map.pages));
                self.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = self.old_directory.as_mut().ok_or("retained ordered-map old directory is missing")?;
                if self.directory_index == old.len() {
                    self.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                self.new_directory.as_mut().ok_or("retained ordered-map new directory is missing")?.push(std::mem::take(&mut old[self.directory_index]));
                self.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                self.map.as_mut().ok_or("retained ordered-map insertion workspace is missing")?.pages = self.new_directory.take().ok_or("retained ordered-map relocated directory is missing")?;
                self.directory_index = 0;
                self.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err("retained ordered-map insertion state is invalid".into()),
        }
    }

    pub fn take(&mut self) -> Option<RetainedOrderedMap<K, V>> {
        let output = self.output.take();
        if output.is_some() {
            self.spent = true;
        }
        output
    }

    pub fn take_refused_workspace(&mut self) -> Option<RetainedOrderedMap<K, V>> {
        if self.phase != 255 {
            return None;
        }
        let map = self.map.take();
        if map.is_some() {
            self.spent = true;
        }
        map
    }

    pub fn begin_close(&mut self) -> bool {
        if self.closing {
            return false;
        }
        self.closing = true;
        true
    }

    pub fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if !self.close.is_empty() {
            let step = self.close.step(maximum_items, maximum_bytes)?;
            return Ok(if step == SnapshotRetirementStep::Complete { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
        }
        if let Some(step) = self.close.begin_option(&mut self.key, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.value, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.old_directory, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.new_directory, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.map, maximum_items)? {
            return Ok(step);
        }
        if let Some(step) = self.close.begin_option(&mut self.output, maximum_items)? {
            return Ok(step);
        }
        Ok(SnapshotRetirementStep::Complete)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing && self.key.is_none() && self.value.is_none() && self.old_directory.is_none() && self.new_directory.is_none() && self.map.is_none() && self.output.is_none() && self.close.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
