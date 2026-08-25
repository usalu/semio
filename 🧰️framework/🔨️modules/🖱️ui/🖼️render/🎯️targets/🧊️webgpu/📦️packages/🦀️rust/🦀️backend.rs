//! @emoji 🌐️ [`WebGpuBackend`] — this crate's concrete `ui_render::GraphicsBackend`. Owns the device/
//! surface/pipelines/resources and drives one frame through `crate::frame::render`. Everything device-
//! shaped lives behind this one type; nothing else in the crate is `pub`.

use crate::buffers::{FrameBuffers, WorldGlobalsRing};
use crate::gpu_context::{configure_surface, create_depth_texture, GpuContext};
use crate::pipelines::Pipelines;
use crate::resources::GpuResources;
use crate::scene_target::SceneColorTarget;
use crate::surface_state::{DeviceHealth, SurfaceState};
use crate::{GpuOutcome, SurfaceGeneration, SurfaceId};
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::Arc;
use ui_render::{BackendError, DeviceCapabilities, DeviceStatus, GraphicsBackend, LossReason, PhysicalSize, RecoveredResources, RenderPacket, RenderReport, ResourceOp};

//#region 🔖️Backend

//#region 🚨️LossFlag

/// 🚨️ `wgpu::Device::set_device_lost_callback` requires a plain `Send` closure even on wasm32 (no
/// `WasmNotSend` relaxation for this one), so the shared flag it writes into must be an `Arc<Atomic*>`
/// rather than the `Rc<Cell<_>>` this crate uses everywhere else single-threaded state is shared.
const LOSS_NONE: u8 = 0;
const LOSS_SURFACE: u8 = 1;
const LOSS_DEVICE: u8 = 2;
const LOSS_TIMEOUT: u8 = 3;

// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn decode_loss_flag(value: u8) -> Option<LossReason> {
    match value {
        LOSS_SURFACE => Some(LossReason::Surface),
        LOSS_DEVICE => Some(LossReason::Device),
        LOSS_TIMEOUT => Some(LossReason::Timeout),
        _ => None,
    }
}

//#endregion 🚨️LossFlag

//#region 🧪️Readback

#[cfg(feature = "backend-testing")]
struct PendingReadback {
    buffer: wgpu::Buffer,
    width: u32,
    height: u32,
    padded_bytes_per_row: u32,
    mapped: std::rc::Rc<std::cell::Cell<Option<Result<(), wgpu::BufferAsyncError>>>>,
}

/// 📏️ `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT`-aligned row stride for an RGBA8 copy.
#[cfg(feature = "backend-testing")]
// 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
fn padded_bytes_per_row(width: u32) -> u32 {
    let unpadded = width * 4;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    unpadded.div_ceil(align) * align
}

//#endregion 🧪️Readback

//#region 🌐️WebGpuBackend

/// 🌐️ Browser WebGPU resources admitted by the owned surface-port state machine.
pub struct WebGpuBackend {
    surface_id: SurfaceId,
    generation: SurfaceGeneration,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: wgpu::Surface<'static>,
    surface_format: wgpu::TextureFormat,
    view_format: wgpu::TextureFormat,
    alpha_mode: wgpu::CompositeAlphaMode,
    pipelines: Pipelines,
    resources: GpuResources,
    frame_buffers: FrameBuffers,
    world_ring: WorldGlobalsRing,
    scene_target: Option<SceneColorTarget>,
    depth_texture: wgpu::Texture,
    depth_view: wgpu::TextureView,
    state: SurfaceState,
    health: DeviceHealth,
    capabilities: DeviceCapabilities,
    loss_flag: Arc<AtomicU8>,
    #[cfg(feature = "backend-testing")]
    pending_readback: Option<PendingReadback>,
}

impl WebGpuBackend {
    /// 🧊️ Consumes only a successful owned create/recover outcome; browser objects remain in JS/A2.
    // 🌐️async: adapter/device construction remains inside the owned renderer interface until P9-C.
    pub async fn from_outcome(outcome: &GpuOutcome) -> Result<Self, BackendError> {
        let (surface_id, generation, metrics) = match outcome {
            GpuOutcome::Created { surface, generation, metrics, .. } => (*surface, *generation, *metrics),
            GpuOutcome::Recovered { surface, generation, metrics, .. } => (*surface, *generation, *metrics),
            _ => return Err(BackendError::CanvasReplaced),
        };
        let width = metrics.width.max(1);
        let height = metrics.height.max(1);
        let context = GpuContext::new(surface_id).await?;
        context.configure(width, height);

        let pipelines = Pipelines::new(&context.device, context.view_format);
        let resources = GpuResources::new(&context.device, &pipelines);
        let world_ring = WorldGlobalsRing::new(&context.device, &pipelines.world_globals_layout, 8);
        let depth_texture = create_depth_texture(&context.device, width, height);
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let loss_flag = Arc::new(AtomicU8::new(LOSS_NONE));
        let callback_flag = Arc::clone(&loss_flag);
        context.device.set_device_lost_callback(move |reason, _message| {
            let encoded = match reason {
                wgpu::DeviceLostReason::Destroyed => LOSS_DEVICE,
                wgpu::DeviceLostReason::Unknown => LOSS_DEVICE,
            };
            callback_flag.store(encoded, Ordering::SeqCst);
        });

        let mut state = SurfaceState::default();
        state.resize(PhysicalSize::new(metrics.width, metrics.height), metrics.scale_factor);

        Ok(Self {
            surface_id,
            generation,
            device: context.device,
            queue: context.queue,
            surface: context.surface,
            surface_format: context.surface_format,
            view_format: context.view_format,
            alpha_mode: context.alpha_mode,
            pipelines,
            resources,
            frame_buffers: FrameBuffers::default(),
            world_ring,
            scene_target: None,
            depth_texture,
            depth_view,
            state,
            health: DeviceHealth::Healthy,
            capabilities: context.capabilities,
            loss_flag,
            #[cfg(feature = "backend-testing")]
            pending_readback: None,
        })
    }

    pub const fn surface_id(&self) -> SurfaceId {
        self.surface_id
    }

    pub const fn surface_generation(&self) -> SurfaceGeneration {
        self.generation
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn poll_device_lost(&mut self) {
        if let Some(reason) = decode_loss_flag(self.loss_flag.load(Ordering::SeqCst)) {
            self.health = DeviceHealth::Lost(reason);
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn reconfigure(&mut self, width: u32, height: u32) {
        configure_surface(&self.device, &self.surface, self.surface_format, self.view_format, self.alpha_mode, width, height);
        self.scene_target = None;
        self.depth_texture = create_depth_texture(&self.device, width, height);
        self.depth_view = self.depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn map_surface_error(&mut self, error: wgpu::SurfaceError) -> Result<RenderReport, BackendError> {
        match error {
            wgpu::SurfaceError::Timeout => Ok(RenderReport::SkippedOutOfDate),
            wgpu::SurfaceError::Outdated => Ok(RenderReport::SkippedOutOfDate),
            wgpu::SurfaceError::Lost => {
                self.health = DeviceHealth::Lost(LossReason::Surface);
                Err(BackendError::SurfaceLost)
            }
            wgpu::SurfaceError::OutOfMemory => Err(BackendError::OutOfMemory),
            wgpu::SurfaceError::Other => {
                self.health = DeviceHealth::Lost(LossReason::Surface);
                Err(BackendError::SurfaceLost)
            }
        }
    }

    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn kick_off_readback(&mut self, texture: &wgpu::Texture, width: u32, height: u32) {
        let padded = padded_bytes_per_row(width);
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor { label: Some("readback_staging"), size: padded as u64 * height as u64, usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST, mapped_at_creation: false });
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("readback_copy") });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
            wgpu::TexelCopyBufferInfo { buffer: &buffer, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(padded), rows_per_image: Some(height) } },
            wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        );
        self.queue.submit(Some(encoder.finish()));
        let mapped = std::rc::Rc::new(std::cell::Cell::new(None));
        let callback_mapped = std::rc::Rc::clone(&mapped);
        buffer.slice(..).map_async(wgpu::MapMode::Read, move |result| callback_mapped.set(Some(result)));
        self.pending_readback = Some(PendingReadback { buffer, width, height, padded_bytes_per_row: padded, mapped });
    }
}

impl GraphicsBackend for WebGpuBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str {
        "webgpu"
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities {
        self.capabilities
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError> {
        self.state.resize(size, dpr);
        if !self.state.is_parked() {
            self.reconfigure(size.width, size.height);
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ResourceOp]) -> Result<(), BackendError> {
        self.resources.apply(ops, &self.device, &self.queue, &self.pipelines)
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, time_seconds: f32) -> Result<RenderReport, BackendError> {
        self.poll_device_lost();
        if let DeviceHealth::Lost(reason) = self.health {
            return Err(BackendError::DeviceLost(reason));
        }
        if self.state.is_parked() {
            return Ok(RenderReport::SkippedZeroSize);
        }

        let frame = match self.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(error) => return self.map_surface_error(error),
        };
        if frame.suboptimal && !matches!(self.health, DeviceHealth::Lost(_)) {
            self.health = DeviceHealth::Suboptimal;
        }
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.view_format), ..Default::default() });

        let width = self.state.size.width;
        let height = self.state.size.height;
        if !self.scene_target.as_ref().is_some_and(|target| target.matches(width, height)) {
            self.scene_target = Some(SceneColorTarget::new(&self.device, width, height, self.view_format));
        }
        let scene = self.scene_target.as_ref().expect("just ensured above");

        let stats = crate::frame::render(&self.device, &self.queue, &view, &self.depth_view, scene, &self.pipelines, &self.resources, &mut self.frame_buffers, &mut self.world_ring, packet, width as f32, height as f32, time_seconds)?;

        #[cfg(feature = "backend-testing")]
        self.kick_off_readback(&frame.texture, width, height);

        frame.present();
        if !matches!(self.health, DeviceHealth::Lost(_)) {
            self.health = DeviceHealth::Healthy;
        }
        Ok(RenderReport::Presented { stats })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus {
        match decode_loss_flag(self.loss_flag.load(Ordering::SeqCst)) {
            Some(reason) => DeviceStatus::Lost(reason),
            None => self.health.as_status(),
        }
    }

    /// ♻️ **Documented limitation**: a real WebGPU device loss leaves the underlying `wgpu::Device`
    /// permanently unusable, and truly rebuilding one needs `request_adapter`/`request_device` again —
    /// genuinely async steps this trait's `recover` (a plain `fn`) cannot perform. This resets
    /// bookkeeping and reports every id that was resident before the loss, exactly like
    /// `NullBackend::recover` — the caller still needs to reconstruct a fresh `WebGpuBackend` from
    /// the adapter's generation-bearing recovered outcome for rendering to resume.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError> {
        self.loss_flag.store(LOSS_NONE, Ordering::SeqCst);
        self.health = DeviceHealth::Healthy;
        let (lost_textures, lost_meshes, lost_atlases) = self.resources.resident_ids();
        self.resources.clear();
        Ok(RecoveredResources { lost_textures, lost_meshes, lost_atlases })
    }

    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self) {
        self.device.destroy();
        self.loss_flag.store(LOSS_DEVICE, Ordering::SeqCst);
        self.health = DeviceHealth::Lost(LossReason::Device);
    }

    /// 🧪️ **Documented limitation**: WebGPU buffer mapping resolves on a browser microtask, which only
    /// turns between distinct calls into this crate — never inside one synchronous `fn`. `render`
    /// (when `backend-testing` is enabled) kicks off an async copy+map of the just-presented frame at
    /// the end of the previous call; this fn only ever *harvests* whatever mapping already resolved
    /// since then. A caller must let one real frame boundary (e.g. an awaited `requestAnimationFrame`)
    /// elapse between `render` and `read_back` for this to return `Ok` — calling it immediately after
    /// `render` on the same synchronous turn returns `Err(BackendError::Timeout)`, correctly, not a
    /// stale or fabricated image.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ui_render::ReadbackImage, BackendError> {
        if self.state.is_parked() {
            return Err(BackendError::ZeroSizeSurface);
        }
        let Some(pending) = self.pending_readback.take() else {
            return Err(BackendError::Timeout);
        };
        match pending.mapped.take() {
            Some(Ok(())) => {
                let slice = pending.buffer.slice(..);
                let mapped = slice.get_mapped_range();
                let mut pixels = Vec::with_capacity(pending.width as usize * pending.height as usize * 4);
                for row in 0..pending.height {
                    let start = (row * pending.padded_bytes_per_row) as usize;
                    let end = start + pending.width as usize * 4;
                    pixels.extend_from_slice(&mapped[start..end]);
                }
                drop(mapped);
                pending.buffer.unmap();
                Ok(ui_render::ReadbackImage { width: pending.width, height: pending.height, pixels })
            }
            Some(Err(_)) => Err(BackendError::Timeout),
            None => {
                self.pending_readback = Some(pending);
                Err(BackendError::Timeout)
            }
        }
    }
}

//#endregion 🌐️WebGpuBackend

//#endregion 🔖️Backend

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loss_flag_round_trips_every_reason() {
        assert_eq!(decode_loss_flag(LOSS_NONE), None);
        assert_eq!(decode_loss_flag(LOSS_SURFACE), Some(LossReason::Surface));
        assert_eq!(decode_loss_flag(LOSS_DEVICE), Some(LossReason::Device));
        assert_eq!(decode_loss_flag(LOSS_TIMEOUT), Some(LossReason::Timeout));
    }

    #[cfg(feature = "backend-testing")]
    #[test]
    fn padded_row_rounds_up_to_alignment() {
        assert_eq!(padded_bytes_per_row(1), 256);
        assert_eq!(padded_bytes_per_row(64), 256);
        assert_eq!(padded_bytes_per_row(65), 512);
        assert_eq!(padded_bytes_per_row(256), 1024);
    }
}

//#endregion Tests
