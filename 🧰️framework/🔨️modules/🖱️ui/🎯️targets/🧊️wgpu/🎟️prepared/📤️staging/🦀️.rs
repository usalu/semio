//! 📤️ Original atlas and raster upload staging with independent credits and retirement.
use semio_framework_ui_scene_mesh::Mesh3dLease;
use semio_framework_job::{InteractiveJobCloseStep as CloseStep, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::mem::size_of;
use std::sync::atomic::{AtomicPtr, AtomicU64, AtomicU8, Ordering};
use std::sync::{LazyLock, Mutex};
#[path = "♻️retirement/🦀️.rs"]
pub mod retirement;
use retirement::*;

//#region 🧩️PagedRasterProducer
pub use semio_framework_ui_raster_owner::RASTER_CONTENT_PAGE_BYTES as PREPARED_RASTER_PAGE_BYTES;

pub use semio_framework_ui_raster_owner::RasterContentIdentity;
const PREPARED_RASTER_KEY_BYTES: usize = 256;
/// 🖼️ The straight-RGBA ceiling ONE raster item may carry into a prepared packet. `pub` because a
/// PRODUCER of raster sources has to size its own decode against it: the world reference underlay
/// publishes whatever a photograph or plan happens to be, and a source over this ceiling is refused at
/// admission — which the frame reports as a build fault, not as one missing texture (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
pub const PREPARED_RASTER_ITEM_BYTES: usize = 16 * 1024 * 1024;
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
    content_identity: RasterContentIdentity,
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

    fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        if self.release_phase > 3 { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); }
        let progress = atlas_progress(size_of::<u64>() * 2 + size_of::<u8>(), 0, 0);
        if let Some(step) = atlas_gate(grant, progress) { return step; }
        let delta = match self.release_phase {
            0 => prepared_atlas_delta(0, 0, 0, self.backing_units),
            1 => prepared_atlas_delta(0, 0, self.payload_units, 0),
            2 => prepared_atlas_delta(0, self.pages, 0, 0),
            3 => prepared_atlas_delta(self.items, 0, 0, 0),
            _ => None,
        };
        let Some(delta) = delta else { return atlas_refused(ValueRefusalKind::InvariantViolated); };
        let current = PREPARED_ATLAS_PROCESS_PERMITS.load(Ordering::Acquire);
        let Some(next) = current.checked_sub(delta) else { return atlas_refused(ValueRefusalKind::InvariantViolated); };
        if PREPARED_ATLAS_PROCESS_PERMITS.compare_exchange(current, next, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() });
        }
        self.release_phase += 1;
        Ok(CloseStep::Pending { progress })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedAtlasPage {
    bytes: Box<[u8; PREPARED_ATLAS_PAGE_BYTES]>,
    len: u16,
    start_row: u32,
    rows: u32,
}

/// 🎟️ Independent original construction and retirement authority for one atlas owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedAtlasAuthority {
    pub normal: RetainedCloneGrant,
    pub retirement: RetainedCloneGrant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedAtlasPages {
    owner: Option<Box<PreparedAtlasAbandonment>>,
    width: u32,
    height: u32,
    channels: u8,
    byte_len: usize,
    rows_per_page: u32,
    abandonment_slot: u8,
    observed: RetainedCloneProgress,
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedAtlasAbandonment {
    slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>,
    len: usize,
    permit: Option<PreparedAtlasPermit>,
    authority: PreparedAtlasAuthority,
    handoff_pending: bool,
    closing: bool,
}

fn atlas_progress(copied_bytes: usize, retained_capacity_bytes: usize, released_bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes, released_bytes }
}

fn atlas_refused(kind: ValueRefusalKind) -> Result<CloseStep,ValueError> {
    Err(ValueError::literal(kind,"original prepared atlas close refused"))
}

fn atlas_gate(grant: RetainedCloneGrant, progress: RetainedCloneProgress) -> Option<Result<CloseStep,ValueError>> {
    if grant.maximum_depth == 0 { return Some(atlas_refused(ValueRefusalKind::DepthLimit)); }
    (!progress.fits(grant)).then_some(Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }))
}

fn atlas_cap(received: RetainedCloneGrant, original: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: received.maximum_items.min(original.maximum_items), maximum_copy_bytes: received.maximum_copy_bytes.min(original.maximum_copy_bytes), maximum_capacity_bytes: received.maximum_capacity_bytes.min(original.maximum_capacity_bytes), maximum_release_bytes: received.maximum_release_bytes.min(original.maximum_release_bytes), maximum_depth: received.maximum_depth.min(original.maximum_depth) }
}

fn atlas_add(total: &mut RetainedCloneProgress, progress: RetainedCloneProgress) {
    total.copied_items += progress.copied_items;
    total.copied_bytes += progress.copied_bytes;
    total.retained_capacity_bytes += progress.retained_capacity_bytes;
    total.released_bytes += progress.released_bytes;
}

impl PreparedAtlasAbandonment {
    fn terminal_is_empty(&self) -> bool { self.len == 0 && self.slots.is_none() && self.permit.is_none() }

    fn original_close_progress(&self) -> RetainedCloneProgress {
        if self.handoff_pending { return atlas_progress(size_of::<*mut Self>() + size_of::<u8>() + size_of::<bool>() * 3, 0, 0); }
        if self.len != 0 { return atlas_progress(size_of::<Option<PreparedAtlasPage>>() + size_of::<usize>() + size_of::<bool>(), 0, PREPARED_ATLAS_PAGE_BYTES); }
        if self.slots.is_some() { return atlas_progress(size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>() + size_of::<Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>>() + size_of::<bool>(), 0, size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>()); }
        if let Some(permit) = &self.permit {
            return atlas_progress((if permit.release_phase <= 3 { size_of::<u64>() * 2 + size_of::<u8>() } else { size_of::<Option<PreparedAtlasPermit>>() }) + size_of::<bool>(), 0, 0);
        }
        atlas_progress(size_of::<Self>() + size_of::<*mut Self>() + size_of::<u8>(), 0, size_of::<Self>())
    }

    fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let grant = atlas_cap(grant, self.authority.retirement);
        let progress = self.original_close_progress();
        if let Some(step) = atlas_gate(grant, progress) { return step; }
        if self.handoff_pending { self.handoff_pending = false; self.closing = true; return Ok(CloseStep::Pending { progress }); }
        if let Some(index) = self.len.checked_sub(1) {
            let Some(slots) = self.slots.as_mut() else { return atlas_refused(ValueRefusalKind::InvariantViolated); };
            if slots[index].is_none() { return atlas_refused(ValueRefusalKind::InvariantViolated); }
            self.len = index;
            self.closing = true;
            drop(slots[index].take());
            return Ok(CloseStep::Pending { progress });
        }
        if self.slots.is_some() { self.closing = true; drop(self.slots.take()); return Ok(CloseStep::Pending { progress }); }
        if let Some(permit) = self.permit.as_mut() {
            if permit.release_phase <= 3 {
                let step = permit.close_original_step(grant);
                return match step { Ok(CloseStep::Pending { progress: child }) if child.copied_items != 0 => { self.closing = true; Ok(CloseStep::Pending { progress }) }, step => step };
            }
            self.closing = true;
            self.permit = None;
            return Ok(CloseStep::Pending { progress });
        }
        Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
}

impl PreparedAtlasPages {
    pub fn original_owner_progress(&self) -> RetainedCloneProgress { self.observed }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, |owner| owner.original_close_progress().copied_bytes)) }

    fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> { Ok(self.owner.as_ref().map_or(0, |owner| owner.original_close_progress().released_bytes)) }

    pub fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() }); };
        let grant = atlas_cap(grant, owner.authority.retirement);
        let progress = owner.original_close_progress();
        if let Some(step) = atlas_gate(grant, progress) { return step; }
        if !owner.terminal_is_empty() || owner.handoff_pending {
            let step = owner.close_original_step(grant)?;
            atlas_add(&mut self.observed, step.progress());
            return Ok(step);
        }
        let slot = usize::from(self.abandonment_slot);
        if PREPARED_ATLAS_ABANDONMENT_STATE[slot].load(Ordering::Acquire) != 1 { return atlas_refused(ValueRefusalKind::InvariantViolated); }
        drop(self.owner.take());
        PREPARED_ATLAS_ABANDONMENT_STATE[slot].store(0, Ordering::Release);
        self.abandonment_slot = u8::MAX;
        atlas_add(&mut self.observed, progress);
        Ok(CloseStep::Complete { progress })
    }

    fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> { self.close_original_step(grant) }

    pub fn try_new(width: u32, height: u32, channels: u8, byte_len: usize, authority: PreparedAtlasAuthority) -> Result<(Self, RetainedCloneProgress), ValueError> {
        let fault = |message| ValueError::literal(ValueRefusalKind::OwnershipLimit, message);
        let row_bytes = usize::try_from(width).ok().and_then(|width| width.checked_mul(usize::from(channels))).ok_or_else(|| fault("atlas row bytes exhausted"))?;
        let expected = row_bytes.checked_mul(usize::try_from(height).map_err(|_| fault("atlas height exhausted"))?).ok_or_else(|| fault("atlas byte length exhausted"))?;
        if expected != byte_len || row_bytes == 0 || row_bytes > PREPARED_ATLAS_PAGE_BYTES { return Err(fault("atlas dimensions do not fit fixed page credits")); }
        let rows_per_page = u32::try_from(PREPARED_ATLAS_PAGE_BYTES / row_bytes).map_err(|_| fault("atlas rows per page exhausted"))?;
        let rows_per_page_usize = usize::try_from(rows_per_page).map_err(|_| fault("atlas rows per page exhausted"))?;
        let pages = usize::try_from(height).ok().and_then(|height| height.checked_add(rows_per_page_usize.checked_sub(1)?)).map(|rows| rows / rows_per_page_usize).ok_or_else(|| fault("atlas page count exhausted"))?;
        if pages > PREPARED_ATLAS_PAGE_CAPACITY || byte_len > PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES { return Err(fault("atlas page or byte credits exceeded")); }
        let slot_backing = size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>();
        let page_backing = pages.checked_mul(PREPARED_ATLAS_PAGE_BYTES).ok_or_else(|| fault("atlas page backing credits exhausted"))?;
        let backing_bytes = slot_backing.checked_add(page_backing).and_then(|bytes| bytes.checked_add(size_of::<PreparedAtlasAbandonment>())).ok_or_else(|| fault("atlas aggregate backing credits exhausted"))?;
        let progress = atlas_progress(size_of::<PreparedAtlasAbandonment>() + size_of::<Self>() + size_of::<u64>() * 2 + size_of::<u8>(), size_of::<PreparedAtlasAbandonment>(), 0);
        if !progress.fits(authority.normal) || authority.normal.maximum_depth == 0 { return Err(fault("atlas original header authority exhausted")); }
        if authority.retirement.maximum_items == 0 || authority.retirement.maximum_copy_bytes < size_of::<*mut PreparedAtlasAbandonment>() + size_of::<u8>() + size_of::<bool>() * 3 || authority.retirement.maximum_depth == 0 { return Err(fault("atlas abandonment handoff authority exhausted")); }
        let Some(abandonment_slot) = PREPARED_ATLAS_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return Err(fault("atlas process permit credits exhausted")); };
        let Some(permit) = PreparedAtlasPermit::try_reserve(pages, byte_len, backing_bytes) else {
            PREPARED_ATLAS_ABANDONMENT_STATE[abandonment_slot].store(0, Ordering::Release);
            return Err(fault("atlas process permit credits exhausted"));
        };
        let owner = Box::new(PreparedAtlasAbandonment { slots: None, len: 0, permit: Some(permit), authority, handoff_pending: false, closing: false });
        Ok((Self { owner: Some(owner), width, height, channels, byte_len, rows_per_page, abandonment_slot: abandonment_slot as u8, observed: progress }, progress))
    }

    pub fn admit_original_slots(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneProgress, ValueError> {
        let Some(owner) = self.owner.as_mut() else { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original header was released")); };
        if owner.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original owner is retiring")); }
        if owner.slots.is_some() { return Ok(RetainedCloneProgress::default()); }
        let bytes = size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>();
        let progress = atlas_progress(bytes + size_of::<Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>>(), bytes, 0);
        let grant = atlas_cap(grant, owner.authority.normal);
        if !progress.fits(grant) || grant.maximum_depth == 0 { return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "atlas original slot authority exhausted")); }
        let mut slots = Box::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>::new_uninit();
        let pointer = slots.as_mut_ptr().cast::<Option<PreparedAtlasPage>>();
        for index in 0..PREPARED_ATLAS_PAGE_CAPACITY { unsafe { pointer.add(index).write(None) }; }
        owner.slots = Some(unsafe { slots.assume_init() });
        atlas_add(&mut self.observed, progress);
        Ok(progress)
    }

    pub fn slots_are_admitted(&self) -> bool { self.owner.as_ref().is_some_and(|owner| owner.slots.is_some()) }

    pub fn push_page(&mut self, source: &[u8], start_row: u32, grant: RetainedCloneGrant) -> Result<(bool, RetainedCloneProgress), ValueError> {
        let fault = |message| ValueError::literal(ValueRefusalKind::OwnershipLimit, message);
        let next_row = self.next_row();
        let owner = self.owner.as_mut().ok_or_else(|| fault("atlas original header was released"))?;
        if owner.closing { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original owner is retiring")); }
        if owner.len == PREPARED_ATLAS_PAGE_CAPACITY || start_row >= self.height || start_row != next_row { return Err(fault("atlas page item credits exceeded")); }
        let row_bytes = usize::try_from(self.width).ok().and_then(|width| width.checked_mul(usize::from(self.channels))).ok_or_else(|| fault("atlas row bytes exhausted"))?;
        let rows = (self.height - start_row).min(self.rows_per_page);
        let bytes = usize::try_from(rows).ok().and_then(|rows| rows.checked_mul(row_bytes)).ok_or_else(|| fault("atlas page byte count exhausted"))?;
        let start = usize::try_from(start_row).ok().and_then(|row| row.checked_mul(row_bytes)).ok_or_else(|| fault("atlas page offset exhausted"))?;
        let slice = source.get(start..start.checked_add(bytes).ok_or_else(|| fault("atlas page end exhausted"))?).ok_or_else(|| fault("atlas page source was truncated"))?;
        let progress = atlas_progress(PREPARED_ATLAS_PAGE_BYTES + bytes + size_of::<Option<PreparedAtlasPage>>() + size_of::<usize>(), PREPARED_ATLAS_PAGE_BYTES, 0);
        let grant = atlas_cap(grant, owner.authority.normal);
        if !progress.fits(grant) || grant.maximum_depth == 0 { return Err(fault("atlas original page authority exhausted")); }
        let slots = owner.slots.as_mut().ok_or_else(|| fault("atlas slot backing was not admitted"))?;
        if slots[owner.len].is_some() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original page slot was occupied")); }
        let mut page = Box::<[u8; PREPARED_ATLAS_PAGE_BYTES]>::new_uninit();
        unsafe { page.as_mut_ptr().cast::<u8>().write_bytes(0, PREPARED_ATLAS_PAGE_BYTES) };
        let mut page = unsafe { page.assume_init() };
        page[..bytes].copy_from_slice(slice);
        slots[owner.len] = Some(PreparedAtlasPage { bytes: page, len: u16::try_from(bytes).map_err(|_| fault("atlas page length exhausted"))?, start_row, rows });
        owner.len += 1;
        atlas_add(&mut self.observed, progress);
        Ok((start_row.checked_add(rows) == Some(self.height), progress))
    }

    pub fn page(&self, index: usize) -> Option<(&[u8], u32, u32)> {
        let page = self.owner.as_ref()?.slots.as_ref()?.get(index)?.as_ref()?;
        Some((&page.bytes[..usize::from(page.len)], page.start_row, page.rows))
    }

    pub fn len(&self) -> usize { self.owner.as_ref().map_or(0, |owner| owner.len) }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
    pub fn next_row(&self) -> u32 { self.len().checked_sub(1).and_then(|index| self.owner.as_ref()?.slots.as_ref()?.get(index)?.as_ref()).and_then(|page| page.start_row.checked_add(page.rows)).unwrap_or(0) }
    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }
    pub fn byte_len(&self) -> usize { self.byte_len }
    pub fn terminal_is_empty(&self) -> bool { self.owner.is_none() && self.abandonment_slot == u8::MAX }

    pub fn next_abandoned_close_demands(_copy: usize) -> Result<RetainedCloneGrant, ValueError> {
        for (state, owner) in PREPARED_ATLAS_ABANDONMENT_STATE.iter().zip(PREPARED_ATLAS_ABANDONMENT_OWNER.iter()) {
            if state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { continue; }
            let pointer = owner.load(Ordering::Acquire);
            let result = if pointer.is_null() { Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "atlas original abandonment pointer was absent")) } else {
                let progress = unsafe { &*pointer }.original_close_progress();
                Ok(RetainedCloneGrant { maximum_items: progress.copied_items, maximum_copy_bytes: progress.copied_bytes, maximum_capacity_bytes: progress.retained_capacity_bytes, maximum_release_bytes: progress.released_bytes, maximum_depth: 1 })
            };
            state.store(2, Ordering::Release);
            return result;
        }
        Ok(RetainedCloneGrant::default())
    }

    pub fn close_abandoned_step(grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        for (state, owner) in PREPARED_ATLAS_ABANDONMENT_STATE.iter().zip(PREPARED_ATLAS_ABANDONMENT_OWNER.iter()) {
            if state.load(Ordering::Acquire) != 2 { continue; }
            if grant.maximum_items == 0 { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
            if state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_err() { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
            let pointer = owner.load(Ordering::Acquire);
            if pointer.is_null() { state.store(2, Ordering::Release); return atlas_refused(ValueRefusalKind::InvariantViolated); }
            let value = unsafe { &mut *pointer };
            let grant = atlas_cap(grant, value.authority.retirement);
            let progress = value.original_close_progress();
            if let Some(step) = atlas_gate(grant, progress) { state.store(2, Ordering::Release); return step; }
            if value.terminal_is_empty() && !value.handoff_pending {
                owner.store(std::ptr::null_mut(), Ordering::Release);
                drop(unsafe { Box::from_raw(pointer) });
                state.store(0, Ordering::Release);
                return Ok(CloseStep::Pending { progress });
            }
            let step = value.close_original_step(grant);
            state.store(2, Ordering::Release);
            return step;
        }
        if PREPARED_ATLAS_ABANDONMENT_STATE.iter().any(|state| state.load(Ordering::Acquire) == 3) { return Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }); }
        Ok(CloseStep::Complete { progress: RetainedCloneProgress::default() })
    }
}

impl Drop for PreparedAtlasPages {
    fn drop(&mut self) {
        let Some(mut owner) = self.owner.take() else { return };
        let slot = usize::from(self.abandonment_slot);
        owner.handoff_pending = true;
        PREPARED_ATLAS_ABANDONMENT_OWNER[slot].store(Box::into_raw(owner), Ordering::Release);
        PREPARED_ATLAS_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        self.abandonment_slot = u8::MAX;
    }
}

impl PreparedRasterPages {
    fn next_close_release_byte_demand(&self, key: &String) -> Result<usize, ValueError> {
        if !self.slots.is_empty() || !self.backing.is_empty() { return Ok(0); }
        if !self.backing_released { return prepared_vec_bytes(&self.backing); }
        if !key.is_empty() { return Ok(0); }
        if !self.key_released { return Ok(key.capacity()); }
        if self.close_phase == 0 { return prepared_vec_bytes(&self.slots); }
        Ok(0)
    }

    fn close_with_key_granted(&mut self, key: &mut String, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand(key);
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty() && key.capacity() == 0, demand.clone()) { return step; }
        let phase = self.close_phase;
        let credit = self.credit.is_some();
        self.retire_with_key_step(key);
        if phase == 10 && credit && self.credit.is_some() { return Ok(CloseStep::Blocked); }
        prepared_close_progress(demand.unwrap_or(0))
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

    pub fn content_identity(&self) -> RasterContentIdentity {
        self.content_identity
    }

    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.source_generation
    }

    pub fn frame_generation(&self) -> u64 {
        self.frame_generation
    }

    pub fn page_for_row(&self, row: u32) -> Option<(&[u8], u32)> {
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
            6 => self.content_identity = RasterContentIdentity::pixels(0, 0, 0).expect("empty raster identity"),
            7 => self.source_generation = PreparedRasterGeneration::default(),
            8 => self.frame_generation = 0,
            9 => self.backing_released = true,
            10 => {
                let Some(credit) = self.credit.as_ref() else {
                    self.close_phase = 11;
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

    #[cfg(any(test, feature = "testkit"))]
    fn page_pointer(&self, logical: usize) -> Option<*const u8> {
        self.page_for_row(u32::try_from(logical).ok()?.checked_mul(self.rows_per_page)?).map(|(bytes, _)| bytes.as_ptr())
    }
}

#[derive(Debug)]
pub struct PreparedRasterRejected {
    fault: &'static str,
    /// 🖼️ Whether the refusal is the SOURCE's own ([`Self::is_content_refusal`]).
    content: bool,
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

    /// 🖼️ Whether this refusal is the source's own — undecodable or oversized, a typed per-image outcome its host retires
    /// silently — rather than a raster-authority fault.
    pub fn is_content_refusal(&self) -> bool {
        self.content
    }

    /// 🖼️ This refusal classified as the source's own ([`Self::is_content_refusal`]) — what a host does with a refusal its
    /// measured source caused.
    pub fn into_content_refusal(mut self) -> Self {
        self.content = true;
        self
    }

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{
        let release=if !self.source.is_empty(){0}else if !self.source_released{prepared_vec_bytes(&self.source)?}else if !self.retained_source.is_empty(){0}else if !self.retained_source_released{prepared_vec_bytes(&self.retained_source)?}else if !self.key.is_empty(){0}else if !self.key_released{self.key.capacity()}else{0};
        prepared_owner_demands(self.terminal_is_empty(),Ok(release))
    }

    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<CloseStep,ValueError>{
        let demand=self.next_close_demands(grant.maximum_copy_bytes)?;
        if let Some(step)=prepared_close_gate(grant,self.terminal_is_empty(),Ok(demand.maximum_release_bytes)){return step}
        if !self.source.is_empty(){self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));}
        else if !self.source_released{self.source=Vec::new();self.source_released=true;}
        else if !self.retained_source.is_empty(){self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));}
        else if !self.retained_source_released{self.retained_source=Vec::new();self.retained_source_released=true;}
        else if !self.key.is_empty(){self.key.pop();}
        else if !self.key_released{self.key=String::new();self.key_released=true;}
        else if let Some(credit)=self.credit.as_ref(){let mut ledger=match PREPARED_RASTER_LEDGER.try_lock(){Ok(ledger)=>ledger,Err(std::sync::TryLockError::WouldBlock)=>return Ok(CloseStep::Blocked),Err(std::sync::TryLockError::Poisoned(_))=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original raster credit ledger poisoned"))};if !ledger.release(credit){return Ok(CloseStep::Blocked)}self.credit=None;}
        prepared_close_progress(demand.maximum_release_bytes)
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
    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_reserve(key: String) -> Result<Self, PreparedRasterRejected> {
        Self::try_reserve_source(key, 0)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn try_reserve_source(key: String, source_bytes: usize) -> Result<Self, PreparedRasterRejected> {
        let reject = |fault, key| PreparedRasterRejected { fault, content: false, key, source: Vec::new(), retained_source: Vec::new(), credit: None, source_released: false, retained_source_released: false, key_released: false };
        if key.len() > PREPARED_RASTER_KEY_BYTES {
            return Err(reject("raster producer exceeded fixed key credits", key));
        }
        let Some(key_bytes) = key.capacity().checked_mul(2) else { return Err(reject("raster producer key credits overflowed", key)) };
        let Some(source_peak_bytes) = source_bytes.checked_mul(2) else { return Err(reject("raster producer source credits overflowed", key)) };
        let Some(bytes) = source_peak_bytes.checked_add(key_bytes) else { return Err(reject("raster producer source credits overflowed", key)) };
        if source_bytes > PREPARED_RASTER_ITEM_BYTES {
            return Err(reject("raster producer source exceeded fixed credits", key).into_content_refusal());
        }
        let credit = PREPARED_RASTER_LEDGER.lock().ok().and_then(|mut ledger| ledger.reserve(1, bytes));
        let Some(credit) = credit else { return Err(reject("raster producer process credits exhausted", key)) };
        Ok(Self { key, credit: Some(credit), claim: None, source_bytes, source_peak_bytes })
    }

    pub fn reject(self, fault: &'static str, source: Vec<u8>) -> PreparedRasterRejected {
        self.reject_with_retained(fault, source, Vec::new())
    }

    pub fn reject_with_retained(mut self, fault: &'static str, source: Vec<u8>, retained_source: Vec<u8>) -> PreparedRasterRejected {
        PreparedRasterRejected { fault, content: false, key: std::mem::take(&mut self.key), source, retained_source, credit: self.credit.take(), source_released: false, retained_source_released: false, key_released: false }
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn claim(self, width: u32, height: u32) -> Result<Self, PreparedRasterRejected> {
        self.claim_with_retained(width, height, Vec::new()).map(|(reservation, _)| reservation)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
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

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
    pub fn finalize(mut self, source: Vec<u8>, retained_source: Vec<u8>, width: u32, height: u32) -> Result<(PreparedRasterProducer, String), PreparedRasterRejected> {
        let reject = |reservation: Self, fault, source, retained_source| reservation.reject_with_retained(fault, source, retained_source);
        let Some(claim) = self.claim else { return Err(reject(self, "raster producer was not claimed before materialization", source, retained_source)) };
        if claim.width != width || claim.height != height || source.len() != claim.byte_len || source.capacity() > claim.byte_len || retained_source.capacity() > self.source_bytes {
            return Err(reject(self, "raster materialization did not match its exact claim", source, retained_source));
        }
        let Some(content_identity) = RasterContentIdentity::pixels(width, height, claim.byte_len) else { return Err(reject(self, "raster content identity byte credits overflowed", source, retained_source)) };
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
            content_identity,
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
    #[cfg(feature = "testkit")]
    pub fn original_source(&self) -> &[u8] { &self.source }

    #[cfg(feature = "testkit")]
    pub fn original_retained_source(&self) -> &[u8] { &self.retained_source }

    #[cfg(feature = "testkit")]
    pub fn original_page_count(&self) -> usize { self.pages.as_ref().map_or(0, |pages| pages.slots.len()) }

    pub fn next_close_demands(&self,_copy:usize)->Result<RetainedCloneGrant,ValueError>{prepared_owner_demands(self.terminal_is_empty(),self.next_close_release_byte_demand())}

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        if let Some(pages) = self.pages.as_ref() {
            if !pages.slots.is_empty() || !pages.backing.is_empty() { return Ok(0); }
            if !pages.backing_released { return prepared_vec_bytes(&pages.backing); }
        }
        if !self.source.is_empty() { return Ok(0); }
        if !self.source_released { return prepared_vec_bytes(&self.source); }
        if !self.retained_source.is_empty() { return Ok(0); }
        if !self.retained_source_released { return prepared_vec_bytes(&self.retained_source); }
        if !self.key.is_empty() { return Ok(0); }
        if self.key.capacity() != 0 || self.pages.as_ref().is_some_and(|pages| !pages.key_released) { return Ok(self.key.capacity()); }
        if let Some(pages) = self.pages.as_ref() { if pages.close_phase == 0 { return prepared_vec_bytes(&pages.slots); } }
        Ok(0)
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        if let Some(pages) = self.pages.as_mut() {
            if pages.retire_page_step() || !pages.retire_backing_step() { return prepared_close_progress(demand.unwrap_or(0)); }
        }
        if !self.source.is_empty() { self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES)); }
        else if !self.source_released { self.source = Vec::new(); self.source_released = true; }
        else if !self.retained_source.is_empty() { self.retained_source.truncate(self.retained_source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES)); }
        else if !self.retained_source_released { self.retained_source = Vec::new(); self.retained_source_released = true; }
        else if !self.key.is_empty() || self.key.capacity() != 0 || self.pages.as_ref().is_some_and(|pages| !pages.key_released) {
            prepared_close_text(&mut self.key);
            if self.key.capacity() == 0 { if let Some(pages) = self.pages.as_mut() { pages.key_released = true; } }
        } else if let Some(pages) = self.pages.as_mut() {
            let phase = pages.close_phase;
            let credit = pages.credit.is_some();
            pages.retire_metadata_step();
            if phase == 10 && credit && pages.credit.is_some() { return Ok(CloseStep::Blocked); }
        }
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn source_generation(&self) -> PreparedRasterGeneration {
        self.pages.as_ref().map_or_else(PreparedRasterGeneration::default, PreparedRasterPages::source_generation)
    }

    #[cfg_attr(target_pointer_width = "64", expect(clippy::result_large_err, reason = "Rejected preparation returns its exact source, key, and reservation credit without allocating beyond admission."))]
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
        pages.content_identity.mix_bytes(start, &self.source[start..end]);
        pages.slots.push(PreparedRasterPage { start_row, rows });
        PreparedRasterProducerStep::Pending
    }

    pub fn begin_close(&mut self) {
        self.closing = true;
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
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
    #[cfg(any(test, feature = "testkit"))]
    GlyphAtlas {
        pixels: Vec<u8>,
        width: u32,
        height: u32,
    },
    #[cfg(any(test, feature = "testkit"))]
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
    #[cfg(any(test, feature = "testkit"))]
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
    SceneRaster {
        key: String,
        lease: semio_framework_ui_raster_owner::SceneRasterLease,
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
    pub fn next_close_demands(&self, maximum_copy_bytes: usize) -> Result<RetainedCloneGrant, ValueError> {
        if let Self::SceneRaster { key, lease } = self {
            if key.capacity() == 0 && !lease.terminal_is_empty() {
                return Ok(RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: lease.next_close_copy_byte_demand()?, maximum_capacity_bytes: lease.next_close_capacity_byte_demand(maximum_copy_bytes)?, maximum_release_bytes: lease.next_close_release_byte_demand()?, maximum_depth: lease.next_close_depth_demand()? });
            }
        }
        let mut demand = prepared_owner_demands(self.terminal_is_empty(), self.next_close_release_byte_demand())?;
        demand.maximum_copy_bytes = self.next_close_copy_byte_demand()?;
        Ok(demand)
    }

    pub fn next_close_copy_byte_demand(&self) -> Result<usize, ValueError> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } | Self::Raster { pixels, .. } => Ok(usize::from(!pixels.is_empty())),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.next_close_copy_byte_demand(),
            _ => Ok(0),
        }
    }

    pub fn terminal_is_empty(&self) -> bool {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => pixels.capacity() == 0,
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.terminal_is_empty(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => pixels.capacity() == 0 && key.capacity() == 0,
            Self::RasterPages { key, pixels } => pixels.terminal_is_empty() && key.capacity() == 0,
            Self::SceneRaster { key, lease } => key.capacity() == 0 && lease.terminal_is_empty(),
            Self::Mesh { key, .. } => key.capacity() == 0,
        }
    }

    pub fn next_close_release_byte_demand(&self) -> Result<usize, ValueError> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => if pixels.is_empty() { prepared_vec_bytes(pixels) } else { Ok(0) },
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => pixels.next_close_release_byte_demand(),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => if !pixels.is_empty() { Ok(0) } else if pixels.capacity() != 0 { prepared_vec_bytes(pixels) } else { Ok(if key.is_empty() { key.capacity() } else { 0 }) },
            Self::RasterPages { key, pixels } => pixels.next_close_release_byte_demand(key),
            Self::SceneRaster { key, lease } => if key.capacity() != 0 { Ok(if key.is_empty() { key.capacity() } else { 0 }) } else { lease.next_close_release_byte_demand() },
            Self::Mesh { key, .. } => Ok(if key.is_empty() { key.capacity() } else { 0 }),
        }
    }

    pub fn close_granted(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        let demand = self.next_close_release_byte_demand();
        if let Some(step) = prepared_close_gate(grant, self.terminal_is_empty(), demand.clone()) { return step; }
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => return prepared_close_bytes(pixels, grant),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => return pixels.close_original_step(grant),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => { if !pixels.is_empty() || pixels.capacity() != 0 { return prepared_close_bytes(pixels, grant); } else { prepared_close_text(key); } }
            Self::RasterPages { key, pixels } => return pixels.close_with_key_granted(key, grant),
            Self::SceneRaster { key, lease } => {
                if key.capacity() != 0 { prepared_close_text(key); }
                else { return lease.close_step(grant).map(|step|match step { semio_framework_value::retained_clone::RetainedCloneStep::Progress(progress) => CloseStep::Pending { progress }, semio_framework_value::retained_clone::RetainedCloneStep::Complete(progress) => CloseStep::Complete { progress } }); }
            }
            Self::Mesh { key, .. } => { prepared_close_text(key); }
        }
        prepared_close_progress(demand.unwrap_or(0))
    }

    pub fn byte_len(&self) -> Option<usize> {
        match self {
            #[cfg(any(test, feature = "testkit"))]
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => Some(pixels.len()),
            Self::GlyphAtlasPages { pixels } | Self::IconAtlasPages { pixels } => Some(pixels.byte_len()),
            #[cfg(any(test, feature = "testkit"))]
            Self::Raster { key, pixels, .. } => key.len().checked_add(pixels.len()),
            Self::RasterPages { key, pixels } => key.len().checked_add(pixels.byte_len()),
            Self::SceneRaster { key, .. } => key.len().checked_add(size_of::<semio_framework_ui_raster_owner::SceneRasterLease>()),
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

    pub fn scene_raster_byte_len(&self) -> usize {
        match self {
            Self::SceneRaster { lease, .. } => lease.byte_len(),
            _ => 0,
        }
    }

    /// 🧹 Releases one page, byte, or key scalar from a rejected upload owner.
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
        self.close_granted(grant)
    }
}


#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
#[cfg(test)]
#[path = "../🖼️atlas/🧪️tests/🦀️.rs"]
mod atlas_original_owner_tests;

#[cfg(test)]
#[global_allocator]
static ALLOCATOR: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[cfg(test)]
fn measured<T>(operation: impl FnOnce() -> T) -> (T, usize, usize) {
    let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(operation);
    assert!(!heap.overflowed);
    (result, heap.requested_bytes, heap.released_bytes)
}

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️tests/♻️abandoned/🦀️.rs"]
mod abandoned_original_owner_tests;
