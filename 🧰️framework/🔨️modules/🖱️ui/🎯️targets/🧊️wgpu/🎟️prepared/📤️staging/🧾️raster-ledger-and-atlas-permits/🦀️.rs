//! 🧾️ Raster ledger and atlas permits: the raster key and item bytes, the raster ledger, credit and generation, the raster pages, the atlas limits, shifts and statics, the atlas permit, pages and abandonment. See [`super`].

use super::*;

//#region 🧩️PagedRasterProducer
pub(super) const PREPARED_RASTER_KEY_BYTES: usize = 256;

/// 🖼️ The straight-RGBA ceiling ONE raster item may carry into a prepared packet. `pub` because a
/// PRODUCER of raster sources has to size its own decode against it: the world reference underlay
/// publishes whatever a photograph or plan happens to be, and a source over this ceiling is refused at
/// admission — which the frame reports as a build fault, not as one missing texture (ticket
/// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
pub const PREPARED_RASTER_ITEM_BYTES: usize = 16 * 1024 * 1024;

pub(super) const PREPARED_RASTER_PRODUCER_CAPACITY: usize = 256;

pub(super) const PREPARED_RASTER_PRODUCER_ITEMS: usize = 4_096;

pub(super) const PREPARED_RASTER_PRODUCER_BYTES: usize = 32 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct PreparedRasterLedgerSlot {
    pub(super) epoch: u64,
    pub(super) items: usize,
    pub(super) bytes: usize,
    occupied: bool,
}

pub(super) struct PreparedRasterLedger {
    pub(super) slots: [PreparedRasterLedgerSlot; PREPARED_RASTER_PRODUCER_CAPACITY],
    pub(super) items: usize,
    pub(super) bytes: usize,
}

impl Default for PreparedRasterLedger {
    fn default() -> Self {
        Self { slots: [PreparedRasterLedgerSlot::default(); PREPARED_RASTER_PRODUCER_CAPACITY], items: 0, bytes: 0 }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedRasterCredit {
    pub(super) slot: u16,
    pub(super) epoch: u64,
    pub(super) items: usize,
    pub(super) bytes: usize,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PreparedRasterGeneration {
    pub(super) slot: u16,
    pub(super) epoch: u64,
}

impl PreparedRasterGeneration {
    pub fn slot(&self) -> u16 {
        self.slot
    }

    pub fn epoch(&self) -> u64 {
        self.epoch
    }
}

pub(super) static PREPARED_RASTER_LEDGER: LazyLock<Mutex<PreparedRasterLedger>> = LazyLock::new(|| Mutex::new(PreparedRasterLedger::default()));

impl PreparedRasterLedger {
    pub(super) fn reserve(&mut self, items: usize, bytes: usize) -> Option<PreparedRasterCredit> {
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

    pub(super) fn release(&mut self, credit: &PreparedRasterCredit) -> bool {
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

    pub(super) fn resize(&mut self, credit: &mut PreparedRasterCredit, items: usize, bytes: usize) -> bool {
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
pub(super) struct PreparedRasterPage {
    pub(super) start_row: u32,
    pub(super) rows: u32,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedRasterPages {
    pub(super) slots: Vec<PreparedRasterPage>,
    pub(super) backing: Vec<u8>,
    pub(super) page_capacity: usize,
    pub(super) rows_per_page: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) byte_len: usize,
    pub(super) content_identity: RasterContentIdentity,
    pub(super) source_generation: PreparedRasterGeneration,
    pub(super) frame_generation: u64,
    pub(super) credit: Option<PreparedRasterCredit>,
    pub(super) backing_released: bool,
    pub(super) key_released: bool,
    pub(super) close_phase: u8,
}

pub const PREPARED_ATLAS_PAGE_BYTES: usize = 16 * 1024;

pub const PREPARED_ATLAS_PAGE_CAPACITY: usize = 2_048;

pub(super) const PREPARED_ATLAS_PROCESS_ITEMS: usize = 64;

const PREPARED_ATLAS_PROCESS_PAGES: usize = 4_096;

pub(super) const PREPARED_ATLAS_PROCESS_PAYLOAD_BYTES: usize = 64 * 1024 * 1024;

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

pub(super) static PREPARED_ATLAS_PROCESS_PERMITS: AtomicU64 = AtomicU64::new(0);

static PREPARED_ATLAS_GENERATION: AtomicU64 = AtomicU64::new(1);

pub(super) static PREPARED_ATLAS_ABANDONMENT_STATE: [AtomicU8; PREPARED_ATLAS_ABANDONMENT_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_ATLAS_ABANDONMENT_SLOTS];

pub(super) static PREPARED_ATLAS_ABANDONMENT_OWNER: [AtomicPtr<PreparedAtlasAbandonment>; PREPARED_ATLAS_ABANDONMENT_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_ATLAS_ABANDONMENT_SLOTS];

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
pub(super) struct PreparedAtlasPermit {
    pub(super) generation: u64,
    pub(super) items: usize,
    pub(super) pages: usize,
    payload_bytes: usize,
    pub(super) backing_bytes: usize,
    payload_units: usize,
    backing_units: usize,
    release_phase: u8,
}

impl PreparedAtlasPermit {
    pub(super) fn try_reserve(pages: usize, payload_bytes: usize, backing_bytes: usize) -> Option<Self> {
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

    pub(super) fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
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
pub(super) struct PreparedAtlasPage {
    pub(super) bytes: Box<[u8; PREPARED_ATLAS_PAGE_BYTES]>,
    pub(super) len: u16,
    pub(super) start_row: u32,
    pub(super) rows: u32,
}

/// 🎟️ Independent original construction and retirement authority for one atlas owner.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PreparedAtlasAuthority {
    pub normal: RetainedCloneGrant,
    pub retirement: RetainedCloneGrant,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedAtlasPages {
    pub(super) owner: Option<Box<PreparedAtlasAbandonment>>,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) channels: u8,
    pub(super) byte_len: usize,
    pub(super) rows_per_page: u32,
    pub(super) abandonment_slot: u8,
    pub(super) observed: RetainedCloneProgress,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedAtlasAbandonment {
    pub(super) slots: Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>,
    pub(super) len: usize,
    pub(super) permit: Option<PreparedAtlasPermit>,
    pub(super) authority: PreparedAtlasAuthority,
    pub(super) handoff_pending: bool,
    pub(super) closing: bool,
}

pub(super) fn atlas_progress(copied_bytes: usize, retained_capacity_bytes: usize, released_bytes: usize) -> RetainedCloneProgress {
    RetainedCloneProgress { copied_items: 1, copied_bytes, retained_capacity_bytes, released_bytes }
}

pub(super) fn atlas_refused(kind: ValueRefusalKind) -> Result<CloseStep,ValueError> {
    Err(ValueError::literal(kind,"original prepared atlas close refused"))
}

pub(super) fn atlas_gate(grant: RetainedCloneGrant, progress: RetainedCloneProgress) -> Option<Result<CloseStep,ValueError>> {
    if grant.maximum_depth == 0 { return Some(atlas_refused(ValueRefusalKind::DepthLimit)); }
    (!progress.fits(grant)).then_some(Ok(CloseStep::Pending { progress: RetainedCloneProgress::default() }))
}

pub(super) fn atlas_cap(received: RetainedCloneGrant, original: RetainedCloneGrant) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: received.maximum_items.min(original.maximum_items), maximum_copy_bytes: received.maximum_copy_bytes.min(original.maximum_copy_bytes), maximum_capacity_bytes: received.maximum_capacity_bytes.min(original.maximum_capacity_bytes), maximum_release_bytes: received.maximum_release_bytes.min(original.maximum_release_bytes), maximum_depth: received.maximum_depth.min(original.maximum_depth) }
}

pub(super) fn atlas_add(total: &mut RetainedCloneProgress, progress: RetainedCloneProgress) {
    total.copied_items += progress.copied_items;
    total.copied_bytes += progress.copied_bytes;
    total.retained_capacity_bytes += progress.retained_capacity_bytes;
    total.released_bytes += progress.released_bytes;
}

impl PreparedAtlasAbandonment {
    pub(super) fn terminal_is_empty(&self) -> bool { self.len == 0 && self.slots.is_none() && self.permit.is_none() }

    pub(super) fn original_close_progress(&self) -> RetainedCloneProgress {
        if self.handoff_pending { return atlas_progress(size_of::<*mut Self>() + size_of::<u8>() + size_of::<bool>() * 3, 0, 0); }
        if self.len != 0 { return atlas_progress(size_of::<Option<PreparedAtlasPage>>() + size_of::<usize>() + size_of::<bool>(), 0, PREPARED_ATLAS_PAGE_BYTES); }
        if self.slots.is_some() { return atlas_progress(size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>() + size_of::<Option<Box<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>>>() + size_of::<bool>(), 0, size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>()); }
        if let Some(permit) = &self.permit {
            return atlas_progress((if permit.release_phase <= 3 { size_of::<u64>() * 2 + size_of::<u8>() } else { size_of::<Option<PreparedAtlasPermit>>() }) + size_of::<bool>(), 0, 0);
        }
        atlas_progress(size_of::<Self>() + size_of::<*mut Self>() + size_of::<u8>(), 0, size_of::<Self>())
    }

    pub(super) fn close_original_step(&mut self, grant: RetainedCloneGrant) -> Result<CloseStep,ValueError> {
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
//#endregion 🧩️PagedRasterProducer
