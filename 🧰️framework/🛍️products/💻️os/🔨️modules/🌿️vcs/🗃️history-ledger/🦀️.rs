//! 🗃️ History ledger: the generation-keyed history ledger, its reservations, iterators and trait implementations.
//!
//! Holds items of the module; the root re-exports them.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactHistoryKey {
    pub index: u32,
    pub generation: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactHistoryReservation {
    authority: usize,
    pub(super) index: u32,
    pub(super) generation: u32,
}

#[path="../🔗️read/🦀️.rs"]
pub mod history_read;
pub use history_read::ArtifactHistoryReadKey;

pub(super) struct ArtifactHistorySlot<T> {
    pub(super) generation: u32,
    pub(super) visibility: Option<std::sync::Arc<ArtifactGroupVisibility>>,
    previous: Option<u32>,
    pub(super) next: Option<u32>,
    free_next: Option<u32>,
    value: Option<T>,
    replacement: Option<Box<semio_framework_value::RetainedPublicationRoot<T>>>,
    read: Option<Box<dyn semio_framework_value_original_slot::RetainedOriginalReadOwner<T>>>,
    read_previous:Option<u32>,
    read_next:Option<u32>,
}

impl<T> ArtifactHistorySlot<T> {
    fn get(&self)->Option<&T>{self.replacement.as_deref().map(|root|&**root).or_else(||self.value.as_ref()).or_else(||self.read.as_ref().map(|read|read.get()))}
    fn is_occupied(&self)->bool{self.value.is_some()||self.read.is_some()||self.replacement.is_some()}
}

pub(super) struct ArtifactHistorySlotPage<T> {
    pub(super) slots: Box<[std::mem::MaybeUninit<ArtifactHistorySlot<T>>]>,
}

pub(super) struct ArtifactHistoryPageChunk<T> {
    pub(super) pages: [Option<Box<ArtifactHistorySlotPage<T>>>; ARTIFACT_HISTORY_DIRECTORY_PAGES],
    pub(super) count: u16,
    pub(super) next: Option<Box<ArtifactHistoryPageChunk<T>>>,
}

impl<T> ArtifactHistoryPageChunk<T> {
    pub(super) fn blank() -> Self {
        Self { pages: std::array::from_fn(|_| None), count: 0, next: None }
    }
}

fn artifact_history_page_slots<T>()->usize{(ARTIFACT_HISTORY_RELEASE_PAGE_BYTES/4/size_of::<ArtifactHistorySlot<T>>().max(1)).clamp(1,ARTIFACT_HISTORY_PAGE_SLOTS)}

fn artifact_history_slot_page<T>() -> Box<[std::mem::MaybeUninit<ArtifactHistorySlot<T>>]> {
    let bytes = size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().saturating_mul(artifact_history_page_slots::<T>());
    assert!(bytes <= ARTIFACT_HISTORY_CONTIGUOUS_CEILING, "one history ledger page must stay under the 64 KiB contiguous ceiling");
    let mut slots = Vec::new();
    slots.try_reserve_exact(artifact_history_page_slots::<T>()).expect("history ledger page allocation");
    slots.resize_with(artifact_history_page_slots::<T>(), std::mem::MaybeUninit::uninit);
    slots.into_boxed_slice()
}

fn artifact_history_descend<'a, T>(chunk: &'a mut ArtifactHistoryPageChunk<T>, remaining: usize) -> &'a mut ArtifactHistoryPageChunk<T> {
    if remaining == 0 {
        return chunk;
    }
    if chunk.next.is_none() {
        chunk.next = Some(Box::new(ArtifactHistoryPageChunk::blank()));
    }
    artifact_history_descend(chunk.next.as_mut().expect("history ledger chunk link"), remaining - 1)
}

fn artifact_history_chunk<'a, T>(chunk: &'a ArtifactHistoryPageChunk<T>, remaining: usize) -> &'a ArtifactHistoryPageChunk<T> {
    if remaining == 0 { chunk } else { artifact_history_chunk(chunk.next.as_ref().expect("history ledger chunk link"), remaining - 1) }
}

/// 📚️ Paged generation-keyed history authority. Live entries form one stable linked order;
/// removed slots are tombstoned and reused only after their generation advances. A full page
/// opens another fixed page instead of refusing the entry. `seek` remembers the `(position, slot)` of the last entry a
/// positional read or a search reached, so reads that walk the order forward (applied order, replay order, a retained
/// initializer's cursor) cost O(1) amortized instead of O(position); every structural change forgets it.
static NEXT_HISTORY_READ_ISSUANCE:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);

pub struct ArtifactHistoryLedger<T> {
    pub(super) pages: std::mem::ManuallyDrop<Option<Box<ArtifactHistoryPageChunk<T>>>>,
    page_count: u32,
    read_issuance:u64,
    read_count:usize,
    replacement_count:usize,
    replacement_decision:Option<usize>,
    read_head:Option<u32>,
    initialized: u32,
    pub(super) head: Option<u32>,
    tail: Option<u32>,
    free_head: Option<u32>,
    pub(super) reservation: Option<ArtifactHistoryReservation>,
    group: Option<ArtifactHistoryGroupSuffix>,
    pub(super) len: usize,
    seek: std::sync::atomic::AtomicU64,
}

/// 🧭️ No remembered seek position.
const ARTIFACT_HISTORY_NO_SEEK: u64 = u64::MAX;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactHistoryReservationFault {
    Busy,
    Capacity,
    GenerationExhausted,
    GroupUnavailable,
}

impl<T> Default for ArtifactHistoryLedger<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> ArtifactHistoryLedger<T> {
    /// 📐️ Computes actual fixed page cardinality from the typed slot footprint and native physical page policy.
    pub fn page_slot_count()->usize{artifact_history_page_slots::<T>()}

    /// 🪹️ Creates an inline ledger frame without native slot or directory backing.
    pub fn empty() -> Self { Self { pages: std::mem::ManuallyDrop::new(None), page_count: 0, read_count:0, replacement_count:0, replacement_decision:None, read_head:None, read_issuance:NEXT_HISTORY_READ_ISSUANCE.try_update(std::sync::atomic::Ordering::Relaxed,std::sync::atomic::Ordering::Relaxed,|value|value.checked_add(1)).expect("original history read issuance exhausted"), initialized: 0, head: None, tail: None, free_head: None, reservation: None, group: None, len: 0, seek: std::sync::atomic::AtomicU64::new(ARTIFACT_HISTORY_NO_SEEK) } }

    /// 🧮 Quotes the exact slot backing, original page and directory before birth.
    pub fn initial_page_birth_demand(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.page_count != 0 { return Ok(Default::default()); }
        let capacity_bytes = size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().checked_mul(artifact_history_page_slots::<T>())
            .and_then(|bytes| bytes.checked_add(size_of::<ArtifactHistorySlotPage<T>>()))
            .and_then(|bytes| bytes.checked_add(size_of::<ArtifactHistoryPageChunk<T>>()))
            .ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger original page extent overflowed"))?;
        Ok(semio_framework_value::RetirementDemand { capacity_bytes, depth: 1, ..Default::default() })
    }
    /// 🧱 True after the original first page remains installed.
    pub fn initial_page_is_admitted(&self) -> bool { self.page_count != 0 }
    /// 🎟️ Installs one genuine history page under the supplied independent currencies.
    pub fn admit_initial_page(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneProgress, ValueError> {
        if self.page_count != 0 { return Ok(Default::default()); }
        let demand = self.initial_page_birth_demand()?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history ledger birth requires original depth")); }
        if grant.maximum_items == 0 || grant.maximum_capacity_bytes < demand.capacity_bytes { return Ok(Default::default()); }
        self.open_page().map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger original page allocation refused"))?;
        let progress = semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        semio_framework_value::retained_clone::admit_retained_clone_progress(grant, progress, "original history ledger first page")?;
        Ok(progress)
    }

    pub fn new() -> Self {
        let mut ledger = Self::empty();
        ledger.open_page().expect("history ledger first page");
        ledger
    }

    pub(super) fn open_page(&mut self) -> Result<(), ()> {
        if self.page_count == u32::MAX {
            return Err(());
        }
        let page = Box::new(ArtifactHistorySlotPage { slots: artifact_history_slot_page() });
        let page_index = self.page_count as usize;
        if self.pages.is_none() {
            *self.pages = Some(Box::new(ArtifactHistoryPageChunk::blank()));
        }
        {
            let chunk = artifact_history_descend(self.pages.as_mut().expect("history ledger chunk"), page_index / ARTIFACT_HISTORY_DIRECTORY_PAGES);
            let offset = page_index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
            chunk.pages[offset] = Some(page);
            chunk.count = chunk.count.saturating_add(1);
        }
        self.page_count += 1;
        Ok(())
    }

    fn slot_page(&self, page: usize) -> &ArtifactHistorySlotPage<T> {
        let head = self.pages.as_ref().expect("history ledger owns its first page");
        artifact_history_chunk(head, page / ARTIFACT_HISTORY_DIRECTORY_PAGES).pages[page % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_ref().expect("history ledger page")
    }

    fn slot_page_mut(&mut self, page: usize) -> &mut ArtifactHistorySlotPage<T> {
        let head = self.pages.as_mut().expect("history ledger owns its first page");
        artifact_history_descend(head, page / ARTIFACT_HISTORY_DIRECTORY_PAGES).pages[page % ARTIFACT_HISTORY_DIRECTORY_PAGES].as_mut().expect("history ledger page")
    }

    fn slot(&self, index: u32) -> &ArtifactHistorySlot<T> {
        let index = index as usize;
        unsafe { self.slot_page(index / artifact_history_page_slots::<T>()).slots[index % artifact_history_page_slots::<T>()].assume_init_ref() }
    }

    fn slot_mut(&mut self, index: u32) -> &mut ArtifactHistorySlot<T> {
        let index = index as usize;
        unsafe { self.slot_page_mut(index / artifact_history_page_slots::<T>()).slots[index % artifact_history_page_slots::<T>()].assume_init_mut() }
    }

    fn authority(&self) -> usize {
        self.pages.as_ref().map(|chunk| chunk.as_ref() as *const ArtifactHistoryPageChunk<T> as usize).unwrap_or(0)
    }

    fn allocated_slots(&self) -> usize {
        self.page_count as usize * artifact_history_page_slots::<T>()
    }

    /// 🎟️ Reports backing required by the next unoccupied history reservation.
    pub fn next_reservation_allocation_bytes(&self) -> usize {
        if self.reservation.is_some() || self.free_head.is_some() || (self.initialized as usize) < self.allocated_slots() { return 0; }
        size_of::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>().saturating_mul(artifact_history_page_slots::<T>())
            .saturating_add(size_of::<ArtifactHistorySlotPage<T>>())
            .saturating_add(if self.page_count as usize % ARTIFACT_HISTORY_DIRECTORY_PAGES == 0 { size_of::<ArtifactHistoryPageChunk<T>>() } else { 0 })
    }

    /// 📏️ Prices exactly one original row header and the native backing needed by its real next slot.
    pub fn owned_row_admission_demands(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:size_of::<T>()*2+size_of::<ArtifactHistoryReservation>()*2,capacity_bytes:self.next_reservation_allocation_bytes(),depth:1,..Default::default()}}
    /// 📥️ Moves one original deque row into this ledger without cloning payloads or reallocating its source backing.
    pub fn admit_deque_front(&mut self,source:&mut std::collections::VecDeque<T>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{if self.group.is_some()||self.reservation.is_some(){return Err(ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"native row admission requires its exact unreserved original ledger"))}if source.is_empty(){return Ok(Default::default())}let d=self.owned_row_admission_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}let row=source.pop_front().unwrap();if let Err(row)=self.try_push(row){source.push_front(row);return Err(ValueError::literal(protocol::value::ValueRefusalKind::OwnershipLimit,"native original row admission exhausted its ledger"))}Ok(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})}

    pub fn reserve_one(&mut self) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if self.group.is_some() || self.replacement_count!=0 {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        self.reserve_slot()
    }

    fn reserve_slot(&mut self) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if self.reservation.is_some() {
            return Err(ArtifactHistoryReservationFault::Busy);
        }
        let (index, generation) = if let Some(index) = self.free_head {
            let generation = self.slot(index).generation.checked_add(1).ok_or(ArtifactHistoryReservationFault::GenerationExhausted)?;
            (index, generation)
        } else {
            if self.initialized as usize == self.allocated_slots() && self.open_page().is_err() {
                return Err(ArtifactHistoryReservationFault::Capacity);
            }
            (self.initialized, 1)
        };
        let reservation = ArtifactHistoryReservation { authority: self.authority(), index, generation };
        self.reservation = Some(ArtifactHistoryReservation { authority: reservation.authority, index, generation });
        Ok(reservation)
    }

    pub fn cancel_reservation(&mut self, reservation: ArtifactHistoryReservation) -> Result<(), ArtifactHistoryReservation> {
        if self.reservation.as_ref() != Some(&reservation) {
            return Err(reservation);
        }
        self.reservation = None;
        Ok(())
    }

    pub fn insert_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if self.group.is_some() {
            return Err((reservation, value));
        }
        let key = self.insert_owned_slot(reservation, value, self.tail)?;
        if self.head.is_none() {
            self.head = Some(key.index);
        }
        self.tail = Some(key.index);
        self.len += 1;
        Ok(key)
    }

    fn insert_owned_slot(&mut self, reservation: ArtifactHistoryReservation, value: T, previous: Option<u32>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        if reservation.authority != self.authority() || self.reservation.as_ref() != Some(&reservation) {
            return Err((reservation, value));
        }
        let index = reservation.index;
        let generation = reservation.generation;
        self.reservation = None;
        self.forget_seek();
        if Some(index) == self.free_head {
            let free_next = self.slot(index).free_next;
            self.free_head = free_next;
            let slot = self.slot_mut(index);
            slot.generation = generation;
            slot.visibility = None;
            slot.previous = previous;
            slot.next = None;
            slot.free_next = None;
            slot.value = Some(value);
        } else if index == self.initialized && (index as usize) < self.allocated_slots() {
            let offset = index as usize;
            self.slot_page_mut(offset / artifact_history_page_slots::<T>()).slots[offset % artifact_history_page_slots::<T>()].write(ArtifactHistorySlot { generation, visibility: None, previous, next: None, free_next: None, value: Some(value), replacement:None, read: None, read_previous:None, read_next:None });
            self.initialized += 1;
        } else {
            return Err((reservation, value));
        }
        if let Some(tail) = previous {
            self.slot_mut(tail).next = Some(index);
        }
        Ok(ArtifactHistoryKey { index, generation })
    }

    /// 🪢️ Binds an already reserved slot to its exact still-private visibility owner.
    pub fn bind_group_reservation(&mut self, reservation: &ArtifactHistoryReservation, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<(), ()> {
        if self.reservation.as_ref() != Some(reservation) || !visibility.pending() || self.group.as_ref().is_some_and(|group| !std::sync::Arc::ptr_eq(&group.visibility, visibility)) { return Err(()); }
        if self.group.is_none() { self.group = Some(ArtifactHistoryGroupSuffix { visibility: std::sync::Arc::clone(visibility), head: None, tail: None, len: 0 }); }
        Ok(())
    }

    /// 📏️ Prices the next exact native reservation, its typed backing and shared original decision header.
    pub fn group_reservation_demands(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{copy_bytes:size_of::<ArtifactHistoryReservation>()*2+size_of::<ArtifactHistoryGroupSuffix>(),capacity_bytes:self.next_reservation_allocation_bytes(),depth:1,..Default::default()}}
    /// 🎟️ Stages one real reservation only after every independent original currency is admitted.
    pub fn reserve_group_controlled(&mut self,visibility:&std::sync::Arc<ArtifactGroupVisibility>,slot:&mut Option<ArtifactHistoryReservation>,grant:semio_framework_value::RetainedCloneGrant)->Result<semio_framework_value::RetainedCloneProgress,ValueError>{if slot.is_some()||!visibility.pending()||self.group.as_ref().is_some_and(|group|!std::sync::Arc::ptr_eq(&group.visibility,visibility)){return Err(ValueError::literal(protocol::value::ValueRefusalKind::InvariantViolated,"original group reservation changed its source or recipient"))}let d=self.group_reservation_demands();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}*slot=Some(self.reserve_group_one(visibility).map_err(|_|ValueError::literal(protocol::value::ValueRefusalKind::OwnershipLimit,"original group history reservation refused"))?);Ok(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()})}

    pub fn reserve_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryReservation, ArtifactHistoryReservationFault> {
        if !visibility.pending() || self.group.as_ref().is_some_and(|group| !std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        let reservation = self.reserve_slot()?;
        if self.group.is_none() {
            self.group = Some(ArtifactHistoryGroupSuffix { visibility: std::sync::Arc::clone(visibility), head: None, tail: None, len: 0 });
        }
        Ok(reservation)
    }

    pub fn stage_group_reserved(&mut self, reservation: ArtifactHistoryReservation, value: T, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<ArtifactHistoryKey, (ArtifactHistoryReservation, T)> {
        let Some(group) = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility) && visibility.pending()) else {
            return Err((reservation, value));
        };
        let previous = group.tail.or(self.tail);
        let key = self.insert_owned_slot(reservation, value, previous)?;
        self.slot_mut(key.index).visibility = Some(std::sync::Arc::clone(visibility));
        let group = self.group.as_mut().expect("validated group suffix remains owned");
        if group.head.is_none() {
            group.head = Some(key.index);
        }
        group.tail = Some(key.index);
        group.len += 1;
        Ok(key)
    }

    pub fn adopt_group(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<(), ()> {
        if self.reservation.is_some() || !visibility.committed() || !self.group.as_ref().is_some_and(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)) {
            return Err(());
        }
        let group = self.group.take().expect("validated group suffix remains owned");
        self.forget_seek();
        self.head = self.head.or(group.head);
        self.tail = group.tail.or(self.tail);
        self.len += group.len;
        Ok(())
    }

    pub fn abort_group_one(&mut self, visibility: &std::sync::Arc<ArtifactGroupVisibility>) -> Result<Option<T>, ()> {
        if self.reservation.is_some() || visibility.pending() || visibility.committed() {
            return Err(());
        }
        let group = self.group.as_ref().filter(|group| std::sync::Arc::ptr_eq(&group.visibility, visibility)).ok_or(())?;
        let Some(index) = group.tail else {
            self.group = None;
            return Ok(None);
        };
        self.forget_seek();
        let previous = self.slot(index).previous;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = None;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.value.take().expect("aborted group slot retains its exact entry owner")
        };
        self.free_head = Some(index);
        let group = self.group.as_mut().expect("validated aborted group remains owned");
        group.len -= 1;
        group.tail = if group.len == 0 { None } else { previous };
        if group.len == 0 {
            group.head = None;
        }
        Ok(Some(value))
    }

    fn visible_bounds(&self) -> (Option<u32>, Option<u32>, usize) {
        match self.group.as_ref().filter(|group| group.visibility.committed()) {
            Some(group) => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            None => (self.head, self.tail, self.len),
        }
    }

    pub fn group_visibility(&self) -> Option<&ArtifactGroupVisibility> {
        self.group.as_ref().map(|group| group.visibility.as_ref())
    }

    pub fn read_group(&self, decision: Option<&ArtifactGroupReadDecision<'_>>) -> Result<ArtifactHistoryIter<'_, T>, ()> {
        let (front, back, remaining) = match self.group.as_ref() {
            Some(group) if decision.ok_or(())?.committed_for(&group.visibility)? => (self.head.or(group.head), group.tail.or(self.tail), self.len + group.len),
            _ => (self.head, self.tail, self.len),
        };
        Ok(ArtifactHistoryIter { ledger: self, front, back, remaining })
    }

    pub fn try_push(&mut self, value: T) -> Result<ArtifactHistoryKey, T> {
        let reservation = match self.reserve_one() {
            Ok(reservation) => reservation,
            Err(_) => return Err(value),
        };
        self.insert_reserved(reservation, value).map_err(|(_, value)| value)
    }

    /// 📎️ Inserts before an exact generation key without moving any existing payload or changing its identity.
    pub fn try_insert_before(&mut self,next:ArtifactHistoryKey,value:T)->Result<ArtifactHistoryKey,T>{if self.group.is_some()||self.get_key(next).is_none(){return Err(value)}let previous=self.slot(next.index).previous;let reservation=match self.reserve_one(){Ok(reservation)=>reservation,Err(_)=>return Err(value)};let key=match self.insert_owned_slot(reservation,value,previous){Ok(key)=>key,Err((reservation,value))=>{self.cancel_reservation(reservation).ok();return Err(value)}};self.slot_mut(key.index).next=Some(next.index);self.slot_mut(next.index).previous=Some(key.index);if previous.is_none(){self.head=Some(key.index)}self.len+=1;self.forget_seek();Ok(key)}

    pub fn try_from_preflighted(values: Vec<T>) -> Result<Self, Vec<T>> {
        let mut ledger = Self::new();
        let mut pending = values.into_iter();
        while let Some(value) = pending.next() {
            if let Err(value) = ledger.try_push(value) {
                let mut rejected = Vec::new();
                while let Some(established) = ledger.pop() {
                    rejected.push(established);
                }
                rejected.reverse();
                rejected.push(value);
                rejected.extend(pending);
                return Err(rejected);
            }
        }
        Ok(ledger)
    }

    /// 🪃️ Quotes one exact original unlink before a publisher changes any companion roots.
    pub fn removal_demands(&self,key:ArtifactHistoryKey)->Result<semio_framework_value::RetirementDemand,ValueError>{if self.reservation.is_some()||self.group.is_some()||self.replacement_count!=0||key.index>=self.initialized{return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history removal requires its settled original generation"))}let slot=self.slot(key.index);if slot.generation!=key.generation||slot.value.is_none()||slot.read.is_some(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"history removal requires its unborrowed original row"))}Ok(semio_framework_value::RetirementDemand{copy_bytes:2*size_of::<T>()+12*size_of::<usize>(),depth:1,..Default::default()})}

    pub fn remove_key(&mut self, key: ArtifactHistoryKey) -> Result<T, ArtifactHistoryKey> {
        if self.reservation.is_some() || self.group.is_some() || self.replacement_count!=0 {
            return Err(key);
        }
        if key.index >= self.initialized {
            return Err(key);
        }
        let slot = self.slot(key.index);
        if slot.generation != key.generation || slot.value.is_none() || slot.read.is_some() {
            return Err(key);
        }
        self.forget_seek();
        let previous = slot.previous;
        let next = slot.next;
        if let Some(previous) = previous {
            self.slot_mut(previous).next = next;
        } else {
            self.head = next;
        }
        if let Some(next) = next {
            self.slot_mut(next).previous = previous;
        } else {
            self.tail = previous;
        }
        let free_head = self.free_head;
        let value = {
            let slot = self.slot_mut(key.index);
            slot.previous = None;
            slot.next = None;
            slot.free_next = free_head;
            slot.value.take().expect("validated history slot retains its exact owner")
        };
        self.free_head = Some(key.index);
        self.len -= 1;
        Ok(value)
    }

    /// ✂️ Removes and answers every entry `remove` selects, in ledger order — how a ledger follows a fold whose facts shrank
    /// (a retracted transition). Refused, removing nothing, while a reservation or a group decision is open.
    pub fn extract_if(&mut self, mut remove: impl FnMut(&T) -> bool) -> Result<Vec<T>, ArtifactHistoryReservationFault> {
        if self.group.is_some() {
            return Err(ArtifactHistoryReservationFault::GroupUnavailable);
        }
        if self.reservation.is_some() {
            return Err(ArtifactHistoryReservationFault::Busy);
        }
        let mut keys = Vec::new();
        let mut cursor = self.head;
        while let Some(index) = cursor {
            let slot = self.slot(index);
            if slot.get().is_some_and(&mut remove) {
                if slot.read.is_some(){return Err(ArtifactHistoryReservationFault::Busy);}
                keys.push(ArtifactHistoryKey { index, generation: slot.generation });
            }
            cursor = slot.next;
        }
        Ok(keys.into_iter().filter_map(|key| self.remove_key(key).ok()).collect())
    }

    pub fn pop(&mut self) -> Option<T> {
        let index = self.tail?;
        let generation = self.slot(index).generation;
        self.remove_key(ArtifactHistoryKey { index, generation }).ok()
    }

    pub fn first(&self) -> Option<&T> {
        self.visible_bounds().0.and_then(|index| self.slot(index).get())
    }

    pub fn last(&self) -> Option<&T> {
        self.visible_bounds().1.and_then(|index| self.slot(index).get())
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        if self.group.is_some() || self.replacement_count!=0 {
            return None;
        }
        let index = self.tail?;
        self.slot_mut(index).value.as_mut()
    }

    /// 🎯️ The entry at `position` of the visible order — O(1) amortized when reads walk forward (see `seek`).
    pub fn get(&self, position: usize) -> Option<&T> {
        let index = self.seek_slot(position)?;
        self.slot(index).get()
    }

    /// 🗝️ Reads an exact live slot generation while honoring its staged group decision.
    pub fn get_key(&self, key: ArtifactHistoryKey) -> Option<&T> {
        if key.index >= self.initialized { return None; }
        let slot = self.slot(key.index);
        if slot.generation != key.generation || slot.visibility.as_ref().is_some_and(|visibility| !visibility.committed()) { return None; }
        slot.get()
    }

    /// 🔚️ Borrows the visible tail generation without walking positional history.
    pub fn last_key(&self)->Option<ArtifactHistoryKey>{let index=self.visible_bounds().1?;Some(ArtifactHistoryKey{index,generation:self.slot(index).generation})}

    /// ⏮️ Borrows one preceding visible link from the unchanged original slot generation.
    pub fn previous_key(&self,key:ArtifactHistoryKey)->Option<ArtifactHistoryKey>{self.get_key(key)?;let index=self.slot(key.index).previous?;let previous=ArtifactHistoryKey{index,generation:self.slot(index).generation};self.get_key(previous)?;Some(previous)}

    /// 🪪️ Identifies the genuine original ledger independently of moves of its containing Store.
    pub fn retained_issuance(&self) -> u64 { self.read_issuance }

    /// 🗝️ Mutably borrows one exact original generation only after every loan and staged history group has returned.
    pub fn get_key_mut(&mut self, key: ArtifactHistoryKey) -> Option<&mut T> {
        if self.group.is_some() || self.replacement_count!=0 || key.index >= self.initialized { return None; }
        let slot = self.slot_mut(key.index);
        if slot.generation != key.generation || slot.visibility.as_ref().is_some_and(|visibility| !visibility.committed()) || slot.read.is_some() { return None; }
        slot.value.as_mut()
    }

    /// 🧭️ Captures the exact generation of the visible entry at `position`.
    pub fn key_at(&self, position: usize) -> Option<ArtifactHistoryKey> {
        let index = self.seek_slot(position)?;
        Some(ArtifactHistoryKey { index, generation: self.slot(index).generation })
    }

    pub fn get_mut(&mut self, position: usize) -> Option<&mut T> {
        assert!(self.group.is_none(), "mutable history access requires its staged group to be adopted or aborted");
        if self.replacement_count!=0{return None}
        let index = self.seek_slot(position)?;
        self.slot_mut(index).value.as_mut()
    }

    /// 🔎️ The first visible entry `found` accepts, searched from the last entry a read reached onward and then from the
    /// front — O(1) amortized for lookups that follow the order (an id per applied edit, a replay's next edit).
    pub fn find_near(&self, found: impl FnMut(&T) -> bool) -> Option<&T> {
        self.find_near_position(found).map(|(_, value)| value)
    }

    /// 🔎️ [`Self::find_near`] answering the entry's position in the visible order too.
    pub fn find_near_position(&self, found: impl FnMut(&T) -> bool) -> Option<(usize, &T)> {
        self.find_near_entry(found).map(|(position, _, value)| (position, value))
    }

    /// 🗝️ [`Self::find_near`] with the matched generation key.
    pub fn find_near_key(&self, found: impl FnMut(&T) -> bool) -> Option<(ArtifactHistoryKey, &T)> {
        self.find_near_entry(found).map(|(_, key, value)| (key, value))
    }

    fn find_near_entry(&self, mut found: impl FnMut(&T) -> bool) -> Option<(usize, ArtifactHistoryKey, &T)> {
        let (front, _, remaining) = self.visible_bounds();
        let (start_position, start) = match self.remembered_seek() {
            Some((position, index)) if position < remaining => (position, Some(index)),
            _ => (0, front),
        };
        let mut cursor = (start_position, start);
        for _ in 0..remaining {
            let (position, Some(index)) = cursor else { break };
            let slot = self.slot(index);
            if slot.get().is_some_and(&mut found) {
                self.remember_seek(position, index);
                return slot.get().map(|value| (position, ArtifactHistoryKey { index, generation: slot.generation }, value));
            }
            cursor = match slot.next {
                Some(next) if position + 1 < remaining => (position + 1, Some(next)),
                _ => (0, front),
            };
        }
        None
    }

    /// 🧭️ The slot at `position` of the visible order, walking forward from the remembered seek when it lies at or
    /// before `position`, else from the front; remembers where it landed.
    fn seek_slot(&self, position: usize) -> Option<u32> {
        let (front, _, remaining) = self.visible_bounds();
        if position >= remaining {
            return None;
        }
        let (mut at, mut index) = match self.remembered_seek() {
            Some((known, index)) if known <= position => (known, index),
            _ => (0, front?),
        };
        while at < position {
            index = self.slot(index).next?;
            at += 1;
        }
        self.remember_seek(position, index);
        Some(index)
    }

    /// 🧭️ The remembered `(position, slot)`, only while it still names a live entry and no staged group shifts the order.
    fn remembered_seek(&self) -> Option<(usize, u32)> {
        let packed = self.seek.load(std::sync::atomic::Ordering::Relaxed);
        if packed == ARTIFACT_HISTORY_NO_SEEK || self.group.is_some() {
            return None;
        }
        let (position, index) = ((packed >> 32) as usize, packed as u32);
        (index < self.initialized && self.slot(index).is_occupied()).then_some((position, index))
    }

    fn remember_seek(&self, position: usize, index: u32) {
        if let Ok(position) = u32::try_from(position) {
            self.seek.store((u64::from(position) << 32) | u64::from(index), std::sync::atomic::Ordering::Relaxed);
        }
    }

    fn forget_seek(&self) {
        self.seek.store(ARTIFACT_HISTORY_NO_SEEK, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn len(&self) -> usize {
        self.visible_bounds().2
    }

    /// 🪑️ How many more entries the ledger admits one reservation at a time — the preflight of an atomic batch of inserts.
    pub fn vacancies(&self) -> usize {
        if self.group.is_some() || self.reservation.is_some() || self.replacement_count!=0 {
            return 0;
        }
        let mut vacancies = 0;
        let mut free = self.free_head;
        while let Some(index) = free {
            let slot = self.slot(index);
            if slot.generation == u32::MAX {
                return vacancies;
            }
            vacancies += 1;
            free = slot.free_next;
        }
        let unused = self.allocated_slots().saturating_sub(self.initialized as usize);
        let next_page = if unused == 0 && self.page_count < u32::MAX { artifact_history_page_slots::<T>() } else { 0 };
        vacancies + unused + next_page
    }

    pub fn has_capacity(&self) -> bool {
        self.replacement_count==0 && self.group.is_none() && self.reservation.is_none() && (self.free_head.is_some_and(|index| self.slot(index).generation != u32::MAX) || (self.initialized as usize) < self.allocated_slots() || self.page_count < u32::MAX)
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn iter(&self) -> ArtifactHistoryIter<'_, T> {
        let (front, back, remaining) = self.visible_bounds();
        ArtifactHistoryIter { ledger: self, front, back, remaining }
    }

    pub fn iter_mut(&mut self) -> ArtifactHistoryIterMut<'_, T> {
        assert!(self.group.is_none(), "mutable history iteration requires its staged group to be adopted or aborted");
        assert!(self.replacement_count==0,"mutable history iteration requires every original replacement to settle");
        assert!(self.read_count==0, "mutable history iteration requires every original retained read to return");
        ArtifactHistoryIterMut { ledger: self, front: self.head, back: self.tail, remaining: self.len, marker: std::marker::PhantomData }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.replacement_count == 0 && self.read_count == 0 && self.len == 0 && self.head.is_none() && self.tail.is_none() && self.reservation.is_none() && self.group.is_none()
    }

    /// 🪹️ Observes the original native backing after logical entry ownership has left.
    pub fn backing_is_empty(&self) -> bool { self.pages.is_none() && self.page_count == 0 }

    /// 🧮️ Observes initialized original headers still retaining their slot visibility leases.
    pub fn retained_terminal_slot_count(&self) -> usize { self.initialized as usize }

    /// 👁️ Borrows the exact next slot visibility without copying or transferring its Arc.
    pub fn next_terminal_slot_visibility(&self) -> Result<Option<&std::sync::Arc<ArtifactGroupVisibility>>, ValueError> {
        if !self.terminal_is_empty() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history ledger still retains logical entry owners")); }
        if self.initialized == 0 { return Ok(None); }
        let slot = self.slot(self.initialized - 1);
        if slot.is_occupied() { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "terminal history header still contains its original value")); }
        Ok(slot.visibility.as_ref())
    }

    /// 🎟️ Transfers at most one original empty slot's visibility after item and depth admission.
    pub fn retire_terminal_slot(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<(Option<std::sync::Arc<ArtifactGroupVisibility>>, semio_framework_value::retained_clone::RetainedCloneProgress), ValueError> {
        self.next_terminal_slot_visibility()?;
        if grant.maximum_items == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit, "history slot retirement requires one admitted item")); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history slot retirement requires original owner depth")); }
        if self.initialized == 0 { return Ok((None, Default::default())); }
        let index = self.initialized - 1;
        let visibility = self.slot_mut(index).visibility.take();
        self.initialized = index;
        self.free_head = None;
        self.forget_seek();
        Ok((visibility, semio_framework_value::retained_clone::RetainedCloneProgress { copied_items: 1, ..Default::default() }))
    }

    /// 📏️ Declares the actual last slot slice, page shell, and separately owned empty directory.
    pub fn next_empty_page_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.terminal_is_empty() || self.initialized != 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history page retains original entry or visibility headers")); }
        if self.backing_is_empty() { return Ok(0); }
        let index = self.page_count.checked_sub(1).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history directory has no corresponding page"))? as usize;
        let mut chunk = self.pages.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page lost its original directory"))?;
        for _ in 0..index / ARTIFACT_HISTORY_DIRECTORY_PAGES { chunk = chunk.next.as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page directory chain is incomplete"))?; }
        let offset = index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
        if chunk.next.is_some() || usize::from(chunk.count) != offset + 1 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page directory differs from its actual last allocation")); }
        let page = chunk.pages[offset].as_ref().ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "history page slot backing is absent"))?;
        let slots = std::alloc::Layout::array::<std::mem::MaybeUninit<ArtifactHistorySlot<T>>>(page.slots.len()).map_err(|_| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history slot backing layout overflow"))?.size();
        slots.checked_add(size_of::<ArtifactHistorySlotPage<T>>()).and_then(|bytes| bytes.checked_add(if offset == 0 { size_of::<ArtifactHistoryPageChunk<T>>() } else { 0 })).ok_or_else(|| ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit, "history page whole release extent overflow"))
    }

    /// ♻️ Releases one real native page only after its visibility owners and full release grant.
    pub fn release_empty_page(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, ValueError> {
        use semio_framework_value::retained_clone::{RetainedCloneProgress, RetainedCloneStep};
        let empty = RetainedCloneProgress::default();
        let demand = self.next_empty_page_release_byte_demand()?;
        if self.backing_is_empty() { return Ok(RetainedCloneStep::Complete(empty)); }
        if grant.maximum_items == 0 || grant.maximum_release_bytes < demand { return Ok(RetainedCloneStep::Progress(empty)); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "history page release requires original owner depth")); }
        let index = self.page_count as usize - 1;
        let directory = index / ARTIFACT_HISTORY_DIRECTORY_PAGES;
        let offset = index % ARTIFACT_HISTORY_DIRECTORY_PAGES;
        let mut chunk = self.pages.as_mut().unwrap();
        for _ in 0..directory { chunk = chunk.next.as_mut().unwrap(); }
        drop(chunk.pages[offset].take());
        chunk.count -= 1;
        if offset == 0 {
            if directory == 0 { drop(self.pages.take()); }
            else {
                let mut previous = self.pages.as_mut().unwrap();
                for _ in 0..directory - 1 { previous = previous.next.as_mut().unwrap(); }
                drop(previous.next.take());
            }
        }
        self.page_count -= 1;
        let progress = RetainedCloneProgress { copied_items: 1, released_bytes: demand, ..empty };
        Ok(if self.backing_is_empty() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) })
    }

}

impl<T> Drop for ArtifactHistoryLedger<T> {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || self.terminal_is_empty(), "artifact history ledger reached Drop before every exact entry owner was retired");
        unsafe { std::mem::ManuallyDrop::drop(&mut self.pages) };
    }
}

/// 🧬️ Rebuilds a fresh, independent ledger from every currently-visible entry (mirrors `Serialize`'s
/// own `read_group`-scoped view) — a pending, uncommitted group's staged entries clone away exactly
/// as they already serialize away, since a clone is just another reader. `try_from_preflighted` can
/// only fail when the page address space is exhausted, which an already-admitted source ledger never does.
impl<T: Clone> Clone for ArtifactHistoryLedger<T> {
    fn clone(&self) -> Self {
        Self::try_from_preflighted(self.iter().cloned().collect()).unwrap_or_else(|_| unreachable!("a source ledger's own entries never exhaust its page address space"))
    }
}

/// 🧬️ `ToValue`/`FromValue` for `ArtifactHistoryLedger<T>` — serde's own `Serialize`/`Deserialize`
/// were dropped from this type (see the `Change` struct's docstring above); this is now the only
/// codec. Same admission path (`try_from_preflighted`) every other ledger construction uses.
impl<T: ToValue> ToValue for ArtifactHistoryLedger<T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.iter().map(ToValue::to_value).collect())
    }
}

impl<T: FromValue> FromValue for ArtifactHistoryLedger<T> {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let values = Vec::<T>::from_value(value)?;
        Self::try_from_preflighted(values).map_err(|rejected| ValueError::new(protocol::value::ValueRefusalKind::OwnershipLimit,format!("a history ledger within its page address space ({} entries rejected)", rejected.len())))
    }
}

pub struct ArtifactHistoryIter<'a, T> {
    ledger: &'a ArtifactHistoryLedger<T>,
    front: Option<u32>,
    back: Option<u32>,
    pub(super) remaining: usize,
}

impl<T> Clone for ArtifactHistoryIter<'_, T> {
    fn clone(&self) -> Self {
        Self { ledger: self.ledger, front: self.front, back: self.back, remaining: self.remaining }
    }
}

/// 🧬️ `ToValue` for `ArtifactHistoryIter` — serde's own `Serialize` was dropped from this type
/// (see the `Change` struct's docstring above). This is the exact shape `ArtifactVcsRead` (below)
/// needs for each of its four group-visibility-consistent fields.
impl<T: ToValue> ToValue for ArtifactHistoryIter<'_, T> {
    fn to_value(&self) -> DslValue {
        DslValue::Array(self.clone().map(ToValue::to_value).collect())
    }
}

impl<'a, T> Iterator for ArtifactHistoryIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = self.ledger.slot(index);
        self.front = slot.next;
        self.remaining -= 1;
        slot.get()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIter<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = self.ledger.slot(index);
        self.back = slot.previous;
        self.remaining -= 1;
        slot.get()
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIter<'_, T> {}

pub struct ArtifactHistoryIterMut<'a, T> {
    ledger: *mut ArtifactHistoryLedger<T>,
    front: Option<u32>,
    back: Option<u32>,
    pub(super) remaining: usize,
    marker: std::marker::PhantomData<&'a mut T>,
}

impl<'a, T> Iterator for ArtifactHistoryIterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.front?;
        let slot = unsafe { (*self.ledger).slot_mut(index) };
        self.front = slot.next;
        self.remaining -= 1;
        slot.value.as_mut().map(|value| unsafe { &mut *(value as *mut T) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T> DoubleEndedIterator for ArtifactHistoryIterMut<'_, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.remaining == 0 {
            return None;
        }
        let index = self.back?;
        let slot = unsafe { (*self.ledger).slot_mut(index) };
        self.back = slot.previous;
        self.remaining -= 1;
        slot.value.as_mut().map(|value| unsafe { &mut *(value as *mut T) })
    }
}

impl<T> ExactSizeIterator for ArtifactHistoryIterMut<'_, T> {}

impl<'a, T> IntoIterator for &'a ArtifactHistoryLedger<T> {
    type Item = &'a T;
    type IntoIter = ArtifactHistoryIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut ArtifactHistoryLedger<T> {
    type Item = &'a mut T;
    type IntoIter = ArtifactHistoryIterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T> std::ops::Index<usize> for ArtifactHistoryLedger<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T> std::ops::IndexMut<usize> for ArtifactHistoryLedger<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("artifact history index outside the live deterministic order")
    }
}

impl<T: std::fmt::Debug> std::fmt::Debug for ArtifactHistoryLedger<T> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_list().entries(self.iter()).finish()
    }
}

impl<T: PartialEq> PartialEq for ArtifactHistoryLedger<T> {
    fn eq(&self, other: &Self) -> bool {
        let left = self.iter();
        let right = other.iter();
        left.len() == right.len() && left.zip(right).all(|(left, right)| left == right)
    }
}

impl<T: Eq> Eq for ArtifactHistoryLedger<T> {}

#[path="♻️retirement/🦀️.rs"]
mod intrinsic_retirement;

impl protocol::HistoryTransitionEvents for ArtifactHistoryLedger<protocol::MutationEnvelope>{fn event_count(&self)->usize{self.len()}fn event_at(&self,index:usize)->Option<&protocol::MutationEnvelope>{self.get(index)}}

#[path="⏪️selection/🦀️.rs"]
mod edit_selection;
pub use edit_selection::OriginalHistoryEditSelection;

#[cfg(test)]
#[path="replacements-tests.rs"]
mod replacements_tests;

#[path="replacements.rs"]
mod replacements;
