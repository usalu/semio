// #region gpu
//! 🖥️ WebGPU device, surface, and frame loop.

use crate::wgpu::draw::{FrameBuffers, MeshGpuTable, RasterTextureAdmission, RasterTextureCleanupStep, RasterTextureStageFault, RasterTextureTable, RasterTextureWitness, RasterUploadPixels, SceneColorTarget, UiPipelines, SCENE_MIP_LEVELS};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use crate::wgpu::prepared::OffscreenPresentToken;
use crate::wgpu::prepared::{DrawMeasureCursor, PreparedRenderEviction, PreparedRenderGate, PreparedRenderPacket, PreparedRenderUpload, UiPresentToken, PREPARED_RENDER_COMMAND_PAGES, PREPARED_RENDER_COMMAND_PAGE_ITEMS};
use crate::wgpu::text::FontAtlas;
use std::sync::atomic::{AtomicPtr, AtomicU8, Ordering};
#[cfg(not(target_os = "wasi"))]
use std::sync::Arc;
use wgpu::Surface;

#[derive(Clone, Copy)]
struct PreparedAtlasUploadCursor {
    generation: u64,
    upload: usize,
    page: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum PreparedGpuPresentPhase {
    EnsureTarget,
    ClearScene,
    Commands,
    AcquireSurface,
    CreateView,
    BlurScene,
    EncodeComposite,
    GlassCommands,
    Present,
    Complete,
    Closing,
}

const PREPARED_GPU_ABANDONMENT_SLOTS: usize = 64;
static PREPARED_GPU_ABANDONMENT_STATE: [AtomicU8; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_GPU_ABANDONMENT_SLOTS];
static PREPARED_GPU_ABANDONMENT_OWNER: [AtomicPtr<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_GPU_ABANDONMENT_SLOTS];

/// 🎟️ Generation-qualified retained surface and command submission cursor.
pub struct PreparedGpuPresentCursor {
    scene_revision: u64,
    preview_generation: u64,
    command: usize,
    glass_command: usize,
    blur_mip: u32,
    frame: Option<wgpu::SurfaceTexture>,
    view: Option<wgpu::TextureView>,
    phase: PreparedGpuPresentPhase,
    abandonment_slot: u8,
}

impl PreparedGpuPresentCursor {
    pub fn begin(scene_revision: u64, preview_generation: u64) -> Option<Self> {
        if scene_revision == 0 || scene_revision == u64::MAX || preview_generation == 0 || preview_generation == u64::MAX {
            return None;
        }
        let slot = PREPARED_GPU_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok())?;
        Some(Self { scene_revision, preview_generation, command: 0, glass_command: 0, blur_mip: 1, frame: None, view: None, phase: PreparedGpuPresentPhase::EnsureTarget, abandonment_slot: slot as u8 })
    }

    fn matches(&self, packet: &PreparedRenderPacket) -> bool {
        self.scene_revision == packet.scene_revision() && self.preview_generation == packet.preview_generation() && packet.is_within_credits()
    }

    pub fn begin_close(&mut self) {
        self.phase = PreparedGpuPresentPhase::Closing;
    }

    pub fn close_step(&mut self) -> bool {
        if self.view.take().is_some() {
            return false;
        }
        if self.frame.take().is_some() {
            return false;
        }
        self.command = 0;
        self.glass_command = 0;
        self.blur_mip = 0;
        self.scene_revision = 0;
        self.preview_generation = 0;
        self.phase = PreparedGpuPresentPhase::Complete;
        if self.abandonment_slot != u8::MAX {
            let slot = usize::from(self.abandonment_slot);
            let Some(state) = PREPARED_GPU_ABANDONMENT_STATE.get(slot) else { return false };
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
        self.frame.is_none()
            && self.view.is_none()
            && self.command == 0
            && self.glass_command == 0
            && self.blur_mip == 0
            && self.scene_revision == 0
            && self.preview_generation == 0
            && self.phase == PreparedGpuPresentPhase::Complete
            && self.abandonment_slot == u8::MAX
    }

    /// 🧹 Advances one exact GPU cursor owner recovered from an interrupted presentation.
    pub fn close_abandoned_step() -> bool {
        let Some(slot) = PREPARED_GPU_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(2, 3, Ordering::AcqRel, Ordering::Acquire).is_ok()) else { return true };
        let pointer = PREPARED_GPU_ABANDONMENT_OWNER[slot].swap(std::ptr::null_mut(), Ordering::AcqRel);
        if pointer.is_null() {
            PREPARED_GPU_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
            return false;
        }
        let mut cursor = unsafe { Box::from_raw(pointer) };
        if cursor.close_step() || cursor.abandonment_slot == u8::MAX {
            drop(cursor);
        } else {
            PREPARED_GPU_ABANDONMENT_OWNER[slot].store(Box::into_raw(cursor), Ordering::Release);
            PREPARED_GPU_ABANDONMENT_STATE[slot].store(2, Ordering::Release);
        }
        false
    }
}

impl Drop for PreparedGpuPresentCursor {
    fn drop(&mut self) {
        if self.terminal_is_empty() || self.abandonment_slot == u8::MAX {
            return;
        }
        let slot = usize::from(self.abandonment_slot);
        let Some(state) = PREPARED_GPU_ABANDONMENT_STATE.get(slot) else { return };
        if state.load(Ordering::Acquire) != 1 {
            return;
        }
        let cursor = Box::new(Self {
            scene_revision: self.scene_revision,
            preview_generation: self.preview_generation,
            command: self.command,
            glass_command: self.glass_command,
            blur_mip: self.blur_mip,
            frame: self.frame.take(),
            view: self.view.take(),
            phase: PreparedGpuPresentPhase::Closing,
            abandonment_slot: self.abandonment_slot,
        });
        self.scene_revision = 0;
        self.preview_generation = 0;
        self.command = 0;
        self.glass_command = 0;
        self.blur_mip = 0;
        self.phase = PreparedGpuPresentPhase::Complete;
        self.abandonment_slot = u8::MAX;
        PREPARED_GPU_ABANDONMENT_OWNER[slot].store(Box::into_raw(cursor), Ordering::Release);
        state.store(2, Ordering::Release);
    }
}

pub struct GpuContext {
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface: Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    color_target_format: wgpu::TextureFormat,
    pipelines: UiPipelines,
    frame_buffers: FrameBuffers,
    depth_texture: Option<wgpu::Texture>,
    depth_view: Option<wgpu::TextureView>,
    mesh_store: MeshGpuTable,
    raster_store: RasterTextureTable,
    scene_color: Option<SceneColorTarget>,
    atlas_upload: Option<PreparedAtlasUploadCursor>,
    prepared_command_buffer: wgpu::Buffer,
    width: u32,
    height: u32,
    dpr: f32,
}

impl GpuContext {
    #[cfg(not(target_os = "wasi"))]
    pub async fn from_window(window: Arc<winit::window::Window>) -> Result<Self, String> {
        let dpr = window.scale_factor() as f32;
        let size = window.inner_size();
        let css_width = size.width as f32 / dpr;
        let css_height = size.height as f32 / dpr;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY }, ..Default::default() });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Window(Box::new(window))).map_err(|err| format!("surface: {err:?}"))?;
        Self::from_surface(instance, surface, css_width, css_height, dpr).await
    }

    /// 🧵️ Creates the browser GPU surface directly in a dedicated Worker from a transferred canvas.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub async fn from_offscreen_canvas(canvas: web_sys::OffscreenCanvas, css_width: f32, css_height: f32, dpr: f32) -> Result<Self, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: wgpu::Backends::BROWSER_WEBGPU, ..Default::default() });
        let surface = instance.create_surface(wgpu::SurfaceTarget::OffscreenCanvas(canvas)).map_err(|err| format!("offscreen surface: {err:?}"))?;
        Self::from_surface(instance, surface, css_width, css_height, dpr).await
    }

    async fn from_surface(instance: wgpu::Instance, surface: Surface<'static>, css_width: f32, css_height: f32, dpr: f32) -> Result<Self, String> {
        let adapter =
            instance.request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false }).await.map_err(|err| format!("adapter: {err:?}"))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("ui_wgpu"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default().using_resolution(adapter.limits()),
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await
            .map_err(|err| format!("device: {err:?}"))?;
        let caps = surface.get_capabilities(&adapter);
        let surface_format = caps.formats.iter().copied().find(|f| !f.is_srgb()).unwrap_or(caps.formats[0]);
        let color_target_format = if surface_format.is_srgb() { surface_format } else { surface_format.add_srgb_suffix() };
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![color_target_format],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);
        let pipelines = UiPipelines::new(&device, &queue, color_target_format);
        let raster_store = RasterTextureTable::new(&device, pipelines.bind_group_layout());
        let prepared_command_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("prepared_render_command_pages"),
            size: (PREPARED_RENDER_COMMAND_PAGES as u64) * (PREPARED_RENDER_COMMAND_PAGE_ITEMS as u64) * 16,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let mut gpu = Self {
            device,
            queue,
            surface,
            config,
            color_target_format,
            pipelines,
            frame_buffers: FrameBuffers::default(),
            depth_texture: None,
            depth_view: None,
            mesh_store: MeshGpuTable::default(),
            raster_store,
            scene_color: None,
            atlas_upload: None,
            prepared_command_buffer,
            width,
            height,
            dpr,
        };
        gpu.ensure_depth();
        Ok(gpu)
    }

    fn ensure_depth(&mut self) {
        let depth_texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("ui_depth"),
            size: wgpu::Extent3d { width: self.width.max(1), height: self.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.pipelines.depth_format(),
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        self.depth_texture = Some(depth_texture);
        self.depth_view = Some(depth_view);
    }

    pub fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.dpr = dpr;
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        if width == self.width && height == self.height {
            return;
        }
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.scene_color = None;
        self.ensure_depth();
    }

    fn ensure_scene_color(&mut self) {
        SceneColorTarget::ensure(&self.device, &mut self.scene_color, self.width, self.height, self.color_target_format);
    }

    pub fn mesh_store_mut(&mut self) -> &mut MeshGpuTable {
        &mut self.mesh_store
    }

    pub fn ensure_mesh_step(&mut self, key: &str, version: u64, lease: crate::wgpu::kernel_3d_scene::Mesh3dLease) -> Result<bool, &'static str> {
        self.mesh_store.ensure_mesh_step(&self.device, &self.queue, key, version, lease)
    }

    pub fn close_mesh_upload_step(&mut self) -> bool {
        self.mesh_store.close_upload_step()
    }

    pub fn mesh_upload_terminal_is_empty(&self) -> bool {
        self.mesh_store.upload_terminal_is_empty()
    }

    pub fn retire_mesh_exact_step(&mut self, key: &str, version: u64) -> Result<bool, &'static str> {
        self.mesh_store.retire_exact_step(key, version)
    }

    pub fn close_mesh_table_step(&mut self) -> bool {
        self.mesh_store.close_step()
    }

    pub fn mesh_table_terminal_is_empty(&self) -> bool {
        self.mesh_store.terminal_is_empty()
    }

    pub fn begin_prepared(&self, _token: &UiPresentToken, gate: &PreparedRenderGate, packet: &PreparedRenderPacket, live_revision: u64, live_generation: u64) -> Result<(), String> {
        gate.validate(packet, live_revision, live_generation).map_err(|error| error.to_string())
    }

    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub fn begin_prepared_offscreen(&self, _token: &OffscreenPresentToken, gate: &PreparedRenderGate, packet: &PreparedRenderPacket, live_revision: u64, live_generation: u64) -> Result<(), String> {
        gate.validate(packet, live_revision, live_generation).map_err(|error| error.to_string())
    }

    pub fn apply_prepared_eviction_step(&mut self, packet: &PreparedRenderPacket, index: usize, keep_versions: &[u64]) -> Result<bool, String> {
        let Some(eviction) = packet.evictions().get(index) else { return Ok(true) };
        match eviction {
            PreparedRenderEviction::Mesh { key } => self.mesh_store.evict_mesh_except_step(key, keep_versions).map_err(str::to_owned),
        }
    }

    pub fn apply_prepared_upload_step(&mut self, packet: &PreparedRenderPacket, index: usize, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, String> {
        let Some(upload) = packet.uploads().get(index) else { return Ok(true) };
        let complete = match upload {
            #[cfg(test)]
            PreparedRenderUpload::GlyphAtlas { pixels, width, height } => {
                self.atlas_upload = None;
                self.pipelines.upload_glyph_atlas(&self.queue, pixels, *width, *height);
                true
            }
            #[cfg(test)]
            PreparedRenderUpload::IconAtlas { pixels, width, height } => {
                self.atlas_upload = None;
                self.pipelines.upload_icon_atlas(&self.queue, pixels, *width, *height);
                true
            }
            PreparedRenderUpload::GlyphAtlasPages { pixels } => {
                let cursor = match self.atlas_upload {
                    Some(cursor) if cursor.generation == packet.preview_generation() && cursor.upload == index => cursor,
                    _ => PreparedAtlasUploadCursor { generation: packet.preview_generation(), upload: index, page: 0 },
                };
                let Some((bytes, start_row, rows)) = pixels.page(cursor.page) else {
                    self.atlas_upload = None;
                    return Ok(true);
                };
                self.pipelines.upload_glyph_atlas_page(&self.queue, bytes, pixels.width(), start_row, rows);
                let page = cursor.page.checked_add(1).ok_or_else(|| "glyph atlas page cursor exhausted".to_string())?;
                if page == pixels.len() {
                    self.atlas_upload = None;
                    true
                } else {
                    self.atlas_upload = Some(PreparedAtlasUploadCursor { page, ..cursor });
                    false
                }
            }
            PreparedRenderUpload::IconAtlasPages { pixels } => {
                let cursor = match self.atlas_upload {
                    Some(cursor) if cursor.generation == packet.preview_generation() && cursor.upload == index => cursor,
                    _ => PreparedAtlasUploadCursor { generation: packet.preview_generation(), upload: index, page: 0 },
                };
                let Some((bytes, start_row, rows)) = pixels.page(cursor.page) else {
                    self.atlas_upload = None;
                    return Ok(true);
                };
                self.pipelines.upload_icon_atlas_page(&self.queue, bytes, pixels.width(), start_row, rows);
                let page = cursor.page.checked_add(1).ok_or_else(|| "icon atlas page cursor exhausted".to_string())?;
                if page == pixels.len() {
                    self.atlas_upload = None;
                    true
                } else {
                    self.atlas_upload = Some(PreparedAtlasUploadCursor { page, ..cursor });
                    false
                }
            }
            #[cfg(test)]
            PreparedRenderUpload::Raster { key, pixels, width, height } => self.ensure_raster_texture_step(key, RasterUploadPixels::Contiguous(pixels), *width, *height, candidate, expected).map_err(str::to_owned)?,
            PreparedRenderUpload::RasterPages { key, pixels } => {
                if pixels.frame_generation() != packet.preview_generation() {
                    return Err("prepared raster producer generation is stale".into());
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Pages(pixels), pixels.width(), pixels.height(), candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::Mesh { key, version, lease } => self.ensure_mesh_step(key, *version, *lease).map_err(str::to_owned)?,
        };
        Ok(complete)
    }

    pub fn begin_prepared_present(&mut self, packet: &PreparedRenderPacket, witness: RasterTextureWitness) -> Result<PreparedGpuPresentCursor, String> {
        let cursor = PreparedGpuPresentCursor::begin(packet.scene_revision(), packet.preview_generation()).ok_or_else(|| "prepared GPU cursor generation or abandonment admission was exhausted".to_string())?;
        self.raster_store.begin_presenting(witness).map_err(str::to_owned)?;
        Ok(cursor)
    }

    /// 🚦 Advances one fixed command scalar or one bounded platform submission opportunity.
    pub fn prepared_present_step(&mut self, packet: &PreparedRenderPacket, cursor: &mut PreparedGpuPresentCursor) -> Result<bool, String> {
        if !cursor.matches(packet) || cursor.phase == PreparedGpuPresentPhase::Closing {
            return Err("prepared GPU cursor was stale, uncredited, or closing".to_string());
        }
        let started = semio_framework_job::default_now_us().ok_or_else(|| "GPU opportunity requires a real monotonic clock".to_string())?;
        match cursor.phase {
            PreparedGpuPresentPhase::EnsureTarget => {
                self.ensure_scene_color();
                cursor.phase = PreparedGpuPresentPhase::ClearScene;
            }
            PreparedGpuPresentPhase::ClearScene => {
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_scene_packet") });
                self.pipelines.clear_prepared_scene(&mut encoder, scene, self.depth_view.as_ref());
                self.queue.submit(Some(encoder.finish()));
                cursor.phase = PreparedGpuPresentPhase::Commands;
            }
            PreparedGpuPresentPhase::Commands => {
                let Some(command) = packet.command_pages().get(cursor.command) else {
                    cursor.phase = PreparedGpuPresentPhase::AcquireSurface;
                    return Ok(false);
                };
                let source = u32::try_from(command.source()).map_err(|_| "prepared command source exceeded fixed GPU record".to_string())?;
                let digest = command.digest();
                let record = [command.kind().code(), source, digest as u32, (digest >> 32) as u32];
                let offset = u64::try_from(cursor.command).ok().and_then(|value| value.checked_mul(16)).ok_or_else(|| "prepared command buffer offset exhausted".to_string())?;
                self.queue.write_buffer(&self.prepared_command_buffer, offset, bytemuck::cast_slice(&record));
                if let Some(draw_cursor) = command.draw_cursor() {
                    self.encode_prepared_draw_scalar(packet, draw_cursor, command.packet_overlay())?;
                }
                cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::AcquireSurface => {
                cursor.frame = Some(self.surface.get_current_texture().map_err(|error| format!("prepared surface acquisition: {error:?}"))?);
                cursor.phase = PreparedGpuPresentPhase::CreateView;
            }
            PreparedGpuPresentPhase::CreateView => {
                let Some(frame) = cursor.frame.as_ref() else { return Err("prepared surface owner was missing".to_string()) };
                cursor.view = Some(frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.color_target_format), ..Default::default() }));
                cursor.phase = PreparedGpuPresentPhase::BlurScene;
            }
            PreparedGpuPresentPhase::BlurScene => {
                if cursor.blur_mip >= SCENE_MIP_LEVELS {
                    cursor.phase = PreparedGpuPresentPhase::EncodeComposite;
                    return Ok(false);
                }
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                self.pipelines.encode_prepared_blur_mip(&self.device, &self.queue, scene, cursor.blur_mip).map_err(str::to_owned)?;
                cursor.blur_mip = cursor.blur_mip.checked_add(1).ok_or_else(|| "prepared blur cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::EncodeComposite => {
                let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                let Some(view) = cursor.view.as_ref() else { return Err("prepared surface view was missing".to_string()) };
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_composite_packet") });
                self.pipelines.blit_prepared_scene(&self.device, &mut encoder, view, scene);
                self.queue.submit(Some(encoder.finish()));
                cursor.phase = PreparedGpuPresentPhase::GlassCommands;
            }
            PreparedGpuPresentPhase::GlassCommands => {
                let Some(command) = packet.command_pages().get(cursor.glass_command) else {
                    cursor.phase = PreparedGpuPresentPhase::Present;
                    return Ok(false);
                };
                if let Some(DrawMeasureCursor::Glass(region)) = command.draw_cursor() {
                    let draw = if command.packet_overlay() { packet.overlay.as_ref().ok_or_else(|| "prepared glass overlay owner was missing".to_string())? } else { &packet.draw };
                    let region = draw.glass_regions.get(region).ok_or_else(|| "prepared glass region cursor was stale".to_string())?;
                    let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                    let Some(view) = cursor.view.as_ref() else { return Err("prepared surface view was missing".to_string()) };
                    let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_glass_scalar") });
                    self.pipelines.encode_prepared_glass_scalar(&self.device, &self.queue, &mut encoder, view, scene, &mut self.frame_buffers, region).map_err(str::to_owned)?;
                    self.queue.submit(Some(encoder.finish()));
                }
                cursor.glass_command = cursor.glass_command.checked_add(1).ok_or_else(|| "prepared glass command cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::Present => {
                cursor.view = None;
                let Some(frame) = cursor.frame.take() else { return Err("prepared surface owner was missing before present".to_string()) };
                frame.present();
                cursor.phase = PreparedGpuPresentPhase::Complete;
            }
            PreparedGpuPresentPhase::Complete => return Ok(true),
            PreparedGpuPresentPhase::Closing => return Err("prepared GPU cursor was closing".to_string()),
        }
        if !cursor.matches(packet) {
            return Err("prepared GPU cursor became stale after a platform call".to_string());
        }
        if semio_framework_job::default_now_us().and_then(|now| now.checked_sub(started)).is_none_or(|elapsed| elapsed > 2_000) {
            return Err("prepared GPU opportunity exceeded the two millisecond ceiling".to_string());
        }
        Ok(cursor.phase == PreparedGpuPresentPhase::Complete)
    }

    fn encode_prepared_draw_scalar(&mut self, packet: &PreparedRenderPacket, cursor: DrawMeasureCursor, packet_overlay: bool) -> Result<(), String> {
        let draw = if packet_overlay { packet.overlay.as_ref().ok_or_else(|| "prepared overlay owner was missing".to_string())? } else { &packet.draw };
        let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
        let Some(depth) = self.depth_view.as_ref() else { return Err("prepared depth owner was missing".to_string()) };
        let width = self.width as f32;
        let height = self.height as f32;
        match cursor {
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared UI layer cursor was stale".to_string())?;
                let instances = if overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
                let instance = instances.get(item).ok_or_else(|| "prepared UI scalar cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_ui_scalar") });
                self.pipelines.encode_prepared_ui_scalar(&self.device, &self.queue, &mut encoder, scene, depth, &mut self.frame_buffers, &self.raster_store, instance, None, layer.scissor, width, height, packet.time_seconds).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } if item % 3 == 2 => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared vector layer cursor was stale".to_string())?;
                let vertices = if overlay { &layer.overlay_vector_vertices } else { &layer.vector_vertices };
                let start = item.checked_sub(2).ok_or_else(|| "prepared vector triangle cursor underflowed".to_string())?;
                let triangle = vertices.get(start..=item).ok_or_else(|| "prepared vector triangle cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_vector_triangle") });
                self.pipelines.encode_prepared_vector_triangle(&self.device, &self.queue, &mut encoder, scene, depth, &mut self.frame_buffers, triangle, layer.scissor, width, height, packet.time_seconds).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::LayerRaster { layer, raster } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared raster layer cursor was stale".to_string())?;
                let (key, instance) = layer.raster_instances.get(raster).ok_or_else(|| "prepared raster scalar cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_raster_scalar") });
                self.pipelines
                    .encode_prepared_ui_scalar(&self.device, &self.queue, &mut encoder, scene, depth, &mut self.frame_buffers, &self.raster_store, instance, Some(key), layer.scissor, width, height, packet.time_seconds)
                    .map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared world pass cursor was stale".to_string())?;
                let draws = if translucent { &pass_owner.translucent_draws } else { &pass_owner.draws };
                let draw_owner = draws.get(draw_index).ok_or_else(|| "prepared world draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared world instance cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_instance") });
                self.pipelines
                    .encode_prepared_world_instance(
                        &self.device,
                        &self.queue,
                        &mut encoder,
                        scene,
                        depth,
                        &mut self.frame_buffers,
                        &self.mesh_store,
                        pass_owner,
                        &draw_owner.mesh_key,
                        draw_owner.mesh_version,
                        instance_owner,
                        translucent,
                        width,
                        height,
                    )
                    .map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } if vertex % 2 == 1 => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared line pass cursor was stale".to_string())?;
                let line_owner = pass_owner.line_draws.get(draw_index).ok_or_else(|| "prepared line draw cursor was stale".to_string())?;
                let start = vertex.checked_sub(1).ok_or_else(|| "prepared line segment cursor underflowed".to_string())?;
                let segment = line_owner.vertices.get(start..=vertex).ok_or_else(|| "prepared line segment cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_line") });
                self.pipelines.encode_prepared_world_line(&self.device, &self.queue, &mut encoder, scene, depth, &mut self.frame_buffers, pass_owner, segment).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            _ => {}
        }
        Ok(())
    }

    pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
        self.pipelines.upload_glyph_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub fn upload_icon_atlas(&self, atlas: &crate::wgpu::draw::IconAtlas) {
        self.pipelines.upload_icon_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    fn ensure_raster_texture_step(&mut self, key: &str, pixels: RasterUploadPixels<'_>, width: u32, height: u32, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, &'static str> {
        self.raster_store.ensure_raster_step(
            &self.device,
            &self.queue,
            self.pipelines.globals_buffer(),
            &self.pipelines.glyph_view(),
            self.pipelines.glyph_sampler(),
            &self.pipelines.icon_view(),
            self.pipelines.icon_sampler(),
            key,
            pixels,
            width,
            height,
            candidate,
            expected,
        )
    }

    pub fn commit_presented_rasters_step(&mut self, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.commit_presented_step(witness).map_err(str::to_owned)
    }

    pub fn abort_presented_rasters_step(&mut self, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.abort_presented_step(witness).map_err(str::to_owned)
    }

    pub fn close_raster_upload_step(&mut self) -> RasterTextureCleanupStep {
        self.raster_store.close_upload_step()
    }

    pub fn close_raster_table_step(&mut self) -> Result<bool, String> {
        self.raster_store.close_step().map_err(str::to_owned)
    }

    pub fn raster_table_terminal_is_empty(&self) -> bool {
        self.raster_store.terminal_is_empty()
    }

    pub fn device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn queue(&self) -> &wgpu::Queue {
        &self.queue
    }

    pub fn dpr(&self) -> f32 {
        self.dpr
    }

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, String> {
        self.raster_store.reserve_engine_texture(key, width, height, candidate, expected).map_err(str::to_owned)
    }

    pub fn cancel_engine_texture_admission(&mut self, admission: RasterTextureAdmission) -> Result<(), String> {
        self.raster_store.cancel_engine_texture_admission(admission).map_err(str::to_owned)
    }

    pub fn validate_engine_renderer_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_renderer_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_target_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_target_texture_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_target_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_target_view_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_replacement_texture_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_replacement_texture_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn validate_engine_replacement_view_allocation(&self, admission: &RasterTextureAdmission, expected: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.validate_engine_replacement_view_allocation(admission, expected).map_err(str::to_owned)
    }

    pub fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<wgpu::Texture>, view: Option<wgpu::TextureView>) {
        self.raster_store.retain_engine_allocation_fault(admission, texture, view);
    }

    pub fn stage_engine_texture(&mut self, admission: RasterTextureAdmission, texture: wgpu::Texture, view: wgpu::TextureView, expected: RasterTextureWitness) -> Result<(), RasterTextureStageFault> {
        self.raster_store.stage_gpu_bind_group(&self.device, self.pipelines.globals_buffer(), &self.pipelines.glyph_view(), self.pipelines.glyph_sampler(), admission, view, texture, expected)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(test)]
mod prepared_present_tests {
    use super::*;

    static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    fn guard() -> std::sync::MutexGuard<'static, ()> {
        match TEST_LOCK.lock() {
            Ok(guard) => guard,
            Err(poisoned) => poisoned.into_inner(),
        }
    }

    fn drain() {
        while !PreparedGpuPresentCursor::close_abandoned_step() {}
    }

    #[test]
    fn interrupted_present_cursor_hands_back_generation_and_fixed_owners() {
        let _guard = guard();
        drain();
        let cursor = match PreparedGpuPresentCursor::begin(7, 3) {
            Some(cursor) => cursor,
            None => panic!("fixed present cursor admission"),
        };
        drop(cursor);
        assert!(!PreparedGpuPresentCursor::close_abandoned_step());
        assert!(PreparedGpuPresentCursor::close_abandoned_step());
        assert!(PREPARED_GPU_ABANDONMENT_STATE.iter().all(|state| state.load(Ordering::Acquire) == 0));
    }

    #[test]
    fn present_cursor_generation_and_capacity_boundaries_refuse_before_ownership() {
        let _guard = guard();
        drain();
        assert!(PreparedGpuPresentCursor::begin(0, 3).is_none());
        assert!(PreparedGpuPresentCursor::begin(7, u64::MAX).is_none());
        let mut owners: [Option<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = std::array::from_fn(|_| None);
        for owner in &mut owners {
            *owner = PreparedGpuPresentCursor::begin(7, 3);
            assert!(owner.is_some());
        }
        assert!(PreparedGpuPresentCursor::begin(7, 3).is_none());
        for owner in owners.iter_mut().filter_map(Option::as_mut) {
            owner.begin_close();
            while !owner.close_step() {}
            assert!(owner.terminal_is_empty());
        }
    }
}

#[cfg(all(target_arch = "wasm32", not(target_os = "wasi")))]
pub fn schedule_frame(window: &winit::window::Window, callback: impl FnMut() + 'static) {
    use wasm_bindgen::closure::Closure;
    use wasm_bindgen::JsCast;

    let mut callback = callback;
    let closure = Closure::wrap(Box::new(move || {
        callback();
    }) as Box<dyn FnMut()>);
    web_sys::window().and_then(|w| w.request_animation_frame(closure.as_ref().unchecked_ref()).ok());
    closure.forget();
    let _ = window;
}

#[cfg(all(not(target_arch = "wasm32"), not(target_os = "wasi")))]
pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {
    window.request_redraw();
}
// #endregion gpu
