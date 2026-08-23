//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use crate::wgpu::draw::{DrawLayer, DrawList, ScissorRect};
use crate::wgpu::kernel_3d_scene::Mesh3dLease;
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, StepContext, StepOutcome};
use std::mem::size_of;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

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

//#region 📦️Packet
/// 🧲️ Presentation behavior selected during worker preparation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RenderDirective {
    FullRedraw,
    DamageOnly,
    PreservePreviousOnFailure,
}

/// 📤️ Typed upload data owned by the prepared transaction.
#[derive(Clone, Debug, PartialEq)]
pub enum PreparedRenderUpload {
    GlyphAtlas { pixels: Vec<u8>, width: u32, height: u32 },
    IconAtlas { pixels: Vec<u8>, width: u32, height: u32 },
    Raster { key: String, pixels: Vec<u8>, width: u32, height: u32 },
    Mesh { key: String, version: u64, lease: Mesh3dLease },
}

/// 🧹️ UI-thread GPU cache invalidation selected during worker preparation.
#[derive(Clone, Debug, PartialEq, Eq)]
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
#[derive(Clone)]
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
}

impl PreparedRenderPacket {
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
}

/// 🧰️ Owned inputs consumed by the resumable preparation job.
pub struct PreparedRenderInput {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: Vec<ScissorRect>,
    pub clips: Vec<ScissorRect>,
    pub directives: Vec<RenderDirective>,
    pub uploads: Vec<PreparedRenderUpload>,
    pub evictions: Vec<PreparedRenderEviction>,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub limits: PreparedRenderLimits,
}

impl PreparedRenderInput {
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        Self {
            scene_revision,
            preview_generation,
            damage: Vec::new(),
            clips: Vec::new(),
            directives: vec![RenderDirective::PreservePreviousOnFailure],
            uploads: Vec::new(),
            evictions: Vec::new(),
            draw,
            overlay,
            time_seconds,
            limits: PreparedRenderLimits::default(),
        }
    }
}
//#endregion 📦️Packet

//#region ⚙️PreparationJob
/// 📬️ Bounded single-packet handoff retained after a worker consumes the job.
#[derive(Clone, Default)]
pub struct PreparedRenderReceiver {
    latest: Arc<Mutex<Option<Arc<PreparedRenderPacket>>>>,
}

impl PreparedRenderReceiver {
    pub fn acquire_latest(&self) -> Option<Arc<PreparedRenderPacket>> {
        self.latest.lock().expect("prepared render receiver").clone()
    }

    pub fn take_latest(&self) -> Option<Arc<PreparedRenderPacket>> {
        self.latest.lock().expect("prepared render receiver").take()
    }

    fn publish(&self, packet: Arc<PreparedRenderPacket>) {
        *self.latest.lock().expect("prepared render receiver") = Some(packet);
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

    pub fn take_packet(&self) -> Option<Arc<PreparedRenderPacket>> {
        self.receiver.take_latest()
    }

    pub fn close_step(&mut self) -> bool {
        let Some(input) = self.input.as_mut() else { return self.receiver.take_latest().is_none() };
        if let Some(upload) = input.uploads.last_mut() {
            let retained = match upload {
                PreparedRenderUpload::GlyphAtlas { pixels, .. } | PreparedRenderUpload::IconAtlas { pixels, .. } => pixels.pop().is_some(),
                PreparedRenderUpload::Raster { key, pixels, .. } => pixels.pop().is_some() || key.pop().is_some(),
                PreparedRenderUpload::Mesh { key, .. } => key.pop().is_some(),
            };
            if retained {
                return false;
            }
            input.uploads.pop();
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
        self.input.is_none() && self.receiver.acquire_latest().is_none()
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
        let packet = Arc::new(PreparedRenderPacket {
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
        });
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
}

impl std::fmt::Display for PreparedRenderRejection {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleRevision { live, packet } => write!(formatter, "prepared render revision is stale: live={live}, packet={packet}"),
            Self::StaleGeneration { live, packet } => write!(formatter, "prepared render generation is stale: live={live}, packet={packet}"),
            Self::Credits => formatter.write_str("prepared render packet exceeds its credits"),
        }
    }
}

impl std::error::Error for PreparedRenderRejection {}

/// 📸️ Last-valid packet state preserved across cancellation, rejection, and device loss.
#[derive(Default)]
pub struct PreparedRenderGate {
    last_valid: Option<Arc<PreparedRenderPacket>>,
}

impl PreparedRenderGate {
    pub fn validate(&self, packet: &PreparedRenderPacket, live_revision: u64, live_generation: u64) -> Result<(), PreparedRenderRejection> {
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

    pub(crate) fn commit_presented(&mut self, packet: Arc<PreparedRenderPacket>) {
        self.last_valid = Some(packet);
    }

    pub fn last_valid(&self) -> Option<Arc<PreparedRenderPacket>> {
        self.last_valid.clone()
    }

    pub fn retain_after_device_loss(&self) -> Option<Arc<PreparedRenderPacket>> {
        self.last_valid()
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
        }
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
        let valid = Arc::new(packet(7, 3));
        gate.commit_presented(valid.clone());
        let stale = packet(6, 3);
        assert!(matches!(gate.validate(&stale, 7, 3), Err(PreparedRenderRejection::StaleRevision { .. })));
        assert!(Arc::ptr_eq(&gate.last_valid().expect("last valid"), &valid));
    }

    #[test]
    fn generation_rejection_happens_before_presentation() {
        let gate = PreparedRenderGate::default();
        assert!(matches!(gate.validate(&packet(7, 2), 7, 3), Err(PreparedRenderRejection::StaleGeneration { .. })));
    }

    #[test]
    fn device_loss_retains_the_last_valid_packet() {
        let mut gate = PreparedRenderGate::default();
        let valid = Arc::new(packet(7, 3));
        gate.commit_presented(valid.clone());
        assert!(Arc::ptr_eq(&gate.retain_after_device_loss().expect("retained"), &valid));
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
