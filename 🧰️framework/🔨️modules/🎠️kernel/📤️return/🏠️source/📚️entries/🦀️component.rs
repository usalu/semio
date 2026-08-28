//#region 📚️ReturnSourceEntries
use std::mem::{size_of, ManuallyDrop};

type Page<T> = Vec<Node<T>>;
type Head<T> = Option<Page<T>>;

struct Node<T> { next: Head<T>, value: Option<T> }

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ReturnSourceAllocationError { pub reason: &'static str, pub allocated_bytes: usize }

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct ReturnSourceReservation { pub ready: bool, pub allocated_bytes: usize }

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct ReturnSourceEntryStep {
    pub advanced_items: usize,
    pub copied_bytes: usize,
    pub released_bytes: usize,
    pub complete: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Building, Freezing, Frozen, Closing }

/// 📚️ Owns one-item allocations and relinks descriptors without moving initialized payloads.
pub(crate) struct ReturnSourceEntries<T> {
    building: ManuallyDrop<Head<T>>,
    frozen: ManuallyDrop<Head<T>>,
    reserved: ManuallyDrop<Head<T>>,
    maximum_entries: usize,
    length: usize,
    allocated_bytes: u128,
    phase: Phase,
    allocation_fault: bool,
}

impl<T> ReturnSourceEntries<T> {
    pub(crate) fn new(maximum_entries: usize) -> Self {
        Self { building: ManuallyDrop::new(None), frozen: ManuallyDrop::new(None), reserved: ManuallyDrop::new(None), maximum_entries, length: 0, allocated_bytes: 0, phase: Phase::Building, allocation_fault: false }
    }

    pub(crate) const fn required_allocation_bytes() -> usize { size_of::<Node<T>>() }
    pub(crate) const fn required_placement_bytes() -> usize { size_of::<Node<T>>() + size_of::<Head<T>>() * 2 }
    pub(crate) const fn required_freeze_bytes() -> usize { size_of::<Head<T>>() * 4 }
    pub(crate) const fn required_handoff_bytes() -> usize { size_of::<ReturnSourceEntry<T>>() + size_of::<Head<T>>() * 3 }
    pub(crate) fn allocated_bytes(&self) -> u128 { self.allocated_bytes }
    pub(crate) fn terminal_is_empty(&self) -> bool { self.building.is_none() && self.frozen.is_none() && self.reserved.is_none() && self.length == 0 && self.allocated_bytes == 0 }

    /// 🎟️ Reserves only one uninitialized node; any allocator delta on error remains owned.
    pub(crate) fn reserve_step(&mut self, maximum_allocation_bytes: usize) -> Result<ReturnSourceReservation, ReturnSourceAllocationError> {
        self.reserve_capacity(maximum_allocation_bytes, 1)
    }

    #[cfg(test)]
    pub(crate) fn reserve_step_with_capacity_for_test(&mut self, maximum_allocation_bytes: usize, capacity: usize) -> Result<ReturnSourceReservation, ReturnSourceAllocationError> {
        self.reserve_capacity(maximum_allocation_bytes, capacity)
    }

    fn reserve_capacity(&mut self, maximum_allocation_bytes: usize, capacity: usize) -> Result<ReturnSourceReservation, ReturnSourceAllocationError> {
        let error = |reason, allocated_bytes| ReturnSourceAllocationError { reason, allocated_bytes };
        if self.phase != Phase::Building || self.allocation_fault { return Err(error("return-source.entries-not-building", 0)); }
        if self.reserved.is_some() { return Ok(ReturnSourceReservation { ready: true, allocated_bytes: 0 }); }
        if self.length >= self.maximum_entries { return Err(error("return-source.entry-limit", 0)); }
        if maximum_allocation_bytes < Self::required_allocation_bytes() { return Ok(Default::default()); }
        if capacity == 0 { return Err(error("return-source.empty-reservation-request", 0)); }
        let mut page = Vec::new();
        let allocation = page.try_reserve_exact(capacity);
        let actual = page_bytes(&page);
        if actual != 0 {
            *self.reserved = Some(page);
            self.allocated_bytes += actual as u128;
        }
        if allocation.is_err() {
            self.allocation_fault = true;
            return Err(error("return-source.entry-allocation", actual));
        }
        if actual > maximum_allocation_bytes {
            self.allocation_fault = true;
            return Err(error("return-source.entry-allocation-exceeds-admission", actual));
        }
        Ok(ReturnSourceReservation { ready: true, allocated_bytes: actual })
    }

    /// 📥️ Moves an admitted value only after both backing and placement grants are present.
    pub(crate) fn try_push_reserved(&mut self, source: &mut Option<T>, maximum_placement_bytes: usize) -> Result<usize, &'static str> {
        if self.phase != Phase::Building || self.allocation_fault { return Err("return-source.entries-not-building"); }
        if source.is_none() { return Err("return-source.entry-source-missing"); }
        if self.length >= self.maximum_entries { return Err("return-source.entry-limit"); }
        let page = self.reserved.as_ref().ok_or("return-source.entry-backing-missing")?;
        if !page.is_empty() || page.capacity() == 0 { return Err("return-source.entry-backing-not-empty"); }
        let required = Self::required_placement_bytes();
        if maximum_placement_bytes < required { return Ok(0); }
        self.reserved.as_mut().unwrap().push(Node { next: self.building.take(), value: source.take() });
        *self.building = self.reserved.take();
        self.length += 1;
        Ok(required)
    }

    /// 🔁️ Reverses one link into FIFO order without copying or cloning the typed payload.
    pub(crate) fn freeze_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<ReturnSourceEntryStep, &'static str> {
        if self.allocation_fault || self.phase == Phase::Closing { return Err("return-source.entries-cannot-freeze"); }
        if self.phase == Phase::Frozen { return Ok(ReturnSourceEntryStep { complete: true, ..Default::default() }); }
        let required = Self::required_freeze_bytes();
        if maximum_items == 0 || maximum_bytes < required { return Ok(Default::default()); }
        self.phase = Phase::Freezing;
        if let Some(mut page) = self.building.take() {
            *self.building = page[0].next.take();
            page[0].next = self.frozen.take();
            *self.frozen = Some(page);
            let complete = self.building.is_none();
            if complete { self.phase = Phase::Frozen; }
            return Ok(ReturnSourceEntryStep { advanced_items: 1, copied_bytes: required, complete, released_bytes: 0 });
        }
        self.phase = Phase::Frozen;
        Ok(ReturnSourceEntryStep { complete: true, ..Default::default() })
    }

    pub(crate) fn take_front_into(&mut self, target: &mut Option<ReturnSourceEntry<T>>, maximum_bytes: usize) -> Result<bool, &'static str> {
        if target.is_some() { return Err("return-source.entry-target-occupied"); }
        if self.phase != Phase::Frozen || self.allocation_fault { return Err("return-source.entries-not-frozen"); }
        Self::handoff(&mut self.frozen, &mut self.length, &mut self.allocated_bytes, target, maximum_bytes)
    }

    pub(crate) fn begin_close(&mut self) { self.phase = Phase::Closing; }

    /// ♻️ Hands back one exact node, including an unused reservation, without dropping its value.
    pub(crate) fn take_close_entry_into(&mut self, target: &mut Option<ReturnSourceEntry<T>>, maximum_bytes: usize) -> Result<bool, &'static str> {
        if target.is_some() { return Err("return-source.entry-target-occupied"); }
        if self.phase != Phase::Closing { return Err("return-source.entries-not-closing"); }
        let head = if self.reserved.is_some() { &mut self.reserved } else if self.building.is_some() { &mut self.building } else { &mut self.frozen };
        Self::handoff(head, &mut self.length, &mut self.allocated_bytes, target, maximum_bytes)
    }

    fn handoff(head: &mut Head<T>, length: &mut usize, allocated_bytes: &mut u128, target: &mut Option<ReturnSourceEntry<T>>, maximum_bytes: usize) -> Result<bool, &'static str> {
        let Some(page) = head.as_ref() else { return Ok(false); };
        if maximum_bytes < Self::required_handoff_bytes() { return Ok(false); }
        let actual = page_bytes(page);
        let remaining = allocated_bytes.checked_sub(actual as u128).ok_or("return-source.entry-accounting-underflow")?;
        let remaining_length = if page.is_empty() { *length } else { length.checked_sub(1).ok_or("return-source.entry-count-underflow")? };
        let mut page = head.take().unwrap();
        if let Some(node) = page.first_mut() { *head = node.next.take(); }
        *target = Some(ReturnSourceEntry { page: ManuallyDrop::new(Some(page)), allocated_bytes: actual });
        *allocated_bytes = remaining;
        *length = remaining_length;
        Ok(true)
    }
}

impl<T> Drop for ReturnSourceEntries<T> {
    fn drop(&mut self) { assert!(self.terminal_is_empty(), "return source entries require exact structural handoff before Drop"); }
}
//#endregion 📚️ReturnSourceEntries

//#region 📦️ReturnSourceEntry
/// 📦️ Owns one isolated original allocation; generic release never destroys a live payload.
pub(crate) struct ReturnSourceEntry<T> { page: ManuallyDrop<Head<T>>, allocated_bytes: usize }

impl<T> ReturnSourceEntry<T> {
    pub(crate) fn value(&self) -> Option<&T> { self.page.as_ref().and_then(|page| page.first()).and_then(|node| node.value.as_ref()) }
    pub(crate) fn allocated_bytes(&self) -> usize { self.allocated_bytes }

    /// 📤️ Transfers a value into an admitted empty owner; this is copying, not retirement credit.
    pub(crate) fn take_value_into(&mut self, target: &mut Option<T>, maximum_bytes: usize) -> Result<usize, &'static str> {
        if target.is_some() { return Err("return-source.value-target-occupied"); }
        let Some(node) = self.page.as_mut().and_then(|page| page.first_mut()) else { return Ok(0); };
        if node.next.is_some() { return Err("return-source.entry-not-isolated"); }
        if node.value.is_none() || maximum_bytes < size_of::<Option<T>>() { return Ok(0); }
        *target = node.value.take();
        Ok(size_of::<Option<T>>())
    }

    pub(crate) fn close_empty_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> Result<ReturnSourceEntryStep, &'static str> {
        let Some(page) = self.page.as_ref() else { return Ok(ReturnSourceEntryStep { complete: true, ..Default::default() }); };
        if page.first().is_some_and(|node| node.next.is_some()) { return Err("return-source.entry-not-isolated"); }
        if self.value().is_some() || maximum_items == 0 || maximum_bytes < self.allocated_bytes { return Ok(Default::default()); }
        drop(self.page.take());
        let released_bytes = std::mem::replace(&mut self.allocated_bytes, 0);
        Ok(ReturnSourceEntryStep { advanced_items: 1, released_bytes, complete: true, copied_bytes: 0 })
    }
}

impl<T> Drop for ReturnSourceEntry<T> {
    fn drop(&mut self) { assert!(self.page.is_none() && self.allocated_bytes == 0, "return source entry requires exact value and backing retirement before Drop"); }
}

fn page_bytes<T>(page: &Page<T>) -> usize { page.capacity() * size_of::<Node<T>>() }
//#endregion 📦️ReturnSourceEntry
