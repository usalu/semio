//! 🗺️ Fixed-page ordered owners with resumable native key comparison and insertion.

use super::{RetainedClone, RetainedCloneBinding, RetainedCloneClose, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep, admit_retained_clone_close, admit_retained_clone_progress, close_retained_binding};
use crate::{retirement::RetireOwned};
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
    pub(crate) fn from_sorted_entries_for_test(entries: Vec<(K, V)>) -> Result<Self, crate::ValueError>
    where
        K: Ord,
    {
        if entries.windows(2).any(|pair| pair[0].0 >= pair[1].0) {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvalidValue, "retained ordered-map fixture entries are not strictly ordered"));
        }
        let len = entries.len();
        let page_count = len.div_ceil(RETAINED_ORDERED_MAP_PAGE_CAPACITY);
        let mut pages = Vec::with_capacity(page_count.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map fixture page count overflow"))?);
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
    fn retirement_birth_bytes(&self) -> Option<usize> { self.pages.retirement_birth_bytes() }
    fn controlled_retirement_supported() -> bool { K::controlled_retirement_supported() && V::controlled_retirement_supported() }
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
    fn progress_from_close(step: RetainedCloneStep, terminal_is_empty: bool, grant: RetainedCloneGrant, label: &str) -> Result<RetainedCloneStep, crate::ValueError> {
        let progress = super::admit_retained_clone_close(grant, step, terminal_is_empty, &format!("retained ordered-map {label} scaffold close"))?.progress();
        Ok(RetainedCloneStep::Progress(progress))
    }
}

impl<K: RetainedClone, V: RetainedClone> RetainedCloneCursor<RetainedOrderedMap<K, V>> for RetainedOrderedMapCloneCursor<K, V> {
    fn advance(&mut self, source: RetainedCloneRef<'_, RetainedOrderedMap<K, V>>, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone cursor is spent"));
        }
        if self.output.is_some() {
            return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));
        }
        source.bind(&mut self.source)?;
        let source_value = source.get();
        match self.phase {
            0 => {
                let planned_pages = source_value.pages.len().checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory page count overflow"))?;
                let planned_capacity = planned_pages.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned_capacity, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                self.pages.try_reserve_exact(planned_pages).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory allocation failed"))?;
                let actual = self.pages.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
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
                let source_page = source_value.pages.get(self.page).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page is missing"))?;
                if source_page.is_empty() || source_page.len() > RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map source page violates its fixed capacity"));
                }
                let planned = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page capacity overflow"))?;
                let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: 0, retained_capacity_bytes: planned, released_bytes: 0 };
                if !progress.fits(grant) {
                    return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));
                }
                let mut page = Vec::new();
                page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map page allocation failed"))?;
                let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map page actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map page allocator exceeded its admitted capacity"));
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
                        self.key = Some(self.key_cursor.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map key completed without an owner"))?);
                        let _ = self.key_cursor.begin_close();
                        self.phase = 3;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            3 => {
                if !self.key_cursor.terminal_is_empty() {
                    let step = self.key_cursor.close_step(grant)?;
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
                        self.value = Some(self.value_cursor.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map value completed without an owner"))?);
                        let _ = self.value_cursor.begin_close();
                        self.phase = 5;
                        Ok(RetainedCloneStep::Progress(progress))
                    }
                }
            }
            5 => {
                if !self.value_cursor.terminal_is_empty() {
                    let step = self.value_cursor.close_step(grant)?;
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
                self.page_output.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map output page is missing"))?.push((
                    self.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map key owner is missing"))?,
                    self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map value owner is missing"))?,
                ));
                self.entry += 1;
                self.phase = 1;
                Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }))
            }
            7 => Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default())),
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map clone state is invalid")),
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

    

    fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "ordered map must begin close before granted retirement")); }
        if !self.key_cursor.terminal_is_empty() {
            if self.key_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.key_cursor.close_step(grant)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.key_cursor.terminal_is_empty(), "retained ordered-map key close")?.progress()));
        }
        if !self.value_cursor.terminal_is_empty() {
            if self.value_cursor.begin_close() { return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() })); }
            let step = self.value_cursor.close_step(grant)?;
            return Ok(RetainedCloneStep::Progress(admit_retained_clone_close(grant, step, self.value_cursor.terminal_is_empty(), "retained ordered-map value close")?.progress()));
        }
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.key, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.page_output, grant)? { return Ok(step); }
        if !self.pages.is_empty() || self.pages.capacity() != 0 {
            if let Some(step) = self.close.begin_default_granted(&mut self.pages, grant)? { return Ok(step); }
        }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        close_retained_binding(&mut self.source, grant)
    }

    fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_depth_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_depth_demand(); }
        self.close.next_owner_depth_with_binding(self.key.is_some()||self.value.is_some()||self.page_output.is_some()||self.pages.capacity()!=0||self.output.is_some(),&self.source)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_copy_byte_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_copy_byte_demand(); }
        self.close.next_copy_with_binding(&self.source)
    }
    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_capacity_byte_demand(body); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_capacity_byte_demand(body); }
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(body); }
        if self.key.is_some() { return self.close.next_owner_capacity_with_binding::<K>(true,body,&self.source); }
        if self.value.is_some() { return self.close.next_owner_capacity_with_binding::<V>(true,body,&self.source); }
        if self.page_output.is_some() { return self.close.next_owner_capacity_with_binding::<Vec<(K,V)>>(true,body,&self.source); }
        if !self.pages.is_empty()||self.pages.capacity()!=0 { return self.close.next_owner_capacity_with_binding::<Vec<Vec<(K,V)>>>(true,body,&self.source); }
        self.close.next_owner_capacity_with_binding::<RetainedOrderedMap<K,V>>(self.output.is_some(),body,&self.source)
    }
    fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> {
        if !self.closing { return Ok(0); }
        if !self.key_cursor.terminal_is_empty() { return self.key_cursor.next_close_release_byte_demand(); }
        if !self.value_cursor.terminal_is_empty() { return self.value_cursor.next_close_release_byte_demand(); }
        self.close.next_release_with_binding(&self.source)
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
    fn begin_close(&mut self)->bool;
    fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>;
    fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>;
    fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>;
    fn terminal_is_empty(&self)->bool;
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>;
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, crate::ValueError>;
}

pub struct ScalarBoundedOrdCursor<T> {
    closing:bool,
    complete: Option<Ordering>,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> Default for ScalarBoundedOrdCursor<T> {
    fn default() -> Self {
        Self { closing:false, complete: None, left: None, right: None, marker: std::marker::PhantomData }
    }
}

impl<T: BoundedOrd + Copy> BoundedOrdCursor<T> for ScalarBoundedOrdCursor<T> {
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}super::close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant)}
    fn compare(&mut self, left: RetainedCloneRef<'_, T>, right: RetainedCloneRef<'_, T>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, crate::ValueError> {
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}
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
    closing:bool,
    left: Option<RetainedCloneBinding>,
    right: Option<RetainedCloneBinding>,
    offset: usize,
    complete: Option<Ordering>,
}

impl BoundedOrdCursor<String> for StringBoundedOrdCursor {
fn begin_close(&mut self)->bool {let started=!self.closing;self.closing=true;started}
fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::copy_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{RetainedCloneBinding::capacity_demand(if self.left.is_some(){&self.left}else{&self.right},body)}
fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::release_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{RetainedCloneBinding::depth_demand(if self.left.is_some(){&self.left}else{&self.right})}
fn terminal_is_empty(&self)->bool{self.left.is_none()&&self.right.is_none()}
fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator must begin close"));}super::close_retained_binding(if self.left.is_some(){&mut self.left}else{&mut self.right},grant)}
    fn compare(&mut self, left: RetainedCloneRef<'_, String>, right: RetainedCloneRef<'_, String>, grant: BoundedOrdGrant) -> Result<BoundedOrdStep, crate::ValueError> {
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"comparator is closing"));}
        if self.left.is_none()&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}
        if let Some(ordering) = self.complete {
            return Ok(BoundedOrdStep::Complete { ordering, progress: BoundedOrdProgress::default() });
        }
        let first = self.left.is_none();
        if first&&grant.maximum_items==0{return Ok(BoundedOrdStep::Progress(Default::default()));}
        left.bind(&mut self.left)?;
        right.bind(&mut self.right)?;
        if first {
            if grant.maximum_items == 0 {
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
    closing:bool,
}

impl<K: BoundedOrd> Default for RetainedOrderedMapLookupCursor<K> {
    fn default() -> Self {
        Self { low: 0, high: 0, middle: 0, comparison: None, pending: None, source: None, complete: None, closing:false }
    }
}

impl<K:BoundedOrd> RetainedOrderedMapLookupCursor<K>{
    pub fn advance<V>(&mut self,source:RetainedCloneRef<'_,RetainedOrderedMap<K,V>>,target:RetainedCloneRef<'_,K>,grant:BoundedOrdGrant,retirement:RetainedCloneGrant)->Result<(Option<RetainedOrderedMapLookup>,BoundedOrdProgress,RetainedCloneProgress),crate::ValueError>{
        if self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup is closing"));}
        if let Some(result)=self.complete{return Ok((Some(result),Default::default(),Default::default()));}
        let first=self.source.is_none();
        if first&&grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
        source.bind(&mut self.source)?;
        let map=source.get();
        if first{self.high=map.len();return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));}
        if let Some(ordering)=self.pending{
            if let Some(comparison)=self.comparison.as_mut(){
                comparison.begin_close();let step=comparison.close_step(retirement)?;
                if comparison.terminal_is_empty(){self.comparison=None;}
                return Ok((None,Default::default(),step.progress()));
            }
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            self.pending=None;
            match ordering{Ordering::Less=>self.low=self.middle+1,Ordering::Greater=>self.high=self.middle,Ordering::Equal=>unreachable!()}
            return Ok((None,BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        if self.low==self.high{
            if grant.maximum_items==0{return Ok((None,Default::default(),Default::default()));}
            let result=RetainedOrderedMapLookup::Missing(self.low);self.complete=Some(result);
            return Ok((Some(result),BoundedOrdProgress{compared_items:1,compared_bytes:0},Default::default()));
        }
        self.middle=self.low+(self.high-self.low)/2;
        let comparison=self.comparison.get_or_insert_with(K::bounded_ord_cursor);
        match comparison.compare(source.project(self.middle+1,|map|map.get_index(self.middle).unwrap().0),target,grant)?{
            BoundedOrdStep::Progress(p)=>Ok((None,p,Default::default())),
            BoundedOrdStep::Complete{ordering:Ordering::Equal,progress}=>{let result=RetainedOrderedMapLookup::Found(self.middle);self.complete=Some(result);Ok((Some(result),progress,Default::default()))},
            BoundedOrdStep::Complete{ordering,progress}=>{self.pending=Some(ordering);Ok((None,progress,Default::default()))}
        }
    }
    pub fn begin_close(&mut self)->bool{let started=!self.closing;self.closing=true;if let Some(c)=self.comparison.as_mut(){c.begin_close();}started}
    pub fn next_close_copy_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_copy_byte_demand()}else{RetainedCloneBinding::copy_demand(&self.source)}}
    pub fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_capacity_byte_demand(body)}else{RetainedCloneBinding::capacity_demand(&self.source,body)}}
    pub fn next_close_release_byte_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_release_byte_demand()}else{RetainedCloneBinding::release_demand(&self.source)}}
    pub fn next_close_depth_demand(&self)->Result<usize,crate::ValueError>{if let Some(c)=self.comparison.as_ref(){c.next_close_depth_demand()}else{RetainedCloneBinding::depth_demand(&self.source)}}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,crate::ValueError>{
        if !self.closing{return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated,"lookup must begin close"));}
        if let Some(c)=self.comparison.as_mut(){let step=c.close_step(grant)?;if c.terminal_is_empty(){self.comparison=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        super::close_retained_binding(&mut self.source,grant)
    }
    pub fn terminal_is_empty(&self)->bool{self.comparison.is_none()&&self.source.is_none()}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RetainedOrderedMapInsertStep {
    Progress(RetainedOrderedMapInsertProgress),
    Complete { ordinal: usize, progress: RetainedOrderedMapInsertProgress },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertGrant {
    pub retirement:RetainedCloneGrant,
    pub comparison: BoundedOrdGrant,
    pub maximum_moved_items: usize,
    pub maximum_moved_bytes: usize,
    pub maximum_capacity_bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RetainedOrderedMapInsertProgress {
    pub retirement:RetainedCloneProgress,
    pub comparison: BoundedOrdProgress,
    pub moved_items: usize,
    pub moved_bytes: usize,
    pub retained_capacity_bytes: usize,
}

impl RetainedOrderedMapInsertProgress {
    pub fn fits(self, grant: RetainedOrderedMapInsertGrant) -> bool {
        self.retirement.fits(grant.retirement) && self.comparison.fits(grant.comparison) && self.moved_items <= grant.maximum_moved_items && self.moved_bytes <= grant.maximum_moved_bytes && self.retained_capacity_bytes <= grant.maximum_capacity_bytes
    }
}

/// 🧯 Owns one exclusive unpublished workspace through completion or bounded cancellation.
pub struct RetainedOrderedMapInsertCursor<K: BoundedOrd + RetireOwned, V: RetireOwned> {
    map: Option<RetainedOrderedMap<K, V>>,
    output: Option<RetainedOrderedMap<K, V>>,
    key: Option<K>,
    value: Option<V>,
    lookup: RetainedOrderedMapLookupCursor<K>,
    lookup_lease: super::RetainedCloneBorrowAuthority,
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
    pub fn constructor_capacity_bytes()->usize{super::RetainedCloneBorrowAuthority::constructor_capacity_bytes::<()>()}
    pub fn admit(map:RetainedOrderedMap<K,V>,key:K,value:V,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(crate::ValueError,RetainedOrderedMap<K,V>,K,V)>{
        if grant.maximum_items==0{return Err((crate::ValueError::literal(crate::ValueRefusalKind::WorkLimit,"insertion constructor requires an item"),map,key,value));}
        let(lookup_lease,receipt)=match super::RetainedCloneBorrowAuthority::admit((),grant){Ok(v)=>v,Err((e,_))=>return Err((e,map,key,value))};
        Ok((Self {
            map: Some(map),
            output: None,
            key: Some(key),
            value: Some(value),
            lookup: Default::default(),
            lookup_lease,
            target: None,
            shift_page: None,
            old_directory: None,
            new_directory: None,
            directory_index: 0,
            phase: 0,
            spent: false,
            closing: false,
            close: Default::default(),
        },receipt))
    }

    pub fn advance(&mut self, grant: RetainedOrderedMapInsertGrant) -> Result<RetainedOrderedMapInsertStep, crate::ValueError> {
        if self.closing {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is closing"));
        }
        if self.spent {
            return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion cursor is spent"));
        }
        match self.phase {
            0 => {
                if self.lookup.closing{let step=self.lookup.close_step(grant.retirement)?;let progress=RetainedOrderedMapInsertProgress{retirement:step.progress(),..Default::default()};if !self.lookup.terminal_is_empty(){return Ok(RetainedOrderedMapInsertStep::Progress(progress));}match self.lookup.complete.unwrap(){RetainedOrderedMapLookup::Found(_)=>{self.phase=255;return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvalidValue,"retained ordered-map insertion refused a duplicate key"));},RetainedOrderedMapLookup::Missing(ordinal)=>{self.target=Some(ordinal);self.phase=1;return Ok(RetainedOrderedMapInsertStep::Progress(progress));}}}
                let map = self.map.as_ref().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let key = self.key.as_ref().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion key is missing"))?;
                let map_ref = self.lookup_lease.borrow(map);
                let key_ref = self.lookup_lease.borrow(key);
                let (result, comparison, retirement) = self.lookup.advance(map_ref, key_ref, grant.comparison,grant.retirement)?;
                let progress = RetainedOrderedMapInsertProgress { comparison, retirement, ..Default::default() };
                if result.is_some(){self.lookup.begin_close();return Ok(RetainedOrderedMapInsertStep::Progress(progress));}
                match result {
                    None => Ok(RetainedOrderedMapInsertStep::Progress(progress)),
                    Some(RetainedOrderedMapLookup::Found(_)) => {
                        self.phase = 255;
                        Err(crate::ValueError::new(crate::ValueRefusalKind::InvalidValue, "retained ordered-map insertion refused a duplicate key"))
                    }
                    Some(RetainedOrderedMapLookup::Missing(ordinal)) => {
                        self.target = Some(ordinal);
                        self.phase = 1;
                        Ok(RetainedOrderedMapInsertStep::Progress(progress))
                    }
                }
            }
            1 => {
                let map = self.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                if map.len % RETAINED_ORDERED_MAP_PAGE_CAPACITY == 0 {
                    if map.pages.len() == map.pages.capacity() {
                        self.phase = 10;
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let capacity = RETAINED_ORDERED_MAP_PAGE_CAPACITY.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page capacity overflow"))?;
                    let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                    if !progress.fits(grant) {
                        return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                    }
                    let mut page = Vec::new();
                    page.try_reserve_exact(RETAINED_ORDERED_MAP_PAGE_CAPACITY).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map insertion page allocation failed"))?;
                    let actual = page.capacity().checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion page actual capacity overflow"))?;
                    if actual > grant.maximum_capacity_bytes {
                        return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion page allocator exceeded its admitted capacity"));
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
                let map = self.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = self.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let target_page = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let shift_page = self.shift_page.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page is missing"))?;
                if shift_page == target_page {
                    self.phase = 3;
                    return Ok(RetainedOrderedMapInsertStep::Progress(RetainedOrderedMapInsertProgress::default()));
                }
                let (left, right) = map.pages.split_at_mut(shift_page);
                let previous = left.get_mut(shift_page - 1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is missing"))?;
                let current = right.first_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion current page is missing"))?;
                if current.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion shift page has no admitted slot"));
                }
                let moved_items = current.len().checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let entry = previous.pop().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion previous page is empty"))?;
                current.insert(0, entry);
                self.shift_page = Some(shift_page - 1);
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            3 => {
                let map = self.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let ordinal = self.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?;
                let page_index = ordinal / RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let entry_index = ordinal % RETAINED_ORDERED_MAP_PAGE_CAPACITY;
                let page = map.pages.get_mut(page_index).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page is missing"))?;
                if page.len() >= RETAINED_ORDERED_MAP_PAGE_CAPACITY {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target page has no admitted slot"));
                }
                let moved_items = page.len().saturating_sub(entry_index).checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion move count overflow"))?;
                let moved_bytes = moved_items.checked_mul(size_of::<(K, V)>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::WorkLimit, "retained ordered-map insertion target shift overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { moved_items, moved_bytes, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let new_len = map.len.checked_add(1).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map insertion length overflow"))?;
                page.insert(
                    entry_index,
                    (
                        self.key.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion key owner is missing"))?,
                        self.value.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion value owner is missing"))?,
                    ),
                );
                map.len = new_len;
                self.output = self.map.take();
                self.phase = 4;
                Ok(RetainedOrderedMapInsertStep::Complete { ordinal, progress })
            }
            4 => Ok(RetainedOrderedMapInsertStep::Complete {
                ordinal: self.target.ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion target is missing"))?,
                progress: RetainedOrderedMapInsertProgress::default(),
            }),
            10 => {
                let map = self.map.as_ref().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                let planned = map.pages.len().checked_mul(2).and_then(|value| value.checked_add(1)).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory growth overflow"))?;
                let capacity = planned.checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory capacity overflow"))?;
                let progress = RetainedOrderedMapInsertProgress { retained_capacity_bytes: capacity, ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let mut directory = Vec::new();
                directory.try_reserve_exact(planned).map_err(|_| crate::ValueError::new(crate::ValueRefusalKind::AllocationFailed, "retained ordered-map directory growth allocation failed"))?;
                let actual = directory.capacity().checked_mul(size_of::<Vec<(K, V)>>()).ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit, "retained ordered-map directory actual capacity overflow"))?;
                if actual > grant.maximum_capacity_bytes {
                    return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map directory allocator exceeded its admitted capacity"));
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
                let map = self.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?;
                self.old_directory = Some(std::mem::take(&mut map.pages));
                self.phase = 12;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            12 => {
                let old = self.old_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map old directory is missing"))?;
                if self.directory_index == old.len() {
                    self.phase = 13;
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<(K, V)>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                self.new_directory.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map new directory is missing"))?.push(std::mem::take(&mut old[self.directory_index]));
                self.directory_index += 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            13 => {
                let progress = RetainedOrderedMapInsertProgress { moved_items: 1, moved_bytes: size_of::<Vec<Vec<(K, V)>>>(), ..Default::default() };
                if !progress.fits(grant) {
                    return Ok(RetainedOrderedMapInsertStep::Progress(Default::default()));
                }
                self.map.as_mut().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion workspace is missing"))?.pages =
                    self.new_directory.take().ok_or_else(|| crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map relocated directory is missing"))?;
                self.directory_index = 0;
                self.phase = 1;
                Ok(RetainedOrderedMapInsertStep::Progress(progress))
            }
            _ => Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated, "retained ordered-map insertion state is invalid")),
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

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, crate::ValueError> { if !self.lookup.terminal_is_empty(){self.lookup.next_close_copy_byte_demand()}else{self.close.next_copy_byte_demand()} }
    pub fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, crate::ValueError> {
        if !self.lookup.terminal_is_empty(){return self.lookup.next_close_capacity_byte_demand(body);}
        if !self.close.is_empty() { return self.close.next_capacity_byte_demand(body); }
        if self.key.is_some() { return self.close.next_owner_capacity_byte_demand::<K>(true, body); }
        if self.value.is_some() { return self.close.next_owner_capacity_byte_demand::<V>(true, body); }
        if self.old_directory.is_some() || self.new_directory.is_some() { return self.close.next_owner_capacity_byte_demand::<Vec<Vec<(K,V)>>>(true, body); }
        if self.map.is_some()||self.output.is_some(){self.close.next_owner_capacity_byte_demand::<RetainedOrderedMap<K,V>>(true,body)}else{self.lookup_lease.next_close_capacity_byte_demand(body)}
    }
    pub fn next_close_release_byte_demand(&self) -> Result<usize, crate::ValueError> { if !self.lookup.terminal_is_empty(){self.lookup.next_close_release_byte_demand()}else if !self.close.is_empty(){self.close.next_release_byte_demand()}else if self.key.is_some()||self.value.is_some()||self.old_directory.is_some()||self.new_directory.is_some()||self.map.is_some()||self.output.is_some(){Ok(0)}else{self.lookup_lease.next_close_release_byte_demand()} }
    pub fn next_close_depth_demand(&self) -> Result<usize, crate::ValueError> { if !self.lookup.terminal_is_empty(){self.lookup.next_close_depth_demand()}else if !self.close.is_empty(){self.close.next_depth_demand()}else if self.key.is_some()||self.value.is_some()||self.old_directory.is_some()||self.new_directory.is_some()||self.map.is_some()||self.output.is_some(){Ok(1)}else{self.lookup_lease.next_close_depth_demand()} }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, crate::ValueError> {
        if grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if !self.closing { return Err(crate::ValueError::literal(crate::ValueRefusalKind::InvariantViolated, "ordered-map insertion must begin close before retiring ownership")); }
        if !self.lookup.terminal_is_empty(){self.lookup.begin_close();return self.lookup.close_step(grant);}
        if !self.close.is_empty() { return self.close.step_granted(grant); }
        if let Some(step) = self.close.begin_granted(&mut self.key, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.value, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.old_directory, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.new_directory, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.map, grant)? { return Ok(step); }
        if let Some(step) = self.close.begin_granted(&mut self.output, grant)? { return Ok(step); }
        self.lookup_lease.close_step(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.lookup.terminal_is_empty()&&self.lookup_lease.terminal_is_empty()&&self.closing && self.key.is_none() && self.value.is_none() && self.old_directory.is_none() && self.new_directory.is_none() && self.map.is_none() && self.output.is_none() && self.close.is_empty()
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
