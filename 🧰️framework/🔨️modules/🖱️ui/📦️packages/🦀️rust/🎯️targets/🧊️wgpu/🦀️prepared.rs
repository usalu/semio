//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use crate::wgpu::draw::{DrawLayer, DrawList, ScissorRect};
use crate::wgpu::kernel_3d_scene::Mesh3dLease;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, StepContext, StepOutcome};
use std::collections::VecDeque;
use std::mem::size_of;
use std::rc::Rc;
use std::sync::{Arc, LazyLock, Mutex};

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
    fn include_draw(&mut self, items: usize, bytes: usize) {
        self.draw_items = self.draw_items.saturating_add(items);
        self.draw_bytes = self.draw_bytes.saturating_add(bytes);
    }

    fn include_upload(&mut self, bytes: usize) {
        self.upload_items = self.upload_items.saturating_add(1);
        self.upload_bytes = self.upload_bytes.saturating_add(bytes);
    }

    pub fn fits(self, limits: PreparedRenderLimits) -> bool {
        self.draw_items <= limits.max_draw_items && self.draw_bytes <= limits.max_draw_bytes && self.upload_items <= limits.max_upload_items && self.upload_bytes <= limits.max_upload_bytes
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
}

#[derive(Debug, PartialEq, Eq)]
struct PreparedRasterPage {
    start_row: u32,
    rows: u32,
    bytes: Box<[u8]>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct PreparedRasterPages {
    slots: Vec<PreparedRasterPage>,
    page_capacity: usize,
    rows_per_page: u32,
    width: u32,
    height: u32,
    byte_len: usize,
    source_generation: PreparedRasterGeneration,
    frame_generation: u64,
    credit: Option<PreparedRasterCredit>,
    key_released: bool,
    close_phase: u8,
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
        (page.start_row == row).then_some((page.bytes.as_ref(), page.rows))
    }

    fn retire_page_step(&mut self) -> bool {
        self.slots.pop().is_some()
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
            8 => {
                let Some(credit) = self.credit.as_ref() else {
                    self.close_phase = 9;
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
        self.close_phase >= 9 && self.slots.is_empty() && self.slots.capacity() == 0 && self.key_released && self.credit.is_none()
    }

    #[cfg(test)]
    fn page_pointer(&self, logical: usize) -> Option<*const u8> {
        let physical = self.page_capacity.checked_sub(logical.checked_add(1)?)?;
        self.slots.get(physical).map(|page| page.bytes.as_ptr())
    }
}

#[derive(Debug)]
pub struct PreparedRasterRejected {
    fault: &'static str,
    key: String,
    source: Vec<u8>,
    source_released: bool,
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
        if self.key.pop().is_some() {
            return false;
        }
        if !self.key_released {
            self.key = String::new();
            self.key_released = true;
            return false;
        }
        true
    }
}

#[derive(Debug)]
pub struct PreparedRasterProducer {
    key: String,
    source: Vec<u8>,
    pages: Option<PreparedRasterPages>,
    frame_generation: Option<u64>,
    source_released: bool,
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
        let reject = |fault, key, source| PreparedRasterRejected { fault, key, source, source_released: false, key_released: false };
        let Some(row_bytes) = usize::try_from(width).ok().and_then(|value| value.checked_mul(4)) else { return Err(reject("raster row byte credits overflowed", key, source)) };
        let Some(byte_len) = row_bytes.checked_mul(usize::try_from(height).unwrap_or(usize::MAX)) else { return Err(reject("raster byte credits overflowed", key, source)) };
        if width == 0 || height == 0 || row_bytes > PREPARED_RASTER_PAGE_BYTES || byte_len > PREPARED_RASTER_ITEM_BYTES || source.len() != byte_len || key.len() > PREPARED_RASTER_KEY_BYTES {
            return Err(reject("raster producer exceeded fixed item or byte credits", key, source));
        }
        let rows_per_page = (PREPARED_RASTER_PAGE_BYTES / row_bytes).max(1);
        let page_capacity = usize::try_from(height).unwrap_or(usize::MAX).div_ceil(rows_per_page);
        let Some(items) = page_capacity.checked_add(5) else { return Err(reject("raster producer item credits overflowed", key, source)) };
        let Some(key_bytes) = key.capacity().checked_mul(2) else { return Err(reject("raster producer key credits overflowed", key, source)) };
        let Some(page_slot_bytes) = page_capacity.checked_mul(size_of::<PreparedRasterPage>()) else { return Err(reject("raster producer page slot credits overflowed", key, source)) };
        let Some(bytes) = source.capacity().checked_add(byte_len).and_then(|value| value.checked_add(key_bytes)).and_then(|value| value.checked_add(page_slot_bytes)) else {
            return Err(reject("raster producer aggregate bytes overflowed", key, source));
        };
        let credit = PREPARED_RASTER_LEDGER.lock().ok().and_then(|mut ledger| ledger.reserve(items, bytes));
        let Some(credit) = credit else { return Err(reject("raster producer process credits exhausted", key, source)) };
        let source_generation = PreparedRasterGeneration { slot: credit.slot, epoch: credit.epoch };
        let pages = PreparedRasterPages {
            slots: Vec::with_capacity(page_capacity),
            page_capacity,
            rows_per_page: rows_per_page as u32,
            width,
            height,
            byte_len,
            source_generation,
            frame_generation: 0,
            credit: Some(credit),
            key_released: false,
            close_phase: 0,
        };
        let published_key = key.clone();
        Ok((Self { key, source, pages: Some(pages), frame_generation: None, source_released: false, closing: false }, published_key))
    }

    pub fn bind_frame_generation(&mut self, generation: u64) -> bool {
        if generation == 0 || self.closing {
            return false;
        }
        match self.frame_generation {
            Some(current) => current == generation,
            None => {
                self.frame_generation = Some(generation);
                self.pages.as_mut().expect("admitted raster pages").frame_generation = generation;
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
            let pages = self.pages.take().expect("completed raster pages");
            let key = std::mem::take(&mut self.key);
            return PreparedRasterProducerStep::Complete(PreparedRenderUpload::RasterPages { key, pixels: pages });
        }
        let pages = self.pages.as_mut().expect("admitted raster pages");
        let page_bytes = usize::try_from(pages.rows_per_page).unwrap_or(usize::MAX) * usize::try_from(pages.width).unwrap_or(usize::MAX) * 4;
        let start = self.source.len().saturating_sub(1) / page_bytes * page_bytes;
        let bytes = self.source.split_off(start).into_boxed_slice();
        let start_row = u32::try_from(start / (usize::try_from(pages.width).unwrap_or(usize::MAX) * 4)).unwrap_or(u32::MAX);
        let rows = u32::try_from(bytes.len() / (usize::try_from(pages.width).unwrap_or(usize::MAX) * 4)).unwrap_or(u32::MAX);
        pages.slots.push(PreparedRasterPage { start_row, rows, bytes });
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
        if !self.source.is_empty() {
            self.source.truncate(self.source.len().saturating_sub(PREPARED_RASTER_PAGE_BYTES));
            return false;
        }
        if !self.source_released {
            self.source = Vec::new();
            self.source_released = true;
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
        self.key.is_empty() && self.source.is_empty() && self.source_released && self.pages.as_ref().is_none_or(PreparedRasterPages::terminal_is_empty)
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
    GlyphAtlas { pixels: Vec<u8>, width: u32, height: u32 },
    IconAtlas { pixels: Vec<u8>, width: u32, height: u32 },
    Raster { key: String, pixels: Vec<u8>, width: u32, height: u32 },
    RasterPages { key: String, pixels: PreparedRasterPages },
    Mesh { key: String, version: u64, lease: Mesh3dLease },
}

/// 🧹️ UI-thread GPU cache invalidation selected during worker preparation.
#[derive(Debug, PartialEq, Eq)]
pub enum PreparedRenderEviction {
    Mesh { key: String },
}

impl PreparedRenderEviction {
    pub fn byte_len(&self) -> usize {
        match self {
            Self::Mesh { key } => key.len(),
        }
    }
}

impl PreparedRenderUpload {
    pub fn byte_len(&self) -> usize {
        match self {
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => pixels.len(),
            Self::Raster { key, pixels, .. } => key.len().saturating_add(pixels.len()),
            Self::RasterPages { key, pixels } => key.len().saturating_add(pixels.byte_len()),
            Self::Mesh { key, lease, .. } => key.len().saturating_add(lease.schema().map_or(0, |schema| {
                usize::try_from(schema.vertices)
                    .unwrap_or(usize::MAX)
                    .saturating_mul(24)
                    .saturating_add(usize::try_from(schema.indices).unwrap_or(usize::MAX).saturating_mul(4))
                    .saturating_add(usize::try_from(schema.face_ids).unwrap_or(usize::MAX).saturating_mul(4))
                    .saturating_add(usize::try_from(schema.vertex_ids).unwrap_or(usize::MAX).saturating_mul(4))
                    .saturating_add(usize::try_from(schema.edges).unwrap_or(usize::MAX).saturating_mul(24))
                    .saturating_add(usize::try_from(schema.edge_ids).unwrap_or(usize::MAX).saturating_mul(4))
                    .saturating_add(usize::try_from(schema.uvs).unwrap_or(usize::MAX).saturating_mul(8))
                    .saturating_add(usize::try_from(schema.colors).unwrap_or(usize::MAX).saturating_mul(16))
            })),
        }
    }
}

/// 🧱️ Send-capable frame data prepared without window, surface, device, or queue access.
pub struct PreparedRenderPacket {
    pub(crate) scene_revision: u64,
    pub(crate) preview_generation: u64,
    pub(crate) damage: Vec<ScissorRect>,
    pub(crate) clips: Vec<ScissorRect>,
    pub(crate) directives: Vec<RenderDirective>,
    pub(crate) uploads: Vec<PreparedRenderUpload>,
    pub(crate) evictions: Vec<PreparedRenderEviction>,
    pub(crate) draw: DrawList,
    pub(crate) overlay: Option<DrawList>,
    pub(crate) time_seconds: f32,
    pub(crate) usage: PreparedRenderUsage,
    pub(crate) limits: PreparedRenderLimits,
    retirement_phase: u8,
}

impl PreparedRenderPacket {
    const RETIRE_PAGE_BYTES: usize = 16 * 1024;

    pub fn scene_revision(&self) -> u64 {
        self.scene_revision
    }

    pub fn preview_generation(&self) -> u64 {
        self.preview_generation
    }

    pub fn damage(&self) -> &[ScissorRect] {
        &self.damage
    }

    pub fn clips(&self) -> &[ScissorRect] {
        &self.clips
    }

    pub fn directives(&self) -> &[RenderDirective] {
        &self.directives
    }

    pub fn uploads(&self) -> &[PreparedRenderUpload] {
        &self.uploads
    }

    pub fn evictions(&self) -> &[PreparedRenderEviction] {
        &self.evictions
    }

    pub fn usage(&self) -> PreparedRenderUsage {
        self.usage
    }

    pub fn limits(&self) -> PreparedRenderLimits {
        self.limits
    }

    pub fn is_within_credits(&self) -> bool {
        self.usage.fits(self.limits)
    }

    /// 🧹️ Releases at most one admitted page, draw owner, string scalar, or metadata item.
    pub fn retire_step(&mut self) -> bool {
        if let Some(upload) = self.uploads.last_mut() {
            let retained = match upload {
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => {
                    let next = pixels.len().saturating_sub(Self::RETIRE_PAGE_BYTES);
                    if next != pixels.len() {
                        pixels.truncate(next);
                        true
                    } else {
                        false
                    }
                }
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
        if self.damage.pop().is_some() || self.clips.pop().is_some() || self.directives.pop().is_some() {
            return false;
        }
        match self.retirement_phase {
            0 => self.scene_revision = 0,
            1 => self.preview_generation = 0,
            2 => self.time_seconds = 0.0,
            3 => self.usage = PreparedRenderUsage::default(),
            4 => self.limits = PreparedRenderLimits::default(),
            _ => return true,
        }
        self.retirement_phase += 1;
        false
    }

    pub fn retirement_is_empty(&self) -> bool {
        self.retirement_phase >= 5
            && self.uploads.is_empty()
            && self.evictions.is_empty()
            && self.draw.retirement_is_empty()
            && self.overlay.as_ref().is_none_or(DrawList::retirement_is_empty)
            && self.damage.is_empty()
            && self.clips.is_empty()
            && self.directives.is_empty()
    }
}

/// 🧰️ Owned inputs consumed by the resumable preparation job.
pub struct PreparedRenderInput {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: Vec<ScissorRect>,
    pub clips: Vec<ScissorRect>,
    pub directives: Vec<RenderDirective>,
    pub uploads: Vec<PreparedRenderUpload>,
    pub raster_producers: VecDeque<PreparedRasterProducer>,
    pub evictions: Vec<PreparedRenderEviction>,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub limits: PreparedRenderLimits,
}

impl PreparedRenderInput {
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        let limits = PreparedRenderLimits::default();
        Self {
            scene_revision,
            preview_generation,
            damage: Vec::new(),
            clips: Vec::new(),
            directives: vec![RenderDirective::PreservePreviousOnFailure],
            uploads: Vec::with_capacity(limits.max_upload_items),
            raster_producers: VecDeque::with_capacity(limits.max_upload_items),
            evictions: Vec::new(),
            draw,
            overlay,
            time_seconds,
            limits,
        }
    }
}
//#endregion 📦️Packet

//#region ⚙️PreparationJob
/// 📬️ Bounded single-packet handoff retained after a worker consumes the job.
#[derive(Clone, Default)]
pub struct PreparedRenderReceiver {
    latest: Arc<Mutex<Option<PreparedRenderPacket>>>,
}

impl PreparedRenderReceiver {
    pub fn take_latest(&self) -> Option<PreparedRenderPacket> {
        self.latest.lock().expect("prepared render receiver").take()
    }

    fn publish(&self, packet: PreparedRenderPacket) {
        *self.latest.lock().expect("prepared render receiver") = Some(packet);
    }

    fn close_step(&self) -> bool {
        let mut latest = self.latest.lock().expect("prepared render receiver");
        let Some(packet) = latest.as_mut() else { return true };
        if !packet.retire_step() {
            return false;
        }
        *latest = None;
        false
    }

    fn terminal_is_empty(&self) -> bool {
        self.latest.lock().expect("prepared render receiver").is_none()
    }
}

/// ⚙️ Bounded worker job that measures and seals one owned render packet.
pub struct PreparedRenderJob {
    input: Option<PreparedRenderInput>,
    usage: PreparedRenderUsage,
    section: PreparationSection,
    draw_cursor: DrawMeasureCursor,
    overlay_cursor: DrawMeasureCursor,
    metadata_cursor: usize,
    items_per_step: usize,
    receiver: PreparedRenderReceiver,
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
    Complete,
}

#[derive(Clone, Copy)]
enum DrawMeasureCursor {
    LayerHeader(usize),
    LayerRaster { layer: usize, raster: usize },
    PassHeader(usize),
    PassDraw { pass: usize, draw: usize, translucent: bool },
    PassInstance { pass: usize, draw: usize, instance: usize, translucent: bool },
    PassLine { pass: usize, draw: usize },
    PassTextured { pass: usize, draw: usize },
    PassTexturedInstance { pass: usize, draw: usize, instance: usize },
    Glass(usize),
    Complete,
}

impl Default for DrawMeasureCursor {
    fn default() -> Self {
        Self::LayerHeader(0)
    }
}

impl PreparedRenderJob {
    pub fn new(input: PreparedRenderInput, items_per_step: usize) -> Self {
        Self {
            input: Some(input),
            usage: PreparedRenderUsage::default(),
            section: PreparationSection::Draw,
            draw_cursor: DrawMeasureCursor::default(),
            overlay_cursor: DrawMeasureCursor::default(),
            metadata_cursor: 0,
            items_per_step: items_per_step.max(1),
            receiver: PreparedRenderReceiver::default(),
        }
    }

    pub fn receiver(&self) -> PreparedRenderReceiver {
        self.receiver.clone()
    }

    pub fn take_packet(&self) -> Option<PreparedRenderPacket> {
        self.receiver.take_latest()
    }

    pub fn close_step(&mut self) -> bool {
        let Some(input) = self.input.as_mut() else { return self.receiver.close_step() };
        if let Some(upload) = input.uploads.last_mut() {
            let retained = match upload {
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => pixels.pop().is_some(),
                PreparedRenderUpload::Raster { key, pixels, .. } => pixels.pop().is_some() || key.pop().is_some(),
                PreparedRenderUpload::RasterPages { key, pixels } => !pixels.retire_with_key_step(key),
                PreparedRenderUpload::Mesh { key, .. } => key.pop().is_some(),
            };
            if retained {
                return false;
            }
            input.uploads.pop();
            return false;
        }
        if let Some(producer) = input.raster_producers.back_mut() {
            producer.begin_close();
            if !producer.close_step() {
                return false;
            }
            assert!(producer.terminal_is_empty(), "closed raster producer must be terminal-empty");
            input.raster_producers.pop_back();
            return false;
        }
        if let Some(PreparedRenderEviction::Mesh { key }) = input.evictions.last_mut() {
            if key.pop().is_some() {
                return false;
            }
            input.evictions.pop();
            return false;
        }
        if !input.draw.retire_step() {
            return false;
        }
        if let Some(overlay) = input.overlay.as_mut() {
            if !overlay.retire_step() {
                return false;
            }
            input.overlay = None;
            return false;
        }
        if input.damage.pop().is_some() || input.clips.pop().is_some() || input.directives.pop().is_some() {
            return false;
        }
        self.input = None;
        false
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.input.is_none() && self.receiver.terminal_is_empty()
    }

    fn input(&self) -> &PreparedRenderInput {
        self.input.as_ref().expect("prepared render input")
    }

    fn measure_layer_header(layer: &DrawLayer) -> PreparedRenderUsage {
        let ui = layer.ui_instances.len().saturating_add(layer.overlay_ui_instances.len());
        let vector = layer.vector_vertices.len().saturating_add(layer.overlay_vector_vertices.len());
        let items = ui.saturating_add(vector);
        let bytes = ui.saturating_mul(size_of::<crate::wgpu::draw::UiInstance>()).saturating_add(vector.saturating_mul(size_of::<crate::wgpu::draw::VectorVertex>()));
        PreparedRenderUsage { draw_items: items, draw_bytes: bytes, ..PreparedRenderUsage::default() }
    }

    fn next_draw_usage(draw: &DrawList, cursor: &mut DrawMeasureCursor) -> Option<PreparedRenderUsage> {
        let (usage, next) = match *cursor {
            DrawMeasureCursor::LayerHeader(layer) => {
                let Some(value) = draw.layers.get(layer) else {
                    *cursor = DrawMeasureCursor::PassHeader(0);
                    return Self::next_draw_usage(draw, cursor);
                };
                let next = if value.raster_instances.is_empty() { DrawMeasureCursor::LayerHeader(layer + 1) } else { DrawMeasureCursor::LayerRaster { layer, raster: 0 } };
                (Self::measure_layer_header(value), next)
            }
            DrawMeasureCursor::LayerRaster { layer, raster } => {
                let value = &draw.layers[layer].raster_instances[raster];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::UiInstance>().saturating_add(value.0.len()), ..PreparedRenderUsage::default() };
                let next = if raster + 1 < draw.layers[layer].raster_instances.len() { DrawMeasureCursor::LayerRaster { layer, raster: raster + 1 } } else { DrawMeasureCursor::LayerHeader(layer + 1) };
                (usage, next)
            }
            DrawMeasureCursor::PassHeader(pass) => {
                let Some(value) = draw.scene_passes.get(pass) else {
                    *cursor = DrawMeasureCursor::Glass(0);
                    return Self::next_draw_usage(draw, cursor);
                };
                let next = if value.draws.is_empty() { Self::next_after_opaque(draw, pass) } else { DrawMeasureCursor::PassDraw { pass, draw: 0, translucent: false } };
                (PreparedRenderUsage::default(), next)
            }
            DrawMeasureCursor::PassDraw { pass, draw: draw_index, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index];
                let usage = PreparedRenderUsage { draw_bytes: value.mesh_key.len(), ..PreparedRenderUsage::default() };
                let next = if value.instances.is_empty() { Self::next_pass_draw(draw, pass, draw_index, translucent) } else { DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: 0, translucent } };
                (usage, next)
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let pass_value = &draw.scene_passes[pass];
                let draws = if translucent { &pass_value.translucent_draws } else { &pass_value.draws };
                let value = &draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::Instance3d>().saturating_add(value.id.len()), ..PreparedRenderUsage::default() };
                let next = if instance + 1 < draws[draw_index].instances.len() { DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance: instance + 1, translucent } } else { Self::next_pass_draw(draw, pass, draw_index, translucent) };
                (usage, next)
            }
            DrawMeasureCursor::PassLine { pass, draw: draw_index } => {
                let pass_value = &draw.scene_passes[pass];
                let vertices = pass_value.line_draws[draw_index].vertices.len();
                let usage = PreparedRenderUsage { draw_items: vertices, draw_bytes: vertices.saturating_mul(size_of::<crate::wgpu::kernel_3d_scene::LineVertex3d>()), ..PreparedRenderUsage::default() };
                let next = if draw_index + 1 < pass_value.line_draws.len() { DrawMeasureCursor::PassLine { pass, draw: draw_index + 1 } } else { Self::next_after_lines(draw, pass) };
                (usage, next)
            }
            DrawMeasureCursor::PassTextured { pass, draw: draw_index } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index];
                let next = if value.instances.is_empty() { Self::next_textured_draw(draw, pass, draw_index) } else { DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: 0 } };
                (PreparedRenderUsage::default(), next)
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let value = &draw.scene_passes[pass].textured_draws[draw_index].instances[instance];
                let usage = PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::kernel_3d_scene::TexturedInstance3d>().saturating_add(value.texture_key.len()), ..PreparedRenderUsage::default() };
                let next = if instance + 1 < draw.scene_passes[pass].textured_draws[draw_index].instances.len() {
                    DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance: instance + 1 }
                } else {
                    Self::next_textured_draw(draw, pass, draw_index)
                };
                (usage, next)
            }
            DrawMeasureCursor::Glass(index) => {
                if index >= draw.glass_regions.len() {
                    *cursor = DrawMeasureCursor::Complete;
                    return None;
                }
                (PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<crate::wgpu::draw::GlassRegion>(), ..PreparedRenderUsage::default() }, DrawMeasureCursor::Glass(index + 1))
            }
            DrawMeasureCursor::Complete => return None,
        };
        *cursor = next;
        Some(usage)
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
                let input = self.input.as_ref().expect("prepared render input");
                if let Some(usage) = Self::next_draw_usage(&input.draw, &mut self.draw_cursor) {
                    return Some(usage);
                }
                self.section = PreparationSection::Overlay;
                self.measure_next()
            }
            PreparationSection::Overlay => {
                let input = self.input.as_ref().expect("prepared render input");
                if let Some(overlay) = &input.overlay {
                    if let Some(usage) = Self::next_draw_usage(overlay, &mut self.overlay_cursor) {
                        return Some(usage);
                    }
                }
                self.section = PreparationSection::Uploads;
                self.metadata_cursor = 0;
                self.measure_next()
            }
            PreparationSection::Uploads => {
                let input = self.input.as_ref().expect("prepared render input");
                if let Some(upload) = input.uploads.get(self.metadata_cursor) {
                    self.metadata_cursor += 1;
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: upload.byte_len(), ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Evictions;
                self.metadata_cursor = 0;
                self.measure_next()
            }
            PreparationSection::Evictions => {
                let input = self.input.as_ref().expect("prepared render input");
                if let Some(eviction) = input.evictions.get(self.metadata_cursor) {
                    self.metadata_cursor += 1;
                    return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: eviction.byte_len(), ..PreparedRenderUsage::default() });
                }
                self.section = PreparationSection::Damage;
                self.metadata_cursor = 0;
                self.measure_next()
            }
            PreparationSection::Damage => self.measure_metadata(PreparationSection::Clips, |input| input.damage.len(), size_of::<ScissorRect>()),
            PreparationSection::Clips => self.measure_metadata(PreparationSection::Directives, |input| input.clips.len(), size_of::<ScissorRect>()),
            PreparationSection::Directives => self.measure_metadata(PreparationSection::Complete, |input| input.directives.len(), size_of::<RenderDirective>()),
            PreparationSection::Complete => None,
        }
    }

    fn measure_metadata(&mut self, next: PreparationSection, len: impl FnOnce(&PreparedRenderInput) -> usize, bytes: usize) -> Option<PreparedRenderUsage> {
        if self.metadata_cursor < len(self.input()) {
            self.metadata_cursor += 1;
            Some(PreparedRenderUsage { draw_items: 1, draw_bytes: bytes, ..PreparedRenderUsage::default() })
        } else {
            self.section = next;
            self.metadata_cursor = 0;
            self.measure_next()
        }
    }

    fn include_usage(&mut self, usage: PreparedRenderUsage) {
        self.usage.include_draw(usage.draw_items, usage.draw_bytes);
        if usage.upload_items > 0 {
            self.usage.include_upload(usage.upload_bytes);
        }
    }

    fn complete(&mut self) -> StepOutcome {
        let input = self.input.take().expect("prepared render input");
        let revision = input.scene_revision;
        let generation = input.preview_generation;
        let packet = PreparedRenderPacket {
            scene_revision: revision,
            preview_generation: generation,
            damage: input.damage,
            clips: input.clips,
            directives: input.directives,
            uploads: input.uploads,
            evictions: input.evictions,
            draw: input.draw,
            overlay: input.overlay,
            time_seconds: input.time_seconds,
            usage: self.usage,
            limits: input.limits,
            retirement_phase: 0,
        };
        self.receiver.publish(packet);
        let mut output = Vec::with_capacity(16);
        output.extend_from_slice(&revision.to_le_bytes());
        output.extend_from_slice(&generation.to_le_bytes());
        StepOutcome::Complete(CommitCandidate { state: output.clone(), output })
    }
}

impl InteractiveJob for PreparedRenderJob {
    fn step(&mut self, cx: &mut StepContext<'_>) -> StepOutcome {
        if cx.is_cancelled() {
            return StepOutcome::Cancelled;
        }
        if self.input().preview_generation != cx.generation().0 {
            return StepOutcome::Fault(JobFault { detail: b"prepared render generation is stale".to_vec() });
        }
        if let Some(producer) = self.input.as_mut().expect("prepared render input").raster_producers.front_mut() {
            match producer.step(cx.generation().0) {
                PreparedRasterProducerStep::Pending => return StepOutcome::Yield,
                PreparedRasterProducerStep::Complete(upload) => {
                    let input = self.input.as_mut().expect("prepared render input");
                    input.raster_producers.pop_front();
                    input.uploads.push(upload);
                    return StepOutcome::Yield;
                }
                PreparedRasterProducerStep::Fault(fault) => return StepOutcome::Fault(JobFault { detail: fault.as_bytes().to_vec() }),
            }
        }
        let mut processed = 0usize;
        while processed < self.items_per_step && !cx.should_yield() {
            let Some(usage) = self.measure_next() else {
                return self.complete();
            };
            self.include_usage(usage);
            if !self.usage.fits(self.input().limits) {
                return StepOutcome::Fault(JobFault { detail: b"prepared render credits exceeded".to_vec() });
            }
            processed += 1;
            cx.consume_fuel(1);
        }
        StepOutcome::Yield
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
        let pending = self.pending.take().expect("validated pending presentation");
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

    fn now_ms() -> u64 {
        1
    }

    fn packet(revision: u64, generation: u64) -> PreparedRenderPacket {
        PreparedRenderPacket {
            scene_revision: revision,
            preview_generation: generation,
            damage: Vec::new(),
            clips: Vec::new(),
            directives: vec![RenderDirective::PreservePreviousOnFailure],
            uploads: Vec::new(),
            evictions: Vec::new(),
            draw: DrawList::default(),
            overlay: None,
            time_seconds: 0.0,
            usage: PreparedRenderUsage::default(),
            limits: PreparedRenderLimits::default(),
            retirement_phase: 0,
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
    fn raster_process_bytes_plus_one_returns_the_exact_reserved_backing() {
        let mut source = Vec::with_capacity(PREPARED_RASTER_PRODUCER_BYTES);
        source.extend_from_slice(&[1, 2, 3, 4]);
        let pointer = source.as_ptr();
        let capacity = source.capacity();
        let mut rejected = PreparedRasterProducer::try_admit("bytes-plus-one".into(), source, 1, 1).expect_err("source backing plus derived page exceeds process bytes");
        assert_eq!(rejected.source.as_ptr(), pointer);
        assert_eq!(rejected.source.capacity(), capacity);
        assert_eq!(rejected.fault(), "raster producer process credits exhausted");
        while !rejected.close_step() {}
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
    fn cancellation_retires_large_upload_incrementally_before_terminal_empty() {
        let mut input = PreparedRenderInput::new(7, 3, DrawList::default(), None, 0.0);
        input.uploads.push(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4_096], width: 64, height: 64 });
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
        let receiver = job.receiver();
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
        superseding.uploads.push(PreparedRenderUpload::GlyphAtlas { pixels: vec![7; 16_385], width: 1, height: 1 });
        let pixels = match &superseding.uploads[0] {
            PreparedRenderUpload::GlyphAtlas { pixels, .. } => pixels.as_ptr(),
            _ => unreachable!(),
        };
        let mut returned = match gate.stage_presented(superseding) {
            Ok(_) => panic!("second presenter witness must fail closed"),
            Err(packet) => packet,
        };
        assert_eq!((returned.scene_revision, returned.preview_generation), (8, 4));
        assert!(matches!(&returned.uploads[0], PreparedRenderUpload::GlyphAtlas { pixels: returned_pixels, .. } if returned_pixels.as_ptr() == pixels));
        assert!(!returned.retire_step(), "one close grant retires only one admitted pixel page");
        assert!(matches!(&returned.uploads[0], PreparedRenderUpload::GlyphAtlas { pixels, .. } if pixels.len() == 1));
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
        input.uploads.push(PreparedRenderUpload::GlyphAtlas { pixels: vec![0; 4], width: 2, height: 2 });
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
        input.evictions.push(PreparedRenderEviction::Mesh { key: "mesh".into() });
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
}
