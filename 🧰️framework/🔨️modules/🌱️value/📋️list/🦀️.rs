//! 📋️ Fixed-fanout metadata and separately admitted payload pages for bounded value lists.

use std::mem::size_of;

#[path = "🎮️edit/🦀️.rs"]
mod edit;
pub use edit::{PagedListEditCursor, PagedListEditStep};

//#region 🎟️Progress
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct PagedListReturnProgress{pub progressed:bool,pub returned_allocation_bytes:usize}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListProgress {
    pub progressed: bool,
    pub allocated_bytes: usize,
    pub placed_bytes: usize,
    pub released_allocation_bytes: usize,
}

/// 🏷️ Owned list refusal semantics independent of terminal display prose.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PagedListRefusalKind { OwnershipLimit, AllocationFailed, InvariantViolated }
impl PagedListRefusalKind {
    /// 🔤️ Returns the shared refusal schema spelling.
    pub const fn as_str(self) -> &'static str { match self { Self::OwnershipLimit => "ownershipLimit", Self::AllocationFailed => "allocationFailed", Self::InvariantViolated => "invariantViolated" } }
}
/// 🚨️ A borrowed static refusal retains its authority before allocation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PagedListError { pub kind: PagedListRefusalKind, pub reason: &'static str }
impl std::fmt::Display for PagedListError { fn fmt(&self, output: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { output.write_str(self.reason) } }
impl std::error::Error for PagedListError {}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PagedListAllocationError {
    pub allocated_bytes: usize,
    pub kind: PagedListRefusalKind,
    pub reason: &'static str,
}
impl PagedListAllocationError {
    /// 🧭️ Retains refusal identity independently of admitted bytes still owned by the list.
    pub fn refusal(self) -> PagedListError { PagedListError { kind: self.kind, reason: self.reason } }
}
//#endregion 🎟️Progress

//#region 🌳️Pages
const FANOUT: usize = 16;
const PAGE_BYTES: usize = 4096;

pub(crate) trait PageAllocation {
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
    Branch { height: usize, children: [Vec<Page<T>>; FANOUT] },
    Leaf { items: Vec<T>, slots: usize },
}

enum MutableFrame<'a, T> {
    Branch(std::slice::IterMut<'a, Vec<Page<T>>>),
    Leaf(std::slice::IterMut<'a, T>),
}

impl<'a, T> MutableFrame<'a, T> {
    fn new(page: &'a mut Page<T>) -> Self {
        match page {
            Page::Branch { children, .. } => Self::Branch(children.iter_mut()),
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

pub struct PagedIter<'a, T, const N: usize> {
    owner: &'a PagedList<T, N>,
    front: usize,
    back: usize,
}

impl<'a, T, const N: usize> Iterator for PagedIter<'a, T, N> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        let index = self.front;
        self.front += 1;
        self.owner.get(index)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.back - self.front;
        (remaining, Some(remaining))
    }
}

impl<T, const N: usize> DoubleEndedIterator for PagedIter<'_, T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front == self.back {
            return None;
        }
        self.back -= 1;
        self.owner.get(self.back)
    }
}

impl<T, const N: usize> ExactSizeIterator for PagedIter<'_, T, N> {}

/// 📚️ Logical indexed ownership with separately admitted metadata and payload backing.
pub struct PagedList<T, const N: usize> {
    root: Vec<Page<T>>,
    length: usize,
    capacity: usize,
    allocated: usize,
    payload_page_bytes: usize,
}

impl<T, const N: usize> Default for PagedList<T, N> {
    fn default() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0, payload_page_bytes: PAGE_BYTES }
    }
}

impl<T, const N: usize> PagedList<T, N> {
    pub const fn empty() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0, payload_page_bytes: PAGE_BYTES }
    }

    /// 🎟️ Selects a payload allocation ceiling before this owner acquires any backing.
    pub fn with_payload_page_bytes(maximum_bytes: usize) -> Result<Self, PagedListError> {
        if maximum_bytes == 0 || maximum_bytes < size_of::<T>() {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "payload page ceiling cannot own one complete element" });
        }
        Ok(Self { payload_page_bytes: maximum_bytes.min(PAGE_BYTES), ..Self::empty() })
    }

    /// 🌱️ Builds a paged owner without first materializing a contiguous collection.
    pub fn try_from_iter(values: impl IntoIterator<Item = T>) -> Result<Self, PagedListError> {
        let mut output = Self::default();
        for value in values {
            while !output.has_reserved_slot() {
                let required = output.next_allocation_bytes()?;
                let progress = output.reserve_one(required).map_err(PagedListAllocationError::refusal)?;
                if !progress.progressed {
                    return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list exact cold allocation did not progress" });
                }
            }
            output.push_reserved(value).map_err(|_| PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list rejected its reserved slot" })?;
        }
        Ok(output)
    }

    /// 🌱️ Builds a paged owner from fallible domain conversion without an intermediate collection.
    pub fn try_from_fallible_iter<E: From<PagedListError>>(values: impl IntoIterator<Item = Result<T, E>>) -> Result<Self, E> {
        let mut output = Self::default();
        for value in values {
            let value = value?;
            while !output.has_reserved_slot() {
                let required = output.next_allocation_bytes().map_err(E::from)?;
                let progress = output.reserve_one(required).map_err(|error| E::from(error.refusal()))?;
                if !progress.progressed {
                    return Err(E::from(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list exact cold allocation did not progress" }));
                }
            }
            output.push_reserved(value).map_err(|_| E::from(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list rejected its reserved slot" }))?;
        }
        Ok(output)
    }
    fn page_items(&self) -> usize {
        if size_of::<T>() == 0 {
            N.max(1)
        } else {
            (self.payload_page_bytes / size_of::<T>()).max(1).min(N.max(1))
        }
    }
    fn height(&self) -> usize {
        match self.root.first() { Some(Page::Branch { height, .. }) => *height, _ => 0 }
    }
    fn page_height(mut page: usize) -> usize {
        let mut height = 0;
        while page != 0 { page >>= 4; height += 1; }
        height
    }
    fn slot(index: usize, height: usize) -> usize {
        (index >> ((height - 1) * 4)) & (FANOUT - 1)
    }
    pub fn is_empty(&self) -> bool { self.length == 0 }

    pub fn iter(&self) -> PagedIter<'_, T, N> {
        PagedIter { owner: self, front: 0, back: self.length }
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
    /// 🧭️ Quotes the existing original page path before one logical slot is borrowed or changed.
    pub fn next_get_depth_demand(&self,index:usize)->Result<usize,PagedListError>{
        if index>=self.length{return Ok(0);}
        self.height().checked_add(1).ok_or(PagedListError {kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list lookup depth overflow"})
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
        let page = index / self.page_items();
        let root_height = self.height();
        if Self::page_height(page) > root_height { return None; }
        for height in (0..=root_height).rev() {
            match link.first()? {
                Page::Branch { children, .. } => link = &children[Self::slot(page, height)],
                Page::Leaf { items, .. } => return Some(items),
            }
        }
        None
    }

    fn leaf_mut(&mut self, index: usize) -> Option<&mut Vec<T>> {
        let page = index / self.page_items();
        let root_height = self.height();
        if Self::page_height(page) > root_height { return None; }
        let mut link = &mut self.root;
        for height in (0..=root_height).rev() {
            match link.first_mut()? {
                Page::Branch { children, .. } => link = &mut children[Self::slot(page, height)],
                Page::Leaf { items, .. } => return Some(items),
            }
        }
        None
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.leaf(index)?.get(index % self.page_items())
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.length {
            return None;
        }
        let offset = index % self.page_items();
        self.leaf_mut(index)?.get_mut(offset)
    }

    pub fn swap(&mut self, left: usize, right: usize) {
        assert!(left < self.length && right < self.length, "paged list swap index out of bounds");
        if left == right {
            return;
        }
        let left = self.get_mut(left).expect("validated paged list left index") as *mut T;
        let right = self.get_mut(right).expect("validated paged list right index") as *mut T;
        unsafe { std::ptr::swap(left, right) };
    }

    pub fn reverse(&mut self) {
        for left in 0..self.length / 2 {
            self.swap(left, self.length - left - 1);
        }
    }

    pub fn sort_unstable_by(&mut self, mut compare: impl FnMut(&T, &T) -> std::cmp::Ordering) {
        fn sift_down<T, const N: usize>(owner: &mut PagedList<T, N>, start: usize, end: usize, compare: &mut impl FnMut(&T, &T) -> std::cmp::Ordering) {
            let mut root = start;
            loop {
                let child = root.saturating_mul(2).saturating_add(1);
                if child >= end {
                    return;
                }
                let mut selected = child;
                if child + 1 < end && compare(&owner[child], &owner[child + 1]).is_lt() {
                    selected = child + 1;
                }
                if !compare(&owner[root], &owner[selected]).is_lt() {
                    return;
                }
                owner.swap(root, selected);
                root = selected;
            }
        }
        for start in (0..self.length / 2).rev() {
            sift_down(self, start, self.length, &mut compare);
        }
        for end in (1..self.length).rev() {
            self.swap(0, end);
            sift_down(self, 0, end, &mut compare);
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.length, "paged list removal index out of bounds");
        for position in index..self.length - 1 {
            self.swap(position, position + 1);
        }
        self.pop().expect("validated paged list removal")
    }

    pub fn retain(&mut self, mut keep: impl FnMut(&T) -> bool) {
        let mut index = 0;
        while index < self.length {
            if keep(&self[index]) {
                index += 1;
            } else {
                drop(self.remove(index));
            }
        }
    }

    pub fn next_allocation_bytes(&self) -> Result<usize, PagedListError> {
        if self.has_reserved_slot() {
            return Ok(0);
        }
        self.next_page_allocation_bytes()
    }

    /// 🪜️ Quotes the actual prospective page path before its next backing allocation.
    pub fn next_reserve_depth_demand(&self)->Result<usize,PagedListError>{
        if self.has_reserved_slot(){return Ok(0)}
        self.next_page_allocation_bytes()?;
        self.height().max(Self::page_height(self.capacity/self.page_items())).checked_add(3).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list reserve depth overflow"})
    }

    /// 🎟️ Returns the next single backing allocation needed to reach a requested logical capacity.
    pub fn next_capacity_allocation_bytes(&self, capacity: usize) -> Result<Option<usize>, PagedListError> {
        if capacity > N {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list logical capacity exhausted" });
        }
        if capacity <= self.capacity {
            return Ok(None);
        }
        self.next_page_allocation_bytes().map(Some)
    }

    fn next_page_allocation_bytes(&self) -> Result<usize, PagedListError> { self.next_page_allocation_bytes_for_limit(N) }

    fn next_page_allocation_bytes_for_limit(&self, limit: usize) -> Result<usize, PagedListError> {
        if limit > N || self.capacity >= limit {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list logical capacity exhausted" });
        }
        if self.capacity % self.page_items() != 0 { return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "funded final payload extent cannot extend or reallocate" }); }
        let page = self.capacity / self.page_items();
        let root_height = self.height();
        if Self::page_height(page) > root_height { return Ok(size_of::<Page<T>>()); }
        let mut link = &self.root;
        for height in (0..=root_height).rev() {
            match link.first() {
                None => return Ok(size_of::<Page<T>>()),
                Some(Page::Branch { children, .. }) => link = &children[Self::slot(page, height)],
                Some(Page::Leaf { .. }) => return self.page_items().min(limit - self.capacity).checked_mul(size_of::<T>()).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list allocation overflow" }),
            }
        }
        Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list page authority is missing" })
    }

    /// 🎟️ Reserves one original metadata or payload page only with independent structural funding.
    pub fn reserve_one_funded(&mut self, grant:crate::retained_clone::RetainedCloneGrant) -> Result<crate::retained_clone::RetainedCloneProgress,PagedListAllocationError> {
        if self.has_reserved_slot() { return Ok(Default::default()); }
        let copy=self.next_reserve_copy_byte_demand().map_err(|error|PagedListAllocationError{allocated_bytes:0,kind:error.kind,reason:error.reason})?;
        let capacity=self.next_allocation_bytes().map_err(|error|PagedListAllocationError{allocated_bytes:0,kind:error.kind,reason:error.reason})?;
        if grant.maximum_items==0||grant.maximum_depth==0||copy>grant.maximum_copy_bytes||capacity>grant.maximum_capacity_bytes { return Ok(Default::default()); }
        let progress=self.reserve_one(grant.maximum_capacity_bytes)?;
        Ok(crate::retained_clone::RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:if progress.progressed{copy}else{0},retained_capacity_bytes:progress.allocated_bytes,released_bytes:0})
    }
    /// 📏️ Admits the actual remaining final payload extent without reserving a full unused page.
    pub fn next_exact_capacity_allocation_bytes(&self, limit: usize) -> Result<Option<usize>, PagedListError> {
        if limit > N || limit < self.length {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" });
        }
        if limit <= self.capacity { return Ok(None); }
        self.next_page_allocation_bytes_for_limit(limit).map(Some)
    }

    /// 🧱️ The source's declared final extent funds one genuine metadata or payload allocation.
    pub fn reserve_exact_capacity_one(&mut self, limit: usize, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if limit > N || limit < self.length {
            return Err(PagedListAllocationError { allocated_bytes: 0, kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" });
        }
        if limit <= self.capacity { return Ok(PagedListProgress::default()); }
        self.reserve_page_using_for_limit::<ExactAllocation>(grant, limit)
    }

    pub fn reserve_one(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if self.has_reserved_slot() {
            return Ok(PagedListProgress::default());
        }
        self.reserve_page(grant)
    }

    #[cfg(test)]
    pub(crate) fn reserve_one_using<A: PageAllocation>(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if self.has_reserved_slot() { return Ok(PagedListProgress::default()); }
        self.reserve_page_using::<A>(grant)
    }

    /// 🎟️ Admits at most one backing allocation toward a requested logical capacity.
    pub fn reserve_capacity_one(&mut self, capacity: usize, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if capacity > N {
            return Err(PagedListAllocationError { allocated_bytes: 0, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list logical capacity exhausted" });
        }
        if capacity <= self.capacity { return Ok(PagedListProgress::default()); }
        self.reserve_page(grant)
    }

    pub fn reserve_full(&mut self) -> Result<bool, PagedListError> {
        let before = self.capacity;
        while self.capacity < N {
            let requested = self.next_page_allocation_bytes()?;
            self.reserve_page(requested).map_err(PagedListAllocationError::refusal)?;
        }
        Ok(self.capacity != before)
    }

    fn reserve_page(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        self.reserve_page_using::<ExactAllocation>(grant)
    }

    fn reserve_page_using<A: PageAllocation>(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> { self.reserve_page_using_for_limit::<A>(grant, N) }

    fn reserve_page_using_for_limit<A: PageAllocation>(&mut self, grant: usize, limit: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        let rejected = |kind, reason| PagedListAllocationError { allocated_bytes: 0, kind, reason };
        let requested = self.next_page_allocation_bytes_for_limit(limit).map_err(|error| rejected(error.kind, error.reason))?;
        if grant < requested {
            return Ok(PagedListProgress::default());
        }
        self.allocated.checked_add(requested).filter(|total| *total <= isize::MAX as usize).ok_or_else(|| rejected(PagedListRefusalKind::OwnershipLimit, "fixed list allocation counter exceeds addressable ownership"))?;
        let page_items = self.page_items();
        let page = self.capacity / page_items;
        let root_height = self.height();
        if Self::page_height(page) > root_height {
            let mut root = Vec::new();
            A::reserve(&mut root, 1).map_err(|_| rejected(PagedListRefusalKind::AllocationFailed, "fixed list metadata allocation failed"))?;
            let actual = root.capacity() * size_of::<Page<T>>();
            let mut children = std::array::from_fn(|_| Vec::new());
            children[0] = std::mem::take(&mut self.root);
            root.push(Page::Branch { height: root_height + 1, children });
            self.root = root;
            self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
            if self.allocated > isize::MAX as usize {
                return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" });
            }
            if actual > grant {
                return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list metadata allocation exceeded admission; owner retained" });
            }
            return Ok(PagedListProgress { progressed: true, allocated_bytes: actual, ..Default::default() });
        }
        let mut link = &mut self.root;
        for height in (0..=root_height).rev() {
            if link.is_empty() {
                A::reserve(link, 1).map_err(|_| rejected(PagedListRefusalKind::AllocationFailed, "fixed list metadata allocation failed"))?;
                let actual = link.capacity() * size_of::<Page<T>>();
                link.push(if height == 0 { Page::Leaf { items: Vec::new(), slots: 0 } } else { Page::Branch { height, children: std::array::from_fn(|_| Vec::new()) } });
                self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                if self.allocated > isize::MAX as usize {
                    return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" });
                }
                if actual > grant {
                    return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list metadata allocation exceeded admission; owner retained" });
                }
                return Ok(PagedListProgress { progressed: true, allocated_bytes: actual, ..Default::default() });
            }
            match &mut link[0] {
                Page::Branch { children, .. } => link = &mut children[Self::slot(page, height)],
                Page::Leaf { items, slots } => {
                    let admitted_slots = page_items.min(limit - self.capacity);
                    A::reserve(items, admitted_slots).map_err(|_| rejected(PagedListRefusalKind::AllocationFailed, "fixed list payload allocation failed"))?;
                    let actual = if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() };
                    *slots = admitted_slots;
                    self.capacity += admitted_slots;
                    self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                    if self.allocated > isize::MAX as usize {
                        return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" });
                    }
                    if actual > grant {
                        return Err(PagedListAllocationError { allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload allocation exceeded admission; owner retained" });
                    }
                    return Ok(PagedListProgress { progressed: true, allocated_bytes: actual, ..Default::default() });
                }
            }
        }
        Err(rejected(PagedListRefusalKind::InvariantViolated, "fixed list page authority is missing"))
    }

    /// 📏️ Quotes the original reserved node path before one owner enters its next slot.
    pub fn next_push_depth_demand(&self)->Result<usize,PagedListError>{
        if !self.has_reserved_slot(){return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list push requires its original reserved slot"});}
        self.height().checked_add(3).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list push depth overflow"})
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

    /// 🌱️ Appends one cold owner while admitting every backing allocation before it is made.
    pub fn try_push(&mut self, value: T) -> Result<(), PagedListError> {
        while !self.has_reserved_slot() {
            let required = self.next_allocation_bytes()?;
            let progress = self.reserve_one(required).map_err(PagedListAllocationError::refusal)?;
            if !progress.progressed {
                return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list exact cold append did not progress" });
            }
        }
        self.push_reserved(value).map_err(|_| PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list rejected its admitted cold append" })
    }

    pub fn place_reserved(&mut self, source: &mut Option<T>, grant: usize) -> Result<PagedListProgress, PagedListError> {
        if source.is_none() || !self.has_reserved_slot() || grant < size_of::<T>() {
            return Ok(PagedListProgress::default());
        }
        if let Err(owner) = self.push_reserved(source.take().expect("checked source owner")) {
            *source = Some(owner);
            return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list reserved authority rejected exact owner" });
        }
        Ok(PagedListProgress { progressed: true, placed_bytes: size_of::<T>(), ..Default::default() })
    }

    /// 🤝 Adopts an already-copied owner into admitted backing without charging another payload copy.
    pub fn adopt_reserved(&mut self, source: &mut Option<T>) -> Result<PagedListProgress, PagedListError> {
        if source.is_none() || !self.has_reserved_slot() {
            return Ok(PagedListProgress::default());
        }
        if let Err(owner) = self.push_reserved(source.take().expect("checked source owner")) {
            *source = Some(owner);
            return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list reserved authority rejected adopted owner" });
        }
        Ok(PagedListProgress { progressed: true, ..Default::default() })
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.length.checked_sub(1)?;
        let owner = self.leaf_mut(index)?.pop()?;
        self.length -= 1;
        Some(owner)
    }

    /// ♻️ Returns the exact physical byte grant needed by the next releasable backing.
    pub fn next_release_allocation_bytes(&self) -> Result<usize, PagedListError> {
        fn next<T>(link: &[Page<T>], capacity: usize) -> Result<usize, PagedListError> {
            let Some(node) = link.first() else {
                return Ok(capacity * size_of::<Page<T>>());
            };
            match node {
                Page::Branch { children, .. } => {
                    if let Some(child) = children.iter().rfind(|child| !child.is_empty()) {
                        return next(child, child.capacity());
                    }
                }
                Page::Leaf { items, slots } => {
                    if !items.is_empty() {
                        return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload must retire before its page" });
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

    /// 📏️ Quotes the actual node path followed by the next retained tail removal.
    pub fn next_pop_depth_demand(&self) -> Result<usize, PagedListError> {
        if self.is_empty() { return Ok(0); }
        self.height().checked_add(3).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list tail depth overflow" })
    }

    /// 📏️ Quotes the actual retained node path of the next whole empty backing release.
    pub fn next_release_depth_demand(&self) -> Result<usize, PagedListError> {
        fn next<T>(link: &[Page<T>], capacity: usize, depth: usize) -> Result<usize, PagedListError> {
            let Some(node) = link.first() else { return Ok(if capacity == 0 { 0 } else { depth }); };
            match node {
                Page::Branch { children, .. } => {
                    if let Some(child) = children.iter().rfind(|child| !child.is_empty()) {
                        let depth = depth.checked_add(1).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list release depth overflow" })?;
                        return next(child, child.capacity(), depth);
                    }
                }
                Page::Leaf { items, slots } => {
                    if !items.is_empty() { return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload must retire before its page" }); }
                    if *slots != 0 { return depth.checked_add(1).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list leaf release depth overflow" }); }
                }
            }
            Ok(depth)
        }
        next(&self.root, self.root.capacity(), 2)
    }

    pub fn truncate_retired_last(&mut self) -> Result<(), PagedListError> {
        let index = self.length.checked_sub(1).ok_or(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list has no retired payload" })?;
        let items = self.leaf_mut(index).ok_or(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload page is missing" })?;
        items.truncate(items.len() - 1);
        self.length -= 1;
        Ok(())
    }

    /// 🏠️ Hands one genuine empty allocation to its pre-admitted parent without physical disposal credit.
    pub fn return_empty_page<const P:usize>(&mut self,parent:&mut crate::retirement::allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<PagedListReturnProgress,crate::ValueError>{
        fn transfer<T,const P:usize>(link:&mut Vec<Page<T>>,slots:&mut usize,parent:&mut crate::retirement::allocation_return::ParentAllocationReturn<P>)->Result<PagedListReturnProgress,crate::ValueError>{
            let Some(node)=link.first_mut()else{return Ok(PagedListReturnProgress::default());};
            match node{
                Page::Branch { children, .. }=>{
                    if let Some(index)=children.iter().rposition(|child|!child.is_empty()){return transfer(&mut children[index],slots,parent);}
                }
                Page::Leaf{items,slots:reserved}=>{
                    if !items.is_empty(){return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated,"paged payload must retire before parent backing handoff"));}
                    if *reserved!=0{
                        let bytes=items.capacity().checked_mul(size_of::<T>()).ok_or_else(||crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"paged payload backing layout is invalid"))?;
                        if !parent.return_empty_vec(items,1)?{return Ok(PagedListReturnProgress::default());}
                        *slots-=*reserved;*reserved=0;
                        return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:bytes});
                    }
                }
            }
            let bytes=link.capacity().checked_mul(size_of::<Page<T>>()).ok_or_else(||crate::ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"paged metadata backing layout is invalid"))?;
            if !parent.can_receive_vec_backing::<Page<T>>(link.capacity())?{return Ok(PagedListReturnProgress::default());}
            link.clear();
            if !parent.return_empty_vec(link,1)?{return Err(crate::ValueError::new(crate::ValueRefusalKind::InvariantViolated,"pre-admitted parent backing slot disappeared"));}
            Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:bytes})
        }
        if maximum_items==0{return Ok(PagedListReturnProgress::default());}
        let progress=transfer(&mut self.root,&mut self.capacity,parent)?;
        self.allocated-=progress.returned_allocation_bytes;
        Ok(progress)
    }


    pub fn release_empty_page(&mut self, maximum_bytes: usize) -> Result<PagedListProgress, PagedListError> {
        fn release<T>(link: &mut Vec<Page<T>>, slots: &mut usize, maximum_bytes: usize) -> Result<PagedListProgress, PagedListError> {
            let Some(node) = link.first_mut() else {
                return Ok(PagedListProgress::default());
            };
            match node {
                Page::Branch { children, .. } => {
                    if let Some(index) = children.iter().rposition(|child| !child.is_empty()) {
                        return release(&mut children[index], slots, maximum_bytes);
                    }
                }
                Page::Leaf { items, slots: reserved } => {
                    if !items.is_empty() {
                        return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload must retire before its page" });
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
                Some(Page::Branch { children, .. }) => children.iter().map(|child| count(child)).sum(),
            }
        }
        count(&self.root)
    }
}
//#endregion 🌳️Pages

impl<T, const N: usize> std::ops::Index<usize> for PagedList<T, N> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("paged list index out of bounds")
    }
}

impl<T, const N: usize> std::ops::IndexMut<usize> for PagedList<T, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("paged list index out of bounds")
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a PagedList<T, N> {
    type Item = &'a T;
    type IntoIter = PagedIter<'a, T, N>;
    fn into_iter(self) -> Self::IntoIter { self.iter() }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut PagedList<T, N> {
    type Item = &'a mut T;
    type IntoIter = PagedIterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter { self.iter_mut() }
}


/// 🧊️ Cold copies allocate each backing page explicitly; retained work uses admission and placement steps.
impl<T: Clone, const N: usize> Clone for PagedList<T, N> {
    fn clone(&self) -> Self {
        let mut copy = Self { payload_page_bytes: self.payload_page_bytes, ..Self::empty() };
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

impl<T: std::fmt::Debug, const N: usize> std::fmt::Debug for PagedList<T, N> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq, const N: usize> PartialEq for PagedList<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.iter().eq(other.iter())
    }
}

impl<T: Eq, const N: usize> Eq for PagedList<T, N> {}

impl<T, const N: usize> FromIterator<T> for PagedList<T, N> {
    fn from_iter<I: IntoIterator<Item = T>>(values: I) -> Self { Self::try_from_iter(values).expect("cold list construction must fit its declared capacity") }
}

impl<T, const N: usize> From<Vec<T>> for PagedList<T, N> {
    fn from(values: Vec<T>) -> Self { values.into_iter().collect() }
}

/// 📤️ Transfers native paged owners in semantic order without a contiguous output allocation.
pub struct PagedIntoIter<T, const N: usize> { owner: PagedList<T, N> }

impl<T, const N: usize> Iterator for PagedIntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<T> { self.owner.pop() }
    fn size_hint(&self) -> (usize, Option<usize>) { let length = self.owner.len(); (length, Some(length)) }
}

impl<T, const N: usize> ExactSizeIterator for PagedIntoIter<T, N> {}

impl<T, const N: usize> DoubleEndedIterator for PagedIntoIter<T, N> {
    fn next_back(&mut self) -> Option<T> { (!self.owner.is_empty()).then(|| self.owner.remove(0)) }
}

impl<T, const N: usize> IntoIterator for PagedList<T, N> {
    type Item = T;
    type IntoIter = PagedIntoIter<T, N>;
    fn into_iter(mut self) -> Self::IntoIter { self.reverse(); PagedIntoIter { owner: self } }
}

impl<T, const N: usize> PagedList<T, N> {
    /// 🧊️ Constructs an empty native sequence; retained writes use explicit reserve and placement.
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, value: T) { self.try_push(value).expect("cold list append must fit its declared capacity"); }
    pub fn insert(&mut self, index: usize, value: T) {
        assert!(index <= self.len());
        self.push(value);
        for at in (index..self.len() - 1).rev() { self.swap(at, at + 1); }
    }
    pub fn clear(&mut self) { while self.pop().is_some() {} }
    pub fn truncate(&mut self, length: usize) { while self.len() > length { self.pop(); } }
    pub fn first(&self) -> Option<&T> { self.get(0) }
    pub fn first_mut(&mut self) -> Option<&mut T> { self.get_mut(0) }
    pub fn last(&self) -> Option<&T> { self.len().checked_sub(1).and_then(|index| self.get(index)) }
    pub fn last_mut(&mut self) -> Option<&mut T> { self.len().checked_sub(1).and_then(|index| self.get_mut(index)) }
}

impl<T, const N: usize> Extend<T> for PagedList<T, N> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, values: I) { for value in values { self.push(value); } }
}

#[cfg(test)]
#[path = "🧪️tests/⚠️refusal/🦀️.rs"]
mod refusal_tests;
