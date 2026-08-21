//! 📦️ Worker-owned render preparation and UI-authorized presentation contract.

use crate::wgpu::draw::{DrawLayer, DrawList, ScissorRect};
use semio_framework_job::{CommitCandidate, InteractiveJob, JobFault, StepContext, StepOutcome};
use std::mem::size_of;
use std::rc::Rc;
use std::sync::Arc;

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
    Mesh { key: String, version: u64, positions: Vec<f32>, normals: Vec<f32>, indices: Vec<u32> },
}

impl PreparedRenderUpload {
    pub fn byte_len(&self) -> usize {
        match self {
            Self::GlyphAtlas { pixels, .. } | Self::IconAtlas { pixels, .. } => pixels.len(),
            Self::Raster { key, pixels, .. } => key.len().saturating_add(pixels.len()),
            Self::Mesh { key, positions, normals, indices, .. } => {
                key.len().saturating_add(positions.len().saturating_mul(size_of::<f32>())).saturating_add(normals.len().saturating_mul(size_of::<f32>())).saturating_add(indices.len().saturating_mul(size_of::<u32>()))
            }
        }
    }
}

/// 🧱️ Send-capable frame data prepared without window, surface, device, or queue access.
#[derive(Clone)]
pub struct PreparedRenderPacket {
    pub scene_revision: u64,
    pub preview_generation: u64,
    pub damage: Vec<ScissorRect>,
    pub clips: Vec<ScissorRect>,
    pub directives: Vec<RenderDirective>,
    pub uploads: Vec<PreparedRenderUpload>,
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub usage: PreparedRenderUsage,
    pub limits: PreparedRenderLimits,
}

impl PreparedRenderPacket {
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
    pub draw: DrawList,
    pub overlay: Option<DrawList>,
    pub time_seconds: f32,
    pub limits: PreparedRenderLimits,
}

impl PreparedRenderInput {
    pub fn new(scene_revision: u64, preview_generation: u64, draw: DrawList, overlay: Option<DrawList>, time_seconds: f32) -> Self {
        Self { scene_revision, preview_generation, damage: Vec::new(), clips: Vec::new(), directives: vec![RenderDirective::PreservePreviousOnFailure], uploads: Vec::new(), draw, overlay, time_seconds, limits: PreparedRenderLimits::default() }
    }
}
//#endregion 📦️Packet

//#region ⚙️PreparationJob
/// ⚙️ Bounded worker job that measures and seals one owned render packet.
pub struct PreparedRenderJob {
    input: Option<PreparedRenderInput>,
    usage: PreparedRenderUsage,
    cursor: usize,
    items_per_step: usize,
    packet: Option<PreparedRenderPacket>,
}

impl PreparedRenderJob {
    pub fn new(input: PreparedRenderInput, items_per_step: usize) -> Self {
        Self { input: Some(input), usage: PreparedRenderUsage::default(), cursor: 0, items_per_step: items_per_step.max(1), packet: None }
    }

    pub fn take_packet(&mut self) -> Option<PreparedRenderPacket> {
        self.packet.take()
    }

    fn input(&self) -> &PreparedRenderInput {
        self.input.as_ref().expect("prepared render input")
    }

    fn draw_work_len(draw: &DrawList) -> usize {
        draw.layers.len().saturating_add(draw.scene_passes.len()).saturating_add(draw.glass_regions.len())
    }

    fn total_work(&self) -> usize {
        let input = self.input();
        Self::draw_work_len(&input.draw)
            .saturating_add(input.overlay.as_ref().map_or(0, Self::draw_work_len))
            .saturating_add(input.uploads.len())
            .saturating_add(input.damage.len())
            .saturating_add(input.clips.len())
            .saturating_add(input.directives.len())
    }

    fn measure_layer(layer: &DrawLayer) -> (usize, usize) {
        let ui = layer.ui_instances.len().saturating_add(layer.overlay_ui_instances.len());
        let vector = layer.vector_vertices.len().saturating_add(layer.overlay_vector_vertices.len());
        let raster = layer.raster_instances.len();
        let raster_key_bytes = layer.raster_instances.iter().map(|(key, _)| key.len()).sum::<usize>();
        let items = ui.saturating_add(vector).saturating_add(raster);
        let bytes = ui
            .saturating_mul(size_of::<crate::wgpu::draw::UiInstance>())
            .saturating_add(vector.saturating_mul(size_of::<crate::wgpu::draw::VectorVertex>()))
            .saturating_add(raster.saturating_mul(size_of::<crate::wgpu::draw::UiInstance>()))
            .saturating_add(raster_key_bytes);
        (items, bytes)
    }

    fn measure_draw_item(draw: &DrawList, cursor: usize) -> Option<(usize, usize)> {
        if let Some(layer) = draw.layers.get(cursor) {
            return Some(Self::measure_layer(layer));
        }
        let cursor = cursor.saturating_sub(draw.layers.len());
        if let Some(pass) = draw.scene_passes.get(cursor) {
            let instances = pass.draws.iter().chain(pass.translucent_draws.iter()).map(|draw| draw.instances.len()).sum::<usize>();
            let lines = pass.line_draws.iter().map(|draw| draw.vertices.len()).sum::<usize>();
            let textured = pass.textured_draws.iter().map(|draw| draw.instances.len()).sum::<usize>();
            let keys = pass
                .draws
                .iter()
                .chain(pass.translucent_draws.iter())
                .map(|draw| draw.mesh_key.len())
                .sum::<usize>()
                .saturating_add(pass.textured_draws.iter().flat_map(|draw| draw.instances.iter()).map(|instance| instance.texture_key.len()).sum::<usize>());
            let items = instances.saturating_add(lines).saturating_add(textured);
            let bytes = instances
                .saturating_mul(size_of::<crate::wgpu::kernel_3d_scene::Instance3d>())
                .saturating_add(lines.saturating_mul(size_of::<crate::wgpu::kernel_3d_scene::LineVertex3d>()))
                .saturating_add(textured.saturating_mul(size_of::<crate::wgpu::kernel_3d_scene::TexturedInstance3d>()))
                .saturating_add(keys);
            return Some((items, bytes));
        }
        let cursor = cursor.saturating_sub(draw.scene_passes.len());
        draw.glass_regions.get(cursor).map(|_| (1, size_of::<crate::wgpu::draw::GlassRegion>()))
    }

    fn measure_cursor(&self, cursor: usize) -> Option<PreparedRenderUsage> {
        let input = self.input();
        let draw_len = Self::draw_work_len(&input.draw);
        if cursor < draw_len {
            let (items, bytes) = Self::measure_draw_item(&input.draw, cursor)?;
            return Some(PreparedRenderUsage { draw_items: items, draw_bytes: bytes, ..PreparedRenderUsage::default() });
        }
        let mut cursor = cursor.saturating_sub(draw_len);
        if let Some(overlay) = &input.overlay {
            let overlay_len = Self::draw_work_len(overlay);
            if cursor < overlay_len {
                let (items, bytes) = Self::measure_draw_item(overlay, cursor)?;
                return Some(PreparedRenderUsage { draw_items: items, draw_bytes: bytes, ..PreparedRenderUsage::default() });
            }
            cursor = cursor.saturating_sub(overlay_len);
        }
        if let Some(upload) = input.uploads.get(cursor) {
            return Some(PreparedRenderUsage { upload_items: 1, upload_bytes: upload.byte_len(), ..PreparedRenderUsage::default() });
        }
        cursor = cursor.saturating_sub(input.uploads.len());
        let metadata = input.damage.len().saturating_add(input.clips.len()).saturating_add(input.directives.len());
        (cursor < metadata).then_some(PreparedRenderUsage { draw_items: 1, draw_bytes: size_of::<ScissorRect>(), ..PreparedRenderUsage::default() })
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
        self.packet = Some(PreparedRenderPacket {
            scene_revision: revision,
            preview_generation: generation,
            damage: input.damage,
            clips: input.clips,
            directives: input.directives,
            uploads: input.uploads,
            draw: input.draw,
            overlay: input.overlay,
            time_seconds: input.time_seconds,
            usage: self.usage,
            limits: input.limits,
        });
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
        let total = self.total_work();
        let mut processed = 0usize;
        while self.cursor < total && processed < self.items_per_step && !cx.should_yield() {
            let usage = self.measure_cursor(self.cursor).expect("prepared render work cursor");
            self.include_usage(usage);
            if !self.usage.fits(self.input().limits) {
                return StepOutcome::Fault(JobFault { detail: b"prepared render credits exceeded".to_vec() });
            }
            self.cursor += 1;
            processed += 1;
            cx.consume_fuel(1);
        }
        if self.cursor < total {
            StepOutcome::Yield
        } else {
            self.complete()
        }
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

    pub fn commit_presented(&mut self, packet: Arc<PreparedRenderPacket>) {
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
            draw: DrawList::default(),
            overlay: None,
            time_seconds: 0.0,
            usage: PreparedRenderUsage::default(),
            limits: PreparedRenderLimits::default(),
        }
    }

    fn assert_send<T: Send>() {}

    #[test]
    fn prepared_packet_is_send_owned_data() {
        assert_send::<PreparedRenderPacket>();
        assert_send::<PreparedRenderJob>();
        let packet = packet(7, 3);
        let identity = std::thread::spawn(move || (packet.scene_revision, packet.preview_generation)).join().expect("worker packet");
        assert_eq!(identity, (7, 3));
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
