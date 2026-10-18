//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use semio_framework_ui_draw_contract::{DrawLayer, DrawList, ScissorRect};
use semio_framework_job::{InteractiveJob, JobOutcomeBorrow, JobOutcomeDescriptor, JobOutcomeKind, JobOutcomeView, JobPayloadStream, RetainedPayloadBuilder, StepContext};
use std::mem::size_of;
use semio_framework_ui_prepared_staging::retirement::*;
use std::rc::Rc;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicU8, AtomicUsize, Ordering};
use std::sync::Mutex;
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};

fn prepared_abandoned_demands<'a, T: 'a>(mut slots: impl Iterator<Item = (&'a AtomicU8, &'a AtomicPtr<T>)>, queued: u8, copy: usize, demand: fn(&T, usize) -> Result<RetainedCloneGrant, ValueError>, empty: fn(&T) -> bool) -> Result<RetainedCloneGrant, ValueError> {
    let Some((state, owner)) = slots.find(|(state, _)| state.compare_exchange(queued, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(RetainedCloneGrant::default()) };
    let pointer = owner.load(Ordering::Acquire);
    let result = if pointer.is_null() { Ok(RetainedCloneGrant::default()) } else {
        let value = unsafe { &*pointer };
        if empty(value) { prepared_owner_demands(false, Ok(size_of::<T>())) } else { demand(value, copy) }
    };
    state.store(queued, Ordering::Release);
    result
}
fn prepared_abandoned_close<'a, T: 'a>(mut slots: impl Iterator<Item = (&'a AtomicU8, &'a AtomicPtr<T>)>, queued: u8, grant: RetainedCloneGrant, empty: fn(&T) -> bool, close: fn(&mut T, RetainedCloneGrant) -> Result<CloseStep,ValueError>) -> Result<CloseStep,ValueError> {
    let Some((state, owner)) = slots.find(|(state, _)| state.compare_exchange(queued, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); };
    let pointer = owner.swap(std::ptr::null_mut(), Ordering::AcqRel);
    if pointer.is_null() { state.store(queued, Ordering::Release); return Ok(CloseStep::Blocked); }
    let mut value = unsafe { Box::from_raw(pointer) };
    let step = if empty(&value) {
        if grant.maximum_items == 0 || grant.maximum_release_bytes < size_of::<T>() { Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }) }
        else if grant.maximum_depth == 0 { Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original abandoned prepared backing close requires depth")) }
        else { drop(value); state.store(0, Ordering::Release); return prepared_close_progress(size_of::<T>()) }
    } else { prepared_close_child(close(&mut value, grant)) };
    owner.store(Box::into_raw(value), Ordering::Release);
    state.store(queued, Ordering::Release);
    step
}

//#region 📊️Credits
/// 🎛️ Hard item and byte credits for one prepared frame transaction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedRenderLimits {
    pub max_draw_items: usize,
    pub max_draw_bytes: usize,
    pub max_upload_items: usize,
    pub max_upload_bytes: usize,
    pub max_scene_raster_bytes: usize,
}

impl Default for PreparedRenderLimits {
    fn default() -> Self {
        Self { max_draw_items: 262_144, max_draw_bytes: 64 * 1024 * 1024, max_upload_items: 256, max_upload_bytes: 32 * 1024 * 1024, max_scene_raster_bytes: 256 * 1024 * 1024 }
    }
}

/// 📏️ Measured ownership cost of a prepared packet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRenderUsage {
    pub draw_items: usize,
    pub draw_bytes: usize,
    pub upload_items: usize,
    pub upload_bytes: usize,
    pub scene_raster_bytes: usize,
}

impl PreparedRenderUsage {
    fn include_draw(&mut self, items: usize, bytes: usize) -> bool {
        let Some(draw_items) = self.draw_items.checked_add(items) else { return false };
        let Some(draw_bytes) = self.draw_bytes.checked_add(bytes) else { return false };
        self.draw_items = draw_items;
        self.draw_bytes = draw_bytes;
        true
    }

    fn include_upload(&mut self, bytes: usize, scene_raster_bytes: usize) -> bool {
        let Some(upload_items) = self.upload_items.checked_add(1) else { return false };
        let Some(upload_bytes) = self.upload_bytes.checked_add(bytes) else { return false };
        let Some(scene_raster_bytes) = self.scene_raster_bytes.checked_add(scene_raster_bytes) else { return false };
        self.upload_items = upload_items;
        self.upload_bytes = upload_bytes;
        self.scene_raster_bytes = scene_raster_bytes;
        true
    }

    pub fn fits(self, limits: PreparedRenderLimits) -> bool {
        self.draw_items <= limits.max_draw_items && self.draw_bytes <= limits.max_draw_bytes && self.upload_items <= limits.max_upload_items && self.upload_bytes <= limits.max_upload_bytes && self.scene_raster_bytes <= limits.max_scene_raster_bytes
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
pub struct PreparedFixedPage<T> {
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

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.len == 0 && self.pages.iter().any(Option::is_some) { size_of::<PreparedFixedPage<T>>() } else { 0 })
    }

    fn close_backing_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        if self.len != 0 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared fixed pages still hold members")); }
        self.release_backing_step();
        prepared_close_progress(demand.unwrap_or(0))
    }
}

const PREPARED_RENDER_ITEM_SHIFT: u32 = 0;
const PREPARED_RENDER_PAGE_SHIFT: u32 = 7;
const PREPARED_RENDER_BACKING_SHIFT: u32 = 21;
const PREPARED_RENDER_ITEM_MASK: u64 = 0x7f;
const PREPARED_RENDER_PAGE_MASK: u64 = 0x3fff;
const PREPARED_RENDER_BACKING_MASK: u64 = 0x3fff;
pub static PREPARED_RENDER_PROCESS_PERMITS: AtomicU64 = AtomicU64::new(0);
static PREPARED_RENDER_SLOT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_SLOT_GENERATION: [AtomicU64; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU64::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_PACKET_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_PACKET_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderPacket>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_JOB_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
static PREPARED_RENDER_JOB_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderJob>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];
pub static PREPARED_RENDER_INPUT_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];
pub static PREPARED_RENDER_INPUT_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderInput>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];

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
pub struct PreparedRenderProcessPermit {
    slot: u8,
    generation: u64,
    pages: usize,
    backing_units: usize,
    release_phase: u8,
}

impl PreparedRenderProcessPermit {
    pub fn try_reserve(pages: usize, backing_bytes: usize) -> Option<Self> {
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

pub use semio_framework_ui_prepared_staging::*;

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
    pub kind: PreparedRenderCommandKind,
    pub source: usize,
    pub digest: u64,
    pub draw_cursor: Option<DrawMeasureCursor>,
    pub packet_overlay: bool,
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

    pub fn draw_cursor(&self) -> Option<DrawMeasureCursor> {
        self.draw_cursor
    }

    pub fn packet_overlay(&self) -> bool {
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
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.len != 0 { 0 } else if self.page_count != 0 { size_of::<PreparedRenderCommandPage>() } else if self.directories.iter().any(Option::is_some) { size_of::<PreparedRenderCommandDirectory>() } else { 0 })
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        self.close_step();
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn try_push(&mut self, command: PreparedRenderCommand) -> Result<(), PreparedRenderCommand> {
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

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn close_step(&mut self) -> bool {
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

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.page_count == 0 && self.directories.iter().all(Option::is_none)
    }
}

/// 🧱️ Send-capable frame data prepared without window, surface, device, or queue access.
pub struct PreparedRenderPacket {
    has_animated_primitives: bool,
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub(crate) damage: PreparedRenderScissors,
    pub(crate) clips: PreparedRenderScissors,
    pub(crate) directives: PreparedRenderDirectives,
    pub uploads: PreparedRenderUploads,
    pub(crate) evictions: PreparedRenderEvictions,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub commands: PreparedRenderCommandPages,
    pub(crate) time_seconds: f32,
    pub(crate) usage: PreparedRenderUsage,
    pub(crate) limits: PreparedRenderLimits,
    permit: Option<PreparedRenderProcessPermit>,
    retirement_phase: u8,
    abandonment_slot: u8,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRasterKeepCursorV1 {
    owner: u8,
    draw: semio_framework_ui_draw_contract::RasterKeepCursorV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PreparedRasterKeepStepV1<'a> {
    Pending,
    Key(&'a str),
    Complete,
}

impl PreparedRenderPacket {
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.uploads.get(self.uploads.len().saturating_sub(1)).map_or(Ok(0), PreparedRenderUpload::next_close_copy_byte_demand) }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { return upload.next_close_release_byte_demand(); }
        
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.get(self.evictions.len().saturating_sub(1)) { return Ok(if key.is_empty() { key.capacity() } else { 0 }); }
        if !self.draw.retirement_is_empty() { return self.draw.next_retirement_release_byte_demand(); }
        if let Some(overlay) = self.overlay.as_ref() { return overlay.next_retirement_release_byte_demand(); }
        if !self.commands.terminal_is_empty() { return self.commands.next_close_release_byte_demand(); }
        if !self.damage.is_empty() || !self.clips.is_empty() || !self.directives.is_empty() { return Ok(0); }
        if !self.uploads.terminal_is_empty() { return self.uploads.next_close_release_byte_demand(); }
        if !self.evictions.terminal_is_empty() { return self.evictions.next_close_release_byte_demand(); }
        if !self.damage.terminal_is_empty() { return self.damage.next_close_release_byte_demand(); }
        if !self.clips.terminal_is_empty() { return self.clips.next_close_release_byte_demand(); }
        if !self.directives.terminal_is_empty() { return self.directives.next_close_release_byte_demand(); }
        Ok(0)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.retirement_is_empty(), demand) { return step; }
        if let Some(upload) = self.uploads.last_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)); }
            self.uploads.pop(); return prepared_close_progress(0);
        }
        
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if !key.is_empty() || key.capacity() != 0 { return prepared_close_progress(prepared_close_text(key)); }
            self.evictions.pop(); return prepared_close_progress(0);
        }
        if !self.draw.retirement_is_empty() { return prepared_close_child(self.draw.retire_granted(grant)); }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retirement_is_empty() { return prepared_close_child(overlay.retire_granted(grant)); }
            self.overlay = None; return prepared_close_progress(0);
        }
        if !self.commands.terminal_is_empty() { return prepared_close_child(self.commands.close_granted(grant)); }
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() { return prepared_close_progress(0); }
        if !self.uploads.terminal_is_empty() { return prepared_close_child(self.uploads.close_backing_granted(grant)); }
        if !self.evictions.terminal_is_empty() { return prepared_close_child(self.evictions.close_backing_granted(grant)); }
        if !self.damage.terminal_is_empty() { return prepared_close_child(self.damage.close_backing_granted(grant)); }
        if !self.clips.terminal_is_empty() { return prepared_close_child(self.clips.close_backing_granted(grant)); }
        if !self.directives.terminal_is_empty() { return prepared_close_child(self.directives.close_backing_granted(grant)); }
        if let Some(permit) = self.permit.as_mut() {
            let phase = permit.release_phase;
            if permit.release_step() { self.permit = None; }
            else if permit.release_phase == phase { return Ok(CloseStep::Blocked); }
            return prepared_close_progress(0);
        }
        match self.retirement_phase {
            0 => self.scene_revision = 0,
            1 => self.preview_generation = 0,
            2 => self.time_seconds = 0.0,
            3 => self.usage = PreparedRenderUsage::default(),
            4 => self.limits = PreparedRenderLimits::default(),
            5 => {
                if self.abandonment_slot != u8::MAX {
                    let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared packet abandonment slot is absent")); };
                    let current = state.load(Ordering::Acquire);
                    if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
                    self.abandonment_slot = u8::MAX;
                }
            }
            _ => return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }),
        }
        self.retirement_phase += 1;
        prepared_close_progress(0)
    }

    pub fn draw(&self) -> &DrawList { &self.draw }

    pub fn overlay(&self) -> Option<&DrawList> { self.overlay.as_ref() }

    pub fn time_seconds(&self) -> f32 { self.time_seconds }

    pub fn has_animated_primitives(&self) -> bool {
        self.has_animated_primitives
    }

    #[cfg(test)]
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

    pub fn raster_keep_step<'a>(&'a self, cursor: &mut PreparedRasterKeepCursorV1) -> PreparedRasterKeepStepV1<'a> {
        let draw = match cursor.owner {
            0 => Some(&self.draw),
            1 => self.overlay.as_ref(),
            _ => return PreparedRasterKeepStepV1::Complete,
        };
        let Some(draw) = draw else {
            cursor.owner += 1;
            cursor.draw = semio_framework_ui_draw_contract::RasterKeepCursorV1::default();
            return PreparedRasterKeepStepV1::Pending;
        };
        match draw.raster_keep_step(&mut cursor.draw) {
            semio_framework_ui_draw_contract::RasterKeepStepV1::Pending => PreparedRasterKeepStepV1::Pending,
            semio_framework_ui_draw_contract::RasterKeepStepV1::Key(key) => PreparedRasterKeepStepV1::Key(key),
            semio_framework_ui_draw_contract::RasterKeepStepV1::Complete => {
                cursor.owner += 1;
                cursor.draw = semio_framework_ui_draw_contract::RasterKeepCursorV1::default();
                PreparedRasterKeepStepV1::Pending
            }
        }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact admitted packet without allocating another owner.")]
    pub fn try_arm_abandonment(mut self) -> Result<Self, Self> {
        let Some(slot) = self.permit.as_ref().map(|permit| usize::from(permit.slot)) else { return Err(self) };
        let Some(state) = PREPARED_RENDER_PACKET_ABANDONMENT_STATE.get(slot) else { return Err(self) };
        if state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err(self);
        }
        self.abandonment_slot = slot as u8;
        Ok(self)
    }

    /// 🧹 Advances one owner from an interrupted packet Drop handback.

    pub fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        let mut demand = prepared_owner_demands(self.retirement_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> { self.close_granted(grant) }
    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_PACKET_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_PACKET_ABANDONMENT_OWNER.iter()), 2, copy, Self::next_close_demands, Self::retirement_is_empty)
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_PACKET_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_PACKET_ABANDONMENT_OWNER.iter()), 2, grant, Self::retirement_is_empty, Self::close_granted)
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
            has_animated_primitives: self.has_animated_primitives,
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
    pub abandonment_slot: u8,
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

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        let release=if let Some(draw)=self.draw.as_ref(){draw.next_retirement_release_byte_demand()?}else if let Some(draw)=self.overlay.as_ref(){draw.next_retirement_release_byte_demand()?}else{0};
        prepared_owner_demands(self.terminal_is_empty(),Ok(release))
    }

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let release=self.next_close_demands(grant.maximum_copy_bytes)?.maximum_release_bytes;
        if let Some(step)=prepared_close_gate(grant,self.terminal_is_empty(),Ok(release)){return step}
        if let Some(draw)=self.draw.as_mut(){if !draw.retirement_is_empty(){return prepared_close_child(draw.retire_granted(grant))}self.draw=None;return prepared_close_progress(0)}
        if let Some(draw)=self.overlay.as_mut(){if !draw.retirement_is_empty(){return prepared_close_child(draw.retire_granted(grant))}self.overlay=None;return prepared_close_progress(0)}
        if let Some(permit)=self.permit.as_mut(){let phase=permit.release_phase;if permit.release_step(){self.permit=None}else if permit.release_phase==phase{return Ok(CloseStep::Blocked)}return prepared_close_progress(0)}
        Ok(CloseStep::Complete{progress:Default::default()})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.draw.is_none() && self.overlay.is_none() && self.permit.is_none()
    }
}

impl PreparedRenderInput {
    pub fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        let mut demand = prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }

    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.uploads.get(self.uploads.len().saturating_sub(1)).map_or(Ok(0), PreparedRenderUpload::next_close_copy_byte_demand) }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.uploads.get(self.uploads.len().saturating_sub(1)) { return upload.next_close_release_byte_demand(); }
        if let Some(producer) = self.raster_producers.get(self.raster_producers.len().saturating_sub(1)) { return producer.next_close_release_byte_demand(); }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.get(self.evictions.len().saturating_sub(1)) { return Ok(if key.is_empty() { key.capacity() } else { 0 }); }
        if !self.draw.retirement_is_empty() { return self.draw.next_retirement_release_byte_demand(); }
        if let Some(overlay) = self.overlay.as_ref() { return overlay.next_retirement_release_byte_demand(); }
        
        if !self.damage.is_empty() || !self.clips.is_empty() || !self.directives.is_empty() { return Ok(0); }
        if !self.uploads.terminal_is_empty() { return self.uploads.next_close_release_byte_demand(); }
        if !self.raster_producers.terminal_is_empty() { return self.raster_producers.next_close_release_byte_demand(); }
        if !self.evictions.terminal_is_empty() { return self.evictions.next_close_release_byte_demand(); }
        if !self.damage.terminal_is_empty() { return self.damage.next_close_release_byte_demand(); }
        if !self.clips.terminal_is_empty() { return self.clips.next_close_release_byte_demand(); }
        if !self.directives.terminal_is_empty() { return self.directives.next_close_release_byte_demand(); }
        Ok(0)
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand) { return step; }
        if let Some(upload) = self.uploads.last_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)); }
            self.uploads.pop(); return prepared_close_progress(0);
        }
        if let Some(producer) = self.raster_producers.last_mut() {
            producer.begin_close();
            if !producer.terminal_is_empty() { return prepared_close_child(producer.close_granted(grant)); }
            self.raster_producers.pop(); return prepared_close_progress(0);
        }
        if let Some(PreparedRenderEviction::Mesh { key }) = self.evictions.last_mut() {
            if !key.is_empty() || key.capacity() != 0 { return prepared_close_progress(prepared_close_text(key)); }
            self.evictions.pop(); return prepared_close_progress(0);
        }
        if !self.draw.retirement_is_empty() { return prepared_close_child(self.draw.retire_granted(grant)); }
        if let Some(overlay) = self.overlay.as_mut() {
            if !overlay.retirement_is_empty() { return prepared_close_child(overlay.retire_granted(grant)); }
            self.overlay = None; return prepared_close_progress(0);
        }
        
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() { return prepared_close_progress(0); }
        if !self.uploads.terminal_is_empty() { return prepared_close_child(self.uploads.close_backing_granted(grant)); }
        if !self.raster_producers.terminal_is_empty() { return prepared_close_child(self.raster_producers.close_backing_granted(grant)); }
        if !self.evictions.terminal_is_empty() { return prepared_close_child(self.evictions.close_backing_granted(grant)); }
        if !self.damage.terminal_is_empty() { return prepared_close_child(self.damage.close_backing_granted(grant)); }
        if !self.clips.terminal_is_empty() { return prepared_close_child(self.clips.close_backing_granted(grant)); }
        if !self.directives.terminal_is_empty() { return prepared_close_child(self.directives.close_backing_granted(grant)); }
        if let Some(permit) = self.permit.as_mut() {
            let phase = permit.release_phase;
            if permit.release_step() { self.permit = None; }
            else if permit.release_phase == phase { return Ok(CloseStep::Blocked); }
            return prepared_close_progress(0);
        }
        if self.abandonment_slot != u8::MAX {
            let Some(state) = PREPARED_RENDER_INPUT_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared input abandonment slot is absent")); };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
            self.abandonment_slot = u8::MAX; return prepared_close_progress(0);
        }
        prepared_close_progress(0)
    }

    #[expect(clippy::result_large_err, reason = "Admission failure retains both exact draw owners for incremental retirement; boxing would allocate on refusal.")]
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

    #[cfg(any(test, feature = "testkit"))]
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        match Self::try_new(scene_revision, preview_generation, draw, overlay, time_seconds) {
            Ok(input) => input,
            Err(rejected) => {
                let ledger = PREPARED_RENDER_PROCESS_PERMITS.load(Ordering::Acquire);
                panic!(
                    "test prepared input must fit fixed process permits: {} (held items {}/{PREPARED_RENDER_PROCESS_SLOTS} pages {}/{PREPARED_RENDER_PROCESS_PAGES} backing-units {})",
                    rejected.fault,
                    (ledger >> PREPARED_RENDER_ITEM_SHIFT) & PREPARED_RENDER_ITEM_MASK,
                    (ledger >> PREPARED_RENDER_PAGE_SHIFT) & PREPARED_RENDER_PAGE_MASK,
                    (ledger >> PREPARED_RENDER_BACKING_SHIFT) & PREPARED_RENDER_BACKING_MASK
                )
            }
        }
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_push_upload(&mut self, upload: PreparedRenderUpload) -> Result<(), PreparedRenderUpload> {
        if self.raster_producers.len().checked_add(self.uploads.len()).is_none_or(|items| items >= self.limits.max_upload_items) {
            return Err(upload);
        }
        self.uploads.try_push(upload)
    }

    #[expect(clippy::result_large_err, reason = "The full producer is returned unchanged when fixed slots are exhausted; refusal must not allocate.")]
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
    #[expect(clippy::result_large_err, reason = "A rejected bind returns the exact draw and overlay for incremental retirement without another allocation.")]
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

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
    }

    pub fn terminal_is_empty(&self) -> bool {
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
    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_INPUT_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER.iter()), 2, copy, Self::next_close_demands, Self::terminal_is_empty)
    }

    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_INPUT_ABANDONMENT_STATE.iter().zip(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER.iter()), 2, grant, Self::terminal_is_empty, Self::close_granted)
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
    pub packet: AtomicPtr<PreparedRenderPacket>,
}

impl PreparedRenderMailboxSlot {
    const fn new() -> Self {
        Self { state: AtomicU8::new(0), generation: AtomicU64::new(0), references: AtomicUsize::new(0), packet: AtomicPtr::new(std::ptr::null_mut()) }
    }
}

pub static PREPARED_RENDER_MAILBOX: [PreparedRenderMailboxSlot; PREPARED_RENDER_MAILBOX_SLOTS] = [const { PreparedRenderMailboxSlot::new() }; PREPARED_RENDER_MAILBOX_SLOTS];

/// 📬️ Generation-qualified nonblocking capacity-one packet handoff.
pub struct PreparedRenderReceiver {
    slot: u8,
    generation: u64,
    owned: bool,
}

impl PreparedRenderReceiver {
    fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if !self.owned { return Ok(RetainedCloneGrant::default()); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return prepared_owner_demands(false, Ok(0)); };
        if slot.generation.load(Ordering::Acquire) != self.generation || matches!(slot.state.load(Ordering::Acquire), 0 | 1) { return prepared_owner_demands(false, Ok(0)); }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "prepared mailbox ownership is checked out")); }
        let pointer = slot.packet.load(Ordering::Acquire);
        let demand = if pointer.is_null() { prepared_owner_demands(false, Ok(0)) } else {
            let packet = unsafe { &*pointer };
            if packet.retirement_is_empty() { prepared_owner_demands(false, Ok(size_of::<PreparedRenderPacket>())) } else { packet.next_close_demands(copy) }
        };
        slot.state.store(2, Ordering::Release);
        demand
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if !self.owned { return Ok(0); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { return Ok(0); };
        if slot.generation.load(Ordering::Acquire) != self.generation || matches!(slot.state.load(Ordering::Acquire), 0 | 1) { return Ok(0); }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Err(ValueError::literal(ValueRefusalKind::WorkLimit, "prepared mailbox ownership is checked out")); }
        let pointer = slot.packet.load(Ordering::Acquire);
        let demand = if pointer.is_null() { Ok(0) } else {
            let packet = unsafe { &*pointer };
            if packet.retirement_is_empty() { Ok(size_of::<PreparedRenderPacket>()) } else { packet.next_close_release_byte_demand() }
        };
        slot.state.store(2, Ordering::Release);
        demand
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        if !self.owned { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared mailbox close requires depth")); }
        let Some(slot) = PREPARED_RENDER_MAILBOX.get(usize::from(self.slot)) else { self.owned = false; return prepared_close_progress(0); };
        if slot.generation.load(Ordering::Acquire) != self.generation { self.owned = false; return prepared_close_progress(0); }
        if slot.state.load(Ordering::Acquire) == 1 && slot.packet.load(Ordering::Acquire).is_null() {
            let references = slot.references.fetch_sub(1, Ordering::AcqRel);
            if references == 1 { slot.state.store(0, Ordering::Release); }
            self.owned = false;
            return prepared_close_progress(0);
        }
        if slot.state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
        let pointer = slot.packet.load(Ordering::Acquire);
        if pointer.is_null() { slot.state.store(1, Ordering::Release); return prepared_close_progress(0); }
        let packet = unsafe { &mut *pointer };
        let step = if packet.retirement_is_empty() {
            if grant.maximum_release_bytes < size_of::<PreparedRenderPacket>() { Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }) }
            else {
                slot.packet.store(std::ptr::null_mut(), Ordering::Release);
                unsafe { drop(Box::from_raw(pointer)); }
                slot.state.store(1, Ordering::Release);
                return prepared_close_progress(size_of::<PreparedRenderPacket>());
            }
        } else { prepared_close_child(packet.close_granted(grant)) };
        slot.state.store(2, Ordering::Release);
        step
    }

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

    #[expect(clippy::result_large_err, reason = "A stale or occupied mailbox returns the exact admitted packet instead of allocating an error owner.")]
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

    fn terminal_is_empty(&self) -> bool {
        !self.owned
    }

    /// 🧹 Advances one owner from one mailbox abandoned by its final handle.

    pub fn next_abandoned_close_demands(copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        prepared_abandoned_demands(PREPARED_RENDER_MAILBOX.iter().map(|slot| (&slot.state, &slot.packet)), 4, copy, PreparedRenderPacket::next_close_demands, PreparedRenderPacket::retirement_is_empty)
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        prepared_abandoned_close(PREPARED_RENDER_MAILBOX.iter().map(|slot| (&slot.state, &slot.packet)), 4, grant, PreparedRenderPacket::retirement_is_empty, PreparedRenderPacket::close_granted)
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
    has_animated_primitives: bool,
    pub input: Option<PreparedRenderInput>,
    usage: PreparedRenderUsage,
    section: PreparationSection,
    draw_cursor: DrawMeasureCursor,
    overlay_cursor: DrawMeasureCursor,
    pipeline_cursor: DrawMeasureCursor,
    pipeline_overlay_cursor: DrawMeasureCursor,
    pipeline_overlay: bool,
    metadata_cursor: usize,
    pub commands: Option<PreparedRenderCommandPages>,
    digest: u64,
    fault: Option<&'static str>,
    pub fault_payload: RetainedPayloadBuilder,
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
pub enum DrawMeasureCursor {
    LayerHeader(usize),
    LayerUi { layer: usize, item: usize, overlay: bool },
    LayerVector { layer: usize, item: usize, overlay: bool },
    LayerRaster { layer: usize, raster: usize, overlay: bool },
    LayerRasterKey { layer: usize, raster: usize, byte: usize, overlay: bool },
    PassHeader(usize),
    PassBackdropSnapshot { pass: usize },
    PassSceneClear { pass: usize },
    PassShadowBegin(usize),
    PassShadowInstance { pass: usize, draw: usize, instance: usize },
    PassDraw { pass: usize, draw: usize, translucent: bool },
    PassDrawKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassInstanceKey { pass: usize, draw: usize, instance: usize, byte: usize, translucent: bool },
    PassLine { pass: usize, draw: usize },
    PassLineVertex { pass: usize, draw: usize, vertex: usize },
    PassTextured { pass: usize, draw: usize },
    PassTexturedInstance { pass: usize, draw: usize, instance: usize },
    PassTexturedKey { pass: usize, draw: usize, instance: usize, byte: usize },
    PassGrid { pass: usize },
    PassMaterial { pass: usize, draw: usize, translucent: bool },
    PassMaterialMeshKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassMaterialTextureKey { pass: usize, draw: usize, byte: usize, translucent: bool },
    PassMaterialInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassMaterialInstanceKey { pass: usize, draw: usize, instance: usize, byte: usize, translucent: bool },
    PassPostprocess { pass: usize },
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

    pub fn next_close_demands(&self,copy:usize)->Result<RetainedCloneGrant,ValueError>{match self.input.as_ref(){Some(input)if !input.terminal_is_empty()=>input.next_close_demands(copy),_=>prepared_owner_demands(self.terminal_is_empty(),Ok(0))}}

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let Some(input)=self.input.as_mut()else{return Ok(CloseStep::Complete{progress:Default::default()})};
        if !input.terminal_is_empty(){return prepared_close_child(input.close_step(grant))}
        if grant.maximum_items==0{return Ok(CloseStep::Pending{progress:Default::default()})}
        if grant.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original rejected render input header requires depth"))}
        self.input=None;
        Ok(CloseStep::Complete{progress:RetainedCloneProgress{copied_items:1,..Default::default()}})
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none()
    }
}

impl PreparedRenderJob {
    fn next_close_demands(&self, copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Some(upload) = self.rejected_upload.as_ref() { if !upload.terminal_is_empty() { return upload.next_close_demands(copy); } }
        else if let Some(packet) = self.rejected_packet.as_ref() { if !packet.retirement_is_empty() { return packet.next_close_demands(copy); } }
        else if self.commands.is_none() {
            if let Some(input) = self.input.as_ref() { if !input.terminal_is_empty() { return input.next_close_demands(copy); } }
            else if !self.receiver.terminal_is_empty() { return self.receiver.next_close_demands(copy); }
        }
        if self.rejected_upload.is_none() && self.rejected_packet.is_none() && self.commands.is_none() && self.input.is_none() && self.receiver.terminal_is_empty() && !self.fault_payload.terminal_is_empty() {
            let demand = self.fault_payload.retirement_demands()?;
            return Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth });
        }
        prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())
    }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(upload) = self.rejected_upload.as_ref() { return if upload.terminal_is_empty() { Ok(0) } else { upload.next_close_release_byte_demand() } }
        if let Some(packet) = self.rejected_packet.as_ref() { return if packet.retirement_is_empty() { Ok(0) } else { packet.next_close_release_byte_demand() } }
        if let Some(commands) = self.commands.as_ref() { return commands.next_close_release_byte_demand() }
        if let Some(input) = self.input.as_ref() { return input.next_close_release_byte_demand() }
        if !self.receiver.terminal_is_empty() { return self.receiver.next_close_release_byte_demand(); }
        self.fault_payload.retirement_demands().map(|demand| demand.release_bytes)
    }
    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        if !self.closing { return Ok(CloseStep::Blocked); }
        if self.terminal_is_empty() { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        if grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original prepared render job close requires depth")); }
        if let Some(upload) = self.rejected_upload.as_mut() {
            if !upload.terminal_is_empty() { return prepared_close_child(upload.close_granted(grant)) }
            self.rejected_upload = None; return prepared_close_progress(0)
        }
        if let Some(packet) = self.rejected_packet.as_mut() {
            if !packet.retirement_is_empty() { return prepared_close_child(packet.close_granted(grant)) }
            self.rejected_packet = None; return prepared_close_progress(0)
        }
        if let Some(commands) = self.commands.as_mut() {
            if !commands.terminal_is_empty() { return prepared_close_child(commands.close_granted(grant)) }
            self.commands = None; return prepared_close_progress(0)
        }
        if let Some(input) = self.input.as_mut() {
            if !input.terminal_is_empty() { return prepared_close_child(input.close_granted(grant)) }
            self.input = None; return prepared_close_progress(0)
        }
        if !self.receiver.terminal_is_empty() { return prepared_close_child(self.receiver.close_granted(grant)) }
        if !self.fault_payload.terminal_is_empty() {
            return self.fault_payload.close_step_granted(grant).map(|step|CloseStep::Pending { progress: step.progress() });
        }
        if self.abandonment_slot != u8::MAX {
            let Some(state) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.get(usize::from(self.abandonment_slot)) else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared render job abandonment slot is absent")); };
            let current = state.load(Ordering::Acquire);
            if !matches!(current, 1 | 3) || state.compare_exchange(current, if current == 3 { 3 } else { 0 }, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Blocked); }
            self.abandonment_slot = u8::MAX; return prepared_close_progress(0)
        }
        Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })
    }

    /// 🩺️ The exact reason this job refused, once it has refused — the string [`Self::fault_outcome`]
    /// recorded. The original held fault detail carries no text pages here (a fixed `&'static str`
    /// needs no page), so without this accessor a preparation fault is completely invisible to its
    /// driver, which is exactly how one silent refusal froze the wgpu shell at one presented frame per
    /// boot (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md`).
    pub fn fault(&self) -> Option<&'static str> {
        self.fault
    }

    #[expect(clippy::result_large_err, reason = "Job admission failure retains the exact input and its credits for incremental retirement without allocating.")]
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
            has_animated_primitives: false,
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
            fault_payload: RetainedPayloadBuilder::new(JobPayloadStream::Fault),
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

    #[cfg(any(test, feature = "testkit"))]
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

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.commands.is_none() && self.rejected_upload.is_none() && self.rejected_packet.is_none() && self.receiver.terminal_is_empty() && self.fault_payload.terminal_is_empty() && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact job owner recovered after an interrupted worker execution.

    pub fn next_abandoned_close_demands(maximum_copy_bytes: usize) -> Result<RetainedCloneGrant, ValueError> {
        let Some(index) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(RetainedCloneGrant::default()) };
        let pointer = PREPARED_RENDER_JOB_ABANDONMENT_OWNER[index].load(Ordering::Acquire);
        let result = if pointer.is_null() { Ok(RetainedCloneGrant::default()) } else {
            let job = unsafe { &*pointer };
            if job.terminal_is_empty() { prepared_owner_demands(false, Ok(size_of::<Self>())) }
            else { job.next_close_demands(maximum_copy_bytes) }
        };
        PREPARED_RENDER_JOB_ABANDONMENT_STATE[index].store(2, Ordering::Release);
        result
    }
    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let Some(slot) = PREPARED_RENDER_JOB_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); };
        let pointer = PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() { PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release); return Ok(CloseStep::Blocked); }
        let mut job = unsafe { Box::from_raw(pointer) };
        let step = if job.terminal_is_empty() {
            if grant.maximum_items == 0 || grant.maximum_release_bytes < size_of::<Self>() { Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }) }
            else if grant.maximum_depth == 0 { Err(ValueError::literal(ValueRefusalKind::DepthLimit,"original abandoned prepared job close requires depth")) }
            else {
                drop(job); PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
                return prepared_close_progress(size_of::<Self>())
            }
        } else { job.closing = true; prepared_close_child(job.close_granted(grant)) };
        PREPARED_RENDER_JOB_ABANDONMENT_OWNER[slot].store(Box::into_raw(job), Ordering::Release);
        PREPARED_RENDER_JOB_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        step
    }

    fn input(&self) -> Option<&PreparedRenderInput> {
        self.input.as_ref()
    }

    pub fn next_draw_usage(draw: &DrawList, cursor: &mut DrawMeasureCursor) -> Option<PreparedRenderUsage> {
        let (usage, next) = match *cursor {
            DrawMeasureCursor::LayerHeader(layer) => {
                let Some(value) = draw.layers.get(layer) else {
                    *cursor = DrawMeasureCursor::Complete;
                    return Some(PreparedRenderUsage::default());
                };
                let _ = value;
                let region = draw.glass_regions.partition_point(|region| region.layer_index < layer);
                let next = if draw.glass_regions.get(region).is_some_and(|region| region.layer_index == layer) { DrawMeasureCursor::Glass(region) } else { Self::layer_channel_cursor(draw, layer, 0) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<DrawLayer>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_ui_instances } else { &value.ui_instances };
                let next = if item + 1 < items.len() { DrawMeasureCursor::LayerUi { layer, item: item + 1, overlay } } else { Self::layer_channel_cursor(draw, layer, if overlay { 4 } else { 1 }) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::UiInstance>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } => {
                let value = &draw.layers[layer];
                let items = if overlay { &value.overlay_vector_vertices } else { &value.vector_vertices };
                let next = if item + 1 < items.len() { DrawMeasureCursor::LayerVector { layer, item: item + 1, overlay } } else { Self::layer_channel_cursor(draw, layer, if overlay { 5 } else { 2 }) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::VectorVertex>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::LayerRaster { layer, raster, overlay } => {
                let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
                let value = &instances[raster];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::UiInstance>(), ..PreparedRenderUsage::default() };
                let next = if value.0.is_empty() { Self::next_layer_raster(draw, layer, raster, overlay) } else { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: 0, overlay } };
                (usage, next)
            }
            DrawMeasureCursor::LayerRasterKey { layer, raster, byte, overlay } => {
                let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
                let key = &instances[raster].0;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::LayerRasterKey { layer, raster, byte: byte + 1, overlay } } else { Self::next_layer_raster(draw, layer, raster, overlay) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassHeader(pass) => {
                let Some(value) = draw.scene_passes.get(pass) else {
                    *cursor = DrawMeasureCursor::Complete;
                    return Some(PreparedRenderUsage::default());
                };
                let next = if value.viewport_mask == semio_framework_ui_scene_pass::SceneViewportMask3d::Ellipse { DrawMeasureCursor::PassBackdropSnapshot { pass } } else { Self::next_after_scene_prelude(draw, pass) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::ScenePass3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassBackdropSnapshot { pass } => (PreparedRenderUsage { draw_items: 1, ..PreparedRenderUsage::default() }, Self::next_after_scene_prelude(draw, pass)),
            DrawMeasureCursor::PassSceneClear { pass } => {
                let next = if draw.scene_passes[pass].shadow.enabled && draw.scene_passes[pass].shadow_draws.iter().any(|draw| draw.shadow_role.casts && !draw.instances.is_empty()) {
                    DrawMeasureCursor::PassShadowBegin(pass)
                } else {
                    Self::next_after_shadow(draw, pass)
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<[f32; 4]>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassShadowBegin(pass) => {
                let next = Self::first_shadow_instance(draw, pass).unwrap_or_else(|| Self::next_after_shadow(draw, pass));
                (PreparedRenderUsage { draw_items: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance } => {
                let draw_value = &draw.scene_passes[pass].shadow_draws[draw_index];
                let instance_value = &draw_value.instances[instance];
                let next = Self::next_shadow_instance(draw, pass, draw_index, instance);
                let draw_bytes = size_of::<semio_framework_ui_scene_mesh::Instance3d>() + instance_value.id.len() + if instance == 0 { size_of::<semio_framework_ui_scene_pass::SceneDraw3d>() + draw_value.mesh_key.len() } else { 0 };
                (PreparedRenderUsage { draw_items: 1, draw_bytes, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassDraw { pass, draw: draw_index, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneDraw3d>(), ..PreparedRenderUsage::default() };
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
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_mesh::Instance3d>(), ..PreparedRenderUsage::default() };
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
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::LineDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } => {
                let next = if vertex + 1 < draw.scene_passes[pass].line_draws[draw_index].vertices.len() { DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex: vertex + 1 } } else { Self::next_pass_line(draw, pass, draw_index) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::LineVertex3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTextured { pass, draw: draw_index } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index];
                let next = if value.instances.is_empty() { Self::next_textured_draw(draw, pass, draw_index) } else { DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: 0 } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::TexturedDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::TexturedInstance3d>(), ..PreparedRenderUsage::default() };
                let next = if value.texture_key.is_empty() { Self::next_textured_instance(draw, pass, draw_index, instance) } else { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: 0 } };
                (usage, next)
            }
            DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte } => {
                let key = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance].texture_key;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassTexturedKey { pass, draw: draw_index, instance, byte: byte + 1 } } else { Self::next_textured_instance(draw, pass, draw_index, instance) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassGrid { pass } => {
                let next = Self::next_after_grid(draw, pass);
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::ProceduralGrid3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterial { pass, draw: draw_index, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index];
                let next = if !value.mesh_key.is_empty() { DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte: 0, translucent } } else { Self::next_material_after_keys(draw, pass, draw_index, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneMaterialDraw3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index];
                let next = if byte + 1 < value.mesh_key.len() { DrawMeasureCursor::PassMaterialMeshKey { pass, draw: draw_index, byte: byte + 1, translucent } } else { Self::next_material_after_keys(draw, pass, draw_index, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte, translucent } => {
                let texture_bytes:usize = draw.scene_passes[pass].material_draws[draw_index].material.texture_keys().map(str::len).sum();
                let next = if byte + 1 < texture_bytes {
                    DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte: byte + 1, translucent }
                } else if draw.scene_passes[pass].material_draws[draw_index].instances.is_empty() {
                    Self::next_material_draw(draw, pass, draw_index, translucent)
                } else {
                    DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: 0, translucent }
                };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance, translucent } => {
                let value = &draw.scene_passes[pass].material_draws[draw_index].instances[instance];
                let next = if value.id.is_empty() { Self::next_material_instance(draw, pass, draw_index, instance, translucent) } else { DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte: 0, translucent } };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_mesh::Instance3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte, translucent } => {
                let key = &draw.scene_passes[pass].material_draws[draw_index].instances[instance].id;
                let next = if byte + 1 < key.len() { DrawMeasureCursor::PassMaterialInstanceKey { pass, draw: draw_index, instance, byte: byte + 1, translucent } } else { Self::next_material_instance(draw, pass, draw_index, instance, translucent) };
                (PreparedRenderUsage { draw_items: 1, draw_bytes: 1, ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::PassPostprocess { pass } => {
                let next = Self::next_after_scene_pass_content(draw, pass);
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_scene_pass::SceneCurvilinear3d>(), ..PreparedRenderUsage::default() }, next)
            }
            DrawMeasureCursor::Glass(index) => {
                let region = draw.glass_regions.get(index)?;
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<semio_framework_ui_draw_contract::GlassRegion>(), ..PreparedRenderUsage::default() }, Self::layer_channel_cursor(draw, region.layer_index, 0))
            }
            DrawMeasureCursor::Complete => return None,
        };
        *cursor = next;
        Some(usage)
    }

    fn next_layer_raster(draw: &DrawList, layer: usize, raster: usize, overlay: bool) -> DrawMeasureCursor {
        let instances = if overlay { &draw.layers[layer].overlay_raster_instances } else { &draw.layers[layer].raster_instances };
        if raster + 1 < instances.len() {
            DrawMeasureCursor::LayerRaster { layer, raster: raster + 1, overlay }
        } else if overlay {
            Self::layer_channel_cursor(draw, layer, 6)
        } else {
            Self::next_after_layer(draw, layer)
        }
    }

    /// 🥞️ The six channels one `DrawLayer` publishes. Normal ui, vector, and raster channels
    /// terminate at that layer's scene passes; its overlay ui, vector, and raster channels resume
    /// after those passes and before the following layer.
    ///
    /// 🩸️ This walk used to put the RASTERS last, after both overlay channels, so within one layer an
    /// engine surface's opaque vello texture was composited ON TOP of the overlay glyphs painted over
    /// it. The two renderers therefore disagreed about what "overlay" means, and the browser one —
    /// which is the prepared one — lost every caption the shell paints over an engine raster: the live
    /// generation3d flow window on 6118 drew all seven node bodies, every port, every wire and the
    /// minimap, and not one word (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
    /// `📓️w14b-generation3d-labels-preview-layout.md`).
    fn layer_channel_cursor(draw: &DrawList, layer: usize, from_channel: u8) -> DrawMeasureCursor {
        let Some(value) = draw.layers.get(layer) else {
            return DrawMeasureCursor::Complete;
        };
        let terminal_channel = if from_channel < 3 { 3 } else { 6 };
        for channel in from_channel..terminal_channel {
            match channel {
                0 if !value.ui_instances.is_empty() => return DrawMeasureCursor::LayerUi { layer, item: 0, overlay: false },
                1 if !value.vector_vertices.is_empty() => return DrawMeasureCursor::LayerVector { layer, item: 0, overlay: false },
                2 if !value.raster_instances.is_empty() => return DrawMeasureCursor::LayerRaster { layer, raster: 0, overlay: false },
                3 if !value.overlay_ui_instances.is_empty() => return DrawMeasureCursor::LayerUi { layer, item: 0, overlay: true },
                4 if !value.overlay_vector_vertices.is_empty() => return DrawMeasureCursor::LayerVector { layer, item: 0, overlay: true },
                5 if !value.overlay_raster_instances.is_empty() => return DrawMeasureCursor::LayerRaster { layer, raster: 0, overlay: true },
                _ => {}
            }
        }
        if from_channel < 3 {
            Self::next_after_layer(draw, layer)
        } else {
            DrawMeasureCursor::LayerHeader(layer + 1)
        }
    }

    fn next_after_layer(draw: &DrawList, layer: usize) -> DrawMeasureCursor {
        let pass = draw.scene_passes.partition_point(|pass| pass.layer_index < layer);
        if draw.scene_passes.get(pass).is_some_and(|pass| pass.layer_index == layer) {
            DrawMeasureCursor::PassHeader(pass)
        } else {
            Self::layer_channel_cursor(draw, layer, 3)
        }
    }

    fn next_after_scene_pass(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes.get(pass).is_some_and(|value| value.curvilinear.is_some() || value.viewport_mask == semio_framework_ui_scene_pass::SceneViewportMask3d::Ellipse) {
            return DrawMeasureCursor::PassPostprocess { pass };
        }
        Self::next_after_scene_pass_content(draw, pass)
    }

    fn next_after_scene_prelude(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass];
        if value.clear_color.is_some() {
            DrawMeasureCursor::PassSceneClear { pass }
        } else if value.shadow.enabled && value.shadow_draws.iter().any(|draw| draw.shadow_role.casts && !draw.instances.is_empty()) {
            DrawMeasureCursor::PassShadowBegin(pass)
        } else {
            Self::next_after_shadow(draw, pass)
        }
    }

    fn next_after_scene_pass_content(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        let Some(value) = draw.scene_passes.get(pass) else { return DrawMeasureCursor::Complete };
        if draw.scene_passes.get(pass + 1).is_some_and(|next| next.layer_index == value.layer_index) {
            DrawMeasureCursor::PassHeader(pass + 1)
        } else {
            Self::layer_channel_cursor(draw, value.layer_index, 3)
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

    fn first_shadow_instance(draw: &DrawList, pass: usize) -> Option<DrawMeasureCursor> {
        draw.scene_passes[pass].shadow_draws.iter().position(|draw| draw.shadow_role.casts && !draw.instances.is_empty()).map(|draw| DrawMeasureCursor::PassShadowInstance { pass, draw, instance: 0 })
    }

    fn next_shadow_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize) -> DrawMeasureCursor {
        let draws = &draw.scene_passes[pass].shadow_draws;
        if instance + 1 < draws[draw_index].instances.len() {
            return DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance: instance + 1 };
        }
        if let Some(next_draw) = draws.iter().enumerate().skip(draw_index + 1).find_map(|(index, draw)| (draw.shadow_role.casts && !draw.instances.is_empty()).then_some(index)) {
            return DrawMeasureCursor::PassShadowInstance { pass, draw: next_draw, instance: 0 };
        }
        Self::next_after_shadow(draw, pass)
    }

    fn next_after_shadow(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes[pass].textured_draws.is_empty() {
            Self::next_after_textured(draw, pass)
        } else {
            DrawMeasureCursor::PassTextured { pass, draw: 0 }
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

    fn first_material(draw: &DrawList, pass: usize, translucent: bool) -> Option<DrawMeasureCursor> {
        draw.scene_passes[pass].material_draws.iter().position(|value| value.translucent == translucent && !value.instances.is_empty()).map(|draw| DrawMeasureCursor::PassMaterial { pass, draw, translucent })
    }

    fn next_material_after_keys(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        let value = &draw.scene_passes[pass].material_draws[draw_index];
        match &value.material {
            material if material.texture_keys().any(|texture_key| !texture_key.is_empty()) => DrawMeasureCursor::PassMaterialTextureKey { pass, draw: draw_index, byte: 0, translucent },
            _ if value.instances.is_empty() => Self::next_material_draw(draw, pass, draw_index, translucent),
            _ => DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: 0, translucent },
        }
    }

    fn next_material_instance(draw: &DrawList, pass: usize, draw_index: usize, instance: usize, translucent: bool) -> DrawMeasureCursor {
        if instance + 1 < draw.scene_passes[pass].material_draws[draw_index].instances.len() {
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance: instance + 1, translucent }
        } else {
            Self::next_material_draw(draw, pass, draw_index, translucent)
        }
    }

    fn next_material_draw(draw: &DrawList, pass: usize, draw_index: usize, translucent: bool) -> DrawMeasureCursor {
        if let Some((next, _)) = draw.scene_passes[pass].material_draws.iter().enumerate().skip(draw_index + 1).find(|(_, value)| value.translucent == translucent && !value.instances.is_empty()) {
            DrawMeasureCursor::PassMaterial { pass, draw: next, translucent }
        } else if translucent {
            Self::next_after_scene_pass(draw, pass)
        } else if !draw.scene_passes[pass].draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false }
        } else {
            Self::next_after_opaque(draw, pass)
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

    /// 🖼️ A scene pass is measured — and therefore encoded — TEXTURED → opaque → lines → translucent.
    ///
    /// 🩸️ The textured underlay used to come last, so the world reference plane composited OVER the
    /// grid and the model and tinted everything it covered. React draws the same plane at
    /// `renderOrder = -10` (`🎨️r3f/🟦️.tsx`'s `WorldReferencePlaneItem`), under the grid's `-5` and the
    /// model's default `0`. The pass is a non-depth-writing `LessEqual` blend either way, so drawing it
    /// first is what makes the geometry in front of it actually occlude it (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w7b-presenter-one-frame-per-boot.md` §4).
    fn next_after_textured(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if draw.scene_passes[pass].procedural_grid.is_some() {
            DrawMeasureCursor::PassGrid { pass }
        } else {
            Self::next_after_grid(draw, pass)
        }
    }

    fn next_after_grid(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        if let Some(cursor) = Self::first_material(draw, pass, false) {
            cursor
        } else if !draw.scene_passes[pass].draws.is_empty() {
            DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false }
        } else {
            Self::next_after_opaque(draw, pass)
        }
    }

    fn next_after_translucent(draw: &DrawList, pass: usize) -> DrawMeasureCursor {
        Self::first_material(draw, pass, true).unwrap_or_else(|| Self::next_after_scene_pass(draw, pass))
    }

    fn next_textured_draw(draw: &DrawList, pass: usize, draw_index: usize) -> DrawMeasureCursor {
        if draw_index + 1 < draw.scene_passes[pass].textured_draws.len() {
            DrawMeasureCursor::PassTextured { pass, draw: draw_index + 1 }
        } else {
            Self::next_after_textured(draw, pass)
        }
    }

    fn measure_next(&mut self) -> Option<PreparedRenderUsage> {
        match self.section {
            PreparationSection::Draw => {
                let input = self.input.as_ref()?;
                self.has_animated_primitives |= Self::animated_primitive_at(&input.draw, self.draw_cursor);
                if let Some(usage) = Self::next_draw_usage(&input.draw, &mut self.draw_cursor) {
                    return Some(usage);
                }
                self.section = PreparationSection::Overlay;
                Some(PreparedRenderUsage::default())
            }
            PreparationSection::Overlay => {
                let input = self.input.as_ref()?;
                if let Some(overlay) = &input.overlay {
                    self.has_animated_primitives |= Self::animated_primitive_at(overlay, self.overlay_cursor);
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
                    let scene_raster_bytes = upload.scene_raster_byte_len();
                    let Some(digest) = u64::try_from(bytes).ok().zip(u64::try_from(scene_raster_bytes).ok()).map(|(bytes, scene)| bytes ^ scene.rotate_left(31)) else {
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
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: bytes, scene_raster_bytes, ..PreparedRenderUsage::default() });
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

    fn animated_primitive_at(draw: &DrawList, cursor: DrawMeasureCursor) -> bool {
        let DrawMeasureCursor::LayerUi { layer, item, overlay } = cursor else { return false };
        let Some(layer) = draw.layers.get(layer) else { return false };
        let items = if overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
        items.get(item).is_some_and(|instance| {
            let kind = instance.params[2];
            kind == semio_framework_ui_draw_contract::KIND_LOADING_BORDER || kind == semio_framework_ui_draw_contract::KIND_WAITING_BORDER || kind == semio_framework_ui_draw_contract::KIND_INTRODUCING_BORDER
        })
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
        self.usage.include_draw(usage.draw_items, usage.draw_bytes) && (usage.upload_items == 0 || self.usage.include_upload(usage.upload_bytes, usage.scene_raster_bytes))
    }

    fn fault_outcome<'a>(&'a mut self, fault: &'static str, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        self.fault = Some(fault);
        self.closing = true;
        self.publish_fault(cx)
    }

    fn publish_fault<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if !self.fault_payload.advance_initialization(cx)? || !self.fault_payload.seal(cx)? { return Ok(None); }
        JobOutcomeBorrow::admit_fault(cx, self.fault_payload.published().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared fault owner was not sealed"))?).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn complete<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if !self.packet_command_staged {
            let command = PreparedRenderCommand { kind: PreparedRenderCommandKind::Packet, source: 0, digest: self.digest, draw_cursor: None, packet_overlay: false };
            let Some(commands) = self.commands.as_mut() else { return self.fault_outcome("prepared render command owner was missing", cx) };
            if commands.try_push(command).is_err() {
                return self.fault_outcome("prepared render packet command credits exhausted", cx);
            }
            self.packet_command_staged = true;
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if !self.published {
            let Some(input) = self.input.take() else { return self.fault_outcome("prepared render input was missing before publication", cx) };
            if !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
                self.input = Some(input);
                return self.fault_outcome("prepared render process generation was stale before publication", cx);
            }
            let revision = input.scene_revision;
            let generation = input.preview_generation;
            let Some(commands) = self.commands.take() else {
                self.input = Some(input);
                return self.fault_outcome("prepared render command pages were missing before publication", cx);
            };
            let input = std::mem::ManuallyDrop::new(input);
            let packet = PreparedRenderPacket {
                has_animated_primitives: self.has_animated_primitives,
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
                    return self.fault_outcome("prepared render packet abandonment admission was refused", cx);
                }
            };
            if let Err(packet) = self.receiver.publish(packet) {
                self.rejected_packet = Some(packet);
                return self.fault_outcome("prepared render packet mailbox publication refused", cx);
            }
            self.published = true;
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        JobOutcomeBorrow::admit_complete(cx, None, None).map_err(semio_framework_job::OriginalTurnFailure::from)
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
            has_animated_primitives: self.has_animated_primitives,
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
            fault_payload: std::mem::replace(&mut self.fault_payload, RetainedPayloadBuilder::new(JobPayloadStream::Fault)),
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
    fn step<'a>(&'a mut self, cx: &mut StepContext<'_>) -> Result<Option<JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        let grant = cx.retained_grant();
        if grant.maximum_items == 0 || grant.maximum_depth == 0 { return Ok(None); }
        if cx.is_cancelled() {
            self.closing = true;
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if cx.should_yield() { return Ok(None); }
        if self.fault.is_some() { return self.publish_fault(cx); }
        if self.closing { return self.fault_outcome("prepared render job was stepped while closing", cx); }
        let Some(input) = self.input() else {
            return self.complete(cx);
        };
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation is stale", cx);
        }
        if let Some(producer) = self.input.as_mut().and_then(|input| input.raster_producers.front_mut()) {
            cx.consume_fuel(1);
            let step = producer.step(cx.generation().0);
            match step {
                PreparedRasterProducerStep::Pending => {},
                PreparedRasterProducerStep::Complete(upload) => {
                    let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared during raster publication", cx) };
                    input.raster_producers.pop_front();
                    if let Err(upload) = input.try_push_upload(upload) {
                        self.rejected_upload = Some(upload);
                        return self.fault_outcome("prepared render upload admission refused the exact raster owner", cx);
                    }
                }
                PreparedRasterProducerStep::Fault(fault) => return self.fault_outcome(fault, cx),
            }
            if cx.is_cancelled() {
                self.closing = true;
                return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
            }
            if self.input().is_some_and(|input| input.preview_generation != cx.generation().0) {
                return self.fault_outcome("prepared render generation became stale after raster work", cx);
            }
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if !self.raster_backing_closed {
            cx.consume_fuel(1);
            let Some(input) = self.input.as_mut() else { return self.fault_outcome("prepared render input disappeared before producer backing close", cx) };
            if input.raster_producers.release_backing_step() {
                self.raster_backing_closed = true;
            }
            return JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        let Some(usage) = self.measure_next() else {
            return self.complete(cx);
        };
        cx.consume_fuel(1);
        if self.fault.is_some() || !self.include_usage(usage) {
            return self.fault_outcome("prepared render cumulative credits overflowed", cx);
        }
        let Some(input) = self.input() else { return self.fault_outcome("prepared render input disappeared after one worker unit", cx) };
        if !self.usage.fits(input.limits) {
            return self.fault_outcome("prepared render credits exceeded", cx);
        }
        if cx.is_cancelled() {
            self.closing = true;
            return JobOutcomeBorrow::admit_cancelled(cx).map_err(semio_framework_job::OriginalTurnFailure::from);
        }
        if input.preview_generation != cx.generation().0 || !input.permit.as_ref().is_some_and(PreparedRenderProcessPermit::matches) {
            return self.fault_outcome("prepared render generation became stale after one worker unit", cx);
        }
        JobOutcomeBorrow::admit_yield(cx).map_err(semio_framework_job::OriginalTurnFailure::from)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a JobOutcomeDescriptor) -> Result<JobOutcomeView<'a>, ValueError> {
        match descriptor.kind() {
            JobOutcomeKind::Yield => descriptor.yielded(),
            JobOutcomeKind::Complete if self.published => descriptor.complete(None, None),
            JobOutcomeKind::Cancelled => descriptor.cancelled(),
            JobOutcomeKind::Fault if self.fault.is_some() => descriptor.fault(self.fault_payload.published().ok_or_else(|| ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared original fault custody was not published"))?),
            _ => Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "prepared outcome descriptor does not match its original owner")),
        }
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, cx: &mut StepContext<'_>) -> Result<CloseStep,semio_framework_job::OriginalTurnFailure> {
        let grant=cx.retained_grant();
        if cx.should_yield(){return Ok(CloseStep::Pending{progress:Default::default()})}
        let step=self.close_granted(grant).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        let step=step.admit(grant,self.terminal_is_empty()).map_err(|error|semio_framework_job::OriginalTurnFailure::debit_original(error,cx))?;
        cx.consume_retained(step.progress()).map_err(semio_framework_job::OriginalTurnFailure::from)?;
        if step.progress().copied_items!=0{cx.consume_fuel(1)}
        Ok(step)
    }
    fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_copy_bytes) }
    fn next_close_capacity_byte_demand(&self, _copy: usize) -> Result<usize, ValueError> { self.next_close_demands(_copy).map(|grant| grant.maximum_capacity_bytes) }
    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_release_bytes) }
    fn next_close_depth_demand(&self) -> Result<usize, ValueError> { self.next_close_demands(0).map(|grant| grant.maximum_depth) }

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
    pub sequence: u64,
    pub scene_revision: u64,
    pub preview_generation: u64,
}

/// ♻️ Previous last-valid packet returned only after exact presenter acknowledgement.
pub struct PreparedRenderReplacement {
    pub previous: Option<PreparedRenderPacket>,
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

    #[expect(clippy::result_large_err, reason = "A refused presentation returns the exact packet without allocating at the bounded presentation boundary.")]
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
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub struct OffscreenPresentToken {
    _worker_isolate: std::marker::PhantomData<Rc<()>>,
}

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
impl OffscreenPresentToken {
    pub fn mint_for_dedicated_worker() -> Result<Self, &'static str> {
        // 🛡️ `web_sys` is target-gated; detect the UI isolate via `js_sys` only so `wgpu-engine`
        // wasm builds do not require the `Window` binding to resolve at compile time.
        #[cfg(feature = "browser-presenter")]
        {
            let global = js_sys::global();
            if js_sys::Reflect::has(&global, &js_sys::JsString::from("document")).unwrap_or(false) {
                return Err("offscreen presentation authority cannot be minted in the browser UI isolate");
            }
        }
        Ok(Self { _worker_isolate: std::marker::PhantomData })
    }
}
//#endregion 🛡️PresentationGate



#[cfg(test)]
extern crate self as semio_framework_ui_prepared;
#[cfg(any(test,feature="testkit"))]
#[path="🧪️testing/🦀️.rs"]
pub mod testing;
