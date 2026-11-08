//! 📋️ Fixed-fanout metadata and separately admitted payload pages for bounded value lists.

use std::mem::size_of;



#[path = "🎮️edit/🦀️.rs"]
mod edit;
pub use edit::{PagedListEditCursor,PagedListEditStep,PagedListEditFailure,PagedListEditOutcome};
#[path="🎟️reservation/🔎️selection/🦀️.rs"]
mod reservation_selection;
pub use reservation_selection::{PagedListReservationSelection,PagedListReservationKind};
#[path="🎟️reservation/♻️release/🦀️.rs"]
mod release_selection;
pub use release_selection::PagedListReleaseSelection;
#[path="🎟️reservation/🪪️header/🦀️.rs"]
mod selection_header;
pub use selection_header::PagedListSelectionHeader;

//#region 🎟️Progress
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct PagedListReturnProgress{pub progressed:bool,pub returned_allocation_bytes:usize}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PagedListProgress {
    pub work: u64,
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
    pub work: u64,
    pub allocated_bytes: usize,
    pub kind: PagedListRefusalKind,
    pub reason: &'static str,
}
/// 🚦️ Distinguishes a completed denial from an admitted or already-owned page.
#[derive(Debug)]
pub enum PagedListReservationOutcome<T=PagedListProgress>{Denied,Ready(T)}
/// 🪢️ Preserves the original Work refusal separately from the actual allocation refusal.
#[derive(Debug)]
pub enum PagedListReservationFailure{Work(semio_framework_value_control_contract::ValueError),Allocation{error:PagedListAllocationError,copied_bytes:usize}}
impl From<PagedListAllocationError> for PagedListReservationFailure{fn from(error:PagedListAllocationError)->Self{Self::Allocation{error,copied_bytes:0}}}
/// 🚦️ Distinguishes a completed denial from an actual empty-page release.
#[derive(Debug)]
pub enum PagedListReleaseOutcome{Denied,Ready(PagedListProgress)}
/// ♻️ Preserves original Work refusal independently of the page release refusal.
#[derive(Debug)]
pub enum PagedListReleaseFailure{Work(semio_framework_value_control_contract::ValueError),Release(PagedListError)}
impl From<PagedListError> for PagedListReleaseFailure{fn from(error:PagedListError)->Self{Self::Release(error)}}
fn reservation_query_failure(error:PagedListError)->PagedListReservationFailure{PagedListReservationFailure::Allocation{error:PagedListAllocationError{work:0,allocated_bytes:0,kind:error.kind,reason:error.reason},copied_bytes:0}}
impl PagedListAllocationError {
    /// 🧭️ Retains refusal identity independently of admitted bytes still owned by the list.
    pub fn refusal(self) -> PagedListError { PagedListError { kind: self.kind, reason: self.reason } }
}
//#endregion 🎟️Progress

//#region 🌳️Pages
const FANOUT: usize = 16;
const PAGE_BYTES: usize = 4096;

/// 🧷️ Supplies one explicit backing-reservation policy; insufficient successful capacity is refused and retained for funded settlement.
///
/// # Safety
/// Implementations preserve every existing element, order and ownership. An error leaves the vector and its capacity unchanged.
/// One call performs at most one bounded allocation attempt and must not unwind. The marker remains absent for no allocation, is set to zero before an actual nonzero attempt, and records the acquired bytes on success.
pub unsafe trait PageAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize, actual:Option<&mut Option<usize>>) -> Result<(), std::collections::TryReserveError>;
}

struct ExactAllocation;
unsafe impl PageAllocation for ExactAllocation {
    fn reserve<T>(owner: &mut Vec<T>, slots: usize, actual:Option<&mut Option<usize>>) -> Result<(), std::collections::TryReserveError> {
        let allocating=size_of::<T>()!=0&&owner.capacity()<owner.len().saturating_add(slots);let result=owner.try_reserve_exact(slots);if allocating{if let Some(actual)=actual{*actual=Some(if result.is_ok(){owner.capacity()*size_of::<T>()}else{0})}}result
    }
}

#[cfg(test)]
#[path = "🧪️tests/⏱️effect/🦀️.rs"]
pub(crate) mod effect;
#[cfg(test)]
pub(crate) use effect::Granted;
#[cfg(test)]
#[path = "🧪️tests/🔬️counter/🦀️.rs"]
mod counter_tests;
#[cfg(test)]
#[path="🎟️reservation/🧪️tests/🦀️.rs"]
mod original_reservation_tests;

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
    /// ⏭️ Skips by moving the front position: `skip(k)` costs one `get`, not `k`.
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.front = self.front.saturating_add(n).min(self.back);
        self.next()
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
    reservation_epoch: usize,
}

impl<T, const N: usize> Default for PagedList<T, N> {
    fn default() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0, payload_page_bytes: PAGE_BYTES, reservation_epoch: 0 }
    }
}

impl<T, const N: usize> PagedList<T, N> {
    pub const fn empty() -> Self {
        Self { root: Vec::new(), length: 0, capacity: 0, allocated: 0, payload_page_bytes: PAGE_BYTES, reservation_epoch: 0 }
    }

    /// 🪶️ Selects exactly `items` elements per payload page so a small owner never reserves the 4096-byte default page.
    pub fn with_page_items(items: usize) -> Self {
        Self { payload_page_bytes: items.max(1).saturating_mul(size_of::<T>().max(1)).min(PAGE_BYTES).max(size_of::<T>()), ..Self::empty() }
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
                let progress = output.reserve_page(required).map_err(PagedListAllocationError::refusal)?;
                if !progress.progressed {
                    return Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list exact cold allocation did not progress" });
                }
            }
            output.push_reserved(value).map_err(|_| PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list rejected its reserved slot" })?;
        }
        Ok(output)
    }

    /// 🔨️ Builds a paged owner from fallible domain conversion without an intermediate collection.
    pub fn try_from_fallible_iter<E: From<PagedListError>>(values: impl IntoIterator<Item = Result<T, E>>) -> Result<Self, E> {
        let mut output = Self::default();
        for value in values {
            let value = value?;
            while !output.has_reserved_slot() {
                let required = output.next_allocation_bytes().map_err(E::from)?;
                let progress = output.reserve_page(required).map_err(|error| E::from(error.refusal()))?;
                if !progress.progressed {
                    return Err(E::from(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list exact cold allocation did not progress" }));
                }
            }
            output.push_reserved(value).map_err(|_| E::from(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "paged list rejected its reserved slot" }))?;
        }
        Ok(output)
    }
    /// 📐️ The least capacity at or above `length` that ends on a whole payload page, so an owner can keep growing page by page.
    pub fn whole_page_capacity(&self, length: usize) -> usize {
        let items = self.page_items();
        length.div_ceil(items).saturating_mul(items).min(N)
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
    /// ⏭️ Quotes the existing original page path before one logical slot is borrowed or changed.
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

    /// ⛽️ Quotes the exact index-digit and native-link path of one original bounded borrow.
    pub fn get_source_work_demand(&self,index:usize)->Result<u64,PagedListError>{if index>=self.length{return Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"original lookup index exceeds its occupied extent"})}let page=index/self.page_items();let digits=(usize::BITS-page.leading_zeros()).div_ceil(4)as u64;4u64.checked_add(self.height()as u64).and_then(|work|work.checked_add(digits)).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"original lookup work overflow"})}
    /// 📖️ Borrows one actual occupied slot only after its complete bounded path is funded.
    pub fn get_source_step(&self,index:usize,maximum:u64)->semio_framework_value_control_contract::RetainedWorkOutcome<Option<&T>,PagedListError>{let mut work=0;let result=(||{let q=self.get_source_work_demand(index)?;if maximum<q{return Ok(None)}let page=index/self.page_items();let height=self.height();work=2;let mut digits=page;while digits!=0{digits>>=4;work+=1}let mut link=&self.root;for level in(0..=height).rev(){work+=1;match link.first(){Some(Page::Branch{children,..})=>link=&children[Self::slot(page,level)],Some(Page::Leaf{items,..})=>{work+=1;return items.get(index%self.page_items()).map(Some).ok_or(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"original lookup occupied leaf is absent"})},None=>break}}Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"original lookup occupied path is absent"})})();semio_framework_value_control_contract::RetainedWorkOutcome{result,work}}
    /// ✏️ Lends one occupied destination after its full path is funded; the caller separately funds its writes.
    pub fn get_mut_source_step(&mut self,index:usize,maximum:u64)->semio_framework_value_control_contract::RetainedWorkOutcome<Option<&mut T>,PagedListError>{let q=match self.get_source_work_demand(index){Ok(q)=>q,Err(error)=>return semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(error),work:0}};if maximum<q{return semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok(None),work:0}}let page_items=self.page_items();let page=index/page_items;let height=self.height();let mut work=2;let mut digits=page;while digits!=0{digits>>=4;work+=1}let mut link=&mut self.root;for level in(0..=height).rev(){work+=1;match link.first_mut(){Some(Page::Branch{children,..})=>link=&mut children[Self::slot(page,level)],Some(Page::Leaf{items,..})=>{work+=1;let result=items.get_mut(index%page_items).map(Some).ok_or(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"original mutable lookup occupied leaf is absent"});return semio_framework_value_control_contract::RetainedWorkOutcome{result,work}},None=>break}}semio_framework_value_control_contract::RetainedWorkOutcome{result:Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"original mutable lookup occupied path is absent"}),work}}
    /// 🔎️ Returns the first index whose element fails `predicate` on a list partitioned by it; see [`slice::partition_point`].
    pub fn partition_point(&self, mut predicate: impl FnMut(&T) -> bool) -> usize {
        let (mut low, mut high) = (0, self.length);
        while low < high {
            let middle = low + (high - low) / 2;
            match self.get(middle) { Some(value) if predicate(value) => low = middle + 1, _ => high = middle }
        }
        low
    }
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.length {
            return None;
        }
        self.leaf(index)?.get(index % self.page_items())
    }

    /// 📖️ Borrows only the original occupied suffix of the physical page containing this logical position.
    pub fn contiguous_at(&self,index:usize)->Option<&[T]>{if index>=self.length{return None}let leaf=self.leaf(index)?;let offset=index%self.page_items();let count=(self.length-index).min(leaf.len().checked_sub(offset)?);Some(&leaf[offset..offset+count])}

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
        self.next_page_reserve_depth_demand()
    }
    fn next_page_reserve_depth_demand(&self)->Result<usize,PagedListError>{
        self.next_page_allocation_bytes()?;
        self.height().max(Self::page_height(self.capacity/self.page_items())).checked_add(3).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list reserve depth overflow"})
    }

    /// 🪹️ Quotes the actual selected native page and owner-header writes before allocation.
    pub fn next_reserve_copy_byte_demand(&self)->Result<usize,PagedListError>{
        if self.has_reserved_slot(){return Ok(0)}
        self.next_page_copy_byte_demand()
    }
    /// 🦜️ Quotes the selected native page-header writes toward a declared capacity.
    pub fn next_capacity_copy_byte_demand(&self,capacity:usize)->Result<usize,PagedListError>{
        if capacity>N{return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list logical capacity exhausted"})}
        if capacity<=self.capacity{return Ok(0)}
        self.next_page_copy_byte_demand()
    }
    /// 📄️ Quotes the original page path of the next backing allocation toward a declared logical capacity.
    /// 🧭️ Quotes traversal of the next original backing toward a declared capacity, including preadmission before current slots fill.
    pub fn next_capacity_depth_demand(&self,capacity:usize)->Result<usize,PagedListError>{
        if capacity>N{return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list logical capacity exhausted"})}
        if capacity<=self.capacity{return Ok(0)}
        self.next_page_reserve_depth_demand()
    }
    fn next_page_copy_byte_demand(&self)->Result<usize,PagedListError>{
        self.next_page_allocation_bytes()?;let page=self.capacity/self.page_items();let root_height=self.height();let metadata=size_of::<Vec<Page<T>>>()+size_of::<Page<T>>()+size_of::<usize>();if Self::page_height(page)>root_height{return Ok(metadata)}let mut link=&self.root;for height in (0..=root_height).rev(){if link.is_empty(){return Ok(metadata)}match &link[0]{Page::Branch{children,..}=>link=&children[Self::slot(page,height)],Page::Leaf{..}=>return Ok(size_of::<Vec<T>>()+3*size_of::<usize>())}}Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"fixed list page authority is missing"})
    }

    /// ↩️ Returns the next single backing allocation needed to reach a requested logical capacity.
    pub fn next_capacity_allocation_bytes(&self, capacity: usize) -> Result<Option<usize>, PagedListError> {
        if capacity > N {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list logical capacity exhausted" });
        }
        if capacity <= self.capacity {
            return Ok(None);
        }
        self.next_page_allocation_bytes_for_limit(capacity).map(Some)
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
                None => return Ok(if link.capacity()==0{size_of::<Page<T>>()}else{0}),
                Some(Page::Branch { children, .. }) => link = &children[Self::slot(page, height)],
                Some(Page::Leaf { .. }) => return self.page_items().min(limit - self.capacity).checked_mul(size_of::<T>()).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list allocation overflow" }),
            }
        }
        Err(PagedListError { kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list page authority is missing" })
    }

    /// 📃️ Reserves one original metadata or payload page only with independent structural funding.
    pub fn reserve_one_funded(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome<semio_framework_value_control_contract::RetainedCloneProgress>,PagedListReservationFailure>{let turn=self.reserve_one(effect);semio_framework_value_control_contract::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|outcome|match outcome{PagedListReservationOutcome::Denied=>PagedListReservationOutcome::Denied,PagedListReservationOutcome::Ready(progress)=>PagedListReservationOutcome::Ready(semio_framework_value_control_contract::RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:progress.placed_bytes,retained_capacity_bytes:progress.allocated_bytes,released_bytes:0})})}}
    /// 📬️ Reserves one original page toward its declared capacity through the same effect.
    pub fn reserve_capacity_one_funded(&mut self,capacity:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome<semio_framework_value_control_contract::RetainedCloneProgress>,PagedListReservationFailure>{let turn=self.reserve_capacity_one(capacity,effect);semio_framework_value_control_contract::RetainedWorkOutcome{work:turn.work,result:turn.result.map(|outcome|match outcome{PagedListReservationOutcome::Denied=>PagedListReservationOutcome::Denied,PagedListReservationOutcome::Ready(progress)=>PagedListReservationOutcome::Ready(semio_framework_value_control_contract::RetainedCloneProgress{copied_items:usize::from(progress.progressed),copied_bytes:progress.placed_bytes,retained_capacity_bytes:progress.allocated_bytes,released_bytes:0})})}}
    /// 📏️ Admits the actual remaining final payload extent without reserving a full unused page.
    pub fn next_exact_capacity_allocation_bytes(&self, limit: usize) -> Result<Option<usize>, PagedListError> {
        if limit > N || limit < self.length {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" }.into());
        }
        if limit <= self.capacity { return Ok(None); }
        self.next_page_allocation_bytes_for_limit(limit).map(Some)
    }

    /// 🧱️ The source's declared final extent funds one genuine metadata or payload allocation.
    pub fn reserve_exact_capacity_one(&mut self,limit:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>{use semio_framework_value_control_contract::{RetainedWorkOutcome};if limit>N||limit<self.length{return RetainedWorkOutcome{result:Err(PagedListAllocationError{work:0,allocated_bytes:0,kind:PagedListRefusalKind::OwnershipLimit,reason:"exact final extent exceeds logical authority"}.into()),work:0}}if limit<=self.capacity{return RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Ready(PagedListProgress::default())),work:0}}self.reserve_effect::<ExactAllocation>(limit,effect)
    }

    fn reserve_effect<A:PageAllocation>(&mut self,limit:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>{use semio_framework_value_control_contract::{RetainedWorkOutcome};let grant=effect.grant();let selected=(||{let copy=self.next_capacity_copy_byte_demand(limit).map_err(reservation_query_failure)?;let depth=self.next_capacity_depth_demand(limit).map_err(reservation_query_failure)?;Ok::<_,PagedListReservationFailure>((copy,depth))})();let(copy,depth)=match selected{Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Denied),work:0}}let before=effect.performed_work();let result=self.reserve_page_using_for_limit::<A>(grant.maximum_capacity_bytes,limit,Some(effect));let work=effect.performed_work().checked_sub(before).expect("original page work receipt moved backwards");match result{Ok(progress)if !progress.progressed=>RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Denied),work},Ok(progress)=>RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Ready(progress)),work},Err(error)=>RetainedWorkOutcome{result:Err(error),work}}}
    /// 📃️ Reserves the exact original page with the same performed-work receipt.
    pub fn reserve_one(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>{if self.has_reserved_slot(){return semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Ready(PagedListProgress::default())),work:0}}self.reserve_effect::<ExactAllocation>(N,effect)}
    /// 🧰️ Reserves using the original allocation policy and same performed-work receipt.
    pub fn reserve_one_using<A:PageAllocation>(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>{if self.has_reserved_slot(){return semio_framework_value_control_contract::RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Ready(PagedListProgress::default())),work:0}}self.reserve_effect::<A>(N,effect)}
    /// ⏪️ Reserves at most one original page under the same effect and exact requested extent.
    pub fn reserve_capacity_one(&mut self,capacity:usize,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReservationOutcome,PagedListReservationFailure>{use semio_framework_value_control_contract::{RetainedWorkOutcome};if capacity>N{return RetainedWorkOutcome{result:Err(PagedListAllocationError{work:0,allocated_bytes:0,kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list logical capacity exhausted"}.into()),work:0}}if capacity<=self.capacity{return RetainedWorkOutcome{result:Ok(PagedListReservationOutcome::Ready(PagedListProgress::default())),work:0}}self.reserve_effect::<ExactAllocation>(capacity,effect)}

    /// ⛽️ Quotes the selected capacity reservation before the original Work action.
    pub fn reserve_capacity_one_work_demand(&self,capacity:usize)->Result<u64,PagedListError>{if capacity>N{return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list logical capacity exhausted"})}if capacity<=self.capacity{return Ok(0)}Self::reservation_selection_work_demand().checked_add(PagedListReservationSelection::allocation_work_demand()).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"paged reserve work overflow"})}
    pub fn reserve_one_work_demand(&self)->Result<u64,PagedListError>{if self.has_reserved_slot(){return Ok(0)}Self::reservation_selection_work_demand().checked_add(PagedListReservationSelection::allocation_work_demand()).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"paged reserve work overflow"})}

    /// 🧊️ Reserves one slot at an explicitly synchronous boundary whose caller already charged the exact page bytes.
    pub fn reserve_one_cold(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> { self.reserve_page(grant) }

    pub fn reserve_full(&mut self) -> Result<bool, PagedListError> {
        let before = self.capacity;
        while self.capacity < N {
            let requested = self.next_page_allocation_bytes()?;
            self.reserve_page(requested).map_err(PagedListAllocationError::refusal)?;
        }
        Ok(self.capacity != before)
    }

    /// 🧊️ Reserves the exact final extent at an explicitly synchronous boundary whose caller already charged the exact page bytes.
    pub fn reserve_exact_capacity_one_cold(&mut self, limit: usize, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if limit > N || limit < self.length { return Err(PagedListAllocationError { work: 0, allocated_bytes: 0, kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" }); }
        if limit <= self.capacity { return Ok(PagedListProgress::default()); }
        match self.reserve_page_using_for_limit::<ExactAllocation>(grant, limit, None) { Ok(progress) => Ok(progress), Err(PagedListReservationFailure::Allocation { error, .. }) => Err(error), Err(PagedListReservationFailure::Work(_)) => Err(PagedListAllocationError { work: 0, allocated_bytes: 0, kind: PagedListRefusalKind::InvariantViolated, reason: "cold reservation has no Work loan" }) }
    }

    pub(crate) fn reserve_page(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        self.reserve_page_using::<ExactAllocation>(grant)
    }

    fn reserve_page_using<A: PageAllocation>(&mut self, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {match self.reserve_page_using_for_limit::<A>(grant,N,None){Ok(progress)=>Ok(progress),Err(PagedListReservationFailure::Allocation{error,..})=>Err(error),Err(PagedListReservationFailure::Work(_))=>unreachable!("cold reservation has no Work loan")}}

    fn reserve_page_using_for_limit<A: PageAllocation>(&mut self, grant: usize, limit: usize, mut effect:Option<&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>>) -> Result<PagedListProgress,PagedListReservationFailure> {
        fn pay(effect:&mut Option<&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>>)->Result<(),PagedListReservationFailure>{if let Some(effect)=effect.as_deref_mut(){match effect.performed(1){Ok(true)=>{},Ok(false)=>return Err(PagedListReservationFailure::Work(semio_framework_value_control_contract::ValueError::literal(semio_framework_value_control_contract::ValueRefusalKind::WorkLimit,"original page reservation exceeded its work authority"))),Err(error)=>return Err(PagedListReservationFailure::Work(error))}}Ok(())}
        let rejected = |kind, reason| PagedListAllocationError { work: 0, allocated_bytes: 0, kind, reason };
        if effect.as_ref().is_some_and(|effect|effect.maximum_work_units()==0){return Ok(PagedListProgress::default())}
        if effect.as_deref_mut().is_some_and(|effect|effect.actual().is_some()){return Err(rejected(PagedListRefusalKind::InvariantViolated,"original page allocation attempt is occupied").into())}let requested=self.next_page_allocation_bytes_for_limit(limit).map_err(|error|rejected(error.kind,error.reason))?;let copy=self.next_page_copy_byte_demand().map_err(|error|rejected(error.kind,error.reason))?;
        if grant < requested {
            return Ok(PagedListProgress::default());
        }
        self.allocated.checked_add(requested).filter(|total| *total <= isize::MAX as usize).ok_or_else(|| rejected(PagedListRefusalKind::OwnershipLimit, "fixed list allocation counter exceeds addressable ownership"))?;
        let page_items = self.page_items();
        let page = self.capacity / page_items;
        let root_height = self.height();
        if Self::page_height(page) > root_height {
            let mut root = Vec::new();
            pay(&mut effect)?;A::reserve(&mut root, 1,effect.as_deref_mut().map(|effect|effect.actual())).map_err(|_| PagedListAllocationError{work:1,..rejected(PagedListRefusalKind::AllocationFailed, "fixed list metadata allocation failed")})?;
            if root.capacity()<1{return Err(PagedListAllocationError{work:1,..rejected(PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation provided no reserved slot")}.into());}
            let actual = root.capacity() * size_of::<Page<T>>();
            let mut children = std::array::from_fn(|_| Vec::new());
            children[0] = std::mem::take(&mut self.root);
            root.push(Page::Branch { height: root_height + 1, children });
            self.root = root;
            self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
            if self.allocated > isize::MAX as usize {
                return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" },copied_bytes:copy});
            }
            if actual > grant {
                return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list metadata allocation exceeded admission; owner retained" },copied_bytes:copy});
            }
            return Ok(PagedListProgress { work: 1, progressed: true, placed_bytes:copy, allocated_bytes: actual, ..Default::default() });
        }
        let mut link = &mut self.root;
        for height in (0..=root_height).rev() {
            if link.is_empty() {
                pay(&mut effect)?;A::reserve(link, 1,effect.as_deref_mut().map(|effect|effect.actual())).map_err(|_| PagedListAllocationError{work:1,..rejected(PagedListRefusalKind::AllocationFailed, "fixed list metadata allocation failed")})?;
                if link.capacity()<1{return Err(PagedListAllocationError{work:1,..rejected(PagedListRefusalKind::InvariantViolated,"fixed list metadata allocation provided no reserved slot")}.into());}
                let actual = link.capacity() * size_of::<Page<T>>();
                link.push(if height == 0 { Page::Leaf { items: Vec::new(), slots: 0 } } else { Page::Branch { height, children: std::array::from_fn(|_| Vec::new()) } });
                self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                if self.allocated > isize::MAX as usize {
                    return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" },copied_bytes:copy});
                }
                if actual > grant {
                    return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list metadata allocation exceeded admission; owner retained" },copied_bytes:copy});
                }
                return Ok(PagedListProgress { work: 1, progressed: true, placed_bytes:copy, allocated_bytes: actual, ..Default::default() });
            }
            match &mut link[0] {
                Page::Branch { children, .. } => link = &mut children[Self::slot(page, height)],
                Page::Leaf { items, slots } => {
                    let admitted_slots = page_items.min(limit - self.capacity);
                    if *slots==0&&size_of::<T>()!=0&&items.capacity()!=0{return Err(rejected(PagedListRefusalKind::InvariantViolated,"fixed list partial allocation requires funded settlement before reservation").into());}
                    pay(&mut effect)?;A::reserve(items, admitted_slots,effect.as_deref_mut().map(|effect|effect.actual())).map_err(|_| PagedListAllocationError{work:1,..rejected(PagedListRefusalKind::AllocationFailed, "fixed list payload allocation failed")})?;
                    let actual = if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() };
                    self.allocated = self.allocated.checked_add(actual).expect("preflight and Vec backing each fit signed addressable size");
                    if items.capacity()<admitted_slots{return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError{work:1,allocated_bytes:actual,kind:PagedListRefusalKind::InvariantViolated,reason:"fixed list payload allocation provided fewer reserved slots; owner retained"},copied_bytes:size_of::<Vec<T>>()+size_of::<usize>()});}
                    *slots = admitted_slots;
                    self.capacity += admitted_slots;
                    if self.allocated > isize::MAX as usize {
                        return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list actual allocation exceeds addressable ownership; owner retained" },copied_bytes:copy});
                    }
                    if actual > grant {
                        return Err(PagedListReservationFailure::Allocation{error:PagedListAllocationError { work: 1, allocated_bytes: actual, kind: PagedListRefusalKind::InvariantViolated, reason: "fixed list payload allocation exceeded admission; owner retained" },copied_bytes:copy});
                    }
                    return Ok(PagedListProgress { work: 1, progressed: true, placed_bytes:copy, allocated_bytes: actual, ..Default::default() });
                }
            }
        }
        Err(rejected(PagedListRefusalKind::InvariantViolated, "fixed list page authority is missing").into())
    }

    /// ⏩️ Quotes the original reserved node path before one owner enters its next slot.
    pub fn next_push_depth_demand(&self)->Result<usize,PagedListError>{
        if !self.has_reserved_slot(){return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list push requires its original reserved slot"});}
        self.height().checked_add(3).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"fixed list push depth overflow"})
    }


    /// ⛽️ Quotes this exact reserved append's index digits, native path and final owner transfer.
    pub fn push_reserved_source_work_demand(&self)->Result<u64,PagedListError>{
        if !self.has_reserved_slot(){return Err(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"original append requires reserved backing"})}
        let mut page=self.length/self.page_items();let mut work=3u64+self.height()as u64;while page!=0{page>>=4;work+=1}Ok(work)
    }
    /// 📏️ Quotes the exact original donor, payload and length placements of the reserved append.
    pub fn push_reserved_source_demand(&self)->Result<semio_framework_value_control_contract::RetirementDemand,PagedListError>{self.push_reserved_source_work_demand()?;let copy=size_of::<T>().checked_add(size_of::<Option<T>>()).and_then(|n|n.checked_add(2*size_of::<usize>())).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"original append copy extent overflow"})?;Ok(semio_framework_value_control_contract::RetirementDemand{copy_bytes:copy,depth:self.height()+3,..Default::default()})}
    /// 🫴️ Moves the original donor only after its entire selected path and destination are funded.
    pub fn push_reserved_source_step(&mut self,input:&mut Option<T>,maximum_work_units:u64,grant:semio_framework_value_control_contract::RetainedCloneGrant)->semio_framework_value_control_contract::RetainedWorkOutcome{
        use semio_framework_value_control_contract::{RetainedWorkOutcome,RetainedCloneStep,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
        let mut work=0;let result=(||{if maximum_work_units==0||input.is_none(){return Ok(RetainedCloneStep::Progress(Default::default()))}let quoted=self.push_reserved_source_work_demand().map_err(|e|ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason))?;let demand=self.push_reserved_source_demand().map_err(|e|ValueError::literal(ValueRefusalKind::OwnershipLimit,e.reason))?;let copy=demand.copy_bytes;if quoted>maximum_work_units||grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(Default::default()))}
            let index=self.length;let page=index/self.page_items();let root_height=self.height();work=1;let mut remaining=page;let mut page_height=0;while remaining!=0{remaining>>=4;page_height+=1;work+=1}if page_height>root_height{return Err(ValueError::invariant("original append path exceeds its reserved root"))}
            let mut link=&mut self.root;for height in(0..=root_height).rev(){work+=1;match link.first_mut(){Some(Page::Branch{children,..})=>{if height==0{return Err(ValueError::invariant("original append branch exceeds its root height"))}link=&mut children[Self::slot(page,height)]},Some(Page::Leaf{items,..})=>{if size_of::<T>()!=0&&items.len()==items.capacity(){return Err(ValueError::invariant("original append backing is full"))}let value=input.take().expect("original donor checked before path");items.push(value);self.length=index+1;work+=1;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}))},None=>return Err(ValueError::invariant("original append reserved path is absent"))}}Err(ValueError::invariant("original append reserved leaf is absent"))
        })();RetainedWorkOutcome{result,work}
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

    /// ❄️ Appends one cold owner while admitting every backing allocation before it is made.
    pub fn try_push(&mut self, value: T) -> Result<(), PagedListError> {
        while !self.has_reserved_slot() {
            let required = self.next_allocation_bytes()?;
            let progress = self.reserve_page(required).map_err(PagedListAllocationError::refusal)?;
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
        Ok(PagedListProgress { work: 1, progressed: true, placed_bytes: size_of::<T>(), ..Default::default() })
    }

    /// 🤝 Adopts an already-copied owner into admitted backing without charging another payload copy.
    /// 📐️ Quotes the native path depth any original tail adoption needs, independent of the adopted value.
    pub fn adoption_depth_demand(&self)->Result<usize,ValueError>{self.height().checked_add(3).ok_or_else(||ValueError::invariant("paged adoption depth overflow"))}
    pub fn adoption_demand(&self,source:&Option<T>)->Result<semio_framework_value_control_contract::RetirementDemand,ValueError>{if source.is_none(){return Ok(Default::default())}let copy_bytes=size_of::<Option<T>>().checked_add(size_of::<T>()).and_then(|bytes|bytes.checked_add(2*size_of::<usize>())).ok_or_else(||ValueError::invariant("paged adoption header extent overflow"))?;let depth=self.adoption_depth_demand()?;Ok(semio_framework_value_control_contract::RetirementDemand{copy_bytes,depth,..Default::default()})}
    pub fn adopt_reserved(&mut self,source:&mut Option<T>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListProgress,ValueError>{use semio_framework_value_control_contract::{RetainedWorkOutcome,retained_clone_grant_admits};if source.is_none()||!self.has_reserved_slot(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}let demand=match self.adoption_demand(source){Ok(demand)=>demand,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !retained_clone_grant_admits(effect.grant(),demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0}}match effect.remaining_work_units(){Ok(0)=>return RetainedWorkOutcome{result:Ok(Default::default()),work:0},Ok(_)=>{},Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}}let boundary=effect.performed_boundary();let performed=effect.performed(1);let work=effect.performed_since(boundary).expect("paged adoption keeps its original work receipt");match performed{Ok(true)=>{},Ok(false)=>return RetainedWorkOutcome{result:Err(ValueError::literal(ValueRefusalKind::WorkLimit,"paged adoption exceeded its original Work")),work},Err(error)=>return RetainedWorkOutcome{result:Err(error),work}}if let Err(original)=self.push_reserved(source.take().expect("funded original paged value")){*source=Some(original);return RetainedWorkOutcome{result:Err(ValueError::invariant("paged adoption lost original reserved slot")),work}}RetainedWorkOutcome{result:Ok(PagedListProgress{work:1,placed_bytes:demand.copy_bytes,progressed:true,..Default::default()}),work}
    }


    /// ⛽️ Quotes the actual last occupied index and native path before an original tail handoff.
    pub fn pop_source_work_demand(&self)->Result<u64,PagedListError>{if self.is_empty(){return Ok(0)}let mut page=(self.length-1)/self.page_items();let mut work=3+self.height()as u64;while page!=0{page>>=4;work+=1}Ok(work)}
    /// 📐️ Quotes both original tail placements and the actual length fields.
    pub fn pop_source_demand(&self)->Result<semio_framework_value_control_contract::RetirementDemand,PagedListError>{let copy=size_of::<T>().checked_add(size_of::<Option<T>>()).and_then(|n|n.checked_add(2*size_of::<usize>())).ok_or(PagedListError{kind:PagedListRefusalKind::OwnershipLimit,reason:"original tail copy extent overflow"})?;Ok(semio_framework_value_control_contract::RetirementDemand{copy_bytes:copy,depth:self.height()+3,..Default::default()})}
    /// 📤️ Retains every tail owner unless the real caller funds its complete path and destination.
    fn pop_native(&mut self,output:&mut Option<T>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<semio_framework_value_control_contract::RetainedCloneStep,semio_framework_value_control_contract::ValueError>{
        use semio_framework_value_control_contract::{retained_clone_grant_admits,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
        fn pay(effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->Result<(),ValueError>{if effect.performed(1)?{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original tail pop exceeded Work"))}}
        if output.is_some(){return Err(ValueError::invariant("original tail recipient already owns a value"))}if self.is_empty(){return Ok(RetainedCloneStep::Progress(Default::default()))}let work=self.pop_source_work_demand().map_err(ValueError::from)?;let demand=self.pop_source_demand().map_err(ValueError::from)?;if effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        pay(effect)?;let index=self.length-1;let page=index/self.page_items();let root_height=self.height();let mut remaining=page;let mut page_height=0;while remaining!=0{pay(effect)?;remaining>>=4;page_height+=1}if page_height>root_height{return Err(ValueError::invariant("original tail path exceeds its root"))}let mut link=&mut self.root;for height in(0..=root_height).rev(){pay(effect)?;match link.first_mut(){Some(Page::Branch{children,..})=>{if height==0{return Err(ValueError::invariant("original tail branch exceeds its root"))}link=&mut children[Self::slot(page,height)]},Some(Page::Leaf{items,..})=>{if items.is_empty(){return Err(ValueError::invariant("original tail leaf is empty"))}pay(effect)?;*output=items.pop();self.length=index;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))},None=>return Err(ValueError::invariant("original tail path is absent"))}}Err(ValueError::invariant("original tail leaf is absent"))
    }
    /// 📤️ Moves the original tail into its empty resident destination under actual Work.
    pub fn pop_into<'r>(&mut self,output:&mut Option<T>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,semio_framework_value_control_contract::RetainedCloneStep,semio_framework_value_control_contract::ValueError>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,semio_framework_value_control_contract::RetainedCloneStep,semio_framework_value_control_contract::ValueError>,semio_framework_value_control_contract::ValueError>{Ok(reply.complete(self.pop_native(output,effect)))}
    /// ♻️ Deposits the same native tail move directly into the original retirement reply.
    pub fn pop_retirement_into<'r>(&mut self,output:&mut Option<T>,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>,reply:semio_framework_value_control_contract::OriginalEffectReply<'r,semio_framework_value_control_contract::RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>)->Result<semio_framework_value_control_contract::OriginalEffectReplyComplete<'r,semio_framework_value_control_contract::RetainedCloneStep,semio_framework_value_retirement_contract::OriginalRetirementFailure>,semio_framework_value_control_contract::ValueError>{Ok(reply.complete(self.pop_native(output,effect).map_err(semio_framework_value_retirement_contract::OriginalRetirementFailure::from_primary)))}
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
                    if *slots != 0 || (size_of::<T>()!=0&&items.capacity()!=0) {
                        return Ok(if size_of::<T>() == 0 { 0 } else { items.capacity() * size_of::<T>() });
                    }
                }
            }
            Ok(capacity * size_of::<Page<T>>())
        }
        next(&self.root, self.root.capacity())
    }

    /// 🔚️ Quotes the actual node path followed by the next retained tail removal.
    pub fn next_pop_depth_demand(&self) -> Result<usize, PagedListError> {
        if self.is_empty() { return Ok(0); }
        self.height().checked_add(3).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list tail depth overflow" })
    }

    /// 📦️ Quotes the actual retained node path of the next whole empty backing release.
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
                    if *slots != 0 || (size_of::<T>()!=0&&items.capacity()!=0) { return depth.checked_add(1).ok_or(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "fixed list leaf release depth overflow" }); }
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
    pub fn return_empty_page<const P:usize>(&mut self,parent:&mut semio_framework_value_allocation_return::ParentAllocationReturn<P>,maximum_items:usize)->Result<PagedListReturnProgress,ValueError>{
        fn transfer<T,const P:usize>(link:&mut Vec<Page<T>>,slots:&mut usize,parent:&mut semio_framework_value_allocation_return::ParentAllocationReturn<P>)->Result<PagedListReturnProgress,ValueError>{
            let Some(node)=link.first_mut()else{return Ok(PagedListReturnProgress::default());};
            match node{
                Page::Branch { children, .. }=>{
                    if let Some(index)=children.iter().rposition(|child|!child.is_empty()){return transfer(&mut children[index],slots,parent);}
                }
                Page::Leaf{items,slots:reserved}=>{
                    if !items.is_empty(){return Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated,"paged payload must retire before parent backing handoff"));}
                    if *reserved!=0||(size_of::<T>()!=0&&items.capacity()!=0){
                        let bytes=items.capacity().checked_mul(size_of::<T>()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"paged payload backing layout is invalid"))?;
                        if !parent.return_empty_vec(items,1)?{return Ok(PagedListReturnProgress::default());}
                        *slots-=*reserved;*reserved=0;
                        return Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:bytes});
                    }
                }
            }
            let bytes=link.capacity().checked_mul(size_of::<Page<T>>()).ok_or_else(||ValueError::new(crate::ValueRefusalKind::OwnershipLimit,"paged metadata backing layout is invalid"))?;
            if !parent.can_receive_vec_backing::<Page<T>>(link.capacity())?{return Ok(PagedListReturnProgress::default());}
            link.clear();
            if !parent.return_empty_vec(link,1)?{return Err(ValueError::new(crate::ValueRefusalKind::InvariantViolated,"pre-admitted parent backing slot disappeared"));}
            Ok(PagedListReturnProgress{progressed:true,returned_allocation_bytes:bytes})
        }
        if maximum_items==0{return Ok(PagedListReturnProgress::default());}
        let progress=transfer(&mut self.root,&mut self.capacity,parent)?;
        self.allocated-=progress.returned_allocation_bytes;
        if progress.progressed{self.reservation_epoch=self.reservation_epoch.saturating_add(1);}
        Ok(progress)
    }


    /// 📐️ Quotes the exact original headers written by the selected empty-page release.
    pub fn next_release_copy_byte_demand(&self)->Result<usize,PagedListError>{fn next<T>(link:&[Page<T>])->Result<usize,PagedListError>{let Some(node)=link.first()else{return Ok(0)};match node{Page::Branch{children,..}=>{if let Some(child)=children.iter().rfind(|child|!child.is_empty()){return next(child)}},Page::Leaf{items,slots}=>{if !items.is_empty(){return Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"fixed list payload must retire before its page"})}if *slots!=0||(size_of::<T>()!=0&&items.capacity()!=0){return Ok(size_of::<Vec<T>>()+3*size_of::<usize>())}}}Ok(size_of::<Vec<Page<T>>>()+size_of::<usize>())}next(&self.root)}
    /// ♻️ Releases the selected actual empty page through the original performed-work effect.
    pub fn release_empty_page(&mut self,effect:&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->semio_framework_value_control_contract::RetainedWorkOutcome<PagedListReleaseOutcome,PagedListReleaseFailure>{
        use semio_framework_value_control_contract::{RetainedWorkOutcome};if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(PagedListReleaseOutcome::Ready(PagedListProgress::default())),work:0}}let grant=effect.grant();let selected=(||{Ok::<_,PagedListError>((self.next_release_copy_byte_demand()?,self.next_release_depth_demand()?,self.next_release_allocation_bytes()?))})();let(copy,depth,release_bytes)=match selected{Ok(value)=>value,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};if grant.maximum_items==0||grant.maximum_depth<depth||grant.maximum_copy_bytes<copy||grant.maximum_release_bytes<release_bytes||effect.maximum_work_units()==0{return RetainedWorkOutcome{result:Ok(PagedListReleaseOutcome::Denied),work:0}}let before=effect.performed_work();let result=release_page(&mut self.root,&mut self.capacity,grant.maximum_release_bytes,&mut ||match effect.performed(1){Ok(true)=>Ok(()),Ok(false)=>Err(PagedListReleaseFailure::Work(semio_framework_value_control_contract::ValueError::literal(semio_framework_value_control_contract::ValueRefusalKind::WorkLimit,"original page release exceeded its work authority"))),Err(error)=>Err(PagedListReleaseFailure::Work(error))});let work=effect.performed_work().checked_sub(before).expect("original page release work moved backwards");match result{Ok(step)if !step.progressed=>RetainedWorkOutcome{result:Ok(PagedListReleaseOutcome::Denied),work},Ok(step)=>{self.allocated-=step.released_allocation_bytes;RetainedWorkOutcome{result:Ok(PagedListReleaseOutcome::Ready(step)),work}},Err(error)=>RetainedWorkOutcome{result:Err(error),work}}
    }
    /// 🪶️ Releases one empty page inside an exclusive lease whose cursor owner pays one unit per progressed step; no allocation effect exists at this boundary.
    pub fn release_empty_page_leased(&mut self,maximum_bytes:usize)->Result<PagedListProgress,PagedListError>{
        if self.terminal_is_empty(){return Ok(PagedListProgress::default())}
        let step=release_page(&mut self.root,&mut self.capacity,maximum_bytes,&mut||Ok(())).map_err(|failure|match failure{PagedListReleaseFailure::Release(error)=>error,PagedListReleaseFailure::Work(_)=>PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"leased page release has no work authority"}})?;
        self.allocated-=step.released_allocation_bytes;Ok(step)
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
/// ♻️ Releases the last occupied empty page, paying through the caller's own work authority before any release.
fn release_page<T>(link:&mut Vec<Page<T>>,slots:&mut usize,maximum_bytes:usize,pay:&mut dyn FnMut()->Result<(),PagedListReleaseFailure>)->Result<PagedListProgress,PagedListReleaseFailure>{
            let Some(node)=link.first_mut()else{return Ok(PagedListProgress::default())};match node{Page::Branch{children,..}=>{if let Some(index)=children.iter().rposition(|child|!child.is_empty()){return release_page(&mut children[index],slots,maximum_bytes,pay)}},Page::Leaf{items,slots:reserved}=>{if !items.is_empty(){return Err(PagedListError{kind:PagedListRefusalKind::InvariantViolated,reason:"fixed list payload must retire before its page"}.into())}if *reserved!=0||(size_of::<T>()!=0&&items.capacity()!=0){let bytes=if size_of::<T>()==0{0}else{items.capacity()*size_of::<T>()};if bytes>maximum_bytes{return Ok(PagedListProgress::default())}pay()?;*items=Vec::new();*slots-=*reserved;*reserved=0;return Ok(PagedListProgress{work:1,progressed:true,placed_bytes:size_of::<Vec<T>>()+3*size_of::<usize>(),released_allocation_bytes:bytes,..Default::default()})}}}let bytes=link.capacity()*size_of::<Page<T>>();if bytes>maximum_bytes{return Ok(PagedListProgress::default())}pay()?;*link=Vec::new();Ok(PagedListProgress{work:1,progressed:true,placed_bytes:size_of::<Vec<Page<T>>>()+size_of::<usize>(),released_allocation_bytes:bytes,..Default::default()})
}

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
                copy.reserve_page(bytes).expect("cold list clone allocation");
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
    /// 🧲️ Constructs an empty native sequence; retained writes use explicit reserve and placement.
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

use semio_framework_value_control_contract::{ValueError,ValueRefusalKind};
impl From<PagedListError> for ValueError {
    fn from(error: PagedListError) -> Self {
        let kind = match error.kind {
            PagedListRefusalKind::OwnershipLimit => crate::ValueRefusalKind::OwnershipLimit,
            PagedListRefusalKind::AllocationFailed => crate::ValueRefusalKind::AllocationFailed,
            PagedListRefusalKind::InvariantViolated => crate::ValueRefusalKind::InvariantViolated,
        };
        ValueError::literal(kind, error.reason)
    }
}

#[cfg(test)]
#[path="🧪️tests/📏️final/🦀️.rs"]
mod original_exact_final_extent_tests;

#[path="📤️discard/🦀️.rs"]
mod tail_discard;
pub use tail_discard::PagedListDiscardDemand;

#[cfg(test)]
mod partition_tests {
    use super::*;
    #[test]
    fn partition_point_matches_language_neutral_cases() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("partition.cases.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let values: PagedList<i64, 32> = case["values"].as_array().unwrap().iter().map(|value| value.as_i64().unwrap()).collect();
            let limit = case["limit"].as_i64().unwrap();
            assert_eq!(values.partition_point(|value| *value < limit), case["expected"].as_u64().unwrap() as usize);
        }
    }
}
