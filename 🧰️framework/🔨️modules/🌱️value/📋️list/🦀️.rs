//! 📋️ Fixed-fanout metadata and separately admitted payload pages for bounded value lists.

use std::mem::size_of;

//#region 🎟️Progress
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListProgress {
    pub progressed: bool,
    pub allocated_bytes: usize,
    pub placed_bytes: usize,
    pub released_allocation_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PagedListAllocationError {
    pub allocated_bytes: usize,
    pub reason: &'static str,
}
//#endregion 🎟️Progress

//#region 🌳️Pages
const FANOUT: usize = 16;
const PAGE_BYTES: usize = 4096;

trait PageAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize) -> Result<(), std::collections::TryReserveError>;
}

struct ExactAllocation;
impl PageAllocation for ExactAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize) -> Result<(), std::collections::TryReserveError> {
        owner.try_reserve_exact(slots)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️counter/🦀️.rs"]
mod counter_tests;

#[cfg(test)]
#[path = "🧪️tests/📋️list/🦀️.rs"]
mod tests;

#[cfg_attr(target_pointer_width = "64", expect(clippy::large_enum_variant, reason = "Each fixed-fanout branch is one admitted metadata allocation; boxing its children would allocate outside that grant."))]
enum Page<T> {
    Branch([Vec<Page<T>>; FANOUT]),
    Leaf { items: Vec<T>, slots: usize },
}

enum MutableFrame<'a, T> {
    Branch(std::slice::IterMut<'a, Vec<Page<T>>>),
    Leaf(std::slice::IterMut<'a, T>),
}

impl<'a, T> MutableFrame<'a, T> {
    fn new(page: &'a mut Page<T>) -> Self {
        match page {
            Page::Branch(children) => Self::Branch(children.iter_mut()),
            Page::Leaf { items, .. } => Self::Leaf(items.iter_mut()),
        }
    }
}

pub struct PagedIterMut<'a, T> {
    frames: [Option<MutableFrame<'a, T>>; usize::BITS as usize / 4 + 2],
    depth: usize,
    remaining: usize,
}

impl<'a, T> Iterator for PagedIterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        while self.depth != 0 && self.remaining != 0 {
            match self.frames[self.depth - 1].as_mut()? {
                MutableFrame::Leaf(items) => {
                    if let Some(item) = items.next() {
                        self.remaining -= 1;
                        return Some(item);
                    }
                }
                MutableFrame::Branch(children) => {
                    if let Some(child) = children.next() {
                        if let Some(page) = child.first_mut() {
                            self.frames[self.depth] = Some(MutableFrame::new(page));
                            self.depth += 1;
                        }
                        continue;
                    }
                }
            }
            self.depth -= 1;
            self.frames[self.depth] = None;
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}
impl<T> ExactSizeIterator for PagedIterMut<'_, T> {}

/// 📚️ Logical indexed ownership with separately admitted metadata and payload backing.
pub struct PagedList<T, const N: usize> {
    root: Vec<Page<T>>,
    length: usize,
    capacity: usize,
    allocated: usize,
}

impl<T, const N: usize> Default for PagedList<T, N> {
    fn default() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0 }
    }
}

impl<T, const N: usize> PagedList<T, N> {
    pub const fn empty() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0 }
    }
    fn page_items() -> usize {
        if size_of::<T>() == 0 {
            N.max(1)
        } else {
            (PAGE_BYTES / size_of::<T>()).max(1).min(N.max(1))
        }
    }
    fn height() -> usize {
        let slots = Self::page_items();
        let mut pages = N.div_ceil(slots);
        let mut height = 0;
        while pages > 1 {
            pages = pages.div_ceil(FANOUT);
            height += 1;
        }
        height
    }
    fn slot(index: usize, height: usize) -> usize {
        (index >> ((height - 1) * 4)) & (FANOUT - 1)
    }
    pub fn is_empty(&self) -> bool { self.length == 0 }

    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &T> + ExactSizeIterator {
        (0..self.length).map(|index| self.get(index).expect("initialized indexed owner"))
    }

    pub fn len(&self) -> usize {
        self.length
    }
    pub fn capacity(&self) -> usize {
        self.capacity
    }
    pub fn allocated_bytes(&self) -> usize {
        self.allocated
    }
    pub fn has_reserved_slot(&self) -> bool {
        self.length < self.capacity
    }
    pub fn terminal_is_empty(&self) -> bool {
        self.root.capacity() == 0 && self.length == 0 && self.capacity == 0 && self.allocated == 0
    }

    pub fn iter_mut(&mut self) -> PagedIterMut<'_, T> {
        let mut iterator = PagedIterMut { frames: std::array::from_fn(|_| None), depth: 0, remaining: self.length };
        if let Some(page) = self.root.first_mut() {
            iterator.frames[0] = Some(MutableFrame::new(page));
            iterator.depth = 1;
        }
        iterator
    }

    fn leaf(&self, index: usize) -> Option<&Vec<T>> {
        let mut link = &self.root;
        let page = index / Self::page_items();
        for height in (0..=Self::height()).rev() {
            match link.first()? {
                Page::Branch(children) => link = &children[Self::slot(page, height)],
                Page::Leaf { items, .. } => return Some(items),
            }
        }
        None
    }

    fn leaf_mut(&mut self, index: usize) -> Option<&mut Vec<T>> {
        let mut link = &mut self.root;
        let page = index / Self::page_items();
        for height in (0..=Self::height()).rev() {
            match link.first_mut()? {
                Page::Branch(children) => link = &mut children[Self::slot(page, height)],
                Page::Leaf { items, .. } => return Some(items),
            }
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.leaf(index)?.get(index % Self::page_items())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        self.leaf_mut(index)?.get_mut(index % Self::page_items())
    }

    pub fn next_allocation_bytes(&self) -> Result<usize, &'static str> {
        if self.has_reserved_slot() {
            return Ok(0);
        }
        self.next_page_allocation_bytes()
    }

    /// 🎟️ Returns the next single backing allocation needed to reach a requested logical capacity.
    pub fn next_capacity_allocation_bytes(&self, capacity: usize) -> Result<Option<usize>, &'static str> {
        if capacity > N {
            return Err("fixed list logical capacity exhausted");
        }
        if capacity <= self.capacity {
            return Ok(None);
        }
        self.next_page_allocation_bytes().map(Some)
    }

    fn next_page_allocation_bytes(&self) -> Result<usize, &'static str> {
        if self.capacity == N {
            return Err("fixed list logical capacity exhausted");
        }
        let page = self.capacity / Self::page_items();
        let mut link = &self.root;
        for height in (0..=Self::height()).rev() {
            match link.first() {
                None => return Ok(size_of::<Page<T>>()),
                Some(Page::Branch(children)) => link = &children[Self::slot(page, height)],
                Some(Page::Leaf { .. }) => return Self::page_items().min(N - self.capacity).checked_mul(size_of::<T>()).ok_or("fixed list allocation overflow"),
            }
        }
        Err("fixed list page authority is missing")
    }

    pub fn reserve_one(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if self.has_reserved_slot() {
            return Ok(PagedListProgress::default());
        }
        self.reserve_page(grant)
    }

    /// 🎟️ Admits at most one backing allocation toward a requested logical capacity.
    pub fn reserve_capacity_one(&mut self, capacity: usize, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if capacity > N {
            return Err(PagedListAllocationError { allocated_bytes: 0, reason: "fixed list logical capacity exhausted" });
        }
        if capacity <= self.capacity { return Ok(PagedListProgress::default()); }
        self.reserve_page(grant)
    }

    pub fn reserve_full(&mut self) -> Result<bool, &'static str> {
        let before = self.capacity;
        while self.capacity < N {
            let requested = self.next_page_allocation_bytes()?;
            self.reserve_page(requested).map_err(|error| error.reason)?;
        }
        Ok(self.capacity != before)
    }

    fn reserve_page(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        self.reserve_page_using::<ExactAllocation>(grant)
    }

    fn reserve_page_using<A: PageAllocation>(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        let rejected = |reason| PagedListAllocationError { allocated_bytes: 0, reason };
        let requested = self.next_page_allocation_bytes().map_err(rejected)?;
        if grant < requested {
            return Ok(PagedListProgress::default());
        }
        self.allocated.checked_add(requested).filter(|total| *total <= isize::MAX as usize).ok_or_else(|| rejected("fixed list allocation counter exceeds addressable ownership"))?;
        let page = self.capacity / Self::page_items();
        let mut link = &mut self.root;
        for height in (0..=Self::height()).rev() {
            if link.is_empty() {
                A::reserve(link, 1).map_err(|_| rejected("fixed list metadata allocation failed"))?;
                let actual = link.capacity() * size_of::<Page<T>>();
                link.push(if height == 0 { Page::Leaf { items: Vec::new(), slots: 0 } } else { Page::Branch(std::array::from_fn(|_| Vec::new())) });
                self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                if self.allocated > isize::MAX as usize {
                    return Err(PagedListAllocationError { allocated_bytes: actual, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" });
                }
                if actual > grant {
                    return Err(PagedListAllocationError { allocated_bytes: actual, reason: "fixed list metadata allocation exceeded admission; owner retained" });
                }
                return Ok(PagedListProgress { progressed: true, allocated_bytes: actual, ..Default::default() });
            }
            match &mut link[0] {
                Page::Branch(children) => link = &mut children[Self::slot(page, height)],
                Page::Leaf { items, slots } => {
                    let admitted_slots = Self::page_items().min(N - self.capacity);
                    A::reserve(items, admitted_slots).map_err(|_| rejected("fixed list payload allocation failed"))?;
                    let actual = if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() };
                    *slots = admitted_slots;
                    self.capacity += admitted_slots;
                    self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                    if self.allocated > isize::MAX as usize {
                        return Err(PagedListAllocationError { allocated_bytes: actual, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" });
                    }
                    if actual > grant {
                        return Err(PagedListAllocationError { allocated_bytes: actual, reason: "fixed list payload allocation exceeded admission; owner retained" });
                    }
                    return Ok(PagedListProgress { progressed: true, allocated_bytes: actual, ..Default::default() });
                }
            }
        }
        Err(rejected("fixed list page authority is missing"))
    }

    pub fn push_reserved(&mut self, value: T) -> Result<(), T> {
        if !self.has_reserved_slot() {
            return Err(value);
        }
        let index = self.length;
        self.leaf_mut(index).expect("reserved page owns exact next index").push(value);
        self.length += 1;
        Ok(())
    }

    pub fn place_reserved(&mut self, source: &mut Option<T>, grant: usize) -> Result<PagedListProgress, &'static str> {
        if source.is_none() || !self.has_reserved_slot() || grant < size_of::<T>() {
            return Ok(PagedListProgress::default());
        }
        if let Err(owner) = self.push_reserved(source.take().expect("checked source owner")) {
            *source = Some(owner);
            return Err("fixed list reserved authority rejected exact owner");
        }
        Ok(PagedListProgress { progressed: true, placed_bytes: size_of::<T>(), ..Default::default() })
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.length.checked_sub(1)?;
        let owner = self.leaf_mut(index)?.pop()?;
        self.length -= 1;
        Some(owner)
    }

    /// ♻️ Returns the exact physical byte grant needed by the next releasable backing.
    pub fn next_release_allocation_bytes(&self) -> Result<usize, &'static str> {
        fn next<T>(link: &[Page<T>], capacity: usize) -> Result<usize, &'static str> {
            let Some(node) = link.first() else {
                return Ok(capacity * size_of::<Page<T>>());
            };
            match node {
                Page::Branch(children) => {
                    if let Some(child) = children.iter().rfind(|child| !child.is_empty()) {
                        return next(child, child.capacity());
                    }
                }
                Page::Leaf { items, slots } => {
                    if !items.is_empty() {
                        return Err("fixed list payload must retire before its page");
                    }
                    if *slots != 0 {
                        return Ok(if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() });
                    }
                }
            }
            Ok(capacity * size_of::<Page<T>>())
        }
        next(&self.root, self.root.capacity())
    }

    pub fn truncate_retired_last(&mut self) -> Result<(), &'static str> {
        let index = self.length.checked_sub(1).ok_or("fixed list has no retired payload")?;
        let items = self.leaf_mut(index).ok_or("fixed list payload page is missing")?;
        items.truncate(items.len() - 1);
        self.length -= 1;
        Ok(())
    }

    pub fn release_empty_page(&mut self, maximum_bytes: usize) -> Result<PagedListProgress, &'static str> {
        fn release<T>(link: &mut Vec<Page<T>>, slots: &mut usize, maximum_bytes: usize) -> Result<PagedListProgress, &'static str> {
            let Some(node) = link.first_mut() else {
                return Ok(PagedListProgress::default());
            };
            match node {
                Page::Branch(children) => {
                    if let Some(index) = children.iter().rposition(|child| !child.is_empty()) {
                        return release(&mut children[index], slots, maximum_bytes);
                    }
                }
                Page::Leaf { items, slots: reserved } => {
                    if !items.is_empty() {
                        return Err("fixed list payload must retire before its page");
                    }
                    if *reserved != 0 {
                        let bytes = if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() };
                        if bytes > maximum_bytes { return Ok(PagedListProgress::default()); }
                        *items = Vec::new();
                        *slots -= *reserved;
                        *reserved = 0;
                        return Ok(PagedListProgress { progressed: true, released_allocation_bytes: bytes, ..Default::default() });
                    }
                }
            }
            let bytes = link.capacity() * size_of::<Page<T>>();
            if bytes > maximum_bytes { return Ok(PagedListProgress::default()); }
            *link = Vec::new();
            Ok(PagedListProgress { progressed: true, released_allocation_bytes: bytes, ..Default::default() })
        }
        let step = release(&mut self.root, &mut self.capacity, maximum_bytes)?;
        self.allocated -= step.released_allocation_bytes;
        Ok(step)
    }

    #[doc(hidden)]
    pub fn backing_ptr(&self, index: usize) -> Option<*const T> {
        self.leaf(index).map(Vec::as_ptr)
    }

    #[doc(hidden)]
    pub fn initialized_len(&self) -> usize {
        fn count<T>(link: &[Page<T>]) -> usize {
            match link.first() {
                None => 0,
                Some(Page::Leaf { items, .. }) => items.len(),
                Some(Page::Branch(children)) => children.iter().map(|child| count(child)).sum(),
            }
        }
        count(&self.root)
    }
}
//#endregion 🌳️Pages


/// 🧊️ Cold copies allocate each backing page explicitly; retained work uses admission and placement steps.
impl<T: Clone, const N: usize> Clone for PagedList<T, N> {
    fn clone(&self) -> Self {
        let mut copy = Self::default();
        for item in self.iter() {
            while !copy.has_reserved_slot() {
                let bytes = copy.next_allocation_bytes().expect("source fits logical list capacity");
                copy.reserve_one(bytes).expect("cold list clone allocation");
            }
            if copy.push_reserved(item.clone()).is_err() { unreachable!("cold clone admitted exact slot"); }
        }
        copy
    }
}
