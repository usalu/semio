//! 🖼️ Immutable fixed-page Canvas2D packet ownership shared by producers and render workers.

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

//#region 🖼️Canvas2dSnapshot
pub const CANVAS2D_SNAPSHOT_CAPACITY: usize = 8;
pub const CANVAS2D_SNAPSHOT_PAGE_CAPACITY: usize = 4;
pub const CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY: usize = 4_096;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Canvas2dSnapshotLease {
    pub slot: u8,
    pub epoch: u64,
    pub revision: u64,
    pub generation: u64,
    pub page_count: u8,
    pub byte_count: u32,
}

#[derive(Debug)]
pub struct Canvas2dSnapshotPage {
    bytes: Box<[u8; CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY]>,
    byte_len: u16,
    sealed: bool,
}

impl Canvas2dSnapshotPage {
    pub fn new() -> Self {
        Self { bytes: Box::new([0; CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY]), byte_len: 0, sealed: false }
    }

    pub fn push(&mut self, value: &[u8]) -> Result<(), Canvas2dSnapshotFault> {
        if self.sealed {
            return Err(Canvas2dSnapshotFault::PageState);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(value.len()).filter(|end| *end <= CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY).ok_or(Canvas2dSnapshotFault::ByteCredits)?;
        self.bytes[start..end].copy_from_slice(value);
        self.byte_len = end as u16;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), Canvas2dSnapshotFault> {
        if self.sealed {
            return Err(Canvas2dSnapshotFault::PageState);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes[..usize::from(self.byte_len)]
    }

    pub fn remaining(&self) -> usize {
        CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY - usize::from(self.byte_len)
    }

    pub fn backing_identity(&self) -> *const u8 {
        self.bytes.as_ptr()
    }
}

impl Default for Canvas2dSnapshotPage {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Canvas2dSnapshotWriteToken {
    slot: u8,
    epoch: u64,
    revision: u64,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Canvas2dSnapshotDescriptor {
    pub revision: u64,
    pub generation: u64,
    pub page_count: u8,
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Canvas2dSnapshotFault {
    Unavailable,
    Capacity,
    ByteCredits,
    PageState,
    Stale,
    NotSealed,
    Closing,
}

#[derive(Debug)]
pub struct Canvas2dRejectedSnapshotPage {
    pub fault: Canvas2dSnapshotFault,
    pub page: Canvas2dSnapshotPage,
}

struct Canvas2dSnapshotSlot {
    epoch: u64,
    descriptor: Canvas2dSnapshotDescriptor,
    pages: [Option<Box<Canvas2dSnapshotPage>>; CANVAS2D_SNAPSHOT_PAGE_CAPACITY],
    admitted_pages: u8,
    admitted_bytes: u32,
    sealed: bool,
    closing: bool,
}

impl Canvas2dSnapshotSlot {
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.admitted_pages == 0 && self.pages.iter().all(Option::is_none)
    }
}

struct Canvas2dSnapshotStore {
    epochs: [u64; CANVAS2D_SNAPSHOT_CAPACITY],
    slots: [Option<Canvas2dSnapshotSlot>; CANVAS2D_SNAPSHOT_CAPACITY],
}

impl Canvas2dSnapshotStore {
    const fn new() -> Self {
        Self { epochs: [0; CANVAS2D_SNAPSHOT_CAPACITY], slots: [const { None }; CANVAS2D_SNAPSHOT_CAPACITY] }
    }
}

static CANVAS2D_SNAPSHOTS: Mutex<Canvas2dSnapshotStore> = Mutex::new(Canvas2dSnapshotStore::new());

pub fn canvas2d_snapshot_begin(descriptor: Canvas2dSnapshotDescriptor) -> Result<Canvas2dSnapshotWriteToken, Canvas2dSnapshotFault> {
    if descriptor.page_count == 0
        || usize::from(descriptor.page_count) > CANVAS2D_SNAPSHOT_PAGE_CAPACITY
        || usize::try_from(descriptor.byte_count).ok().is_none_or(|bytes| bytes > usize::from(descriptor.page_count) * CANVAS2D_SNAPSHOT_PAGE_BYTE_CAPACITY)
    {
        return Err(Canvas2dSnapshotFault::Capacity);
    }
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let slot = store.slots.iter().position(Option::is_none).ok_or(Canvas2dSnapshotFault::Unavailable)?;
    let epoch = store.epochs[slot].wrapping_add(1).max(1);
    store.epochs[slot] = epoch;
    store.slots[slot] = Some(Canvas2dSnapshotSlot { epoch, descriptor, pages: std::array::from_fn(|_| None), admitted_pages: 0, admitted_bytes: 0, sealed: false, closing: false });
    Ok(Canvas2dSnapshotWriteToken { slot: slot as u8, epoch, revision: descriptor.revision, generation: descriptor.generation })
}

pub fn canvas2d_snapshot_admit_page(token: Canvas2dSnapshotWriteToken, page: Canvas2dSnapshotPage) -> Result<(), Canvas2dRejectedSnapshotPage> {
    let mut store = match CANVAS2D_SNAPSHOTS.lock() {
        Ok(store) => store,
        Err(_) => return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::Unavailable, page }),
    };
    let Some(slot) = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut) else {
        return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::Stale, page });
    };
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation {
        return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::Stale, page });
    }
    if slot.closing {
        return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::Closing, page });
    }
    if slot.sealed || !page.sealed {
        return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::PageState, page });
    }
    let next_pages = slot.admitted_pages.checked_add(1).filter(|count| *count <= slot.descriptor.page_count);
    let next_bytes = slot.admitted_bytes.checked_add(page.byte_len.into()).filter(|count| *count <= slot.descriptor.byte_count);
    let (Some(next_pages), Some(next_bytes)) = (next_pages, next_bytes) else {
        return Err(Canvas2dRejectedSnapshotPage { fault: Canvas2dSnapshotFault::Capacity, page });
    };
    slot.pages[usize::from(slot.admitted_pages)] = Some(Box::new(page));
    slot.admitted_pages = next_pages;
    slot.admitted_bytes = next_bytes;
    Ok(())
}

pub fn canvas2d_snapshot_seal(token: Canvas2dSnapshotWriteToken) -> Result<Canvas2dSnapshotLease, Canvas2dSnapshotFault> {
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation || slot.closing {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    if slot.admitted_pages != slot.descriptor.page_count {
        return Err(Canvas2dSnapshotFault::NotSealed);
    }
    slot.sealed = true;
    Ok(Canvas2dSnapshotLease { slot: token.slot, epoch: token.epoch, revision: token.revision, generation: token.generation, page_count: slot.admitted_pages, byte_count: slot.admitted_bytes })
}

pub fn canvas2d_snapshot_abort_write(token: Canvas2dSnapshotWriteToken) -> Result<(), Canvas2dSnapshotFault> {
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    slot.closing = true;
    Ok(())
}

pub fn canvas2d_snapshot_abort_write_step(token: Canvas2dSnapshotWriteToken) -> Result<bool, Canvas2dSnapshotFault> {
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let index = usize::from(token.slot);
    let slot = store.slots.get_mut(index).and_then(Option::as_mut).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || !slot.closing {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    if slot.admitted_pages > 0 {
        slot.admitted_pages -= 1;
        slot.pages[usize::from(slot.admitted_pages)] = None;
        return Ok(false);
    }
    if !slot.terminal_is_empty() {
        return Err(Canvas2dSnapshotFault::PageState);
    }
    store.slots[index] = None;
    Ok(true)
}

pub fn canvas2d_snapshot_write_terminal_is_empty(token: Canvas2dSnapshotWriteToken) -> bool {
    CANVAS2D_SNAPSHOTS.lock().ok().is_some_and(|store| store.slots.get(usize::from(token.slot)).is_none_or(Option::is_none))
}

pub fn canvas2d_snapshot_with_page<R>(lease: Canvas2dSnapshotLease, page: u8, operation: impl FnOnce(&Canvas2dSnapshotPage) -> R) -> Result<R, Canvas2dSnapshotFault> {
    let store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let slot = store.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || slot.descriptor.revision != lease.revision || slot.descriptor.generation != lease.generation || !slot.sealed || slot.closing {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    let page = slot.pages.get(usize::from(page)).and_then(Option::as_deref).ok_or(Canvas2dSnapshotFault::Capacity)?;
    Ok(operation(page))
}

pub fn canvas2d_snapshot_begin_close(lease: Canvas2dSnapshotLease) -> Result<(), Canvas2dSnapshotFault> {
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || slot.descriptor.revision != lease.revision || slot.descriptor.generation != lease.generation {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    slot.closing = true;
    Ok(())
}

pub fn canvas2d_snapshot_close_step(lease: Canvas2dSnapshotLease) -> Result<bool, Canvas2dSnapshotFault> {
    let mut store = CANVAS2D_SNAPSHOTS.lock().map_err(|_| Canvas2dSnapshotFault::Unavailable)?;
    let index = usize::from(lease.slot);
    let slot = store.slots.get_mut(index).and_then(Option::as_mut).ok_or(Canvas2dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || !slot.closing {
        return Err(Canvas2dSnapshotFault::Stale);
    }
    if slot.admitted_pages > 0 {
        slot.admitted_pages -= 1;
        slot.pages[usize::from(slot.admitted_pages)] = None;
        return Ok(false);
    }
    if !slot.terminal_is_empty() {
        return Err(Canvas2dSnapshotFault::PageState);
    }
    store.slots[index] = None;
    Ok(true)
}

pub fn canvas2d_snapshot_terminal_is_empty(lease: Canvas2dSnapshotLease) -> bool {
    CANVAS2D_SNAPSHOTS.lock().ok().is_some_and(|store| store.slots.get(usize::from(lease.slot)).is_none_or(Option::is_none))
}
//#endregion 🖼️Canvas2dSnapshot
