//! 📊️ Credits, limits and fixed lists: the abandoned demands, the render limits and usage, the metadata and command page constants, the fixed page and list and the process permit. See [`super`].

use super::*;

pub(super) fn prepared_abandoned_demands<'a, T: 'a>(mut slots: impl Iterator<Item = (&'a AtomicU8, &'a AtomicPtr<T>)>, queued: u8, copy: usize, demand: fn(&T, usize) -> Result<RetainedCloneGrant, ValueError>, empty: fn(&T) -> bool) -> Result<RetainedCloneGrant, ValueError> {
    let Some((state, owner)) = slots.find(|(state, _)| state.compare_exchange(queued, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Ok(RetainedCloneGrant::default()) };
    let pointer = owner.load(Ordering::Acquire);
    let result = if pointer.is_null() { Ok(RetainedCloneGrant::default()) } else {
        let value = unsafe { &*pointer };
        if empty(value) { prepared_owner_demands(false, Ok(size_of::<T>())) } else { demand(value, copy) }
    };
    state.store(queued, Ordering::Release);
    result
}

pub(super) fn prepared_abandoned_close<'a, T: 'a>(mut slots: impl Iterator<Item = (&'a AtomicU8, &'a AtomicPtr<T>)>, queued: u8, grant: RetainedCloneGrant, empty: fn(&T) -> bool, close: fn(&mut T, RetainedCloneGrant) -> Result<CloseStep,ValueError>) -> Result<CloseStep,ValueError> {
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
    pub(super) fn include_draw(&mut self, items: usize, bytes: usize) -> bool {
        let Some(draw_items) = self.draw_items.checked_add(items) else { return false };
        let Some(draw_bytes) = self.draw_bytes.checked_add(bytes) else { return false };
        self.draw_items = draw_items;
        self.draw_bytes = draw_bytes;
        true
    }

    pub(super) fn include_upload(&mut self, bytes: usize, scene_raster_bytes: usize) -> bool {
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

pub(super) const PREPARED_RENDER_PROCESS_SLOTS: usize = 64;

pub(super) const PREPARED_RENDER_PROCESS_PAGES: usize = 16_383;

const PREPARED_RENDER_PROCESS_BACKING_BYTES: usize = 128 * 1024 * 1024;

#[derive(Debug)]
pub struct PreparedFixedPage<T> {
    pub(super) slots: [Option<T>; PREPARED_RENDER_METADATA_PAGE_ITEMS],
    pub(super) len: usize,
}

impl<T> Default for PreparedFixedPage<T> {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), len: 0 }
    }
}

/// 🧰 Fixed retained page list whose backing and each populated owner are closed independently.
#[derive(Debug)]
pub struct PreparedFixedList<T> {
    pub(super) pages: [Option<Box<PreparedFixedPage<T>>>; PREPARED_RENDER_METADATA_PAGES],
    head: usize,
    pub(super) len: usize,
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

    pub(super) fn release_backing_step(&mut self) -> bool {
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

    pub(super) fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.head == 0 && self.pages.iter().all(Option::is_none)
    }

    pub(super) fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        Ok(if self.len == 0 && self.pages.iter().any(Option::is_some) { size_of::<PreparedFixedPage<T>>() } else { 0 })
    }

    pub(super) fn close_backing_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        if self.len != 0 { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original prepared fixed pages still hold members")); }
        self.release_backing_step();
        prepared_close_progress(demand.unwrap_or(0))
    }
}

pub(super) const PREPARED_RENDER_ITEM_SHIFT: u32 = 0;

pub(super) const PREPARED_RENDER_PAGE_SHIFT: u32 = 7;

pub(super) const PREPARED_RENDER_BACKING_SHIFT: u32 = 21;

pub(super) const PREPARED_RENDER_ITEM_MASK: u64 = 0x7f;

pub(super) const PREPARED_RENDER_PAGE_MASK: u64 = 0x3fff;

pub(super) const PREPARED_RENDER_BACKING_MASK: u64 = 0x3fff;

pub static PREPARED_RENDER_PROCESS_PERMITS: AtomicU64 = AtomicU64::new(0);

static PREPARED_RENDER_SLOT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];

static PREPARED_RENDER_SLOT_GENERATION: [AtomicU64; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU64::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];

pub(super) static PREPARED_RENDER_PACKET_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];

pub(super) static PREPARED_RENDER_PACKET_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderPacket>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];

pub(super) static PREPARED_RENDER_JOB_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];

pub(super) static PREPARED_RENDER_JOB_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderJob>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];

pub static PREPARED_RENDER_INPUT_ABANDONMENT_STATE: [AtomicU8; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_RENDER_PROCESS_SLOTS];

pub static PREPARED_RENDER_INPUT_ABANDONMENT_OWNER: [AtomicPtr<PreparedRenderInput>; PREPARED_RENDER_PROCESS_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_RENDER_PROCESS_SLOTS];

pub(super) fn prepared_render_units(bytes: usize) -> Option<usize> {
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
    pub(super) slot: u8,
    pub(super) generation: u64,
    pub(super) pages: usize,
    backing_units: usize,
    pub(super) release_phase: u8,
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

    pub(super) fn matches(&self) -> bool {
        let slot = usize::from(self.slot);
        PREPARED_RENDER_SLOT_STATE.get(slot).is_some_and(|state| state.load(Ordering::Acquire) == 1) && PREPARED_RENDER_SLOT_GENERATION.get(slot).is_some_and(|generation| generation.load(Ordering::Acquire) == self.generation)
    }

    pub(super) fn try_grow(&mut self, pages: usize, backing_bytes: usize) -> bool {
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

    pub(super) fn release_step(&mut self) -> bool {
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
