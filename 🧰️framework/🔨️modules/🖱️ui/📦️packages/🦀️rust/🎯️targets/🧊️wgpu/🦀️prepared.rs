//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use crate::wgpu::draw::{DrawLayer, DrawList, ScissorRect};
use crate::wgpu::kernel_3d_scene::Mesh3dLease;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, StepContext, StepOutcome};
use std::mem::size_of;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::{LazyLock, Mutex};

//#region 📊️Credits
/// 🎛️ Hard item and byte credits for one prepared frame transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedRenderLimits {
    pub max_draw_items: usize,
    pub max_draw_bytes: usize,
    pub max_upload_items: usize,
    pub max_upload_bytes: usize,
}

impl Default for PreparedRenderLimits {
    fn default() -> Self {
        Self { max_draw_items: 262_144, max_draw_bytes: 64 * 1024 * 1024, max_upload_items: 256, max_upload_bytes: 32 * 1024 * 1024 }
    }
}

/// 📏️ Measured ownership cost of a prepared packet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRenderUsage {
    pub draw_items: usize,
    pub draw_bytes: usize,
    pub upload_items: usize,
    pub upload_bytes: usize,
}

impl PreparedRenderUsage {
    fn include_draw(&mut self, items: usize, bytes: usize) -> bool {
        let Some(draw_items) = self.draw_items.checked_add(items) else { return false };
        let Some(draw_bytes) = self.draw_bytes.checked_add(bytes) else { return false };
        self.draw_items = draw_items;
        self.draw_bytes = draw_bytes;
        true
    }

    fn include_upload(&mut self, bytes: usize) -> bool {
        let Some(upload_items) = self.upload_items.checked_add(1) else { return false };
        let Some(upload_bytes) = self.upload_bytes.checked_add(bytes) else { return false };
        self.upload_items = upload_items;
        self.upload_bytes = upload_bytes;
        true
    }

    pub fn fits(self, limits: PreparedRenderLimits) -> bool {
        self.draw_items <= limits.max_draw_items && self.draw_bytes <= limits.max_draw_bytes && self.upload_items <= limits.max_upload_items && self.upload_bytes <= limits.max_upload_bytes
    }
}

pub const PREPARED_RENDER_METADATA_ITEMS: usize = 256;
pub const PREPARED_RENDER_METADATA_PAGE_ITEMS: usize = 32;
const PREPARED_RENDER_METADATA_PAGES: usize = PREPARED_RENDER_METADATA_ITEMS / PREPARED_RENDER_METADATA_PAGE_ITEMS;
pub const PREPARED_RENDER_COMMAND_PAGE_ITEMS: usize = 64;
pub const PREPARED_RENDER_COMMAND_PAGES: usize = 4_096;
const PREPARED_RENDER_PROCESS_SLOTS: usize = 64;
const PREPARED_RENDER_PROCESS_PAGES: usize = 16_383;
const PREPARED_RENDER_PROCESS_BACKING_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug)]
struct PreparedFixedPage<T> {
    slots: [Option<T>; PREPARED_RENDER_METADATA_PAGE_ITEMS],
    len: usize,
}

impl<T> Default for PreparedFixedPage<T> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0 }
    }
}

/// 🧰 Fixed retained page list whose backing and each populated owner are closed independently.
#[derive(Debug)]
pub struct PreparedFixedList<T> {
    pages: [Option<Box<PreparedFixedPage<T>>>; PREPARED_RENDER_METADATA_PAGES],
    head: usize,
    len: usize,
}

impl<T> Default for PreparedFixedList<T> {
    fn default() -> Self {
        Self { pages: std::array::from_fn(|_| None), head: 0, len: 0 }
    }
}

impl<T> PreparedFixedList<T> {
    pub fn try_push(&mut self, value: T) -> Result<(), T> {
        if self.len == PREPARED_RENDER_METADATA_ITEMS {
            return Err(value);
        }
        let Some(index) = self.head.checked_add(self.len).map(|value| value % PREPARED_RENDER_METADATA_ITEMS) else { return Err(value) };
        let Some(next) = self.len.checked_add(1) else { return Err(value) };
        let page_index = index / PREPARED_RENDER_METADATA_PAGE_ITEMS;
        let scalar = index % PREPARED_RENDER_METADATA_PAGE_ITEMS;
        if self.pages[page_index].is_none() {
            self.pages[page_index] = Some(Box::new(PreparedFixedPage::default()));
        }
        let Some(page) = self.pages[page_index].as_mut() else { return Err(value) };
        let Some(page_len) = page.len.checked_add(1).filter(|len| *len <= PREPARED_RENDER_METADATA_PAGE_ITEMS) else { return Err(value) };
        page.slots[scalar] = Some(value);
        page.len = page_len;
        self.len = next;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }
        let physical = self.head.checked_add(index)? % PREPARED_RENDER_METADATA_ITEMS;
        self.pages.get(physical / PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_ref()?.slots.get(physical % PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_ref()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }
        let physical = self.head.checked_add(index)? % PREPARED_RENDER_METADATA_ITEMS;
        self.pages.get_mut(physical / PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_mut()?.slots.get_mut(physical % PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_mut()
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        self.get_mut(0)
    }

    pub fn last_mut(&mut self) -> Option<&mut T> {
        self.len.checked_sub(1).and_then(|index| self.get_mut(index))
    }

    pub fn pop(&mut self) -> Option<T> {
        let logical = self.len.checked_sub(1)?;
        let index = self.head.checked_add(logical)? % PREPARED_RENDER_METADATA_ITEMS;
        let page = self.pages.get_mut(index / PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_mut()?;
        let value = page.slots.get_mut(index % PREPARED_RENDER_METADATA_PAGE_ITEMS)?.take();
        page.len = page.len.checked_sub(usize::from(value.is_some()))?;
        self.len = logical;
        value
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let page = self.pages.get_mut(self.head / PREPARED_RENDER_METADATA_PAGE_ITEMS)?.as_mut()?;
        let value = page.slots.get_mut(self.head % PREPARED_RENDER_METADATA_PAGE_ITEMS)?.take();
        page.len = page.len.checked_sub(usize::from(value.is_some()))?;
        self.head = self.head.checked_add(1)? % PREPARED_RENDER_METADATA_ITEMS;
        self.len = self.len.checked_sub(1)?;
        value
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn release_backing_step(&mut self) -> bool {
        if self.len != 0 {
            return false;
        }
        if let Some(index) = self.pages.iter().position(|page| page.as_ref().is_some_and(|page| page.len == 0)) {
            self.pages[index] = None;
            return false;
        }
        if self.head != 0 {
            self.head = 0;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.head == 0 && self.pages.iter().all(Option::is_none)
    }
}

const PREPARED_RENDER_ITEM_SHIFT: u32 = 0;
const PREPARED_RENDER_PAGE_SHIFT: u32 = 7;
const PREPARED_RENDER_BACKING_SHIFT: u32 = 21;
const PREPARED_RENDER_ITEM_MASK: u64 = 0x7f;
const PREPARED_RENDER_PAGE_MASK: u64 = 0x3fff;
const PREPARED_RENDER_BACKING_MASK: u64 = 0x3fff;
static PREPARED_RENDER_PROCESS_PERMITS: AtomicU64 = AtomicU64::new(0);
static PREPARED_RENDER_SLOT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_SLOT_GENERATION: [AtomicU64; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU64::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_PACKET_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_PACKET_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderPacket>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_JOB_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_JOB_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderJob>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_INPUT_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_INPUT_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderInput>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];

fn prepared_render_units(bytes: usize) -> Option<usize> {
    bytes.checked_add(PREPARED_RASTER_PAGE_BYTES.checked_sub(1)?)?.checked_div(PREPARED_RASTER_PAGE_BYTES)
}

fn prepared_render_permit_delta(items: usize, pages: usize, backing_units: usize) -> Option<u64> {
    let items = u64::try_from(items).ok()?;
    let pages = u64::try_from(pages).ok()?;
    let backing = u64::try_from(backing_units).ok()?;
    if items > PREPARED_RENDER_ITEM_MASK || pages > PREPARED_RENDER_PAGE_MASK || backing > PREPARED_RENDER_BACKING_MASK {
        return None;
    }
    Some((items << PREPARED_RENDER_ITEM_SHIFT) | (pages << PREPARED_RENDER_PAGE_SHIFT) | (backing << PREPARED_RENDER_BACKING_SHIFT))
}

#[derive(Debug)]
struct PreparedRenderProcessPermit {
    slot: u8,
    generation: u64,
    pages: usize,
    backing_units: usize,
    release_phase: u8,
}

impl PreparedRenderProcessPermit {
    fn try_reserve(pages: usize, backing_bytes: usize) -> Option<Self> {
        let backing_units = prepared_render_units(backing_bytes)?;
        let current = PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire);
        let items = ((current >> PREPARED_RENDER_ITEM_SHIFT) & PREPARED_RENDER_ITEM_MASK).checked_add(1)?;
        let reserved_pages = ((current >> PREPARED_RENDER_PAGE_SHIFT) & PREPARED_RENDER_PAGE_MASK).checked_add(u64::try_from(pages).ok()?)?;
        let reserved_backing = ((current >> PREPARED_RENDER_BACKING_SHIFT) & PREPARED_RENDER_BACKING_MASK).checked_add(u64::try_from(backing_units).ok()?)?;
        if items > PREPARED_RENDER_PROCESS_SLOTS as u64 || reserved_pages > PREPARED_RENDER_PROCESS_PAGES as u64 || reserved_backing > u64::try_from(prepared_render_units(PREPARED_RENDER_PROCESS_BACKING_BYTES)?).ok()? {
            return None;
        }
        let delta = prepared_render_permit_delta(1, pages, backing_units)?;
        let next = current.checked_add(delta)?;
        PREPARED_RENDER_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).ok()?;
        let Some(slot) = PREPARED_RENDER_SLOT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok()) else {
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
            return None;
        };
        let current_generation = PREPARED_RENDER_SLOT_GENERATION[slot].load(Ordering::Acquire);
        let Some(generation) = current_generation.checked_add(1) else {
            PREPARED_RENDER_SLOT_STATE[slot].store(0, Ordering::Release);
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
            return None;
        };
        if PREPARED_RENDER_SLOT_GENERATION[slot].compare_exchange(current_generation, generation, Ordering::AcqRel, Ordering::Acquire).is_err() {
            PREPARED_RENDER_SLOT_STATE[slot].store(0, Ordering::Release);
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
            return None;
        }
        Some(Self { slot: slot as u8, generation, pages, backing_units, release_phase: 0 })
    }

    fn matches(&self) -> bool {
        let slot = usize::from(self.slot);
        PREPARED_RENDER_SLOT_STATE.get(slot).is_some_and(|state| state.load(Ordering::Acquire) == 1) && PREPARED_RENDER_SLOT_GENERATION.get(slot).is_some_and(|generation| generation.load(Ordering::Acquire) == self.generation)
    }

    fn try_grow(&mut self, pages: usize, backing_bytes: usize) -> bool {
        if !self.matches() {
            return false;
        }
        let Some(backing_units) = prepared_render_units(backing_bytes) else { return false };
        let current = PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire);
        let Some(reserved_pages) = u64::try_from(pages).ok().and_then(|pages| ((current >> PREPARED_RENDER_PAGE_SHIFT) & PREPARED_RENDER_PAGE_MASK).checked_add(pages)) else { return false };
        let Some(reserved_backing) = u64::try_from(backing_units).ok().and_then(|backing| ((current >> PREPARED_RENDER_BACKING_SHIFT) & PREPARED_RENDER_BACKING_MASK).checked_add(backing)) else { return false };
        let Some(backing_limit) = prepared_render_units(PREPARED_RENDER_PROCESS_BACKING_BYTES).and_then(|limit| u64::try_from(limit).ok()) else { return false };
        if reserved_pages > PREPARED_RENDER_PROCESS_PAGES as u64 || reserved_backing > backing_limit {
            return false;
        }
        let Some(delta) = prepared_render_permit_delta(0, pages, backing_units) else { return false };
        let Some(next) = current.checked_add(delta) else { return false };
        if PREPARED_RENDER_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let Some(owned_pages) = self.pages.checked_add(pages) else {
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
            return false;
        };
        let Some(owned_backing) = self.backing_units.checked_add(backing_units) else {
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
            return false;
        };
        self.pages = owned_pages;
        self.backing_units = owned_backing;
        true
    }

    fn release_step(&mut self) -> bool {
        let delta = match self.release_phase {
            0 => prepared_render_permit_delta(0, 0, self.backing_units),
            1 => prepared_render_permit_delta(0, self.pages, 0),
            2 => prepared_render_permit_delta(1, 0, 0),
            3 => {
                let slot = usize::from(self.slot);
                if !self.matches() || PREPARED_RENDER_SLOT_STATE[slot].compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
                    return false;
                }
                self.release_phase = 4;
                return true;
            }
            _ => return true,
        };
        let Some(delta) = delta else { return false };
        let current = PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire);
        let Some(next) = current.checked_sub(delta) else { return false };
        if PREPARED_RENDER_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.release_phase += 1;
        false
    }
}

impl Drop for PreparedRenderProcessPermit {
    fn drop(&mut self) {
        if self.release_phase >= 4 || !self.matches() {
            return;
        }
        let backing = if self.release_phase == 0 { self.backing_units } else { 0 };
        let pages = if self.release_phase <= 1 { self.pages } else { 0 };
        let items = usize::from(self.release_phase <= 2);
        if let Some(delta) = prepared_render_permit_delta(items, pages, backing) {
            PREPARED_RENDER_PROCESS_PERMITS.fetch_sub(delta, Ordering::AcqRel);
        }
        let slot = usize::from(self.slot);
        let _ = PREPARED_RENDER_SLOT_STATE[slot].compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire);
        self.release_phase = 4;
    }
}
//#endregion 📊️Credits

//#region 🧩️PagedRasterProducer
pub const PREPARED_RASTER_PAGE_BYTES: usize = 16 * 1024;
const PREPARED_RASTER_KEY_BYTES: usize = 256;
const PREPARED_RASTER_ITEM_BYTES: usize = 16 * 1024 * 1024;
const PREPARED_RASTER_PRODUCER_CAPACITY: usize = 256;
const PREPARED_RASTER_PRODUCER_ITEMS: usize = 4_096;
const PREPARED_RASTER_PRODUCER_BYTES: usize = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PreparedRasterLedgerSlot {
    epoch: u64,
    items: usize,
    bytes: usize,
    occupied: bool,
}

struct PreparedRasterLedger {
    slots: [PreparedRasterLedgerSlot; PREPARED_RASTER_PRODUCER_CAPACITY],
    items: usize,
    bytes: usize,
}

impl Default for PreparedRasterLedger {
    fn default() -> Self {
        Self { slots: [PreparedRasterLedgerSlot::default(); PREPARED_RASTER_PRODUCER_CAPACITY], items: 0, bytes: 0 }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedRasterCredit {
    slot: u16,
    epoch: u64,
    items: usize,
    bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRasterGeneration {
    slot: u16,
    epoch: u64,
}

impl PreparedRasterGeneration {
    pub fn slot(&self) -> u16 {
        self.slot
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

static PREPARED_RASTER_LEDGER: LazyLock<Mutex<PreparedRasterLedger>> = LazyLock::new(|| Mutex::new(PreparedRasterLedger::default()));

impl PreparedRasterLedger {
    fn reserve(&mut self, items: usize, bytes: usize) -> Option<PreparedRasterCredit> {
        let next_items = self.items.checked_add(items)?;
        let next_bytes = self.bytes.checked_add(bytes)?;
        if next_items > PREPARED_RASTER_PRODUCER_ITEMS || next_bytes > PREPARED_RASTER_PRODUCER_BYTES {
            return None;
        }
        let slot = self.slots.iter().position(|slot| !slot.occupied)?;
        let epoch = self.slots[slot].epoch.checked_add(1)?;
        self.slots[slot] = PreparedRasterLedgerSlot { epoch, items, bytes, occupied: true };
        self.items = next_items;
        self.bytes = next_bytes;
        Some(PreparedRasterCredit { slot: slot as u16, epoch, items, bytes })
    }

    fn release(&mut self, credit: &PreparedRasterCredit) -> bool {
        let Some(slot) = self.slots.get_mut(usize::from(credit.slot)) else { return false };
        if !slot.occupied || slot.epoch != credit.epoch || slot.items != credit.items || slot.bytes != credit.bytes {
            return false;
        }
        slot.occupied = false;
        slot.items = 0;
        slot.bytes = 0;
        self.items -= credit.items;
        self.bytes -= credit.bytes;
        true
    }

    fn resize(&mut self, credit: &mut PreparedRasterCredit, items: usize, bytes: usize) -> bool {
        let Some(slot) = self.slots.get_mut(usize::from(credit.slot)) else { return false };
        if !slot.occupied || slot.epoch != credit.epoch || slot.items != credit.items || slot.bytes != credit.bytes {
            return false;
        }
        let Some(next_items) = self.items.checked_sub(credit.items).and_then(|value| value.checked_add(items)) else { return false };
        let Some(next_bytes) = self.bytes.checked_sub(credit.bytes).and_then(|value| value.checked_add(bytes)) else { return false };
        if next_items > PREPARED_RASTER_PRODUCER_ITEMS || next_bytes > PREPARED_RASTER_PRODUCER_BYTES {
            return false;
        }
        slot.items = items;
        slot.bytes = bytes;
        self.items = next_items;
        self.bytes = next_bytes;
        credit.items = items;
        credit.bytes = bytes;
        true
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedRasterPage {
    start_row: u32,
    rows: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedRasterPages {
    slots: Vec<PreparedRasterPage>,
    backing: Vec<u8>,
    page_capacity: usize,
    rows_per_page: u32,
    width: u32,
    height: u32,
    byte_len: usize,
    source_generation: PreparedRasterGeneration,
    frame_generation: u64,
    credit: Option<PreparedRasterCredit>,
    backing_released: bool,
    key_released: bool,
    close_phase: u8,
}

pub const PREPARED_ATLAS_PAGE_BYTES: usize = 16 * 1024;
pub const PREPARED_ATLAS_PAGE_CAPACITY: usize = 2_048;
const PREPARED_ATLAS_PROCESS_ITEMS: usize = 64;
const PREPARED_ATLAS_PROCESS_PAGES: usize = 4_096;
const PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;
const PREPARED_ATLAS_PROCESS_BACKING_BYTES: usize = 96 * 1024 * 1024;
const PREPARED_ATLAS_ABANDONMENT_SLOTS: usize = PREPARED_ATLAS_PROCESS_ITEMS;

const PREPARED_ATLAS_ITEM_SHIFT: u32 = 0;
const PREPARED_ATLAS_PAGE_SHIFT: u32 = 7;
const PREPARED_ATLAS_PAYLOAD_SHIFT: u32 = 20;
const PREPARED_ATLAS_BACKING_SHIFT: u32 = 33;
const PREPARED_ATLAS_ITEM_MASK: u64 = 0x7f;
const PREPARED_ATLAS_PAGE_MASK: u64 = 0x1fff;
const PREPARED_ATLAS_PAYLOAD_MASK: u64 = 0x1fff;
const PREPARED_ATLAS_BACKING_MASK: u64 = 0x1fff;
static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64 = AtomicU64::new(0);
static PREPARED_ATLAS_GENERATION: AtomicU64 = AtomicU64::new(1);
static PREPARED_ATLAS_ABANDONMENT_STATE: [AtomicU8; PREPARED_ATLAS_ABANDONMENT_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_ATLAS_ABANDONMENT_SLOTS];
static PREPARED_ATLAS_ABANDONMENT_OWNER: [AtomicPtr<PreparedAtlasAbandonment>; PREPARED_ATLAS_ABANDONMENT_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_ATLAS_ABANDONMENT_SLOTS];

fn prepared_atlas_field(value: u64, shift: u32, mask: u64) -> usize {
    ((value >> shift) & mask) as usize
}

fn prepared_atlas_units(bytes: usize) -> Option<usize> {
    bytes.checked_add(PREPARED_ATLAS_PAGE_BYTES.checked_sub(1)?)?.checked_div(PREPARED_ATLAS_PAGE_BYTES)
}

fn prepared_atlas_delta(items: usize, pages: usize, payload_units: usize, backing_units: usize) -> Option<u64> {
    let items = u64::try_from(items).ok()?;
    let pages = u64::try_from(pages).ok()?;
    let payload = u64::try_from(payload_units).ok()?;
    let backing = u64::try_from(backing_units).ok()?;
    if items > PREPARED_ATLAS_ITEM_MASK || pages > PREPARED_ATLAS_PAGE_MASK || payload > PREPARED_ATLAS_PAYLOAD_MASK || backing > PREPARED_ATLAS_BACKING_MASK {
        return None;
    }
    Some((items << PREPARED_ATLAS_ITEM_SHIFT) | (pages << PREPARED_ATLAS_PAGE_SHIFT) | (payload << PREPARED_ATLAS_PAYLOAD_SHIFT) | (backing << PREPARED_ATLAS_BACKING_SHIFT))
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedAtlasPermit {
    generation: u64,
    items: usize,
    pages: usize,
    payload_bytes: usize,
    backing_bytes: usize,
    payload_units: usize,
    backing_units: usize,
    release_phase: u8,
}

impl PreparedAtlasPermit {
    fn try_reserve(pages: usize, payload_bytes: usize, backing_bytes: usize) -> Option<Self> {
        let payload_units = prepared_atlas_units(payload_bytes)?;
        let backing_units = prepared_atlas_units(backing_bytes)?;
        let current = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
        let items = prepared_atlas_field(current, PREPARED_ATLAS_ITEM_SHIFT, PREPARED_ATLAS_ITEM_MASK).checked_add(1)?;
        let reserved_pages = prepared_atlas_field(current, PREPARED_ATLAS_PAGE_SHIFT, PREPARED_ATLAS_PAGE_MASK).checked_add(pages)?;
        let reserved_payload = prepared_atlas_field(current, PREPARED_ATLAS_PAYLOAD_SHIFT, PREPARED_ATLAS_PAYLOAD_MASK).checked_add(payload_units)?;
        let reserved_backing = prepared_atlas_field(current, PREPARED_ATLAS_BACKING_SHIFT, PREPARED_ATLAS_BACKING_MASK).checked_add(backing_units)?;
        let payload_limit = prepared_atlas_units(PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES)?;
        let backing_limit = prepared_atlas_units(PREPARED_ATLAS_PROCESS_BACKING_BYTES)?;
        if items > PREPARED_ATLAS_PROCESS_ITEMS || reserved_pages > PREPARED_ATLAS_PROCESS_PAGES || reserved_payload > payload_limit || reserved_backing > backing_limit {
            return None;
        }
        let delta = prepared_atlas_delta(1, pages, payload_units, backing_units)?;
        let next = current.checked_add(delta)?;
        PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).ok()?;
        let generation = PREPARED_ATLAS_GENERATION.fetch_add(1, Ordering::AcqRel).max(1);
        Some(Self { generation, items: 1, pages, payload_bytes, backing_bytes, payload_units, backing_units, release_phase: 0 })
    }

    fn release_step(&mut self) -> bool {
        let delta = match self.release_phase {
            0 => prepared_atlas_delta(0, 0, 0, self.backing_units),
            1 => prepared_atlas_delta(0, 0, self.payload_units, 0),
            2 => prepared_atlas_delta(0, self.pages, 0, 0),
            3 => prepared_atlas_delta(self.items, 0, 0, 0),
            _ => return true,
        };
        let Some(delta) = delta else { return false };
        let current = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
        let Some(next) = current.checked_sub(delta) else { return false };
        if PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        self.release_phase = self.release_phase.saturating_add(1);
        self.release_phase > 3
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedAtlasPage {
    bytes: Box<[u8; PREPARED_ATLAS_PAGE_BYTES]>,
    len: u16,
    start_row: u32,
    rows: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedAtlasPages {
    slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>,
    len: usize,
    width: u32,
    height: u32,
    channels: u8,
    byte_len: usize,
    rows_per_page: u32,
    permit: Option<PreparedAtlasPermit>,
    abandonment_slot: u8,
}

struct PreparedAtlasAbandonment {
    slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>,
    len: usize,
    permit: Option<PreparedAtlasPermit>,
}

impl PreparedAtlasAbandonment {
    fn close_step(&mut self) -> bool {
        if let Some(index) = self.len.checked_sub(1) {
            self.len = index;
            if let Some(slots) = self.slots.as_mut() {
                slots[index] = None;
            }
            return false;
        }
        if self.slots.take().is_some() {
            return false;
        }
        if let Some(permit) = self.permit.as_mut() {
            if !permit.release_step() {
                return false;
            }
            self.permit = None;
            return false;
        }
        true
    }
}

impl PreparedAtlasPages {
    pub fn try_new(width: u32, height: u32, channels: u8, byte_len: usize) -> Result<Self, &'static str> {
        let row_bytes = usize::try_from(width).ok().and_then(|width| width.checked_mul(usize::from(channels))).ok_or("atlas row bytes exhausted")?;
        let expected = row_bytes.checked_mul(usize::try_from(height).map_err(|_| "atlas height exhausted")?).ok_or("atlas byte length exhausted")?;
        if expected != byte_len || row_bytes == 0 || row_bytes > PREPARED_ATLAS_PAGE_BYTES {
            return Err("atlas dimensions do not fit fixed page credits");
        }
        let rows_per_page = u32::try_from(PREPARED_ATLAS_PAGE_BYTES / row_bytes).map_err(|_| "atlas rows per page exhausted")?;
        let rows_per_page_usize = usize::try_from(rows_per_page).map_err(|_| "atlas rows per page exhausted")?;
        let pages = usize::try_from(height).ok().and_then(|height| height.checked_add(rows_per_page_usize.checked_sub(1)?)).map(|rows| rows / rows_per_page_usize).ok_or("atlas page count exhausted")?;
        if pages > PREPARED_ATLAS_PAGE_CAPACITY || byte_len > PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES {
            return Err("atlas page or byte credits exceeded");
        }
        let slot_backing = PREPARED_ATLAS_PAGE_CAPACITY.checked_mul(size_of::<Option<PreparedAtlasPage>>()).ok_or("atlas slot backing credits exhausted")?;
        let page_backing = pages.checked_mul(PREPARED_ATLAS_PAGE_BYTES).ok_or("atlas page backing credits exhausted")?;
        let backing_bytes = slot_backing.checked_add(page_backing).ok_or("atlas aggregate backing credits exhausted")?;
        let Some(abandonment_slot) = PREPARED_ATLAS_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok()) else {
            return Err("atlas abandonment owner credits exhausted");
        };
        let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes) else {
            PREPARED_ATLAS_ABANDONMENT_STATE[abandonment_slot].store(0, Ordering::Release);
            return Err("atlas process permit credits exhausted");
        };
        Ok(Self { slots: Some(Box::new([const { None }; PREPARED_ATLAS_PAGE_CAPACITY])), len: 0, width, height, channels, byte_len, rows_per_page, permit: Some(permit), abandonment_slot: abandonment_slot as u8 })
    }

    pub fn push_page(&mut self, source: &[u8], start_row: u32) -> Result<bool, &'static str> {
        if self.len == PREPARED_ATLAS_PAGE_CAPACITY || start_row >= self.height {
            return Err("atlas page item credits exceeded");
        }
        let row_bytes = usize::try_from(self.width).ok().and_then(|width| width.checked_mul(usize::from(self.channels))).ok_or("atlas row bytes exhausted")?;
        let remaining_rows = self.height - start_row;
        let rows = remaining_rows.min(self.rows_per_page);
        let bytes = usize::try_from(rows).ok().and_then(|rows| rows.checked_mul(row_bytes)).ok_or("atlas page byte count exhausted")?;
        let start = usize::try_from(start_row).ok().and_then(|row| row.checked_mul(row_bytes)).ok_or("atlas page offset exhausted")?;
        let slice = source.get(start..start.checked_add(bytes).ok_or("atlas page end exhausted")?).ok_or("atlas page source was truncated")?;
        let mut page = Box::new([0; PREPARED_ATLAS_PAGE_BYTES]);
        page[..bytes].copy_from_slice(slice);
        let slots = self.slots.as_mut().ok_or("atlas slot backing was released")?;
        slots[self.len] = Some(PreparedAtlasPage { bytes: page, len: u16::try_from(bytes).map_err(|_| "atlas page length exhausted")?, start_row, rows });
        self.len += 1;
        Ok(start_row.checked_add(rows) == Some(self.height))
    }

    pub fn page(&self, index: usize) -> Option<(&[u8], u32, u32)> {
        let page = self.slots.as_ref()?.get(index)?.as_ref()?;
        Some((&page.bytes[..usize::from(page.len)], page.start_row, page.rows))
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn next_row(&self) -> u32 {
        self.len.checked_sub(1).and_then(|index| self.slots.as_ref()?.get(index)?.as_ref()).and_then(|page| page.start_row.checked_add(page.rows)).unwrap_or(0)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(index) = self.len.checked_sub(1) {
            self.len = index;
            if let Some(slots) = self.slots.as_mut() {
                slots[index] = None;
            }
            return false;
        }
        if self.slots.take().is_some() {
            return false;
        }
        if let Some(permit) = self.permit.as_mut() {
            if !permit.release_step() {
                return false;
            }
            self.permit = None;
            return false;
        }
        let slot = usize::from(self.abandonment_slot);
        if PREPARED_ATLAS_ABANDONMENT_STATE.get(slot).is_some_and(|state| state.compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_ok()) {
            self.abandonment_slot = u8::MAX;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.slots.is_none() && self.permit.is_none() && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one page, backing owner, or permit scalar from one abandoned atlas.
    pub fn close_abandoned_step() -> bool {
        let Some(index) = PREPARED_ATLAS_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_ATLAS_ABANDONMENT_OWNER[index].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_ATLAS_ABANDONMENT_STATE[index].store(2, Ordering::Release);
            return false;
        }
        let mut owner = unsafe { Box::from_raw(pointer) };
        if owner.close_step() {
            PREPARED_ATLAS_ABANDONMENT_STATE[index].store(0, Ordering::Release);
        } else {
            PREPARED_ATLAS_ABANDONMENT_OWNER[index].store(Box::into_raw(owner), Ordering::Release);
            PREPARED_ATLAS_ABANDONMENT_STATE[index].store(2, Ordering::Release);
        }
        false
    }
}

impl Drop for PreparedAtlasPages {
    fn drop(&mut self) {
        if self.terminal_is_empty() {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_ATLAS_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let owner = Box::new(PreparedAtlasAbandonment { slots: self.slots.take(), len: std::mem::take(&mut self.len), permit: self.permit.take() });
        PREPARED_ATLAS_ABANDONMENT_OWNER[slot].store(Box::into_raw(owner), Ordering::Release);
        state.store(2, Ordering::Release);
        self.abandonment_slot = u8::MAX;
    }
}

impl PreparedRasterPages {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.source_generation
    }

    pub fn frame_generation(&self) -> u64 {
        self.frame_generation
    }

    pub(crate) fn page_for_row(&self, row: u32) -> Option<(&[u8], u32)> {
        if row >= self.height || self.rows_per_page == 0 || self.slots.len() != self.page_capacity {
            return None;
        }
        let logical = usize::try_from(row / self.rows_per_page).ok()?;
        let physical = self.page_capacity.checked_sub(logical.checked_add(1)?)?;
        let page = self.slots.get(physical)?;
        let row_bytes = usize::try_from(self.width).ok()?.checked_mul(4)?;
        let start = usize::try_from(page.start_row).ok()?.checked_mul(row_bytes)?;
        let len = usize::try_from(page.rows).ok()?.checked_mul(row_bytes)?;
        let bytes = self.backing.get(start..start.checked_add(len)?)?;
        (page.start_row == row).then_some((bytes, page.rows))
    }

    fn retire_page_step(&mut self) -> bool {
        self.slots.pop().is_some()
    }

    fn retire_backing_step(&mut self) -> bool {
        if !self.backing.is_empty() {
            self.backing.truncate(self.backing.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.backing_released {
            self.backing = Vec::new();
            self.backing_released = true;
            return false;
        }
        true
    }

    fn retire_metadata_step(&mut self) -> bool {
        match self.close_phase {
            0 => self.slots = Vec::new(),
            1 => self.page_capacity = 0,
            2 => self.rows_per_page = 0,
            3 => self.width = 0,
            4 => self.height = 0,
            5 => self.byte_len = 0,
            6 => self.source_generation = PreparedRasterGeneration::default(),
            7 => self.frame_generation = 0,
            8 => self.backing_released = true,
            9 => {
                let Some(credit) = self.credit.as_ref() else {
                    self.close_phase = 10;
                    return true;
                };
                let Ok(mut ledger) = PREPARED_RASTER_LEDGER.lock() else { return false };
                if !ledger.release(credit) {
                    return false;
                }
                self.credit = None;
            }
            _ => return true,
        }
        self.close_phase += 1;
        false
    }

    fn retire_with_key_step(&mut self, key: &mut String) -> bool {
        if self.retire_page_step() {
            return false;
        }
        if !self.retire_backing_step() {
            return false;
        }
        if key.pop().is_some() {
            return false;
        }
        if !self.key_released {
            *key = String::new();
            self.key_released = true;
            return false;
        }
        self.retire_metadata_step()
    }

    fn terminal_is_empty(&self) -> bool {
        self.close_phase >= 10 && self.slots.is_empty() && self.slots.capacity() == 0 && self.backing.is_empty() && self.backing.capacity() == 0 && self.backing_released && self.key_released && self.credit.is_none()
    }

    #[cfg(test)]
    fn page_pointer(&self, logical: usize) -> Option<*const u8> {
        self.page_for_row(u32::try_from(logical).ok()?.checked_mul(self.rows_per_page)?).map(|(bytes, _)| bytes.as_ptr())
    }
}

#[derive(Debug)]
pub struct PreparedRasterRejected {
    fault: &'static str,
    key: String,
    source: Vec<u8>,
    retained_source: Vec<u8>,
    credit: Option<PreparedRasterCredit>,
    source_released: bool,
    retained_source_released: bool,
    key_released: bool,
}

impl PreparedRasterRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    pub fn close_step(&mut self) -> bool {
        if !self.source.is_empty() {
            self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.source_released {
            self.source = Vec::new();
            self.source_released = true;
            return false;
        }
        if !self.retained_source.is_empty() {
            self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.retained_source_released {
            self.retained_source = Vec::new();
            self.retained_source_released = true;
            return false;
        }
        if self.key.pop().is_some() {
            return false;
        }
        if !self.key_released {
            self.key = String::new();
            self.key_released = true;
            return false;
        }
        if let Some(credit) = self.credit.as_ref() {
            let Ok(mut ledger) = PREPARED_RASTER_LEDGER.lock() else { return false };
            if !ledger.release(credit) {
                return false;
            }
            self.credit = None;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.source.is_empty()
            && self.source.capacity() == 0
            && self.source_released
            && self.retained_source.is_empty()
            && self.retained_source.capacity() == 0
            && self.retained_source_released
            && self.key.is_empty()
            && self.key.capacity() == 0
            && self.key_released
            && self.credit.is_none()
    }
}

#[derive(Debug)]
pub struct PreparedRasterReservation {
    key: String,
    credit: Option<PreparedRasterCredit>,
    claim: Option<PreparedRasterClaim>,
    source_bytes: usize,
    source_peak_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PreparedRasterClaim {
    width: u32,
    height: u32,
    byte_len: usize,
    rows_per_page: usize,
    page_capacity: usize,
}

impl PreparedRasterReservation {
    pub fn try_reserve(key: String) -> Result<Self, PreparedRasterRejected> {
        Self::try_reserve_source(key, 0)
    }

    pub fn try_reserve_source(key: String, source_bytes: usize) -> Result<Self, PreparedRasterRejected> {
        let reject = |fault, key| PreparedRasterRejected { fault, key, source: Vec::new(), retained_source: Vec::new(), credit: None, source_released: false, retained_source_released: false, key_released: false };
        if key.len() > PREPARED_RASTER_KEY_BYTES {
            return Err(reject("raster producer exceeded fixed key credits", key));
        }
        let Some(key_bytes) = key.capacity().checked_mul(2) else { return Err(reject("raster producer key credits overflowed", key)) };
        let Some(source_peak_bytes) = source_bytes.checked_mul(2) else { return Err(reject("raster producer source credits overflowed", key)) };
        let Some(bytes) = source_peak_bytes.checked_add(key_bytes) else { return Err(reject("raster producer source credits overflowed", key)) };
        if source_bytes > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject("raster producer source exceeded fixed credits", key));
        }
        let credit = PREPARED_RASTER_LEDGER.lock().ok().and_then(|mut ledger| ledger.reserve(1, bytes));
        let Some(credit) = credit else { return Err(reject("raster producer process credits exhausted", key)) };
        Ok(Self { key, credit: Some(credit), claim: None, source_bytes, source_peak_bytes })
    }

    pub fn reject(mut self, fault: &'static str, source: Vec<u8>) -> PreparedRasterRejected {
        self.reject_with_retained(fault, source, Vec::new())
    }

    pub fn reject_with_retained(mut self, fault: &'static str, source: Vec<u8>, retained_source: Vec<u8>) -> PreparedRasterRejected {
        PreparedRasterRejected { fault, key: std::mem::take(&mut self.key), source, retained_source, credit: self.credit.take(), source_released: false, retained_source_released: false, key_released: false }
    }

    pub fn claim(mut self, width: u32, height: u32) -> Result<Self, PreparedRasterRejected> {
        self.claim_with_retained(width, height, Vec::new()).map(|(reservation, _)| reservation)
    }

    pub fn claim_with_retained(mut self, width: u32, height: u32, retained_source: Vec<u8>) -> Result<(Self, Vec<u8>), PreparedRasterRejected> {
        let reject = |reservation: Self, fault, retained_source| reservation.reject_with_retained(fault, Vec::new(), retained_source);
        if retained_source.capacity() > self.source_bytes {
            return Err(reject(self, "raster retained source exceeded its pre-admitted workspace", retained_source));
        }
        let Some(row_bytes) = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)) else { return Err(reject(self, "raster row byte credits overflowed", retained_source)) };
        let Some(byte_len) = row_bytes.checked_mul(usize::try_from(height).unwrap_or(usize::MAX)) else { return Err(reject(self, "raster byte credits overflowed", retained_source)) };
        if width == 0 || height == 0 || row_bytes > PREPARED_RASTER_PAGE_BYTES || byte_len > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject(self, "raster producer exceeded fixed item or byte credits", retained_source));
        }
        let rows_per_page = (PREPARED_RASTER_PAGE_BYTES / row_bytes).max(1);
        let page_capacity = usize::try_from(height).unwrap_or(usize::MAX).div_ceil(rows_per_page);
        let Some(items) = page_capacity.checked_add(8) else { return Err(reject(self, "raster producer item credits overflowed", retained_source)) };
        let Some(key_bytes) = self.key.capacity().checked_mul(2) else { return Err(reject(self, "raster producer key credits overflowed", retained_source)) };
        let Some(page_slot_bytes) = page_capacity.checked_mul(size_of::<PreparedRasterPage>()) else { return Err(reject(self, "raster producer page slot credits overflowed", retained_source)) };
        let Some(bytes) = self.source_peak_bytes.checked_add(byte_len).and_then(|value| value.checked_add(key_bytes)).and_then(|value| value.checked_add(page_slot_bytes)) else {
            return Err(reject(self, "raster producer aggregate bytes overflowed", retained_source));
        };
        let resized = self.credit.as_mut().is_some_and(|credit| PREPARED_RASTER_LEDGER.lock().is_ok_and(|mut ledger| ledger.resize(credit, items, bytes)));
        if !resized {
            return Err(reject(self, "raster producer exact credit resize failed", retained_source));
        }
        self.claim = Some(PreparedRasterClaim { width, height, byte_len, rows_per_page, page_capacity });
        Ok((self, retained_source))
    }

    pub fn finalize(mut self, source: Vec<u8>, retained_source: Vec<u8>, width: u32, height: u32) -> Result<(PreparedRasterProducer, String), PreparedRasterRejected> {
        let reject = |reservation: Self, fault, source, retained_source| reservation.reject_with_retained(fault, source, retained_source);
        let Some(claim) = self.claim else { return Err(reject(self, "raster producer was not claimed before materialization", source, retained_source)) };
        if claim.width != width || claim.height != height || source.len() != claim.byte_len || source.capacity() > claim.byte_len || retained_source.capacity() > self.source_bytes {
            return Err(reject(self, "raster materialization did not match its exact claim", source, retained_source));
        }
        let Some(credit) = self.credit.take() else { return Err(reject(self, "raster reservation lost its exact credit", source, retained_source)) };
        let source_generation = PreparedRasterGeneration { slot: credit.slot, epoch: credit.epoch };
        let pages = PreparedRasterPages {
            slots: Vec::with_capacity(claim.page_capacity),
            backing: Vec::new(),
            page_capacity: claim.page_capacity,
            rows_per_page: claim.rows_per_page as u32,
            width,
            height,
            byte_len: claim.byte_len,
            source_generation,
            frame_generation: 0,
            credit: Some(credit),
            backing_released: false,
            key_released: false,
            close_phase: 0,
        };
        let published_key = self.key.clone();
        let retained_source_released = retained_source.is_empty();
        Ok((PreparedRasterProducer { key: self.key, source, retained_source, pages: Some(pages), frame_generation: None, source_released: false, retained_source_released, closing: false }, published_key))
    }
}

#[derive(Debug)]
pub struct PreparedRasterProducer {
    key: String,
    source: Vec<u8>,
    retained_source: Vec<u8>,
    pages: Option<PreparedRasterPages>,
    frame_generation: Option<u64>,
    source_released: bool,
    retained_source_released: bool,
    closing: bool,
}

pub enum PreparedRasterProducerStep {
    Pending,
    Complete(PreparedRenderUpload),
    Fault(&'static str),
}

impl PreparedRasterProducer {
    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.pages.as_ref().map_or_else(PreparedRasterGeneration::default, PreparedRasterPages::source_generation)
    }

    pub fn try_admit(key: String, source: Vec<u8>, width: u32, height: u32) -> Result<(Self, String), PreparedRasterRejected> {
        match PreparedRasterReservation::try_reserve(key) {
            Ok(reservation) => match reservation.claim(width, height) {
                Ok(reservation) => reservation.finalize(source, Vec::new(), width, height),
                Err(mut rejected) => {
                    rejected.source = source;
                    Err(rejected)
                }
            },
            Err(mut rejected) => {
                rejected.source = source;
                Err(rejected)
            }
        }
    }

    pub fn bind_frame_generation(&mut self, generation: u64) -> bool {
        if generation == 0 || self.closing {
            return false;
        }
        match self.frame_generation {
            Some(current) => current == generation,
            None => {
                self.frame_generation = Some(generation);
                let Some(pages) = self.pages.as_mut() else { return false };
                pages.frame_generation = generation;
                true
            }
        }
    }

    pub fn step(&mut self, expected_generation: u64) -> PreparedRasterProducerStep {
        if self.closing {
            return PreparedRasterProducerStep::Fault("raster producer is closing");
        }
        if self.frame_generation != Some(expected_generation) {
            return PreparedRasterProducerStep::Fault("raster producer generation is stale");
        }
        if self.source.is_empty() {
            if !self.source_released {
                self.source = Vec::new();
                self.source_released = true;
                return PreparedRasterProducerStep::Pending;
            }
            if !self.retained_source.is_empty() {
                self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
                return PreparedRasterProducerStep::Pending;
            }
            if !self.retained_source_released {
                self.retained_source = Vec::new();
                self.retained_source_released = true;
                return PreparedRasterProducerStep::Pending;
            }
            let Some(pages) = self.pages.take() else { return PreparedRasterProducerStep::Fault("raster producer lost its admitted page owner") };
            let key = std::mem::take(&mut self.key);
            return PreparedRasterProducerStep::Complete(PreparedRenderUpload::RasterPages { key, pixels: pages });
        }
        let Some(pages) = self.pages.as_mut() else { return PreparedRasterProducerStep::Fault("raster producer lost its admitted page owner") };
        if pages.slots.len() == pages.page_capacity {
            pages.backing = std::mem::take(&mut self.source);
            self.source_released = true;
            return PreparedRasterProducerStep::Pending;
        }
        let Some(row_bytes) = usize::try_from(pages.width).ok().and_then(|width| width.checked_mul(4)) else { return PreparedRasterProducerStep::Fault("raster row byte count exhausted") };
        let Some(page_bytes) = usize::try_from(pages.rows_per_page).ok().and_then(|rows| rows.checked_mul(row_bytes)) else {
            return PreparedRasterProducerStep::Fault("raster page byte count exhausted");
        };
        let Some(logical) = pages.slots.len().checked_add(1).and_then(|next| pages.page_capacity.checked_sub(next)) else {
            return PreparedRasterProducerStep::Fault("raster page cursor exceeded its fixed capacity");
        };
        let Some(start) = logical.checked_mul(page_bytes) else { return PreparedRasterProducerStep::Fault("raster page offset exhausted") };
        let Some(end) = start.checked_add(page_bytes).map(|end| end.min(self.source.len())) else { return PreparedRasterProducerStep::Fault("raster page end exhausted") };
        let Ok(start_row) = u32::try_from(start / row_bytes) else { return PreparedRasterProducerStep::Fault("raster start row exhausted") };
        let Ok(rows) = u32::try_from((end - start) / row_bytes) else { return PreparedRasterProducerStep::Fault("raster page rows exhausted") };
        pages.slots.push(PreparedRasterPage { start_row, rows });
        PreparedRasterProducerStep::Pending
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self) -> bool {
        let Some(pages) = self.pages.as_mut() else { return self.key.is_empty() && self.source.is_empty() };
        if pages.retire_page_step() {
            return false;
        }
        if !pages.retire_backing_step() {
            return false;
        }
        if !self.source.is_empty() {
            self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.source_released {
            self.source = Vec::new();
            self.source_released = true;
            return false;
        }
        if !self.retained_source.is_empty() {
            self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.retained_source_released {
            self.retained_source = Vec::new();
            self.retained_source_released = true;
            return false;
        }
        if self.key.pop().is_some() {
            return false;
        }
        if !pages.key_released {
            self.key = String::new();
            pages.key_released = true;
            return false;
        }
        pages.retire_metadata_step()
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.key.is_empty()
            && self.key.capacity() == 0
            && self.source.is_empty()
            && self.source.capacity() == 0
            && self.source_released
            && self.retained_source.is_empty()
            && self.retained_source.capacity() == 0
            && self.retained_source_released
            && self.pages.as_ref().is_none_or(PreparedRasterPages::terminal_is_empty)
    }
}
//#endregion 🧩️PagedRasterProducer

//#region 📦️Packet
/// 🧲️ Presentation behavior selected during worker preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDirective {
    FullRedraw,
    DamageOnly,
    PreservePreviousOnFailure,
}

/// 📤️ Typed upload data owned by the prepared transaction.
#[derive(Debug, PartialEq)]
pub enum PreparedRenderUpload {
    #[cfg(test)]
    GlyphAtlas {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    #[cfg(test)]
    IconAtlas {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    GlyphAtlasPages {
        pixels: PreparedAtlasPages,
    },
    IconAtlasPages {
        pixels: PreparedAtlasPages,
    },
    #[cfg(test)]
    Raster {
        key: String,
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    RasterPages {
        key: String,
        pixels: PreparedRasterPages,
    },
    Mesh {
        key: String,
        version: u64,
        lease: Mesh3dLease,
    },
}

/// 🧹️ UI-thread GPU cache invalidation selected during worker preparation.
#[derive(Debug, PartialEq, Eq)]
pub enum PreparedRenderEviction {
    Mesh { key: String },
}

impl PreparedRenderEviction {
    pub fn byte_len(&self) -> Option<usize> {
        match self {
            Self::Mesh { key } => Some(key.len()),
        }
    }
}

impl PreparedRenderUpload {
    pub fn byte_len(&self) -> Option<usize> {
        match self {
            #[cfg(test)]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => Some(pixels.len()),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => Some(pixels.byte_len()),
            #[cfg(test)]
            Self::Raster { key, pixels, .. } => key.len().checked_add(pixels.len()),
            Self::RasterPages { key, pixels } => key.len().checked_add(pixels.byte_len()),
            Self::Mesh { key, lease, .. } => {
                let Ok(schema) = lease.schema() else { return Some(key.len()) };
                let bytes = usize::try_from(schema.vertices)
                    .ok()?
                    .checked_mul(24)?
                    .checked_add(usize::try_from(schema.indices).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.face_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.vertex_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.edges).ok()?.checked_mul(24)?)?
                    .checked_add(usize::try_from(schema.edge_ids).ok()?.checked_mul(4)?)?
                    .checked_add(usize::try_from(schema.uvs).ok()?.checked_mul(8)?)?
                    .checked_add(usize::try_from(schema.colors).ok()?.checked_mul(16)?)?;
                key.len().checked_add(bytes)
            }
        }
    }

    /// 🧹 Releases one page, byte, or key scalar from a rejected upload owner.
    pub fn close_step(&mut self) -> bool {
        match self {
            #[cfg(test)]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => {
                if pixels.pop().is_some() {
                    false
                } else {
                    true
                }
            }
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.close_step(),
            #[cfg(test)]
            Self::Raster { key, pixels, .. } => {
                if pixels.pop().is_some() {
                    false
                } else {
                    key.pop().is_none()
                }
            }
            Self::RasterPages { key, pixels } => pixels.retire_with_key_step(key),
            Self::Mesh { key, .. } => key.pop().is_none(),
        }
    }
}

pub type PreparedRenderUploads = PreparedFixedList<PreparedRenderUpload>;
pub type PreparedRenderEvictions = PreparedFixedList<PreparedRenderEviction>;
pub type PreparedRenderScissors = PreparedFixedList<ScissorRect>;
pub type PreparedRenderDirectives = PreparedFixedList<RenderDirective>;
pub type PreparedRasterProducers = PreparedFixedList<PreparedRasterProducer>;

/// 🧩 One immutable scalar command emitted by a retained preparation child.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedRenderCommandKind {
    Validate,
    Snap,
    Order,
    Tessellate,
    Batch,
    Hash,
    Upload,
    Packet,
}

impl PreparedRenderCommandKind {
    pub fn code(self) -> u32 {
        match self {
            Self::Validate => 0,
            Self::Snap => 1,
            Self::Order => 2,
            Self::Tessellate => 3,
            Self::Batch => 4,
            Self::Hash => 5,
            Self::Upload => 6,
            Self::Packet => 7,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedRenderCommand {
    kind: PreparedRenderCommandKind,
    source: usize,
    digest: u64,
    draw_cursor: Option<DrawMeasureCursor>,
    packet_overlay: bool,
}

impl PreparedRenderCommand {
    pub fn kind(&self) -> PreparedRenderCommandKind {
        self.kind
    }

    pub fn source(&self) -> usize {
        self.source
    }

    pub fn digest(&self) -> u64 {
        self.digest
    }

    pub(crate) fn draw_cursor(&self) -> Option<DrawMeasureCursor> {
        self.draw_cursor
    }

    pub(crate) fn packet_overlay(&self) -> bool {
        self.packet_overlay
    }
}

#[derive(Debug)]
struct PreparedRenderCommandPage {
    slots: [Option<PreparedRenderCommand>; PREPARED_RENDER_COMMAND_PAGE_ITEMS],
    len: usize,
}

const PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS: usize = 256;
const PREPARED_RENDER_COMMAND_DIRECTORIES: usize = PREPARED_RENDER_COMMAND_PAGES / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;

#[derive(Debug)]
struct PreparedRenderCommandDirectory {
    pages: [Option<Box<PreparedRenderCommandPage>>; PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS],
    len: usize,
}

impl Default for PreparedRenderCommandDirectory {
    fn default() -> Self {
        Self { pages: std::array::from_fn(|_| None), len: 0 }
    }
}

impl Default for PreparedRenderCommandPage {
    fn default() -> Self {
        Self { slots: [None; PREPARED_RENDER_COMMAND_PAGE_ITEMS], len: 0 }
    }
}

#[derive(Debug)]
pub struct PreparedRenderCommandPages {
    directories: [Option<Box<PreparedRenderCommandDirectory>>; PREPARED_RENDER_COMMAND_DIRECTORIES],
    page_count: usize,
    len: usize,
}

impl Default for PreparedRenderCommandPages {
    fn default() -> Self {
        Self { directories: std::array::from_fn(|_| None), page_count: 0, len: 0 }
    }
}

impl PreparedRenderCommandPages {
    fn try_push(&mut self, command: PreparedRenderCommand) -> Result<(), PreparedRenderCommand> {
        let page = self.len / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let scalar = self.len % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        if page >= PREPARED_RENDER_COMMAND_PAGES {
            return Err(command);
        }
        let directory_index = page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
        let page_index = page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
        if self.directories[directory_index].is_none() {
            self.directories[directory_index] = Some(Box::new(PreparedRenderCommandDirectory::default()));
        }
        let Some(directory) = self.directories[directory_index].as_mut() else { return Err(command) };
        if directory.pages[page_index].is_none() {
            directory.pages[page_index] = Some(Box::new(PreparedRenderCommandPage::default()));
            let Some(directory_len) = directory.len.checked_add(1) else { return Err(command) };
            let Some(page_count) = self.page_count.checked_add(1) else { return Err(command) };
            directory.len = directory_len;
            self.page_count = page_count;
        }
        let Some(owner) = directory.pages[page_index].as_mut() else { return Err(command) };
        owner.slots[scalar] = Some(command);
        let Some(owner_len) = owner.len.checked_add(1) else { return Err(command) };
        let Some(len) = self.len.checked_add(1) else { return Err(command) };
        owner.len = owner_len;
        self.len = len;
        Ok(())
    }

    pub fn get(&self, index: usize) -> Option<&PreparedRenderCommand> {
        let page = index / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let scalar = index % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
        let directory = self.directories.get(page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS)?.as_ref()?;
        directory.pages.get(page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS)?.as_ref()?.slots.get(scalar)?.as_ref()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn close_step(&mut self) -> bool {
        if let Some(index) = self.len.checked_sub(1) {
            let page = index / PREPARED_RENDER_COMMAND_PAGE_ITEMS;
            let scalar = index % PREPARED_RENDER_COMMAND_PAGE_ITEMS;
            let Some(directory) = self.directories.get_mut(page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS).and_then(Option::as_mut) else { return false };
            let Some(owner) = directory.pages.get_mut(page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS).and_then(Option::as_mut) else { return false };
            owner.slots[scalar] = None;
            let Some(owner_len) = owner.len.checked_sub(1) else { return false };
            owner.len = owner_len;
            self.len = index;
            return false;
        }
        if let Some(page) = self.page_count.checked_sub(1) {
            let directory_index = page / PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
            let page_index = page % PREPARED_RENDER_COMMAND_DIRECTORY_ITEMS;
            let Some(directory) = self.directories[directory_index].as_mut() else { return false };
            if !directory.pages[page_index].as_ref().is_some_and(|page| page.len == 0) {
                return false;
            }
            directory.pages[page_index] = None;
            let Some(directory_len) = directory.len.checked_sub(1) else { return false };
            directory.len = directory_len;
            self.page_count = page;
            return false;
        }
        if let Some(index) = self.directories.iter().position(|directory| directory.as_ref().is_some_and(|directory| directory.len == 0)) {
            self.directories[index] = None;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.page_count == 0 && self.directories.iter().all(Option::is_none)
    }
}

/// 🧱️ Send-capable frame data prepared without window, surface, device, or queue access.
pub struct PreparedRenderPacket {
    pub(crate) scene_revision: u64,
    pub(crate) preview_generation: u64,
    pub(crate) damage: PreparedRenderScissors,
    pub(crate) clips: PreparedRenderScissors,
    pub(crate) directives: PreparedRenderDirectives,
    pub(crate) uploads: PreparedRenderUploads,
    pub(crate) evictions: PreparedRenderEvictions,
    pub(crate) draw: DrawList,
    pub(crate) overlay: Option<DrawList>,
    pub(crate) commands: PreparedRenderCommandPages,
    pub(crate) time_seconds: f32,
    pub(crate) usage: PreparedRenderUsage,
    pub(crate) limits: PreparedRenderLimits,
    permit: Option<PreparedRenderProcessPermit>,
    retirement_phase: u8,
    abandonment_slot: u8,
}

impl PreparedRenderPacket {
    const RETIRE_PAGE_BYTES: usize = 16 * 1024;

    pub fn scene_revision(&self) -> u64 {
        self.scene_revision
    }

    pub fn preview_generation(&self) -> u64 {
        self.preview_generation
    }

    pub fn damage(&self) -> &PreparedRenderScissors {
        &self.damage
    }

    pub fn clips(&self) -> &PreparedRenderScissors {
        &self.clips
    }

    pub fn directives(&self) -> &PreparedRenderDirectives {
        &self.directives
    }

    pub fn uploads(&self) -> &PreparedRenderUploads {
        &self.uploads
    }

    pub fn evictions(&self) -> &PreparedRenderEvictions {
        &self.evictions
    }

    pub fn usage(&self) -> PreparedRenderUsage {
        self.usage
    }

    pub fn limits(&self) -> PreparedRenderLimits {
        self.limits
    }

    pub fn is_within_credits(&self) -> bool {
        self.usage.fits(self.limits) && self.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches)
    }

    pub fn command_pages(&self) -> &PreparedRenderCommandPages {
        &self.commands
    }

    fn try_arm_abandonment(mut self) -> Result<Self, Self> {
        let Some(slot) = self.permit.as_ref().map(|permit| usize::from(permit.slot)) else { return Err(self) };
        let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return Err(self) };
        if state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self);
        }
        self.abandonment_slot = slot as u8;
        Ok(self)
    }

    /// 🧹 Advances one owner from an interrupted packet Drop handback.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_RENDER_PACKET_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_RENDER_PACKET_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
            return false;
        }
        let mut packet = unsafe { Box::from_raw(pointer) };
        if packet.retire_step() || packet.abandonment_slot == u8::MAX {
            drop(packet);
        } else {
            PREPARED_RENDER_PACKET_ABANDONMENT_OWNER[slot].store(Box::into_raw(packet), Ordering::Release);
            PREPARED_RENDER_PACKET_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        }
        false
    }

    /// 🧹️ Releases at most one admitted page, draw owner, string scalar, or metadata item.
    pub fn retire_step(&mut self) -> bool {
        if let Some(upload) = self.uploads.last_mut() {
            let retained = match upload {
                #[cfg(test)]
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => {
                    let next = pixels.len().saturating_sub(Self::RETIRE_PAGE_BYTES);
                    if next != pixels.len() {
                        pixels.truncate(next);
                        true
                    } else {
                        false
                    }
                }
                PreparedRenderUpload::GlyphAtlasPages { pixels } | PreparedRenderUpload::IconAtlasPages { pixels } => !pixels.close_step(),
                #[cfg(test)]
                PreparedRenderUpload::Raster { key, pixels, .. } => {
                    let next = pixels.len().saturating_sub(Self::RETIRE_PAGE_BYTES);
                    if next != pixels.len() {
                        pixels.truncate(next);
                        true
                    } else {
                        key.pop().is_some()
                    }
                }
                PreparedRenderUpload::RasterPages { key, pixels } => !pixels.retire_with_key_step(key),
                PreparedRenderUpload::Mesh { key, .. } => key.pop().is_some(),
            };
            if retained {
                return false;
            }
            self.uploads.pop();
            return false;
        }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if key.pop().is_some() {
                return false;
            }
            self.evictions.pop();
            return false;
        }
        if !self.draw.retire_step() {
            return false;
        }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retire_step() {
                return false;
            }
            self.overlay = None;
            return false;
        }
        if !self.commands.close_step() {
            return false;
        }
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() {
            return false;
        }
        if !self.uploads.release_backing_step() || !self.evictions.release_backing_step() || !self.damage.release_backing_step() || !self.clips.release_backing_step() || !self.directives.release_backing_step() {
            return false;
        }
        if let Some(permit) = self.permit.as_mut() {
            if !permit.release_step() {
                return false;
            }
            self.permit = None;
            return false;
        }
        match self.retirement_phase {
            0 => self.scene_revision = 0,
            1 => self.preview_generation = 0,
            2 => self.time_seconds = 0.0,
            3 => self.usage = PreparedRenderUsage::default(),
            4 => self.limits = PreparedRenderLimits::default(),
            5 => {
                if self.abandonment_slot == u8::MAX {
                    self.retirement_phase = 6;
                    return false;
                }
                let slot = usize::from(self.abandonment_slot);
                let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return false };
                let current = state.load(Ordering::Acquire);
                if !matches!(current, 1 | 3) || state.compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
                    return false;
                }
                self.abandonment_slot = u8::MAX;
            }
            _ => return true,
        }
        self.retirement_phase += 1;
        false
    }

    pub fn retirement_is_empty(&self) -> bool {
        self.retirement_phase >= 6
            && self.uploads.is_empty()
            && self.evictions.is_empty()
            && self.draw.retirement_is_empty()
            && self.overlay.as_ref().is_none_or(DrawList::retirement_is_empty)
            && self.commands.terminal_is_empty()
            && self.uploads.terminal_is_empty()
            && self.evictions.terminal_is_empty()
            && self.damage.terminal_is_empty()
            && self.clips.terminal_is_empty()
            && self.directives.terminal_is_empty()
            && self.permit.is_none()
            && self.abandonment_slot == u8::MAX
    }
}

impl Drop for PreparedRenderPacket {
    fn drop(&mut self) {
        if self.retirement_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let packet = Box::new(Self {
            scene_revision: std::mem::take(&mut self.scene_revision),
            preview_generation: std::mem::take(&mut self.preview_generation),
            damage: std::mem::take(&mut self.damage),
            clips: std::mem::take(&mut self.clips),
            directives: std::mem::take(&mut self.directives),
            uploads: std::mem::take(&mut self.uploads),
            evictions: std::mem::take(&mut self.evictions),
            draw: std::mem::replace(&mut self.draw, DrawList::empty()),
            overlay: self.overlay.take(),
            commands: std::mem::take(&mut self.commands),
            time_seconds: std::mem::take(&mut self.time_seconds),
            usage: std::mem::take(&mut self.usage),
            limits: std::mem::take(&mut self.limits),
            permit: self.permit.take(),
            retirement_phase: self.retirement_phase,
            abandonment_slot: self.abandonment_slot,
        });
        self.retirement_phase = 6;
        self.abandonment_slot = u8::MAX;
        PREPARED_RENDER_PACKET_ABANDONMENT_OWNER[slot].store(Box::into_raw(packet), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}

/// 🧰️ Owned inputs consumed by the resumable preparation job.
pub struct PreparedRenderInput {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: PreparedRenderScissors,
    pub clips: PreparedRenderScissors,
    pub directives: PreparedRenderDirectives,
    pub uploads: PreparedRenderUploads,
    pub raster_producers: PreparedRasterProducers,
    pub evictions: PreparedRenderEvictions,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub limits: PreparedRenderLimits,
    permit: Option<PreparedRenderProcessPermit>,
    abandonment_slot: u8,
}

/// 🛡️ Exact input owner returned when the process/page reservation refuses admission.
pub struct PreparedRenderInputRejected {
    fault: &'static str,
    draw: Option<DrawList>,
    overlay: Option<DrawList>,
    permit: Option<PreparedRenderProcessPermit>,
}

impl PreparedRenderInputRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    pub fn take_draw(&mut self) -> Option<DrawList> {
        self.draw.take()
    }

    pub fn take_overlay(&mut self) -> Option<DrawList> {
        self.overlay.take()
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(draw) = self.draw.as_mut() {
            if !draw.retire_step() {
                return false;
            }
            self.draw = None;
            return false;
        }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retire_step() {
                return false;
            }
            self.overlay = None;
            return false;
        }
        if let Some(permit) = self.permit.as_mut() {
            if !permit.release_step() {
                return false;
            }
            self.permit = None;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.draw.is_none() && self.overlay.is_none() && self.permit.is_none()
    }
}

impl PreparedRenderInput {
    pub fn try_new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Result<Self, PreparedRenderInputRejected> {
        let limits = PreparedRenderLimits::default();
        let (draw_items, draw_bytes) = draw.prepared_output_usage();
        let (overlay_items, overlay_bytes) = overlay.as_ref().map_or((0, 0), DrawList::prepared_output_usage);
        let Some(items) = draw_items.checked_add(overlay_items) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render item claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(bytes) = draw_bytes.checked_add(overlay_bytes).and_then(|value| value.checked_add(limits.max_upload_bytes)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render byte claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(maximum_bytes) = limits.max_draw_bytes.checked_add(limits.max_upload_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render byte limit overflowed", draw: Some(draw), overlay, permit: None });
        };
        if scene_revision == 0 || scene_revision == u64::MAX || preview_generation == 0 || preview_generation == u64::MAX || items > limits.max_draw_items || bytes > maximum_bytes {
            return Err(PreparedRenderInputRejected { fault: "prepared render source exceeded fixed revision, item, or byte credits", draw: Some(draw), overlay, permit: None });
        }
        let Some(pages) = prepared_render_units(bytes).and_then(|pages| pages.checked_add(PREPARED_RENDER_COMMAND_PAGES)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render page claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let metadata = size_of::<Option<PreparedRenderUpload>>()
            .checked_mul(PREPARED_RENDER_METADATA_ITEMS)
            .and_then(|value| value.checked_add(size_of::<Option<PreparedRenderEviction>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<Option<ScissorRect>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS.checked_mul(2)?)?))
            .and_then(|value| value.checked_add(size_of::<Option<RenderDirective>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<Option<PreparedRasterProducer>>().checked_mul(PREPARED_RENDER_METADATA_ITEMS)?))
            .and_then(|value| value.checked_add(size_of::<PreparedRenderCommandDirectory>().checked_mul(PREPARED_RENDER_COMMAND_DIRECTORIES)?))
            .and_then(|value| value.checked_add(size_of::<PreparedRenderCommandPage>().checked_mul(PREPARED_RENDER_COMMAND_PAGES)?));
        let Some(backing_bytes) = metadata.and_then(|metadata| bytes.checked_add(metadata)) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render backing claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(permit) = PreparedRenderProcessPermit::try_reserve(pages, backing_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render process permits exhausted", draw: Some(draw), overlay, permit: None });
        };
        let slot = usize::from(permit.slot);
        if PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(PreparedRenderInputRejected { fault: "prepared render input abandonment slot was occupied", draw: Some(draw), overlay, permit: Some(permit) });
        }
        let mut directives = PreparedRenderDirectives::default();
        if directives.try_push(RenderDirective::PreservePreviousOnFailure).is_err() {
            PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
            return Err(PreparedRenderInputRejected { fault: "prepared render directive admission failed", draw: Some(draw), overlay, permit: Some(permit) });
        }
        Ok(Self {
            scene_revision,
            preview_generation,
            damage: PreparedRenderScissors::default(),
            clips: PreparedRenderScissors::default(),
            directives,
            uploads: PreparedRenderUploads::default(),
            raster_producers: PreparedRasterProducers::default(),
            evictions: PreparedRenderEvictions::default(),
            draw,
            overlay,
            time_seconds,
            limits,
            permit: Some(permit),
            abandonment_slot: slot as u8,
        })
    }

    #[cfg(test)]
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        match Self::try_new(scene_revision, preview_generation, draw, overlay, time_seconds) {
            Ok(input) => input,
            Err(_) => panic!("test prepared input must fit fixed process permits"),
        }
    }

    pub fn try_push_upload(&mut self, upload: PreparedRenderUpload) -> Result<(), PreparedRenderUpload> {
        if self.raster_producers.len().checked_add(self.uploads.len()).is_none_or(|items| items >= self.limits.max_upload_items) {
            return Err(upload);
        }
        self.uploads.try_push(upload)
    }

    pub fn try_push_raster_producer(&mut self, producer: PreparedRasterProducer) -> Result<(), PreparedRasterProducer> {
        if self.raster_producers.len().checked_add(self.uploads.len()).is_none_or(|items| items >= self.limits.max_upload_items) {
            return Err(producer);
        }
        self.raster_producers.try_push(producer)
    }

    pub fn try_push_eviction(&mut self, eviction: PreparedRenderEviction) -> Result<(), PreparedRenderEviction> {
        self.evictions.try_push(eviction)
    }

    /// 🧱 Admits and transfers the final retained draw owners before worker submission.
    pub fn try_bind_draw(&mut self, draw: DrawList, overlay: Option<DrawList>) -> Result<(), PreparedRenderInputRejected> {
        if !self.draw.retirement_is_empty() || self.overlay.is_some() {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw owner was already bound", draw: Some(draw), overlay, permit: None });
        }
        let (draw_items, draw_bytes) = draw.prepared_output_usage();
        let (overlay_items, overlay_bytes) = overlay.as_ref().map_or((0, 0), DrawList::prepared_output_usage);
        let Some(items) = draw_items.checked_add(overlay_items) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw item claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        let Some(bytes) = draw_bytes.checked_add(overlay_bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw byte claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        if items > self.limits.max_draw_items || bytes > self.limits.max_draw_bytes {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw owner exceeded fixed credits", draw: Some(draw), overlay, permit: None });
        }
        let Some(pages) = prepared_render_units(bytes) else {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw page claim overflowed", draw: Some(draw), overlay, permit: None });
        };
        if !self.permit.as_mut().is_some_and(|permit| permit.try_grow(pages, bytes)) {
            return Err(PreparedRenderInputRejected { fault: "prepared render draw process credits were refused", draw: Some(draw), overlay, permit: None });
        }
        self.draw = draw;
        self.overlay = overlay;
        Ok(())
    }

    fn close_step(&mut self) -> bool {
        if let Some(upload) = self.uploads.last_mut() {
            let retained = match upload {
                #[cfg(test)]
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => pixels.pop().is_some(),
                PreparedRenderUpload::GlyphAtlasPages { pixels } | PreparedRenderUpload::IconAtlasPages { pixels } => !pixels.close_step(),
                #[cfg(test)]
                PreparedRenderUpload::Raster { key, pixels, .. } => pixels.pop().is_some() || key.pop().is_some(),
                PreparedRenderUpload::RasterPages { key, pixels } => !pixels.retire_with_key_step(key),
                PreparedRenderUpload::Mesh { key, .. } => key.pop().is_some(),
            };
            if retained {
                return false;
            }
            self.uploads.pop();
            return false;
        }
        if let Some(producer) = self.raster_producers.last_mut() {
            producer.begin_close();
            if !producer.close_step() {
                return false;
            }
            self.raster_producers.pop();
            return false;
        }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if key.pop().is_some() {
                return false;
            }
            self.evictions.pop();
            return false;
        }
        if !self.draw.retire_step() {
            return false;
        }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retire_step() {
                return false;
            }
            self.overlay = None;
            return false;
        }
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() {
            return false;
        }
        if !self.uploads.release_backing_step()
            || !self.raster_producers.release_backing_step()
            || !self.evictions.release_backing_step()
            || !self.damage.release_backing_step()
            || !self.clips.release_backing_step()
            || !self.directives.release_backing_step()
        {
            return false;
        }
        if let Some(permit) = self.permit.as_mut() {
            if !permit.release_step() {
                return false;
            }
            self.permit = None;
            return false;
        }
        if self.abandonment_slot != u8::MAX {
            let slot = usize::from(self.abandonment_slot);
            let Some(state) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.get(slot) else { return false };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return false;
            }
            self.abandonment_slot = u8::MAX;
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.uploads.terminal_is_empty()
            && self.raster_producers.terminal_is_empty()
            && self.evictions.terminal_is_empty()
            && self.damage.terminal_is_empty()
            && self.clips.terminal_is_empty()
            && self.directives.terminal_is_empty()
            && self.draw.retirement_is_empty()
            && self.overlay.is_none()
            && self.permit.is_none()
            && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact pre-submission input owner recovered after interruption.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
            return false;
        }
        let mut input = unsafe { Box::from_raw(pointer) };
        if input.close_step() || input.abandonment_slot == u8::MAX {
            drop(input);
        } else {
            PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].store(Box::into_raw(input), Ordering::Release);
            PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        }
        false
    }
}

impl Drop for PreparedRenderInput {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let input = Box::new(Self {
            scene_revision: std::mem::take(&mut self.scene_revision),
            preview_generation: std::mem::take(&mut self.preview_generation),
            damage: std::mem::take(&mut self.damage),
            clips: std::mem::take(&mut self.clips),
            directives: std::mem::take(&mut self.directives),
            uploads: std::mem::take(&mut self.uploads),
            raster_producers: std::mem::take(&mut self.raster_producers),
            evictions: std::mem::take(&mut self.evictions),
            draw: std::mem::replace(&mut self.draw, DrawList::empty()),
            overlay: self.overlay.take(),
            time_seconds: std::mem::take(&mut self.time_seconds),
            limits: std::mem::take(&mut self.limits),
            permit: self.permit.take(),
            abandonment_slot: self.abandonment_slot,
        });
        self.abandonment_slot = u8::MAX;
        PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].store(Box::into_raw(input), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}
//#endregion 📦️Packet

//#region ⚙️PreparationJob
const PREPARED_RENDER_MAILBOX_SLOTS: usize = 64;

struct PreparedRenderMailboxSlot {
    state: AtomicU8,
    generation: AtomicU64,
    references: AtomicUsize,
    packet: AtomicPtr<PreparedRenderPacket>,
}

impl PreparedRenderMailboxSlot {
    const fn new() -> Self {
        Self { state: AtomicU8::new(0), generation: AtomicU64::new(0), references: AtomicUsize::new(0), packet: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

static PREPARED_RENDER_MAILBOX: [PreparedRenderMailboxSlot; PREPARED_RENDER_MAILBOX_SLOTS] = [const { PreparedRenderMailboxSlot::new() }; PREPARED_RENDER_MAILBOX_SLOTS];

/// 📬️ Generation-qualified nonblocking capacity-one packet handoff.
pub struct PreparedRenderReceiver {
    slot: u8,
    generation: u64,
    owned: bool,
}

impl PreparedRenderReceiver {
    fn unowned() -> Self {
        Self { slot: u8::MAX, generation: 0, owned: false }
    }

    fn try_reserve() -> Option<Self> {
        let slot = PREPARED_RENDER_MAILBOX.iter().position(|slot| slot.state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok())?;
        let current = PREPARED_RENDER_MAILBOX[slot].generation.load(Ordering::Acquire);
        let Some(generation) = current.checked_add(1) else {
            PREPARED_RENDER_MAILBOX[slot].state.store(0, Ordering::Release);
            return None;
        };
        if PREPARED_RENDER_MAILBOX[slot].generation.compare_exchange(current, generation, Ordering::AcqRel, Ordering::Acquire).is_err() {
            PREPARED_RENDER_MAILBOX[slot].state.store(0, Ordering::Release);
            return None;
        }
        PREPARED_RENDER_MAILBOX[slot].references.store(1, Ordering::Release);
        Some(Self { slot: slot as u8, generation, owned: true })
    }

    pub fn try_clone(&self) -> Option<Self> {
        let slot = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot))?;
        if !self.owned || slot.generation.load(Ordering::Acquire) != self.generation || slot.state.load(Ordering::Acquire) == 0 {
            return None;
        }
        let references = slot.references.load(Ordering::Acquire);
        let next = references.checked_add(1)?;
        slot.references.compare_exchange(references, next, Ordering::AcqRel, Ordering::Acquire).ok()?;
        Some(Self { slot: self.slot, generation: self.generation, owned: true })
    }

    pub fn take_latest(&self) -> Option<PreparedRenderPacket> {
        let slot = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot))?;
        if slot.generation.load(Ordering::Acquire) != self.generation || slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return None;
        }
        let pointer = slot.packet.swap(std::ptr::null_mut(), Ordering::AcqRel);
        slot.state.store(1, Ordering::Release);
        if pointer.is_null() {
            return None;
        }
        Some(*unsafe { Box::from_raw(pointer) })
    }

    fn publish(&self, packet: PreparedRenderPacket) -> Result<(), PreparedRenderPacket> {
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return Err(packet) };
        if slot.generation.load(Ordering::Acquire) != self.generation || slot.state.load(Ordering::Acquire) != 1 {
            return Err(packet);
        }
        let pointer = Box::into_raw(Box::new(packet));
        if slot.packet.compare_exchange(std::ptr::null_mut(), pointer, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(*unsafe { Box::from_raw(pointer) });
        }
        slot.state.store(2, Ordering::Release);
        Ok(())
    }

    fn close_step(&self) -> bool {
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return true };
        if slot.generation.load(Ordering::Acquire) != self.generation {
            return true;
        }
        if slot.state.load(Ordering::Acquire) == 1 {
            return true;
        }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let pointer = slot.packet.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            slot.state.store(1, Ordering::Release);
            return false;
        }
        let mut packet = unsafe { Box::from_raw(pointer) };
        if packet.retire_step() {
            drop(packet);
            slot.state.store(1, Ordering::Release);
        } else {
            slot.packet.store(Box::into_raw(packet), Ordering::Release);
            slot.state.store(2, Ordering::Release);
        }
        false
    }

    fn terminal_is_empty(&self) -> bool {
        PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)).is_none_or(|slot| slot.generation.load(Ordering::Acquire) != self.generation || slot.packet.load(Ordering::Acquire).is_null())
    }

    /// 🧹 Advances one owner from one mailbox abandoned by its final handle.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_RENDER_MAILBOX.iter().find(|slot| slot.state.compare_exchange(4, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = slot.packet.swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            slot.state.store(0, Ordering::Release);
            return false;
        }
        let mut packet = unsafe { Box::from_raw(pointer) };
        if packet.retire_step() {
            drop(packet);
            slot.state.store(0, Ordering::Release);
        } else {
            slot.packet.store(Box::into_raw(packet), Ordering::Release);
            slot.state.store(4, Ordering::Release);
        }
        false
    }
}

impl Drop for PreparedRenderReceiver {
    fn drop(&mut self) {
        if !self.owned {
            return;
        }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return };
        if slot.generation.load(Ordering::Acquire) != self.generation {
            self.owned = false;
            return;
        }
        let references = slot.references.fetch_sub(1, Ordering::AcqRel);
        if references == 1 {
            if slot.packet.load(Ordering::Acquire).is_null() {
                slot.state.store(0, Ordering::Release);
            } else {
                slot.state.store(4, Ordering::Release);
            }
        }
        self.owned = false;
    }
}

/// ⚙️ Bounded worker job that measures and seals one owned render packet.
pub struct PreparedRenderJob {
    input: Option<PreparedRenderInput>,
    usage: PreparedRenderUsage,
    section: PreparationSection,
    draw_cursor: DrawMeasureCursor,
    overlay_cursor: DrawMeasureCursor,
    pipeline_cursor: DrawMeasureCursor,
    pipeline_overlay_cursor: DrawMeasureCursor,
    pipeline_overlay: bool,
    metadata_cursor: usize,
    commands: Option<PreparedRenderCommandPages>,
    digest: u64,
    fault: Option<&'static str>,
    receiver: PreparedRenderReceiver,
    rejected_upload: Option<PreparedRenderUpload>,
    rejected_packet: Option<PreparedRenderPacket>,
    raster_backing_closed: bool,
    packet_command_staged: bool,
    published: bool,
    closing: bool,
    abandonment_slot: u8,
}

#[derive(Clone, Copy)]
enum PreparationSection {
    Draw,
    Overlay,
    Uploads,
    Evictions,
    Damage,
    Clips,
    Directives,
    Validate,
    Snap,
    Order,
    Tessellate,
    Batch,
    Hash,
    Complete,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DrawMeasureCursor {
    LayerHeader(usize),
    LayerUi { layer: usize, item: usize, overlay: bool },
    LayerVector { layer: usize, item: usize, overlay: bool },
    LayerRaster { layer: usize, raster: usize },
    LayerRasterKey { layer: usize, raster: usize, byte: usize },
    PassHeader(usize),
    PassDraw { pass: usize, draw: usize, translucent: bool },
    PassDrawKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassInstanceKey { pass: usize, draw: usize, instance: usize, byte: usize, translucent: bool },
    PassLine { pass: usize, draw: usize },
    PassLineVertex { pass: usize, draw: usize, vertex: usize },
    PassTextured { pass: usize, draw: usize },
    PassTexturedInstance { pass: usize, draw: usize, instance: usize },
    PassTexturedKey { pass: usize, draw: usize, instance: usize, byte: usize },
    Glass(usize),
    Complete,
}

impl Default for DrawMeasureCursor {
    fn default() -> Self {
        Self::LayerHeader(0)
    }
}

/// 🛡️ Exact input returned when the fixed packet mailbox refuses a job.
pub struct PreparedRenderJobRejected {
    fault: &'static str,
    input: Option<PreparedRenderInput>,
}

impl PreparedRenderJobRejected {
    pub fn fault(&self) -> &'static str {
        self.fault
    }

    pub fn take_rejected(&mut self) -> Option<PreparedRenderInput> {
        self.input.take()
    }

    pub fn close_step(&mut self) -> bool {
        let Some(input) = self.input.as_mut() else { return true };
        if !input.close_step() {
            return false;
        }
        self.input = None;
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none()
    }
}

impl PreparedRenderJob {
    pub fn try_new(mut input: PreparedRenderInput) -> Result<Self, PreparedRenderJobRejected> {
        let Some(receiver) = PreparedRenderReceiver::try_reserve() else {
            return Err(PreparedRenderJobRejected { fault: "prepared render packet mailbox exhausted", input: Some(input) });
        };
        let Some(slot) = input.permit.as_ref().map(|permit| usize::from(permit.slot)) else {
            return Err(PreparedRenderJobRejected { fault: "prepared render process owner was missing", input: Some(input) });
        };
        let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(slot) else {
            return Err(PreparedRenderJobRejected { fault: "prepared render abandonment slot was missing", input: Some(input) });
        };
        if state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(PreparedRenderJobRejected { fault: "prepared render abandonment slot was occupied", input: Some(input) });
        }
        let input_slot = usize::from(input.abandonment_slot);
        if input_slot != slot || PREPARED_RENDER_INPUT_ABANDONMENT_STATE[input_slot].compare_exchange(1, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
            state.store(0, Ordering::Release);
            return Err(PreparedRenderJobRejected { fault: "prepared render input handoff generation was stale", input: Some(input) });
        }
        input.abandonment_slot = u8::MAX;
        Ok(Self::from_admitted(input, receiver, slot as u8))
    }

    fn from_admitted(input: PreparedRenderInput, receiver: PreparedRenderReceiver, abandonment_slot: u8) -> Self {
        Self {
            input: Some(input),
            usage: PreparedRenderUsage::default(),
            section: PreparationSection::Draw,
            draw_cursor: DrawMeasureCursor::default(),
            overlay_cursor: DrawMeasureCursor::default(),
            pipeline_cursor: DrawMeasureCursor::default(),
            pipeline_overlay_cursor: DrawMeasureCursor::default(),
            pipeline_overlay: false,
            metadata_cursor: 0,
            commands: Some(PreparedRenderCommandPages::default()),
            digest: 0xcbf29ce484222325,
            fault: None,
            receiver,
            rejected_upload: None,
            rejected_packet: None,
            raster_backing_closed: false,
            packet_command_staged: false,
            published: false,
            closing: false,
            abandonment_slot,
        }
    }

    #[cfg(test)]
    pub fn new(input: PreparedRenderInput, _items_per_step: usize) -> Self {
        match Self::try_new(input) {
            Ok(job) => job,
            Err(_) => panic!("test prepared render mailbox must admit one job"),
        }
    }

    pub fn receiver(&self) -> Option<PreparedRenderReceiver> {
        self.receiver.try_clone()
    }

    pub fn take_packet(&self) -> Option<PreparedRenderPacket> {
        self.receiver.take_latest()
    }

    pub fn close_step(&mut self) -> bool {
        if let Some(upload) = self.rejected_upload.as_mut() {
            let retained = match upload {
                #[cfg(test)]
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => pixels.pop().is_some(),
                PreparedRenderUpload::GlyphAtlasPages { pixels } | PreparedRenderUpload::IconAtlasPages { pixels } => !pixels.close_step(),
                #[cfg(test)]
                PreparedRenderUpload::Raster { key, pixels, .. } => pixels.pop().is_some() || key.pop().is_some(),
                PreparedRenderUpload::RasterPages { key, pixels } => !pixels.retire_with_key_step(key),
                PreparedRenderUpload::Mesh { key, .. } => key.pop().is_some(),
            };
            if retained {
                return false;
            }
            self.rejected_upload = None;
            return false;
        }
        if let Some(packet) = self.rejected_packet.as_mut() {
            if !packet.retire_step() {
                return false;
            }
            self.rejected_packet = None;
            return false;
        }
        if let Some(commands) = self.commands.as_mut() {
            if !commands.close_step() {
                return false;
            }
            self.commands = None;
            return false;
        }
        if let Some(input) = self.input.as_mut() {
            if !input.close_step() {
                return false;
            }
            self.input = None;
            return false;
        }
        if !self.receiver.close_step() {
            return false;
        }
        if self.abandonment_slot != u8::MAX {
            let slot = usize::from(self.abandonment_slot);
            let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(slot) else { return false };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, 0, Ordering::AcqRel, Ordering::Acquire).is_err() {
                return false;
            }
            self.abandonment_slot = u8::MAX;
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.commands.is_none() && self.rejected_upload.is_none() && self.rejected_packet.is_none() && self.receiver.terminal_is_empty() && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact job owner recovered after an interrupted worker execution.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
            return false;
        }
        let mut job = unsafe { Box::from_raw(pointer) };
        if job.close_step() || job.abandonment_slot == u8::MAX {
            drop(job);
        } else {
            PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].store(Box::into_raw(job), Ordering::Release);
            PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        }
        false
    }

    fn input(&self) -> Option<&PreparedRenderInput> {
        self.input.as_ref()
    }

    fn next_draw_usage(draw: &DrawList, cursor: &mut DrawMeasureCursor) -> Option<PreparedRenderUsage> {
        let (usage, next) = match *cursor {
            DrawMeasureCursor::LayerHeader(layer) => {
                let Some(value) = draw.layers.get(layer) else {
                    *cursor = DrawMeasureCursor::PassHeader(0);
                    return Some(PreparedRenderUsage::default());
                };
                let next = if !value.ui_instances.is_empty() {
                    DrawMeasureCursor::LayerUi { layer, item: 0, overlay: false }
                } else if !value.overlay_ui_instances.is_empty() {
                    DrawMeasureCursor::LayerUi { layer, item: 0, overlay: true }
                } else if !value.vector_vertices.is_empty() {
                    DrawMeasureCursor::LayerVector { layer, item: 0, overlay: false }
                } else if !value.overlay_vector_vertices.is_empty() {
                    DrawMeasureCursor::LayerVector { layer, item: 0, overlay: true }
                } else if !value.raster_instances.is_empty() {
                    DrawMeasureCursor::LayerRaster { layer, raster: 0 }
                } else {
                    DrawMeasureCursor::LayerHeader(layer + 1)
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<DrawLayer>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_ui_instances } else { &value.ui_instances };
                let next = if item + 1 < items.len() {
                    DrawMeasureCursor::LayerUi { layer, item: item + 1, overlay }
                } else if !overlay && !value.overlay_ui_instances.is_empty() {
                    DrawMeasureCursor::LayerUi { layer, item: 0, overlay: true }
                } else if !value.vector_vertices.is_empty() {
                    DrawMeasureCursor::LayerVector { layer, item: 0, overlay: false }
                } else if !value.overlay_vector_vertices.is_empty() {
                    DrawMeasureCursor::LayerVector { layer, item: 0, overlay: true }
                } else if !value.raster_instances.is_empty() {
                    DrawMeasureCursor::LayerRaster { layer, raster: 0 }
                } else {
                    DrawMeasureCursor::LayerHeader(layer + 1)
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::UiInstance>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_vector_vertices } else { &value.vector_vertices };
                let next = if item + 1 < items.len() {
                    DrawMeasureCursor::LayerVector { layer, item: item + 1, overlay }
                } else if !overlay && !value.overlay_vector_vertices.is_empty() {
                    DrawMeasureCursor::LayerVector { layer, item: 0, overlay: true }
                } else if !value.raster_instances.is_empty() {
                    DrawMeasureCursor::LayerRaster { layer, raster: 0 }
                } else {
                    DrawMeasureCursor::LayerHeader(layer + 1)
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::VectorVertex>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerRaster { layer, raster } => {
                let value = &draw.layers[layer].raster_instances[raster];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::UiInstance>(), ..PreparedRenderUsage::default() };
                let next = if value.0.is_empty() { Self::next_layer_raster(draw, layer, raster) } else { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: 0 } };
                (usage, next)
            }
            DrawMeasureCursor::LayerRasterKey { layer, raster, byte } => {
                let key = &draw.layers[layer].raster_instances[raster].0;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: byte + 1 } } else { Self::next_layer_raster(draw, layer, raster) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassHeader(pass) => {
                let Some(value) = draw.scene_passes.get(pass) else {
                    *cursor = DrawMeasureCursor::Glass(0);
                    return Some(PreparedRenderUsage::default());
                };
                let next = if value.draws.is_empty() { Self::next_after_opaque(draw, pass) } else { DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::ScenePass3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassDraw { pass, draw: draw_index, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::SceneDraw3d>(), ..PreparedRenderUsage::default() };
                let next = if !value.mesh_key.is_empty() {
                    DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte: 0, translucent }
                } else if value.instances.is_empty() {
                    Self::next_pass_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (usage, next)
            }
            DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let next = if byte + 1 < value.mesh_key.len() {
                    DrawMeasureCursor::PassDrawKey { pass, draw: draw_index, byte: byte + 1, translucent }
                } else if value.instances.is_empty() {
                    Self::next_pass_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::Instance3d>(), ..PreparedRenderUsage::default() };
                let next = if value.id.is_empty() { Self::next_pass_instance(draw, pass, draw_index, instance, translucent) } else { DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte: 0, translucent } };
                (usage, next)
            }
            DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let key = &draws[draw_index].instances[instance].id;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassInstanceKey { pass, draw: draw_index, instance, byte: byte + 1, translucent } } else { Self::next_pass_instance(draw, pass, draw_index, instance, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassLine { pass, draw: draw_index } => {
                let pass_value = &draw.scene_passes[pass];
                let next = if pass_value.line_draws[draw_index].vertices.is_empty() { Self::next_pass_line(draw, pass, draw_index) } else { DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex: 0 } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::LineDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } => {
                let next = if vertex + 1 < draw.scene_passes[pass].line_draws[draw_index].vertices.len() { DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex: vertex + 1 } } else { Self::next_pass_line(draw, pass, draw_index) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::LineVertex3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTextured { pass, draw: draw_index } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index];
                let next = if value.instances.is_empty() { Self::next_textured_draw(draw, pass, draw_index) } else { DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: 0 } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::TexturedDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::TexturedInstance3d>(), ..PreparedRenderUsage::default() };
                let next = if value.texture_key.is_empty() { Self::next_textured_instance(draw, pass, draw_index, instance) } else { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: 0 } };
                (usage, next)
            }
            DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte } => {
                let key = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance].texture_key;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: byte + 1 } } else { Self::next_textured_instance(draw, pass, draw_index, instance) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::Glass(index) => {
                if index >= draw.glass_regions.len() {
                    *cursor = DrawMeasureCursor::Complete;
                    return Some(PreparedRenderUsage::default());
                }
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::GlassRegion>(), ..PreparedRenderUsage::default() }, DrawMeasureCursor::Glass(index + 1))
            }
            DrawMeasureCursor::Complete => return None,
        };
        *cursor = next;
        Some(usage)
    }

    fn next_layer_raster(draw: &DrawList, layer: usize, raster: usize) -> DrawMeasureCursor {
        if raster + 1 < draw.layers[layer].raster_instances.len() {
            DrawMeasureCursor::LayerRaster { layer, raster: raster + 1 }
        } else {
            DrawMeasureCursor::LayerHeader(layer + 1)
        }
    }

    fn next_pass_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize, translucent: bool) -> DrawMeasureCursor {
        let pass_value = &draw.scene_passes[pass];
        let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
        if instance + 1 < draws[draw_index].instances.len() {
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: instance + 1, translucent }
        } else {
            Self::next_pass_draw(draw, pass, draw_index, translucent)
        }
    }

    fn next_pass_line(draw: &DrawList, pass: usize, draw_index: usize) -> DrawMeasureCursor {
        if draw_index + 1 < draw.scene_passes[pass].line_draws.len() {
            DrawMeasureCursor::PassLine { pass, draw: draw_index + 1 }
        } else {
            Self::next_after_lines(draw, pass)
        }
    }

    fn next_textured_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize) -> DrawMeasureCursor {
        if instance + 1 < draw.scene_passes[pass].textured_draws[draw_index].instances.len() {
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: instance + 1 }
        } else {
            Self::next_textured_draw(draw, pass, draw_index)
        }
    }

    fn next_pass_draw(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        let pass_value = &draw.scene_passes[pass];
        let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
        if draw_index + 1 < draws.len() {
            DrawMeasureCursor::PassDraw { pass, draw: draw_index + 1, translucent }
        } else if translucent {
            Self::next_after_translucent(draw, pass)
        } else {
            Self::next_after_opaque(draw, pass)
        }
    }

    fn next_after_opaque(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if !value.line_draws.is_empty() {
            DrawMeasureCursor::PassLine { pass, draw: 0 }
        } else {
            Self::next_after_lines(draw, pass)
        }
    }

    fn next_after_lines(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if !value.translucent_draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: true }
        } else {
            Self::next_after_translucent(draw, pass)
        }
    }

    fn next_after_translucent(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if !draw.scene_passes[pass].textured_draws.is_empty() {
            DrawMeasureCursor::PassTextured { pass, draw: 0 }
        } else {
            DrawMeasureCursor::PassHeader(pass + 1)
        }
    }

    fn next_textured_draw(draw: &DrawList, pass: usize, draw_index: usize) -> DrawMeasureCursor {
        if draw_index + 1 < draw.scene_passes[pass].textured_draws.len() {
            DrawMeasureCursor::PassTextured { pass, draw: draw_index + 1 }
        } else {
            DrawMeasureCursor::PassHeader(pass + 1)
        }
    }

    fn measure_next(&mut self) -> Option<PreparedRenderUsage> {
        match self.section {
            PreparationSection::Draw => {
                let input = self.input.as_ref()?;
                if let Some(usage) = Self::next_draw_usage(&input.draw, &mut self.draw_cursor) {
                    return Some(usage);
                }
                self.section = PreparationSection::Overlay;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Overlay => {
                let input = self.input.as_ref()?;
                if let Some(overlay) = &input.overlay {
                    if let Some(usage) = Self::next_draw_usage(overlay, &mut self.overlay_cursor) {
                        return Some(usage);
                    }
                }
                self.section = PreparationSection::Uploads;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Uploads => {
                let input = self.input.as_ref()?;
                if let Some(upload) = input.uploads.get(self.metadata_cursor) {
                    let source = self.metadata_cursor;
                    let Some(next) = self.metadata_cursor.checked_add(1) else {
                        self.fault = Some("prepared upload cursor exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let Some(bytes) = upload.byte_len() else {
                        self.fault = Some("prepared upload byte claim overflowed");
                        return Some(PreparedRenderUsage::default());
                    };
                    self.metadata_cursor = next;
                    let Some(digest) = u64::try_from(bytes).ok() else {
                        self.fault = Some("prepared upload digest exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Upload, source, digest, draw_cursor: None, packet_overlay: false };
                    let Some(commands) = self.commands.as_mut() else {
                        self.fault = Some("prepared render command owner was missing");
                        return Some(PreparedRenderUsage::default());
                    };
                    if commands.try_push(command).is_err() {
                        self.fault = Some("prepared render command page credits exhausted");
                        return Some(PreparedRenderUsage::default());
                    }
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: bytes, ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Evictions;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Evictions => {
                let input = self.input.as_ref()?;
                if let Some(eviction) = input.evictions.get(self.metadata_cursor) {
                    let Some(next) = self.metadata_cursor.checked_add(1) else {
                        self.fault = Some("prepared eviction cursor exhausted");
                        return Some(PreparedRenderUsage::default());
                    };
                    let Some(bytes) = eviction.byte_len() else {
                        self.fault = Some("prepared eviction byte claim overflowed");
                        return Some(PreparedRenderUsage::default());
                    };
                    self.metadata_cursor = next;
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: bytes, ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Damage;
                self.metadata_cursor = 0;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Damage => self.measure_metadata(PreparationSection::Clips, |input| input.damage.len(), size_of::<ScissorRect>()),
            PreparationSection::Clips => self.measure_metadata(PreparationSection::Directives, |input| input.clips.len(), size_of::<ScissorRect>()),
            PreparationSection::Directives => self.measure_metadata(PreparationSection::Validate, |input| input.directives.len(), size_of::<RenderDirective>()),
            PreparationSection::Validate | PreparationSection::Snap | PreparationSection::Order | PreparationSection::Tessellate | PreparationSection::Batch | PreparationSection::Hash => self.advance_pipeline(),
            PreparationSection::Complete => None,
        }
    }

    fn measure_metadata(&mut self, next: PreparationSection, len: impl FnOnce(&PreparedRenderInput) -> usize, bytes: usize) -> Option<PreparedRenderUsage> {
        if self.metadata_cursor < len(self.input()?) {
            let Some(next) = self.metadata_cursor.checked_add(1) else {
                self.fault = Some("prepared metadata cursor exhausted");
                return Some(PreparedRenderUsage::default());
            };
            self.metadata_cursor = next;
            Some(PreparedRenderUsage { draw_items: 1, draw_bytes: bytes, ..PreparedRenderUsage::default() })
        } else {
            self.section = next;
            self.metadata_cursor = 0;
            Some(PreparedRenderUsage::default())
        }
    }

    fn advance_pipeline(&mut self) -> Option<PreparedRenderUsage> {
        let prepared_cursor = if self.pipeline_overlay { self.pipeline_overlay_cursor } else { self.pipeline_cursor };
        let usage = if self.pipeline_overlay {
            let input = self.input.as_ref()?;
            match input.overlay.as_ref() {
                Some(overlay) => Self::next_draw_usage(overlay, &mut self.pipeline_overlay_cursor),
                None => None,
            }
        } else {
            let input = self.input.as_ref()?;
            Self::next_draw_usage(&input.draw, &mut self.pipeline_cursor)
        };
        if let Some(usage) = usage {
            let source = self.metadata_cursor;
            let Some(next) = self.metadata_cursor.checked_add(1) else {
                self.fault = Some("prepared pipeline cursor exhausted");
                return Some(PreparedRenderUsage::default());
            };
            let Some(items) = u64::try_from(usage.draw_items).ok() else {
                self.fault = Some("prepared pipeline item digest exhausted");
                return Some(PreparedRenderUsage::default());
            };
            let Some(bytes) = u64::try_from(usage.draw_bytes).ok() else {
                self.fault = Some("prepared pipeline byte digest exhausted");
                return Some(PreparedRenderUsage::default());
            };
            self.metadata_cursor = next;
            self.digest = self.digest.rotate_left(7) ^ items ^ bytes.rotate_left(17);
            if matches!(self.section, PreparationSection::Tessellate) {
                let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: self.digest, draw_cursor: Some(prepared_cursor), packet_overlay: self.pipeline_overlay };
                let Some(commands) = self.commands.as_mut() else {
                    self.fault = Some("prepared render command owner was missing");
                    return Some(PreparedRenderUsage::default());
                };
                if commands.try_push(command).is_err() {
                    self.fault = Some("prepared render command page credits exhausted");
                    return Some(PreparedRenderUsage::default());
                }
            }
            return Some(PreparedRenderUsage::default());
        }
        if !self.pipeline_overlay {
            self.pipeline_overlay = true;
            self.metadata_cursor = 0;
            return Some(PreparedRenderUsage::default());
        }
        self.pipeline_overlay = false;
        self.pipeline_cursor = DrawMeasureCursor::default();
        self.pipeline_overlay_cursor = DrawMeasureCursor::default();
        self.metadata_cursor = 0;
        self.section = match self.section {
            PreparationSection::Validate => PreparationSection::Snap,
            PreparationSection::Snap => PreparationSection::Order,
            PreparationSection::Order => PreparationSection::Tessellate,
            PreparationSection::Tessellate => PreparationSection::Batch,
            PreparationSection::Batch => PreparationSection::Hash,
            PreparationSection::Hash => PreparationSection::Complete,
            _ => return None,
        };
        Some(PreparedRenderUsage::default())
    }

    fn include_usage(&mut self, usage: PreparedRenderUsage) -> bool {
        self.usage.include_draw(usage.draw_items, usage.draw_bytes) && (usage.upload_items == 0 || self.usage.include_upload(usage.upload_bytes))
    }

    fn fault_outcome(&mut self, fault: &'static str) -> StepOutcome {
        self.fault = Some(fault);
        self.closing = true;
        StepOutcome::Fault(JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
    }

    fn complete(&mut self) -> StepOutcome {
        if !self.packet_command_staged {
            let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Packet, source: 0, digest: self.digest, draw_cursor: None, packet_overlay: false };
            let Some(commands) = self.commands.as_mut() else { return self.fault_outcome("prepared render command owner was missing") };
            if commands.try_push(command).is_err() {
                return self.fault_outcome("prepared render packet command credits exhausted");
            }
            self.packet_command_staged = true;
            return StepOutcome::Yield;
        }
        if !self.published {
            let Some(input) = self.input.take() else { return self.fault_outcome("prepared render input was missing before publication") };
            if !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
                self.input = Some(input);
                return self.fault_outcome("prepared render process generation was stale before publication");
            }
            let revision = input.scene_revision;
            let generation = input.preview_generation;
            let Some(commands) = self.commands.take() else {
                self.input = Some(input);
                return self.fault_outcome("prepared render command pages were missing before publication");
            };
            let input = std::mem::ManuallyDrop::new(input);
            let packet = PreparedRenderPacket {
                scene_revision: revision,
                preview_generation: generation,
                damage: unsafe { std::ptr::read(&input.damage) },
                clips: unsafe { std::ptr::read(&input.clips) },
                directives: unsafe { std::ptr::read(&input.directives) },
                uploads: unsafe { std::ptr::read(&input.uploads) },
                evictions: unsafe { std::ptr::read(&input.evictions) },
                draw: unsafe { std::ptr::read(&input.draw) },
                overlay: unsafe { std::ptr::read(&input.overlay) },
                commands,
                time_seconds: input.time_seconds,
                usage: self.usage,
                limits: input.limits,
                permit: unsafe { std::ptr::read(&input.permit) },
                retirement_phase: 0,
                abandonment_slot: u8::MAX,
            };
            let packet = match packet.try_arm_abandonment() {
                Ok(packet) => packet,
                Err(packet) => {
                    self.rejected_packet = Some(packet);
                    return self.fault_outcome("prepared render packet abandonment admission was refused");
                }
            };
            if let Err(packet) = self.receiver.publish(packet) {
                self.rejected_packet = Some(packet);
                return self.fault_outcome("prepared render packet mailbox publication refused");
            }
            self.published = true;
            return StepOutcome::Yield;
        }
        StepOutcome::Complete(CommitCandidate {
            state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
            output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
        })
    }
}

impl Drop for PreparedRenderJob {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let job = Box::new(Self {
            input: self.input.take(),
            usage: std::mem::take(&mut self.usage),
            section: self.section,
            draw_cursor: self.draw_cursor,
            overlay_cursor: self.overlay_cursor,
            pipeline_cursor: self.pipeline_cursor,
            pipeline_overlay_cursor: self.pipeline_overlay_cursor,
            pipeline_overlay: self.pipeline_overlay,
            metadata_cursor: self.metadata_cursor,
            commands: self.commands.take(),
            digest: self.digest,
            fault: self.fault.take(),
            receiver: std::mem::replace(&mut self.receiver, PreparedRenderReceiver::unowned()),
            rejected_upload: self.rejected_upload.take(),
            rejected_packet: self.rejected_packet.take(),
            raster_backing_closed: self.raster_backing_closed,
            packet_command_staged: self.packet_command_staged,
            published: self.published,
            closing: true,
            abandonment_slot: self.abandonment_slot,
        });
        self.abandonment_slot = u8::MAX;
        self.closing = true;
        PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].store(Box::into_raw(job), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}

impl InteractiveJob for PreparedRenderJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            self.closing = true;
            return StepOutcome::Cancelled;
        }
        if self.closing {
            return self.fault_outcome("prepared render job was stepped while closing");
        }
        let Some(input) = self.input() else {
            return self.complete();
        };
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation is stale");
        }
        if cx.should_yield() {
            return StepOutcome::Yield;
        }
        if let Some(producer) = self.input.as_mut().and_then(|input| input.raster_producers.front_mut()) {
            cx.consume_fuel(1);
            let step = producer.step(cx.generation().0);
            let outcome = match step {
                PreparedRasterProducerStep::Pending => StepOutcome::Yield,
                PreparedRasterProducerStep::Complete(upload) => {
                    let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared during raster publication") };
                    input.raster_producers.pop_front();
                    if let Err(upload) = input.try_push_upload(upload) {
                        self.rejected_upload = Some(upload);
                        return self.fault_outcome("prepared render upload admission refused the exact raster owner");
                    }
                    StepOutcome::Yield
                }
                PreparedRasterProducerStep::Fault(fault) => self.fault_outcome(fault),
            };
            if cx.is_cancelled() {
                self.closing = true;
                return StepOutcome::Cancelled;
            }
            if self.input().is_some_and(|input| input.preview_generation != cx.generation().0) {
                return self.fault_outcome("prepared render generation became stale after raster work");
            }
            return outcome;
        }
        if !self.raster_backing_closed {
            cx.consume_fuel(1);
            let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared before producer backing close") };
            if input.raster_producers.release_backing_step() {
                self.raster_backing_closed = true;
            }
            return StepOutcome::Yield;
        }
        let Some(usage) = self.measure_next() else {
            return self.complete();
        };
        cx.consume_fuel(1);
        if self.fault.is_some() || !self.include_usage(usage) {
            return self.fault_outcome("prepared render cumulative credits overflowed");
        }
        let Some(input) = self.input() else { return self.fault_outcome("prepared render input disappeared after one worker unit") };
        if !self.usage.fits(input.limits) {
            return self.fault_outcome("prepared render credits exceeded");
        }
        if cx.is_cancelled() {
            self.closing = true;
            return StepOutcome::Cancelled;
        }
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation became stale after one worker unit");
        }
        StepOutcome::Yield
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, maximum_items: usize, maximum_bytes: usize) -> semio_framework_job::InteractiveJobCloseStep {
        if maximum_items == 0 || maximum_bytes == 0 {
            return semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 0, released_bytes: 0 };
        }
        if PreparedRenderJob::close_step(self) {
            semio_framework_job::InteractiveJobCloseStep::Complete
        } else {
            semio_framework_job::InteractiveJobCloseStep::Pending { released_items: 1, released_bytes: 0 }
        }
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && PreparedRenderJob::terminal_is_empty(self)
    }
}
//#endregion ⚙️PreparationJob

//#region 🛡️PresentationGate
/// 🛡️ Candidate rejection returned before GPU state can change.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PreparedRenderRejection {
    StaleRevision { live: u64, packet: u64 },
    StaleGeneration { live: u64, packet: u64 },
    Credits,
    PresentationPending,
    Closing,
}

impl std::fmt::Display for PreparedRenderRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleRevision { live, packet } => write!(formatter, "prepared render revision is stale: live={live}, packet={packet}"),
            Self::StaleGeneration { live, packet } => write!(formatter, "prepared render generation is stale: live={live}, packet={packet}"),
            Self::Credits => formatter.write_str("prepared render packet exceeds its credits"),
            Self::PresentationPending => formatter.write_str("prepared render presenter acknowledgement is pending"),
            Self::Closing => formatter.write_str("prepared render gate is closing"),
        }
    }
}

impl std::error::Error for PreparedRenderRejection {}

/// 🎟️ Exact non-clone acknowledgement required after the platform presents a prepared packet.
#[derive(Debug, PartialEq, Eq)]
pub struct PreparedPresenterWitness {
    sequence: u64,
    scene_revision: u64,
    preview_generation: u64,
}

/// ♻️ Previous last-valid packet returned only after exact presenter acknowledgement.
pub struct PreparedRenderReplacement {
    previous: Option<PreparedRenderPacket>,
}

impl PreparedRenderReplacement {
    pub fn take_previous(&mut self) -> Option<PreparedRenderPacket> {
        self.previous.take()
    }

    pub fn is_empty(&self) -> bool {
        self.previous.is_none()
    }
}

struct PendingPresentedPacket {
    sequence: u64,
    packet: PreparedRenderPacket,
}

/// 📸️ Last-valid packet state preserved across cancellation, rejection, and device loss.
pub struct PreparedRenderGate {
    last_valid: Option<PreparedRenderPacket>,
    pending: Option<PendingPresentedPacket>,
    next_sequence: u64,
    close_phase: u8,
    closing: bool,
    terminal: bool,
}

impl Default for PreparedRenderGate {
    fn default() -> Self {
        Self { last_valid: None, pending: None, next_sequence: 1, close_phase: 0, closing: false, terminal: false }
    }
}

impl PreparedRenderGate {
    pub fn validate(&self, packet: &PreparedRenderPacket, live_revision: u64, live_generation: u64) -> Result<(), PreparedRenderRejection> {
        if self.closing {
            return Err(PreparedRenderRejection::Closing);
        }
        if self.pending.is_some() {
            return Err(PreparedRenderRejection::PresentationPending);
        }
        if packet.scene_revision != live_revision {
            return Err(PreparedRenderRejection::StaleRevision { live: live_revision, packet: packet.scene_revision });
        }
        if packet.preview_generation != live_generation {
            return Err(PreparedRenderRejection::StaleGeneration { live: live_generation, packet: packet.preview_generation });
        }
        if !packet.is_within_credits() {
            return Err(PreparedRenderRejection::Credits);
        }
        Ok(())
    }

    pub fn stage_presented(&mut self, packet: PreparedRenderPacket) -> Result<PreparedPresenterWitness, PreparedRenderPacket> {
        if self.closing || self.pending.is_some() {
            return Err(packet);
        }
        let sequence = self.next_sequence;
        let Some(next_sequence) = sequence.checked_add(1) else { return Err(packet) };
        self.next_sequence = next_sequence;
        let witness = PreparedPresenterWitness { sequence, scene_revision: packet.scene_revision, preview_generation: packet.preview_generation };
        self.pending = Some(PendingPresentedPacket { sequence, packet });
        Ok(witness)
    }

    pub fn acknowledge_presented(&mut self, witness: PreparedPresenterWitness) -> Result<PreparedRenderReplacement, PreparedPresenterWitness> {
        let Some(pending) = self.pending.as_ref() else { return Err(witness) };
        if pending.sequence != witness.sequence || pending.packet.scene_revision != witness.scene_revision || pending.packet.preview_generation != witness.preview_generation {
            return Err(witness);
        }
        let Some(pending) = self.pending.take() else { return Err(witness) };
        let previous = self.last_valid.replace(pending.packet);
        Ok(PreparedRenderReplacement { previous })
    }

    pub fn abort_pending(&mut self) -> Option<PreparedRenderPacket> {
        self.pending.take().map(|pending| pending.packet)
    }

    pub fn pending_presented(&self, witness: &PreparedPresenterWitness) -> Option<&PreparedRenderPacket> {
        self.pending.as_ref().filter(|pending| pending.sequence == witness.sequence && pending.packet.scene_revision == witness.scene_revision && pending.packet.preview_generation == witness.preview_generation).map(|pending| &pending.packet)
    }

    pub fn take_last_valid(&mut self) -> Option<PreparedRenderPacket> {
        self.last_valid.take()
    }

    pub fn last_valid(&self) -> Option<&PreparedRenderPacket> {
        self.last_valid.as_ref()
    }

    pub fn last_valid_identity(&self) -> Option<(u64, u64)> {
        self.last_valid.as_ref().map(|packet| (packet.scene_revision, packet.preview_generation))
    }

    pub fn retain_after_device_loss(&self) -> Option<(u64, u64)> {
        self.last_valid_identity()
    }

    pub fn has_pending_acknowledgement(&self) -> bool {
        self.pending.is_some()
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.pending.is_some() || self.last_valid.is_some() {
            return false;
        }
        match self.close_phase {
            0 => {
                self.next_sequence = 0;
                self.close_phase = 1;
                false
            }
            _ => {
                self.terminal = true;
                true
            }
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.terminal && self.next_sequence == 0 && self.last_valid.is_none() && self.pending.is_none()
    }
}

/// 🔐️ Non-Send capability required for surface acquisition, submission, and presentation.
pub struct UiPresentToken {
    _ui_thread: std::marker::PhantomData<Rc<()>>,
}

impl UiPresentToken {
    pub fn mint_for_current_thread() -> Self {
        Self { _ui_thread: std::marker::PhantomData }
    }
}

/// 🧵️ Non-Send authority for a transferred `OffscreenCanvas` owned by a dedicated browser Worker.
#[cfg(target_arch = "wasm32")]
pub struct OffscreenPresentToken {
    _worker_isolate: std::marker::PhantomData<Rc<()>>,
}

#[cfg(target_arch = "wasm32")]
impl OffscreenPresentToken {
    pub fn mint_for_dedicated_worker() -> Result<Self, &'static str> {
        if web_sys::window().is_some() {
            return Err("offscreen presentation authority cannot be minted in the browser UI isolate");
        }
        Ok(Self { _worker_isolate: std::marker::PhantomData })
    }
}
//#endregion 🛡️PresentationGate

#[cfg(test)]
mod tests {
    use super::*;
    use semio_framework_job::{drive_step, root_cancel_token, Generation, InteractiveStage, OperationId, StepBudget};

    static ATLAS_TEST_LOCK: Mutex<()> = Mutex::new(());

    fn atlas_test_guard() -> std::sync::MutexGuard<'static, ()> {
        match ATLAS_TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn drain_abandoned_atlases() {
        while !PreparedAtlasPages::close_abandoned_step() {}
    }

    fn drain_abandoned_preparations() {
        loop {
            let inputs = PreparedRenderInput::close_abandoned_step();
            let jobs = PreparedRenderJob::close_abandoned_step();
            let mailboxes = PreparedRenderReceiver::close_abandoned_step();
            let packets = PreparedRenderPacket::close_abandoned_step();
            if inputs && jobs && mailboxes && packets {
                break;
            }
        }
    }

    fn now_ms() -> u64 {
        1
    }

    fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
        let mut directives = PreparedRenderDirectives::default();
        assert!(directives.try_push(RenderDirective::PreservePreviousOnFailure).is_ok());
        let packet = PreparedRenderPacket {
            scene_revision: revision,
            preview_generation: generation,
            damage: PreparedRenderScissors::default(),
            clips: PreparedRenderScissors::default(),
            directives,
            uploads: PreparedRenderUploads::default(),
            evictions: PreparedRenderEvictions::default(),
            draw: DrawList::default(),
            overlay: None,
            commands: PreparedRenderCommandPages::default(),
            time_seconds: 0.0,
            usage: PreparedRenderUsage::default(),
            limits: PreparedRenderLimits::default(),
            permit: PreparedRenderProcessPermit::try_reserve(0, 0),
            retirement_phase: 0,
            abandonment_slot: u8::MAX,
        };
        match packet.try_arm_abandonment() {
            Ok(packet) => packet,
            Err(_) => panic!("test packet abandonment slot"),
        }
    }

    fn retire_raster_upload(mut upload: PreparedRenderUpload) {
        let PreparedRenderUpload::RasterPages { key, pixels } = &mut upload else { panic!("paged raster upload") };
        while !pixels.retire_with_key_step(key) {}
        assert!(pixels.terminal_is_empty());
    }

    #[test]
    fn paged_raster_producer_advances_one_page_and_moves_page_identity() {
        let source = vec![7; PREPARED_RASTER_PAGE_BYTES * 2];
        let source_pointer = source.as_ptr();
        let (mut producer, published_key) = PreparedRasterProducer::try_admit("two-pages".into(), source, 4_096, 2).expect("exact two-page admission");
        assert_eq!(published_key, "two-pages");
        assert!(producer.bind_frame_generation(9));
        assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
        assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 1);
        assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending));
        assert_eq!(producer.pages.as_ref().expect("retained pages").slots.len(), 2);
        let first = producer.pages.as_ref().and_then(|pages| pages.page_pointer(0)).expect("first page identity");
        assert!(matches!(producer.step(9), PreparedRasterProducerStep::Pending), "source backing retires on its own grant");
        let upload = match producer.step(9) {
            PreparedRasterProducerStep::Complete(upload) => upload,
            _ => panic!("completed page handoff"),
        };
        assert_eq!(first, source_pointer, "page view borrows the exact decoder backing");
        assert!(matches!(&upload, PreparedRenderUpload::RasterPages { pixels, .. } if pixels.page_pointer(0) == Some(first) && pixels.frame_generation() == 9));
        retire_raster_upload(upload);
    }

    #[test]
    fn stale_generation_does_not_consume_a_prepared_raster_page() {
        let (mut producer, _) = PreparedRasterProducer::try_admit("stale".into(), vec![3; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page admission");
        assert!(producer.bind_frame_generation(11));
        let retained = producer.source.len();
        assert!(matches!(producer.step(12), PreparedRasterProducerStep::Fault(_)));
        assert_eq!(producer.source.len(), retained);
        assert!(producer.pages.as_ref().expect("retained pages").slots.is_empty());
        producer.begin_close();
        while !producer.close_step() {}
        assert!(producer.terminal_is_empty());
    }

    #[test]
    fn raster_cap_plus_one_rejects_the_exact_source_before_page_allocation() {
        let source = vec![5; PREPARED_RASTER_PAGE_BYTES + 4];
        let pointer = source.as_ptr();
        let mut rejected = PreparedRasterProducer::try_admit("wide".into(), source, 4_097, 1).expect_err("row cap plus one");
        assert_eq!(rejected.source.as_ptr(), pointer);
        assert!(rejected.fault().contains("fixed item or byte credits"));
        assert!(!rejected.close_step(), "one rejection grant retires one source page only");
        while !rejected.close_step() {}
    }

    #[test]
    fn atlas_page_cap_plus_one_faults_before_process_credit_transfer() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let height = 33_554_433_u32;
        let byte_len = 33_554_433_usize;
        assert!(matches!(PreparedAtlasPages::try_new(1, height, 1, byte_len), Err("atlas page or byte credits exceeded")));
        let mut admitted = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
            Ok(admitted) => admitted,
            Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
        };
        let mut turns = 0;
        while !admitted.close_step() {
            turns += 1;
        }
        assert!(turns >= 6, "fixed backing and four permit scalars close independently");
        assert!(admitted.terminal_is_empty());
    }

    #[test]
    fn atlas_close_releases_one_fixed_page_then_its_exact_credit() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let source = [9_u8; 32];
        let mut pages = match PreparedAtlasPages::try_new(4, 2, 4, source.len()) {
            Ok(pages) => pages,
            Err(fault) => panic!("fixed atlas admission faulted: {fault}"),
        };
        assert!(matches!(pages.push_page(&source, 0), Ok(true)));
        assert_eq!(pages.page(0), Some((&source[..], 0, 2)));
        assert!(!pages.close_step());
        assert_eq!(pages.len(), 0);
        assert!(!pages.terminal_is_empty());
        let mut turns = 0;
        while !pages.close_step() {
            turns += 1;
        }
        assert!(turns >= 6, "slot backing and permit fields each consume a distinct grant");
        assert!(pages.terminal_is_empty());
    }

    #[test]
    fn atlas_process_item_max_plus_one_is_nonblocking_and_recovers_every_permit() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let mut owners: [Option<PreparedAtlasPages>; PREPARED_ATLAS_PROCESS_ITEMS] = std::array::from_fn(|_| None);
        for owner in &mut owners {
            *owner = Some(match PreparedAtlasPages::try_new(1, 1, 1, 1) {
                Ok(owner) => owner,
                Err(fault) => panic!("one exact process item: {fault}"),
            });
        }
        assert!(matches!(PreparedAtlasPages::try_new(1, 1, 1, 1), Err("atlas process permit credits exhausted")));
        for owner in owners.iter_mut().filter_map(Option::as_mut) {
            while !owner.close_step() {}
            assert!(owner.terminal_is_empty());
        }
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn abandoned_atlas_schedules_the_same_incremental_close_authority() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let mut owner = match PreparedAtlasPages::try_new(4, 2, 4, 32) {
            Ok(owner) => owner,
            Err(fault) => panic!("abandonment owner: {fault}"),
        };
        assert!(owner.push_page(&[7; 32], 0).is_ok());
        drop(owner);
        let mut turns = 0;
        while !PreparedAtlasPages::close_abandoned_step() {
            turns += 1;
            assert!(turns < 16, "one page, backing owner, and four permit fields must converge");
        }
        assert!(turns >= 7);
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn interrupted_atlas_close_rejoins_the_same_abandonment_authority() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let source = vec![3_u8; PREPARED_ATLAS_PAGE_BYTES + 1];
        let mut owner = match PreparedAtlasPages::try_new(1, u32::try_from(source.len()).unwrap_or(u32::MAX), 1, source.len()) {
            Ok(owner) => owner,
            Err(fault) => panic!("two-page abandonment owner: {fault}"),
        };
        assert!(matches!(owner.push_page(&source, 0), Ok(false)));
        assert!(matches!(owner.push_page(&source, owner.next_row()), Ok(true)));
        assert!(!owner.close_step());
        assert_eq!(owner.len(), 1);
        drop(owner);
        let mut turns = 0;
        while !PreparedAtlasPages::close_abandoned_step() {
            turns += 1;
            assert!(turns < 16);
        }
        assert!(turns >= 7);
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn atlas_allocation_refusal_preserves_the_packed_permit_ledger() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let before = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
        assert!(matches!(PreparedAtlasPages::try_new(0, 1, 4, 4), Err("atlas dimensions do not fit fixed page credits")));
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), before);
    }

    #[test]
    fn atlas_contended_permit_attempts_are_nonblocking_and_poison_free() {
        let _guard = atlas_test_guard();
        drain_abandoned_atlases();
        let handles = std::array::from_fn::<_, 8, _>(|_| {
            std::thread::spawn(|| match PreparedAtlasPages::try_new(1, 1, 1, 1) {
                Ok(mut owner) => {
                    while !owner.close_step() {}
                    owner.terminal_is_empty()
                }
                Err("atlas process permit credits exhausted") => true,
                Err(_) => false,
            })
        });
        for handle in handles {
            assert!(match handle.join() {
                Ok(closed_or_refused) => closed_or_refused,
                Err(_) => false,
            });
        }
        assert_eq!(PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn raster_item_bytes_exact_and_plus_one_are_claimed_before_materialization() {
        let reservation = PreparedRasterReservation::try_reserve("item-exact".into()).expect("initial exact reservation");
        let reservation = reservation.claim(4_096, 1_024).expect("sixteen MiB operation claim");
        let mut rejected = reservation.reject("test retirement", Vec::new());
        while !rejected.close_step() {}
        assert!(rejected.terminal_is_empty());

        let reservation = PreparedRasterReservation::try_reserve("item-plus-one".into()).expect("initial plus-one reservation");
        let mut rejected = reservation.claim(4_096, 1_025).expect_err("sixteen MiB plus one row");
        assert_eq!(rejected.fault(), "raster producer exceeded fixed item or byte credits");
        while !rejected.close_step() {}
        assert!(rejected.terminal_is_empty());
    }

    #[test]
    fn raster_simultaneous_source_decode_peak_exact_and_plus_one() {
        let height = 1_023usize;
        let decoded_bytes = PREPARED_RASTER_PAGE_BYTES * height;
        let page_slot_bytes = size_of::<PreparedRasterPage>() * height;
        let source_peak_bytes = PREPARED_RASTER_PRODUCER_BYTES - decoded_bytes - page_slot_bytes;
        assert_eq!(source_peak_bytes % 2, 0, "exact source workspace boundary");
        let source_bytes = source_peak_bytes / 2;

        let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes).expect("exact simultaneous source reservation");
        let reservation = reservation.claim(4_096, height as u32).expect("source plus retained parse plus decoded backing and page slots exactly fit");
        assert_eq!(reservation.credit.as_ref().unwrap().bytes, PREPARED_RASTER_PRODUCER_BYTES);
        let mut rejected = reservation.reject("exact peak retirement", Vec::new());
        while !rejected.close_step() {}

        let reservation = PreparedRasterReservation::try_reserve_source(String::new(), source_bytes + 1).expect("plus one source is initially retained");
        let mut rejected = reservation.claim(4_096, height as u32).expect_err("simultaneous source and decoded peak plus one must fail before decode");
        assert_eq!(rejected.fault(), "raster producer exact credit resize failed");
        while !rejected.close_step() {}
    }

    #[test]
    fn retained_codec_source_moves_once_and_retires_one_page_per_governed_step() {
        let decoded = vec![7; PREPARED_RASTER_PAGE_BYTES];
        let retained_source = vec![9; PREPARED_RASTER_PAGE_BYTES * 2];
        let retained_pointer = retained_source.as_ptr();
        let reservation = PreparedRasterReservation::try_reserve_source("retained-source".into(), retained_source.capacity()).expect("source workspace admitted before decode");
        let reservation = reservation.claim(4_096, 1).expect("decoded owner credited in addition to retained source");
        let (producer, _) = reservation.finalize(decoded, retained_source, 4_096, 1).expect("exact retained source owner");
        assert_eq!(producer.retained_source.as_ptr(), retained_pointer);

        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        assert!(input.try_push_raster_producer(producer).is_ok());
        let mut job = PreparedRenderJob::new(input, 1);
        let mut preview = 0;
        let now_ms = || 0.0;
        for expected in [PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES * 2, PREPARED_RASTER_PAGE_BYTES, 0] {
            let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(11), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview);
            assert!(matches!(outcome, StepOutcome::Yield));
            assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.len(), expected);
        }
        assert_eq!(job.input.as_ref().unwrap().raster_producers.get(0).unwrap().retained_source.as_ptr(), retained_pointer);
        while !job.close_step() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn raster_ledger_exact_item_and_generation_slot_caps_reject_plus_one() {
        let mut ledger = PreparedRasterLedger::default();
        let item = ledger.reserve(PREPARED_RASTER_PRODUCER_ITEMS, 1).expect("exact aggregate items");
        assert!(ledger.reserve(1, 0).is_none(), "aggregate item cap plus one");
        assert!(ledger.release(&item));

        let bytes = ledger.reserve(1, PREPARED_RASTER_PRODUCER_BYTES).expect("exact aggregate bytes");
        assert!(ledger.reserve(0, 1).is_none(), "aggregate byte cap plus one");
        assert!(ledger.release(&bytes));

        let mut credits = Vec::with_capacity(PREPARED_RASTER_PRODUCER_CAPACITY);
        for _ in 0..PREPARED_RASTER_PRODUCER_CAPACITY {
            credits.push(ledger.reserve(1, 1).expect("exact fixed generation slot"));
        }
        assert!(ledger.reserve(1, 1).is_none(), "generation slot cap plus one");
        for credit in credits {
            assert!(ledger.release(&credit));
        }
        assert_eq!((ledger.items, ledger.bytes), (0, 0));
    }

    #[test]
    fn raster_credit_epoch_rejects_aba_and_cancel_retires_one_owner_per_grant() {
        let (mut first, _) = PreparedRasterProducer::try_admit("first".into(), vec![1; PREPARED_RASTER_PAGE_BYTES * 2], 4_096, 2).expect("first admission");
        let first_epoch = first.pages.as_ref().expect("first pages").source_generation();
        assert!(first.bind_frame_generation(3));
        assert!(matches!(first.step(3), PreparedRasterProducerStep::Pending));
        first.begin_close();
        assert!(!first.close_step());
        assert!(first.pages.as_ref().expect("closing pages").slots.is_empty(), "first close grant retires only the built page");
        assert_eq!(first.source.len(), PREPARED_RASTER_PAGE_BYTES, "source remains owned after the page grant");
        while !first.close_step() {}
        let (mut second, _) = PreparedRasterProducer::try_admit("second".into(), vec![2; 4], 1, 1).expect("reused slot admission");
        let second_epoch = second.pages.as_ref().expect("second pages").source_generation();
        assert_eq!(second_epoch.slot(), first_epoch.slot(), "released fixed slot is reused");
        assert!(second_epoch.epoch() > first_epoch.epoch(), "reused fixed slot advances its generation");
        second.begin_close();
        while !second.close_step() {}
    }

    #[test]
    fn zero_fuel_and_expired_deadline_advance_no_raster_page_or_allocation() {
        let (mut producer, _) = PreparedRasterProducer::try_admit("governed".into(), vec![4; PREPARED_RASTER_PAGE_BYTES], 4_096, 1).expect("one-page producer");
        assert!(producer.bind_frame_generation(3));
        let source_pointer = producer.source.as_ptr();
        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        assert!(input.try_push_raster_producer(producer).is_ok());
        let mut job = PreparedRenderJob::new(input, 1);
        let mut preview = 0;

        let zero = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(0, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(zero, StepOutcome::Yield));
        let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
        assert_eq!(retained.source.as_ptr(), source_pointer);
        assert!(retained.pages.as_ref().unwrap().slots.is_empty());

        let expired = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 1), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(expired, StepOutcome::Yield));
        let retained = job.input.as_ref().unwrap().raster_producers.get(0).unwrap();
        assert_eq!(retained.source.as_ptr(), source_pointer);
        assert!(retained.pages.as_ref().unwrap().slots.is_empty());

        while !job.close_step() {}
        assert!(job.terminal_is_empty());
    }

    #[test]
    fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 }).is_ok());
        let mut job = PreparedRenderJob::new(input, 1);
        assert!(!job.close_step());
        assert!(!job.terminal_is_empty());
        let mut turns = 1;
        while !job.close_step() {
            turns += 1;
            assert!(turns < 5_000);
        }
        assert!(turns > 4_096);
        assert!(job.terminal_is_empty());
    }

    fn assert_send<T: Send>() {}

    #[test]
    fn prepared_packet_is_send_owned_data() {
        assert_send::<PreparedRenderPacket>();
        assert_send::<PreparedRenderJob>();
        assert_send::<PreparedRenderReceiver>();
        assert_send::<PreparedRenderGate>();
        let packet = packet(7, 3);
        let identity = std::thread::spawn(move || (packet.scene_revision, packet.preview_generation)).join().expect("worker packet");
        assert_eq!(identity, (7, 3));
    }

    #[test]
    fn receiver_survives_worker_ownership_of_the_job() {
        let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
        let receiver = job.receiver().expect("prepared receiver clone");
        std::thread::spawn(move || {
            let mut job = job;
            let mut preview = 0;
            loop {
                let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
                if outcome.is_terminal() {
                    assert!(matches!(outcome, StepOutcome::Complete(_)));
                    break;
                }
            }
        })
        .join()
        .expect("worker preparation");
        let packet = receiver.take_latest().expect("prepared packet handoff");
        assert_eq!((packet.scene_revision(), packet.preview_generation()), (7, 3));
        assert!(receiver.take_latest().is_none());
    }

    #[test]
    fn preparation_yields_at_the_configured_item_budget() {
        let mut draw = DrawList::default();
        draw.layers.extend((0..4).map(|_| DrawLayer::default()));
        let input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
        let mut job = PreparedRenderJob::new(input, 1);
        let mut preview = 0;
        let first = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(first, StepOutcome::Yield));
    }

    #[test]
    fn preparation_completes_across_bounded_steps() {
        let mut draw = DrawList::default();
        draw.layers.extend((0..2).map(|_| DrawLayer::default()));
        let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
        let mut preview = 0;
        let mut outcome = StepOutcome::Yield;
        for _ in 0..8 {
            outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
            if !matches!(outcome, StepOutcome::Yield) {
                break;
            }
        }
        assert!(matches!(outcome, StepOutcome::Complete(_)));
        let packet = job.take_packet().expect("prepared packet");
        assert_eq!((packet.scene_revision, packet.preview_generation), (7, 3));
    }

    #[test]
    fn preparation_rejects_a_stale_generation_before_publication() {
        let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 2, DrawList::default(), None, 0.0), 8);
        let mut preview = 0;
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert!(job.take_packet().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn preparation_observes_cancellation_without_replacing_a_packet() {
        let cancel = root_cancel_token();
        cancel.cancel().await;
        let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 8);
        let mut preview = 0;
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), cancel, now_ms, &mut preview);
        assert!(matches!(outcome, StepOutcome::Cancelled));
        assert!(job.take_packet().is_none());
    }

    #[test]
    fn stale_packet_rejection_preserves_the_last_valid_packet() {
        let mut gate = PreparedRenderGate::default();
        let witness = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
        assert!(gate.acknowledge_presented(witness).expect("first presenter acknowledgement").is_empty());
        let stale = packet(6, 3);
        assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
        assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    }

    #[test]
    fn generation_rejection_happens_before_presentation() {
        let gate = PreparedRenderGate::default();
        assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
    }

    #[test]
    fn device_loss_retains_the_last_valid_packet() {
        let mut gate = PreparedRenderGate::default();
        let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
        let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
        assert_eq!(gate.retain_after_device_loss(), Some((7, 3)));
    }

    #[test]
    fn presenter_ack_is_exact_one_shot_and_preserves_old_until_acknowledged() {
        let mut gate = PreparedRenderGate::default();
        let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
        let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
        let second = gate.stage_presented(packet(8, 4)).ok().expect("second presenter witness");
        assert_eq!(gate.last_valid_identity(), Some((7, 3)), "candidate is not visible before acknowledgement");
        let stale = PreparedPresenterWitness { sequence: second.sequence.saturating_add(1), scene_revision: second.scene_revision, preview_generation: second.preview_generation };
        assert!(gate.acknowledge_presented(stale).is_err());
        assert_eq!(gate.last_valid_identity(), Some((7, 3)));
        let duplicate = PreparedPresenterWitness { sequence: second.sequence, scene_revision: second.scene_revision, preview_generation: second.preview_generation };
        let mut replacement = gate.acknowledge_presented(second).expect("exact second acknowledgement");
        assert_eq!(gate.last_valid_identity(), Some((8, 4)));
        assert_eq!(replacement.previous.as_ref().map(|packet| (packet.scene_revision, packet.preview_generation)), Some((7, 3)));
        assert!(gate.acknowledge_presented(duplicate).is_err(), "duplicate acknowledgement is stale after publication");
        let mut previous = replacement.take_previous().expect("old last-valid owner");
        while !previous.retire_step() {}
        assert!(previous.retirement_is_empty());
    }

    #[test]
    fn missing_ack_and_abort_return_the_exact_candidate_without_replacing_last_valid() {
        let mut gate = PreparedRenderGate::default();
        let first = gate.stage_presented(packet(7, 3)).ok().expect("first presenter witness");
        let _ = gate.acknowledge_presented(first).expect("first acknowledgement");
        let _missing = gate.stage_presented(packet(8, 4)).ok().expect("pending presenter witness");
        assert_eq!(gate.last_valid_identity(), Some((7, 3)));
        let candidate = gate.abort_pending().expect("exact pending packet handback");
        assert_eq!((candidate.scene_revision, candidate.preview_generation), (8, 4));
        assert_eq!(gate.last_valid_identity(), Some((7, 3)));
    }

    #[test]
    fn pending_presenter_witness_rejects_superseding_packet_with_exact_owner() {
        let mut gate = PreparedRenderGate::default();
        let _pending = gate.stage_presented(packet(7, 3)).ok().expect("pending presenter witness");
        let mut superseding = packet(8, 4);
        assert!(superseding.uploads.try_push(PreparedRenderUpload::GlyphAtlas { pixels: vec![7; 16_385], width: 1, height: 1 }).is_ok());
        let pixels = match superseding.uploads.get(0) {
            PreparedRenderUpload::GlyphAtlas { pixels, .. } => pixels.as_ptr(),
            _ => unreachable!(),
        };
        let mut returned = match gate.stage_presented(superseding) {
            Ok(_) => panic!("second presenter witness must fail closed"),
            Err(packet) => packet,
        };
        assert_eq!((returned.scene_revision, returned.preview_generation), (8, 4));
        assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels: returned_pixels, .. }) if returned_pixels.as_ptr() == pixels));
        assert!(!returned.retire_step(), "one close grant retires only one admitted pixel page");
        assert!(matches!(returned.uploads.get(0), Some(PreparedRenderUpload::GlyphAtlas { pixels, .. }) if pixels.len() == 1));
    }

    #[test]
    fn gate_close_requires_pending_and_last_valid_packet_handback_before_terminal_scalars() {
        let mut gate = PreparedRenderGate::default();
        let witness = gate.stage_presented(packet(7, 3)).ok().expect("presenter witness");
        let _ = gate.acknowledge_presented(witness).expect("presenter acknowledgement");
        assert!(!gate.close_step(), "last-valid owner prevents gate terminalization");
        let mut last = gate.take_last_valid().expect("last-valid owner handback");
        while !last.retire_step() {}
        assert!(!gate.close_step(), "first scalar grant retires only the sequence");
        assert!(gate.close_step(), "second scalar grant publishes the terminal witness");
        assert!(gate.terminal_is_empty());
        assert!(matches!(gate.validate(&packet(8, 4), 8, 4), Err(PreparedRenderRejection::Closing)));
    }

    #[test]
    fn upload_byte_cap_faults_before_packet_publication() {
        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        input.limits.max_upload_bytes = 3;
        assert!(input.try_push_upload(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 }).is_ok());
        let mut job = PreparedRenderJob::new(input, 64);
        let mut preview = 0;
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert!(job.take_packet().is_none());
    }

    #[test]
    fn eviction_byte_cap_faults_before_packet_publication() {
        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        input.limits.max_upload_bytes = 3;
        assert!(input.try_push_eviction(PreparedRenderEviction::Mesh { key: "mesh".into() }).is_ok());
        let mut job = PreparedRenderJob::new(input, 64);
        let mut preview = 0;
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert!(job.take_packet().is_none());
    }

    #[test]
    fn draw_item_cap_faults_before_packet_publication() {
        let mut draw = DrawList::default();
        draw.layers[0].ui_instances.push(crate::wgpu::draw::UiInstance::solid([0.0; 4], crate::wgpu::theme::Rgba::new(0.0, 0.0, 0.0, 0.0)));
        let mut input = PreparedRenderInput::new(7, 3, draw, None, 0.0);
        input.limits.max_draw_items = 0;
        let mut job = PreparedRenderJob::new(input, 64);
        let mut preview = 0;
        let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(1), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(100, 10), root_cancel_token(), now_ms, &mut preview);
        assert!(matches!(outcome, StepOutcome::Fault(_)));
        assert!(job.take_packet().is_none());
    }

    #[test]
    fn input_drop_hands_back_exact_process_permits_for_incremental_close() {
        let _guard = atlas_test_guard();
        drain_abandoned_preparations();
        let input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
        drop(input);
        assert_ne!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
        let mut turns = 0;
        while !PreparedRenderInput::close_abandoned_step() {
            turns += 1;
            assert!(turns < 128);
        }
        assert!(turns > 4);
        assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn worker_panic_hands_back_the_exact_job_and_mailbox_owners() {
        let _guard = atlas_test_guard();
        drain_abandoned_preparations();
        let job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0), 1);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(move || {
            let _owner = job;
            panic!("hostile worker interruption");
        }));
        assert!(result.is_err());
        let mut turns = 0;
        while !PreparedRenderJob::close_abandoned_step() {
            turns += 1;
            assert!(turns < 256);
        }
        assert!(turns > 4);
        assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
        assert!(PREPARED_RENDER_MAILBOX.iter().all(|slot| slot.packet.load(Ordering::Acquire).is_null()));
    }

    #[test]
    fn packet_drop_retires_nested_backings_and_permit_scalars_separately() {
        let _guard = atlas_test_guard();
        drain_abandoned_preparations();
        let mut owner = packet(7, 3);
        owner.draw.push_solid([0.0, 0.0, 8.0, 8.0], crate::wgpu::theme::Rgba::new(1.0, 0.0, 0.0, 1.0));
        drop(owner);
        let mut turns = 0;
        while !PreparedRenderPacket::close_abandoned_step() {
            turns += 1;
            assert!(turns < 256);
        }
        assert!(turns > 8);
        assert_eq!(PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire), 0);
    }

    #[test]
    fn fixed_command_pages_reject_max_plus_one_without_consuming_the_owner() {
        let mut commands = PreparedRenderCommandPages::default();
        for source in 0..PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS {
            let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source, digest: source as u64, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: false };
            assert!(commands.try_push(command).is_ok());
        }
        let rejected = PreparedRenderCommand { kind: PreparedRenderCommandKind::Tessellate, source: usize::MAX, digest: u64::MAX, draw_cursor: Some(DrawMeasureCursor::Complete), packet_overlay: true };
        let returned = match commands.try_push(rejected) {
            Ok(()) => panic!("command cap plus one must refuse"),
            Err(returned) => returned,
        };
        assert_eq!((returned.source, returned.digest, returned.packet_overlay), (usize::MAX, u64::MAX, true));
        let mut turns = 0;
        while !commands.close_step() {
            turns += 1;
        }
        assert!(turns >= PREPARED_RENDER_COMMAND_PAGES * PREPARED_RENDER_COMMAND_PAGE_ITEMS);
        assert!(commands.terminal_is_empty());
    }

    #[test]
    fn tessellation_commands_retain_exact_scalar_and_overlay_cursors() {
        let _guard = atlas_test_guard();
        drain_abandoned_preparations();
        let mut draw = DrawList::default();
        draw.push_solid([0.0, 0.0, 4.0, 4.0], crate::wgpu::theme::Rgba::new(0.0, 1.0, 0.0, 1.0));
        let mut job = PreparedRenderJob::new(PreparedRenderInput::new(7, 3, draw, None, 0.0), 1);
        let mut preview = 0;
        let mut steps = 0;
        loop {
            let outcome = drive_step(&mut job, "ui-wgpu.prepare", OperationId(41), Generation(3), InteractiveStage::BackgroundStep, StepBudget::new(1, 10), root_cancel_token(), now_ms, &mut preview);
            steps += 1;
            if outcome.is_terminal() {
                assert!(matches!(outcome, StepOutcome::Complete(_)));
                break;
            }
            assert!(steps < 128);
        }
        let mut packet = match job.take_packet() {
            Some(packet) => packet,
            None => panic!("prepared packet handoff"),
        };
        assert!((0..packet.commands.len())
            .filter_map(|index| packet.commands.get(index))
            .any(|command| { command.kind == PreparedRenderCommandKind::Tessellate && command.draw_cursor == Some(DrawMeasureCursor::LayerUi { layer: 0, item: 0, overlay: false }) && !command.packet_overlay }));
        while !packet.retire_step() {}
        while !job.close_step() {}
    }
}
