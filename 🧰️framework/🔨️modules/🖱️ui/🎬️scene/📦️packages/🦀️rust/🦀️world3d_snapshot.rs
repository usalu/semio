//! 🌍️ Fixed-page typed World3D scene ownership shared by producers and render workers.

use serde::{Deserialize, Serialize};
use std::sync::Mutex;

//#region 🌍️World3dSnapshot

pub const WORLD3D_SNAPSHOT_CAPACITY: usize = 8;
pub const WORLD3D_SNAPSHOT_PAGE_CAPACITY: usize = 256;
pub const WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY: usize = 64;
pub const WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY: usize = 16 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct World3dSnapshotLease {
    pub slot: u8,
    pub epoch: u64,
    pub revision: u64,
    pub generation: u64,
    pub page_count: u16,
    pub item_count: u32,
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotPageKind {
    Camera,
    Mesh,
    MeshVertex,
    MeshTriangle,
    MeshEdge,
    MeshUv,
    MeshColor,
    Instance,
    Selection,
    Vortex,
    Attraction,
    TargetVolume,
    Reference,
    Brush,
    Interaction,
    Engagement,
    Lod,
    Chunking,
    Environment,
    Frame,
    Fit,
    Terrain,
    Points,
    Status,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct World3dSnapshotSpan {
    pub start: u16,
    pub len: u16,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct World3dSnapshotItem {
    pub strings: [Option<World3dSnapshotSpan>; 4],
    pub numbers: [f64; 16],
    pub indexes: [u32; 8],
    pub number_len: u8,
    pub index_len: u8,
    pub flags: u16,
}

impl Default for World3dSnapshotItem {
    fn default() -> Self {
        Self { strings: [None; 4], numbers: [0.0; 16], indexes: [0; 8], number_len: 0, index_len: 0, flags: 0 }
    }
}

#[derive(Debug)]
pub struct World3dSnapshotPage {
    kind: World3dSnapshotPageKind,
    bytes: Box<[u8; WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY]>,
    byte_len: u16,
    items: Box<[Option<World3dSnapshotItem>; WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY]>,
    item_len: u8,
    sealed: bool,
}

impl World3dSnapshotPage {
    pub fn new(kind: World3dSnapshotPageKind) -> Self {
        Self { kind, bytes: Box::new([0; WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY]), byte_len: 0, items: Box::new([None; WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY]), item_len: 0, sealed: false }
    }

    pub fn push_string(&mut self, value: &str) -> Result<World3dSnapshotSpan, World3dSnapshotFault> {
        if self.sealed || value.len() > u16::MAX as usize {
            return Err(World3dSnapshotFault::PageState);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(value.len()).filter(|end| *end <= WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY).ok_or(World3dSnapshotFault::ByteCredits)?;
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.byte_len = end as u16;
        Ok(World3dSnapshotSpan { start: start as u16, len: value.len() as u16 })
    }

    pub fn push_item(&mut self, item: World3dSnapshotItem) -> Result<(), World3dSnapshotFault> {
        if self.sealed {
            return Err(World3dSnapshotFault::PageState);
        }
        let index = usize::from(self.item_len);
        if index == WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY || usize::from(item.number_len) > item.numbers.len() || usize::from(item.index_len) > item.indexes.len() {
            return Err(World3dSnapshotFault::ItemCredits);
        }
        for span in item.strings.iter().flatten() {
            let end = usize::from(span.start).checked_add(usize::from(span.len)).ok_or(World3dSnapshotFault::ByteCredits)?;
            if end > usize::from(self.byte_len) || std::str::from_utf8(&self.bytes[usize::from(span.start)..end]).is_err() {
                return Err(World3dSnapshotFault::PageState);
            }
        }
        self.items[index] = Some(item);
        self.item_len += 1;
        Ok(())
    }

    pub fn seal(&mut self) -> Result<(), World3dSnapshotFault> {
        if self.sealed {
            return Err(World3dSnapshotFault::PageState);
        }
        self.sealed = true;
        Ok(())
    }

    pub fn kind(&self) -> World3dSnapshotPageKind {
        self.kind
    }

    pub fn item_count(&self) -> usize {
        usize::from(self.item_len)
    }

    pub fn byte_count(&self) -> usize {
        usize::from(self.byte_len)
    }

    pub fn item(&self, index: usize) -> Option<&World3dSnapshotItem> {
        self.items.get(index)?.as_ref()
    }

    pub fn string(&self, span: World3dSnapshotSpan) -> Option<&str> {
        let start = usize::from(span.start);
        let end = start.checked_add(usize::from(span.len))?;
        std::str::from_utf8(self.bytes.get(start..end)?).ok()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dSnapshotWriteToken {
    slot: u8,
    epoch: u64,
    revision: u64,
    generation: u64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct World3dSnapshotDescriptor {
    pub revision: u64,
    pub generation: u64,
    pub page_count: u16,
    pub item_count: u32,
    pub byte_count: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum World3dSnapshotFault {
    Unavailable,
    Capacity,
    ItemCredits,
    ByteCredits,
    PageState,
    Stale,
    NotSealed,
    Closing,
}

#[derive(Debug)]
pub struct World3dRejectedSnapshotPage {
    pub fault: World3dSnapshotFault,
    pub page: World3dSnapshotPage,
}

struct World3dSnapshotSlot {
    epoch: u64,
    descriptor: World3dSnapshotDescriptor,
    pages: Box<[Option<Box<World3dSnapshotPage>>; WORLD3D_SNAPSHOT_PAGE_CAPACITY]>,
    admitted_pages: u16,
    admitted_items: u32,
    admitted_bytes: u32,
    sealed: bool,
    closing: bool,
}

impl World3dSnapshotSlot {
    fn terminal_is_empty(&self) -> bool {
        self.closing && self.admitted_pages == 0 && self.pages.iter().all(Option::is_none)
    }
}

struct World3dSnapshotStore {
    epochs: [u64; WORLD3D_SNAPSHOT_CAPACITY],
    slots: [Option<World3dSnapshotSlot>; WORLD3D_SNAPSHOT_CAPACITY],
}

impl World3dSnapshotStore {
    const fn new() -> Self {
        Self { epochs: [0; WORLD3D_SNAPSHOT_CAPACITY], slots: [const { None }; WORLD3D_SNAPSHOT_CAPACITY] }
    }
}

static WORLD3D_SNAPSHOTS: Mutex<World3dSnapshotStore> = Mutex::new(World3dSnapshotStore::new());

pub fn world3d_snapshot_begin(descriptor: World3dSnapshotDescriptor) -> Result<World3dSnapshotWriteToken, World3dSnapshotFault> {
    let pages = usize::from(descriptor.page_count);
    if pages == 0
        || pages > WORLD3D_SNAPSHOT_PAGE_CAPACITY
        || usize::try_from(descriptor.item_count).ok().is_none_or(|items| items > pages * WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY)
        || usize::try_from(descriptor.byte_count).ok().is_none_or(|bytes| bytes > pages * WORLD3D_SNAPSHOT_PAGE_BYTE_CAPACITY)
    {
        return Err(World3dSnapshotFault::Capacity);
    }
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let slot = store.slots.iter().position(Option::is_none).ok_or(World3dSnapshotFault::Unavailable)?;
    let epoch = store.epochs[slot].wrapping_add(1).max(1);
    store.epochs[slot] = epoch;
    store.slots[slot] = Some(World3dSnapshotSlot { epoch, descriptor, pages: Box::new([const { None }; WORLD3D_SNAPSHOT_PAGE_CAPACITY]), admitted_pages: 0, admitted_items: 0, admitted_bytes: 0, sealed: false, closing: false });
    Ok(World3dSnapshotWriteToken { slot: slot as u8, epoch, revision: descriptor.revision, generation: descriptor.generation })
}

pub fn world3d_snapshot_admit_page(token: World3dSnapshotWriteToken, page: World3dSnapshotPage) -> Result<(), World3dRejectedSnapshotPage> {
    let mut store = match WORLD3D_SNAPSHOTS.lock() {
        Ok(store) => store,
        Err(_) => return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::Unavailable, page }),
    };
    let Some(slot) = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut) else {
        return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::Stale, page });
    };
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation {
        return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::Stale, page });
    }
    if slot.closing {
        return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::Closing, page });
    }
    if slot.sealed || !page.sealed {
        return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::PageState, page });
    }
    let next_pages = slot.admitted_pages.checked_add(1).filter(|count| *count <= slot.descriptor.page_count);
    let next_items = slot.admitted_items.checked_add(page.item_len.into()).filter(|count| *count <= slot.descriptor.item_count);
    let next_bytes = slot.admitted_bytes.checked_add(page.byte_len.into()).filter(|count| *count <= slot.descriptor.byte_count);
    let (Some(next_pages), Some(next_items), Some(next_bytes)) = (next_pages, next_items, next_bytes) else {
        return Err(World3dRejectedSnapshotPage { fault: World3dSnapshotFault::Capacity, page });
    };
    slot.pages[usize::from(slot.admitted_pages)] = Some(Box::new(page));
    slot.admitted_pages = next_pages;
    slot.admitted_items = next_items;
    slot.admitted_bytes = next_bytes;
    Ok(())
}

pub fn world3d_snapshot_seal(token: World3dSnapshotWriteToken) -> Result<World3dSnapshotLease, World3dSnapshotFault> {
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation || slot.closing {
        return Err(World3dSnapshotFault::Stale);
    }
    if slot.admitted_pages != slot.descriptor.page_count || slot.admitted_items != slot.descriptor.item_count || slot.admitted_bytes != slot.descriptor.byte_count {
        return Err(World3dSnapshotFault::NotSealed);
    }
    slot.sealed = true;
    Ok(World3dSnapshotLease { slot: token.slot, epoch: token.epoch, revision: token.revision, generation: token.generation, page_count: slot.admitted_pages, item_count: slot.admitted_items, byte_count: slot.admitted_bytes })
}

pub fn world3d_snapshot_abort_write(token: World3dSnapshotWriteToken) -> Result<(), World3dSnapshotFault> {
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(token.slot)).and_then(Option::as_mut).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation {
        return Err(World3dSnapshotFault::Stale);
    }
    slot.closing = true;
    Ok(())
}

pub fn world3d_snapshot_abort_write_step(token: World3dSnapshotWriteToken) -> Result<bool, World3dSnapshotFault> {
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let index = usize::from(token.slot);
    let slot = store.slots.get_mut(index).and_then(Option::as_mut).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != token.epoch || slot.descriptor.revision != token.revision || slot.descriptor.generation != token.generation || !slot.closing {
        return Err(World3dSnapshotFault::Stale);
    }
    if slot.admitted_pages > 0 {
        slot.admitted_pages -= 1;
        slot.pages[usize::from(slot.admitted_pages)] = None;
        return Ok(false);
    }
    if !slot.terminal_is_empty() {
        return Err(World3dSnapshotFault::PageState);
    }
    store.slots[index] = None;
    Ok(true)
}

pub fn world3d_snapshot_write_terminal_is_empty(token: World3dSnapshotWriteToken) -> bool {
    WORLD3D_SNAPSHOTS.lock().ok().is_some_and(|store| store.slots.get(usize::from(token.slot)).is_none_or(Option::is_none))
}

pub fn world3d_snapshot_with_page<R>(lease: World3dSnapshotLease, page: u16, operation: impl FnOnce(&World3dSnapshotPage) -> R) -> Result<R, World3dSnapshotFault> {
    let store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let slot = store.slots.get(usize::from(lease.slot)).and_then(Option::as_ref).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || slot.descriptor.revision != lease.revision || slot.descriptor.generation != lease.generation || !slot.sealed || slot.closing {
        return Err(World3dSnapshotFault::Stale);
    }
    let page = slot.pages.get(usize::from(page)).and_then(Option::as_deref).ok_or(World3dSnapshotFault::Capacity)?;
    Ok(operation(page))
}

pub fn world3d_snapshot_begin_close(lease: World3dSnapshotLease) -> Result<(), World3dSnapshotFault> {
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let slot = store.slots.get_mut(usize::from(lease.slot)).and_then(Option::as_mut).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || slot.descriptor.revision != lease.revision || slot.descriptor.generation != lease.generation {
        return Err(World3dSnapshotFault::Stale);
    }
    slot.closing = true;
    Ok(())
}

pub fn world3d_snapshot_close_step(lease: World3dSnapshotLease) -> Result<bool, World3dSnapshotFault> {
    let mut store = WORLD3D_SNAPSHOTS.lock().map_err(|_| World3dSnapshotFault::Unavailable)?;
    let index = usize::from(lease.slot);
    let slot = store.slots.get_mut(index).and_then(Option::as_mut).ok_or(World3dSnapshotFault::Stale)?;
    if slot.epoch != lease.epoch || !slot.closing {
        return Err(World3dSnapshotFault::Stale);
    }
    if slot.admitted_pages > 0 {
        slot.admitted_pages -= 1;
        slot.pages[usize::from(slot.admitted_pages)] = None;
        return Ok(false);
    }
    if !slot.terminal_is_empty() {
        return Err(World3dSnapshotFault::PageState);
    }
    store.slots[index] = None;
    Ok(true)
}

pub fn world3d_snapshot_terminal_is_empty(lease: World3dSnapshotLease) -> bool {
    WORLD3D_SNAPSHOTS.lock().ok().is_some_and(|store| store.slots.get(usize::from(lease.slot)).is_none_or(Option::is_none))
}

//#endregion 🌍️World3dSnapshot

#[cfg(test)]
mod tests {
    use super::*;

    fn page(kind: World3dSnapshotPageKind, value: &str) -> World3dSnapshotPage {
        let mut page = World3dSnapshotPage::new(kind);
        let id = page.push_string(value).unwrap();
        page.push_item(World3dSnapshotItem { strings: [Some(id), None, None, None], ..Default::default() }).unwrap();
        page.seal().unwrap();
        page
    }

    #[test]
    fn fixed_page_snapshot_validates_aba_iteration_and_one_page_close() {
        let descriptor = World3dSnapshotDescriptor { revision: 7, generation: 9, page_count: 2, item_count: 2, byte_count: 7 };
        let token = world3d_snapshot_begin(descriptor).unwrap();
        world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Camera, "cam")).unwrap();
        world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Instance, "item")).unwrap();
        let lease = world3d_snapshot_seal(token).unwrap();
        assert_eq!(world3d_snapshot_with_page(lease, 1, |page| page.string(page.item(0).unwrap().strings[0].unwrap()).unwrap().to_owned()).unwrap(), "item");
        let stale = World3dSnapshotLease { epoch: lease.epoch.wrapping_add(1), ..lease };
        assert_eq!(world3d_snapshot_with_page(stale, 0, |_| ()), Err(World3dSnapshotFault::Stale));
        world3d_snapshot_begin_close(lease).unwrap();
        assert!(!world3d_snapshot_close_step(lease).unwrap());
        assert!(!world3d_snapshot_close_step(lease).unwrap());
        assert!(world3d_snapshot_close_step(lease).unwrap());
        assert!(world3d_snapshot_terminal_is_empty(lease));
    }

    #[test]
    fn descriptor_and_page_capacity_plus_one_fail_before_publication() {
        let descriptor = World3dSnapshotDescriptor { revision: 1, generation: 1, page_count: WORLD3D_SNAPSHOT_PAGE_CAPACITY as u16 + 1, item_count: 0, byte_count: 0 };
        assert_eq!(world3d_snapshot_begin(descriptor), Err(World3dSnapshotFault::Capacity));
        let mut full = World3dSnapshotPage::new(World3dSnapshotPageKind::Instance);
        for _ in 0..WORLD3D_SNAPSHOT_PAGE_ITEM_CAPACITY {
            full.push_item(World3dSnapshotItem::default()).unwrap();
        }
        assert_eq!(full.push_item(World3dSnapshotItem::default()), Err(World3dSnapshotFault::ItemCredits));
    }

    #[test]
    fn interrupted_writer_aborts_one_admitted_page_per_step() {
        let descriptor = World3dSnapshotDescriptor { revision: 3, generation: 5, page_count: 2, item_count: 2, byte_count: 2 };
        let token = world3d_snapshot_begin(descriptor).unwrap();
        world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Status, "a")).unwrap();
        world3d_snapshot_admit_page(token, page(World3dSnapshotPageKind::Engagement, "b")).unwrap();
        world3d_snapshot_abort_write(token).unwrap();
        assert!(!world3d_snapshot_abort_write_step(token).unwrap());
        assert!(!world3d_snapshot_abort_write_step(token).unwrap());
        assert!(world3d_snapshot_abort_write_step(token).unwrap());
        assert!(world3d_snapshot_write_terminal_is_empty(token));
    }
}
