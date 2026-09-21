// #region gpu
//! 🖥️ WebGPU device, surface, and frame loop.

use crate::wgpu::draw::{
    FrameBuffers, MeshGpuTable, PreparedCompositeTarget, RasterTextureAdmission, RasterTextureCleanupStep, RasterTextureStageFault, RasterTextureTable, RasterTextureWitness, RasterUploadPixels, SceneColorTarget, UiPipelines, SCENE_MIP_LEVELS,
};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
use crate::wgpu::prepared::OffscreenPresentToken;
use crate::wgpu::prepared::{DrawMeasureCursor, PreparedRenderEviction, PreparedRenderGate, PreparedRenderPacket, PreparedRenderUpload, RasterContentIdentity, UiPresentToken, PREPARED_RENDER_COMMAND_PAGES, PREPARED_RENDER_COMMAND_PAGE_ITEMS};
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
    BlurScene,
    EncodeComposite,
    GlassCommands,
    ForegroundCommands,
    Present,
    Complete,
    Closing,
}

/// ⏱️ One prepared-present opportunity's wall ceiling. A breach names the PHASE it measured and the
/// microseconds it took: a verdict that cannot say what it measured is undiagnosable from the fault
/// banner alone (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
const PREPARED_GPU_OPPORTUNITY_CEILING_US: u64 = 2_000;

/// 🫧 Resolves one `DrawMeasureCursor::Glass` command page against the draw list it was measured
/// on. `advance_pipeline` emits one command page per MEASURED step, and the step that retires the
/// glass section is measured too: `Glass(index)` with `index == glass_regions.len()` sets the cursor
/// `Complete` and reports zero usage (`📦️prepared.rs`'s own boundary rule). That terminal page
/// therefore addresses no region by construction — a document with no glass at all publishes exactly
/// one `Glass(0)` page against an empty list — so `Ok(None)` is "nothing to encode", and only an
/// index PAST the terminal one is a genuinely stale cursor (`Err` carrying the length it measured).
/// Treating the terminal page as stale failed every first present of the wgpu browser shell
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
fn address_prepared_glass_region(region: usize, len: usize) -> Result<Option<usize>, usize> {
    if region < len {
        return Ok(Some(region));
    }
    if region == len {
        Ok(None)
    } else {
        Err(len)
    }
}

/// ⚖️ Admits ONE measured opportunity against [`PREPARED_GPU_OPPORTUNITY_CEILING_US`]. `Ok` carries
/// the cursor's new consecutive-overrun run (`0` once an opportunity fits); `Err` carries the run
/// that reached [`semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS`] and is therefore
/// terminal. Pure so the law is testable without a GPU adapter.
fn admit_prepared_gpu_opportunity(run: u32, elapsed_us: u64) -> Result<u32, u32> {
    if elapsed_us <= PREPARED_GPU_OPPORTUNITY_CEILING_US {
        return Ok(0);
    }
    let run = run.saturating_add(1);
    if run >= semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS {
        Err(run)
    } else {
        Ok(run)
    }
}

const PREPARED_GPU_ABANDONMENT_SLOTS: usize = 64;
static PREPARED_GPU_ABANDONMENT_STATE: [AtomicU8; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicU8::new(0) }; PREPARED_GPU_ABANDONMENT_SLOTS];
static PREPARED_GPU_ABANDONMENT_OWNER: [AtomicPtr<PreparedGpuPresentCursor>; PREPARED_GPU_ABANDONMENT_SLOTS] = [const { AtomicPtr::new(std::ptr::null_mut()) }; PREPARED_GPU_ABANDONMENT_SLOTS];

/// 🎯 Which surface-sized colour target one prepared draw scalar is encoded into. Everything a glass
/// region may blur is `Scene`; the content a glass region carries on its face is `Composite`, so it
/// survives the glass pass that would otherwise sample it away.
#[derive(Clone, Copy, PartialEq, Eq)]
enum PreparedDrawTarget {
    Scene,
    Composite,
}

/// 🫧 Whether one measured draw scalar belongs to a layer opened by `begin_glass_content` — the
/// content that sits ON a glass region rather than under it.
///
/// 🩸️ The prepared ladder used to encode every layer into the scene and then composite the glass
/// regions over it, which sampled the blurred backdrop straight over the window cap's own chips: a
/// `Puzzle 3D` title and its Focus/Close controls were painted, then blurred away by the very region
/// they label (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY). The batch renderer has always split the
/// two with `LayerBatchFilter`; the scalar ladder now honours the same split.
fn prepared_draw_scalar_glass_region(draw: &crate::wgpu::draw::DrawList, cursor: DrawMeasureCursor) -> Option<usize> {
    let layer = match cursor {
        DrawMeasureCursor::LayerUi { layer, .. } | DrawMeasureCursor::LayerVector { layer, .. } | DrawMeasureCursor::LayerRaster { layer, .. } => layer,
        DrawMeasureCursor::PassInstance { pass, .. }
        | DrawMeasureCursor::PassMaterialInstance { pass, .. }
        | DrawMeasureCursor::PassLineVertex { pass, .. }
        | DrawMeasureCursor::PassTexturedInstance { pass, .. }
        | DrawMeasureCursor::PassGrid { pass } => draw.scene_passes.get(pass)?.layer_index,
        _ => return None,
    };
    draw.layers.get(layer)?.foreground_of
}

fn prepared_draw_scalar_is_glass_foreground(draw: &crate::wgpu::draw::DrawList, cursor: DrawMeasureCursor) -> bool {
    let overlay = matches!(cursor, DrawMeasureCursor::LayerUi { overlay: true, .. } | DrawMeasureCursor::LayerVector { overlay: true, .. } | DrawMeasureCursor::LayerRaster { overlay: true, .. });
    overlay || prepared_draw_scalar_glass_region(draw, cursor).is_some()
}

fn prepared_draw_scalar_uses_world_encoded_attachment(cursor: DrawMeasureCursor) -> bool {
    matches!(cursor, DrawMeasureCursor::PassInstance { .. } | DrawMeasureCursor::PassMaterialInstance { .. } | DrawMeasureCursor::PassTexturedInstance { .. } | DrawMeasureCursor::PassGrid { .. } | DrawMeasureCursor::PassLineVertex { .. })
}

/// 🫧 Whether `outer` fully covers `inner` — the containment CSS stacking gives a later
/// `backdrop-filter` element over an earlier one it encloses.
fn prepared_glass_region_covers(outer: [f32; 4], inner: [f32; 4]) -> bool {
    outer[2] > 0.0 && outer[3] > 0.0 && inner[0] >= outer[0] && inner[1] >= outer[1] && inner[0] + inner[2] <= outer[0] + outer[2] && inner[1] + inner[3] <= outer[1] + outer[3]
}

/// 🫧 Whether a glass-content scalar sits under a LATER glass region that encloses its own — the
/// veil over a window cap, a dialog over a floating panel.
///
/// 🩸️ `ForegroundCommands` re-encodes EVERY glass-content layer after the glass pass, with no notion
/// of which region a later region should cover, so a window cap's chips stayed CRISP over the
/// introduction veil while React blurs the whole shell except the card
/// (`📓️w8a-tour-crispness-and-symbol-glyphs.md` §5, hand-off 1). An enclosed layer is therefore
/// encoded into the SCENE instead: it is mipped by the blur chain, its own region re-frosts it, and
/// the covering region then frosts it again — which is what "the caps are under the veil" means on a
/// renderer whose blur chain reads one scene texture. Containment (not mere overlap) is the
/// predicate, so a context menu that clips the corner of a panel never pushes that panel's whole
/// content into the backdrop, and a step that spotlights an element — whose veil is BANDS around the
/// cutout, enclosing nothing that straddles them — leaves it crisp exactly as React's
/// `useIntroductionElevation` does. `overlay_after` is the OVERLAY draw list when `draw` is the main
/// one — the overlay is encoded after it, so every overlay region is later than every region of the
/// main list, which is the veil-over-cap case itself.
fn prepared_foreground_scalar_is_enclosed(draw: &crate::wgpu::draw::DrawList, overlay_after: Option<&crate::wgpu::draw::DrawList>, cursor: DrawMeasureCursor) -> bool {
    let Some(region) = prepared_draw_scalar_glass_region(draw, cursor) else { return false };
    let Some(own) = draw.glass_regions.get(region).map(|glass| glass.rect) else { return false };
    draw.glass_regions.iter().skip(region.saturating_add(1)).chain(overlay_after.into_iter().flat_map(|overlay| overlay.glass_regions.iter())).any(|glass| prepared_glass_region_covers(glass.rect, own))
}

/// 🎟️ Generation-qualified retained surface and command submission cursor.
pub struct PreparedGpuPresentCursor {
    scene_revision: u64,
    preview_generation: u64,
    command: usize,
    glass_command: usize,
    foreground_command: usize,
    blur_mip: u32,
    frame: Option<wgpu::SurfaceTexture>,
    view: Option<wgpu::TextureView>,
    phase: PreparedGpuPresentPhase,
    abandonment_slot: u8,
    overrun_run: u32,
}

impl PreparedGpuPresentCursor {
    pub fn begin(scene_revision: u64, preview_generation: u64) -> Option<Self> {
        if scene_revision == 0 || scene_revision == u64::MAX || preview_generation == 0 || preview_generation == u64::MAX {
            return None;
        }
        let slot = PREPARED_GPU_ABANDONMENT_STATE.iter().position(|state| state.compare_exchange(0, 1, Ordering::AcqRel, Ordering::Acquire).is_ok())?;
        Some(Self { scene_revision, preview_generation, command: 0, glass_command: 0, foreground_command: 0, blur_mip: 1, frame: None, view: None, phase: PreparedGpuPresentPhase::EnsureTarget, abandonment_slot: slot as u8, overrun_run: 0 })
    }

    fn matches(&self, packet: &PreparedRenderPacket) -> bool {
        self.scene_revision == packet.scene_revision() && self.preview_generation == packet.preview_generation() && packet.is_within_credits()
    }

    pub fn begin_close(&mut self) {
        self.phase = PreparedGpuPresentPhase::Closing;
    }

    /// 🐕️ Every index a healthy [`GpuContext::prepared_present_step`] moves — the ladder phase, the
    /// draw command, the glass command, the glass-foreground command and the blur mip. EVERY cursor
    /// the ladder walks belongs here: `ForegroundCommands` was added without its index and the
    /// watchdog quarantined the surface `13 770` pages into a perfectly healthy walk.
    ///
    /// ⚖️ A host watchdog over the OUTER presentation cursor cannot see any of them: from outside,
    /// `AppPresentPhase::Render` holds one `gpu_cursor` for the whole submit and looks frozen for as
    /// many steps as the scene has commands. A ceiling read against the outer shape alone therefore
    /// aborts a perfectly healthy present — measured on 6118, where a boot's own composite pass held
    /// `phase=Render engine=1 upload=1 gpu-cursor=true` past 4 096 outer steps on every example
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-regressions-sweep-2026-09-15.md`). */
    pub fn progress(&self) -> (u8, usize, usize, usize, u32) {
        (self.phase as u8, self.command, self.glass_command, self.foreground_command, self.blur_mip)
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
        self.foreground_command = 0;
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
            && self.foreground_command == 0
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
            foreground_command: self.foreground_command,
            blur_mip: self.blur_mip,
            frame: self.frame.take(),
            view: self.view.take(),
            phase: PreparedGpuPresentPhase::Closing,
            abandonment_slot: self.abandonment_slot,
            overrun_run: self.overrun_run,
        });
        self.scene_revision = 0;
        self.preview_generation = 0;
        self.command = 0;
        self.glass_command = 0;
        self.foreground_command = 0;
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
    composite_color: Option<PreparedCompositeTarget>,
    atlas_upload: Option<PreparedAtlasUploadCursor>,
    /// 🧷️ The last prepared world draw whose mesh was not resident at submit, kept for ONE readable
    /// host report instead of the fatal `present_step` fault that used to quarantine the surface.
    missing_world_mesh: Option<(String, u64)>,
    prepared_command_buffer: wgpu::Buffer,
    /// 📐️ PHYSICAL (device) surface extent — the only place physical pixels are legal. Everything
    /// a draw list, a hit test or a chrome constant speaks is logical; see `logical_width`.
    width: u32,
    height: u32,
    /// 📐️ LOGICAL (CSS) surface extent — what the projection divides by, so a draw list authored in
    /// logical pixels lands on the full physical surface at any scale factor.
    logical_width: f32,
    logical_height: f32,
    dpr: f32,
}

impl GpuContext {
    #[cfg(not(target_os = "wasi"))]
    pub async fn from_window(window: Arc<winit::window::Window>) -> Result<Self, String> {
        let dpr = window.scale_factor() as f32;
        let size = window.inner_size();
        let css_width = size.width as f32 / dpr;
        let css_height = size.height as f32 / dpr;
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY },
            ..wgpu::InstanceDescriptor::new_with_display_handle(Box::new(window.clone()))
        });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Window(Box::new(window))).map_err(|err| format!("surface: {err:?}"))?;
        Self::from_surface(instance, surface, css_width, css_height, dpr).await
    }

    /// 🧵️ Creates the browser GPU surface directly in a dedicated Worker from a transferred canvas.
    #[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
    pub async fn from_offscreen_canvas(canvas: web_sys::OffscreenCanvas, css_width: f32, css_height: f32, dpr: f32) -> Result<Self, String> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::BROWSER_WEBGPU, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let surface = instance.create_surface(wgpu::SurfaceTarget::OffscreenCanvas(canvas)).map_err(|err| format!("offscreen surface: {err:?}"))?;
        Self::from_surface(instance, surface, css_width, css_height, dpr).await
    }

    /// 🖥️ `css_width`/`css_height` are LOGICAL pixels and `dpr` is the surface scale factor; the
    /// physical configuration is derived here and nowhere else.
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
        let mut pipelines = UiPipelines::new(&device, &queue, color_target_format);
        pipelines.set_surface_scale(dpr);
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
            composite_color: None,
            atlas_upload: None,
            missing_world_mesh: None,
            prepared_command_buffer,
            width,
            height,
            logical_width: css_width.max(1.0),
            logical_height: css_height.max(1.0),
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

    /// 📐️ `css_width`/`css_height` are LOGICAL pixels; `dpr` is the surface scale factor. A scale
    /// change alone (the window dragged onto a display with a different density) still has to land,
    /// because the projection divisor and the scissor scale both move with it even when the
    /// physical extent happens to be identical.
    pub fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        let scale_changed = (dpr - self.dpr).abs() >= f32::EPSILON;
        self.dpr = dpr;
        self.logical_width = css_width.max(1.0);
        self.logical_height = css_height.max(1.0);
        self.pipelines.set_surface_scale(dpr);
        let width = (css_width * dpr).max(1.0) as u32;
        let height = (css_height * dpr).max(1.0) as u32;
        if width == self.width && height == self.height && !scale_changed {
            return;
        }
        self.width = width;
        self.height = height;
        self.config.width = width;
        self.config.height = height;
        self.surface.configure(&self.device, &self.config);
        self.scene_color = None;
        self.composite_color = None;
        self.ensure_depth();
    }

    /// 🎯️ Both surface-sized colour targets a prepared frame needs: the scene it draws into and the
    /// composite the swapchain is finally handed.
    fn ensure_prepared_targets(&mut self) {
        SceneColorTarget::ensure(&self.device, &mut self.scene_color, self.width, self.height, self.color_target_format);
        PreparedCompositeTarget::ensure(&self.device, &mut self.composite_color, self.width, self.height, self.color_target_format);
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

    /// 🧷️ The last prepared world draw that named a non-resident mesh, TAKEN so the host reports it
    /// exactly once per occurrence instead of once per instance scalar.
    pub fn take_missing_world_mesh(&mut self) -> Option<(String, u64)> {
        self.missing_world_mesh.take()
    }

    /// 🐕️ Every index a healthy `Uploads` step moves WITHIN one upload item: the mesh cursor's vertex
    /// and index counters and the atlas cursor's page.
    ///
    /// 🩸️ `AppPresentCursor::upload` only moves when a whole item COMPLETES, and one mesh item is
    /// uploaded one vertex per step — so a perfectly healthy 293-vertex mesh looked to the presenter
    /// watchdog exactly like a frozen cursor, and the host's own gate census reported
    /// `phase=Some(Uploads) … stall-steps=293` with no reason attached (ticket
    /// 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w11a-prepared-world-mesh-missing.md`). With the inner
    /// counters in the signature a progressing upload resets the watchdog and only a genuinely stuck
    /// one accumulates toward its bound.
    pub fn prepared_upload_progress(&self) -> (u32, u32, usize) {
        let (vertex, index) = self.mesh_store.upload_progress();
        (vertex, index, self.atlas_upload.map_or(0, |cursor| cursor.page))
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
            PreparedRenderUpload::Raster { key, pixels, width, height } => {
                let identity = RasterContentIdentity::test_pixels(*width, *height, pixels).ok_or_else(|| "raster test identity overflowed".to_string())?;
                if !self.raster_store.prepare_admission_step(key, *width, *height, identity, candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Contiguous(pixels), *width, *height, candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::RasterPages { key, pixels } => {
                if pixels.frame_generation() != packet.preview_generation() {
                    return Err("prepared raster producer generation is stale".into());
                }
                if !self.raster_store.prepare_admission_step(key, pixels.width(), pixels.height(), pixels.content_identity(), candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Pages(pixels), pixels.width(), pixels.height(), candidate, expected).map_err(str::to_owned)?
            }
            PreparedRenderUpload::SceneRaster { key, lease } => {
                let descriptor = lease.identity().descriptor();
                if !self.raster_store.prepare_admission_step(key, descriptor.width, descriptor.height, lease.identity().content(), candidate).map_err(str::to_owned)? {
                    return Ok(false);
                }
                self.ensure_raster_texture_step(key, RasterUploadPixels::Scene(lease), descriptor.width, descriptor.height, candidate, expected).map_err(str::to_owned)?
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
    ///
    /// 🖼️ **The swapchain is touched by exactly one phase.** `Present` acquires the surface texture,
    /// views it, blits the finished [`PreparedCompositeTarget`] onto it, submits and presents inside
    /// ONE call, because a browser expires the canvas's current texture at the end of the task that
    /// obtained it and presents whatever it held then. Every earlier phase composites offscreen, so
    /// a host pump may yield between any two of them without ever publishing an empty frame.
    ///
    /// ⚖️ A single over-ceiling wall sample is a MEASUREMENT, not a verdict — the ratified law of
    /// [`semio_framework_job::SUSTAINED_OVERRUN_QUARANTINE_STEPS`]. The first frame of a cold surface
    /// pays the platform's pipeline warm-up inside one opportunity (measured on Apple metal-3 through
    /// a browser `OffscreenCanvas`: `ClearScene` 2 601 µs against the 2 000 µs ceiling), and failing
    /// that one sample quarantined the whole surface before it had ever presented
    /// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). Only a RUN of consecutive over-ceiling
    /// opportunities — a cursor that is genuinely not converging — is terminal; any admitted
    /// opportunity clears the run.
    pub fn prepared_present_step(&mut self, packet: &PreparedRenderPacket, cursor: &mut PreparedGpuPresentCursor) -> Result<bool, String> {
        if !cursor.matches(packet) || cursor.phase == PreparedGpuPresentPhase::Closing {
            return Err("prepared GPU cursor was stale, uncredited, or closing".to_string());
        }
        let started = semio_framework_job::default_now_us().ok_or_else(|| "GPU opportunity requires a real monotonic clock".to_string())?;
        let opportunity = cursor.phase;
        match cursor.phase {
            PreparedGpuPresentPhase::EnsureTarget => {
                self.ensure_prepared_targets();
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
                    cursor.phase = PreparedGpuPresentPhase::BlurScene;
                    return Ok(false);
                };
                let source = u32::try_from(command.source()).map_err(|_| "prepared command source exceeded fixed GPU record".to_string())?;
                let digest = command.digest();
                let record = [command.kind().code(), source, digest as u32, (digest >> 32) as u32];
                let offset = u64::try_from(cursor.command).ok().and_then(|value| value.checked_mul(16)).ok_or_else(|| "prepared command buffer offset exhausted".to_string())?;
                self.queue.write_buffer(&self.prepared_command_buffer, offset, bytemuck::cast_slice(&record));
                if let Some(draw_cursor) = command.draw_cursor() {
                    let overlay_owner = command.packet_overlay();
                    let owner = if overlay_owner { packet.overlay.as_ref() } else { Some(&packet.draw) };
                    let overlay_after = if overlay_owner { None } else { packet.overlay.as_ref() };
                    if !owner.is_some_and(|draw| prepared_draw_scalar_is_glass_foreground(draw, draw_cursor) && !prepared_foreground_scalar_is_enclosed(draw, overlay_after, draw_cursor)) {
                        self.encode_prepared_draw_scalar(packet, draw_cursor, overlay_owner, PreparedDrawTarget::Scene)?;
                    }
                }
                cursor.command = cursor.command.checked_add(1).ok_or_else(|| "prepared command cursor exhausted".to_string())?;
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
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_composite_packet") });
                self.pipelines.blit_prepared_scene(&self.device, &mut encoder, composite.view(), scene);
                self.queue.submit(Some(encoder.finish()));
                cursor.phase = PreparedGpuPresentPhase::GlassCommands;
            }
            PreparedGpuPresentPhase::GlassCommands => {
                let Some(command) = packet.command_pages().get(cursor.glass_command) else {
                    cursor.phase = PreparedGpuPresentPhase::ForegroundCommands;
                    return Ok(false);
                };
                if let Some(DrawMeasureCursor::Glass(region)) = command.draw_cursor() {
                    let overlay_owner = command.packet_overlay();
                    let draw = if overlay_owner { packet.overlay.as_ref().ok_or_else(|| "prepared glass overlay owner was missing".to_string())? } else { &packet.draw };
                    let addressed =
                        address_prepared_glass_region(region, draw.glass_regions.len()).map_err(|len| format!("prepared glass region cursor was stale: region {region} of {len} on the {} owner", if overlay_owner { "overlay" } else { "draw" }))?;
                    if let Some(glass) = addressed.and_then(|index| draw.glass_regions.get(index)) {
                        let Some(scene) = self.scene_color.as_ref() else { return Err("prepared scene target was missing".to_string()) };
                        let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_glass_scalar") });
                        self.pipelines.encode_prepared_glass_scalar(&self.device, &self.queue, &mut encoder, composite.view(), scene, &mut self.frame_buffers, glass).map_err(str::to_owned)?;
                        self.queue.submit(Some(encoder.finish()));
                    }
                }
                cursor.glass_command = cursor.glass_command.checked_add(1).ok_or_else(|| "prepared glass command cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::ForegroundCommands => {
                let Some(command) = packet.command_pages().get(cursor.foreground_command) else {
                    cursor.phase = PreparedGpuPresentPhase::Present;
                    return Ok(false);
                };
                if let Some(draw_cursor) = command.draw_cursor() {
                    let overlay_owner = command.packet_overlay();
                    let owner = if overlay_owner { packet.overlay.as_ref() } else { Some(&packet.draw) };
                    let overlay_after = if overlay_owner { None } else { packet.overlay.as_ref() };
                    if owner.is_some_and(|draw| prepared_draw_scalar_is_glass_foreground(draw, draw_cursor) && !prepared_foreground_scalar_is_enclosed(draw, overlay_after, draw_cursor)) {
                        self.encode_prepared_draw_scalar(packet, draw_cursor, overlay_owner, PreparedDrawTarget::Composite)?;
                    }
                }
                cursor.foreground_command = cursor.foreground_command.checked_add(1).ok_or_else(|| "prepared foreground command cursor exhausted".to_string())?;
            }
            PreparedGpuPresentPhase::Present => {
                let Some(composite) = self.composite_color.as_ref() else { return Err("prepared composite target was missing".to_string()) };
                cursor.frame = Some(match self.surface.get_current_texture() {
                    wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                    outcome => return Err(format!("prepared surface acquisition: {outcome:?}")),
                });
                let Some(frame) = cursor.frame.as_ref() else { return Err("prepared surface owner was missing".to_string()) };
                cursor.view = Some(frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.color_target_format), ..Default::default() }));
                let Some(view) = cursor.view.as_ref() else { return Err("prepared surface view was missing".to_string()) };
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_surface_present") });
                self.pipelines.blit_prepared_composite(&self.device, &mut encoder, view, composite);
                self.queue.submit(Some(encoder.finish()));
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
        let Some(elapsed) = semio_framework_job::default_now_us().and_then(|now| now.checked_sub(started)) else {
            return Err("prepared GPU opportunity lost its monotonic clock".to_string());
        };
        match admit_prepared_gpu_opportunity(cursor.overrun_run, elapsed) {
            Ok(run) => cursor.overrun_run = run,
            Err(run) => return Err(format!("prepared GPU opportunity exceeded the two millisecond ceiling for {run} consecutive opportunities: {opportunity:?} took {elapsed} us")),
        }
        Ok(cursor.phase == PreparedGpuPresentPhase::Complete)
    }

    fn encode_prepared_draw_scalar(&mut self, packet: &PreparedRenderPacket, cursor: DrawMeasureCursor, packet_overlay: bool, target: PreparedDrawTarget) -> Result<(), String> {
        let draw = if packet_overlay { packet.overlay.as_ref().ok_or_else(|| "prepared overlay owner was missing".to_string())? } else { &packet.draw };
        let world_encoded = prepared_draw_scalar_uses_world_encoded_attachment(cursor);
        let color_view = match target {
            PreparedDrawTarget::Scene => self
                .scene_color
                .as_ref()
                .map(|scene| if world_encoded { scene.world_encoded_view() } else { scene.mip_view(0) })
                .ok_or_else(|| "prepared scene target was missing".to_string())?,
            PreparedDrawTarget::Composite => self
                .composite_color
                .as_ref()
                .map(|composite| if world_encoded { composite.world_encoded_view() } else { composite.view() })
                .ok_or_else(|| "prepared composite target was missing".to_string())?,
        };
        let Some(depth) = self.depth_view.as_ref() else { return Err("prepared depth owner was missing".to_string()) };
        let width = self.logical_width;
        let height = self.logical_height;
        match cursor {
            DrawMeasureCursor::LayerUi { layer, item, overlay } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared UI layer cursor was stale".to_string())?;
                let instances = if overlay { &layer.overlay_ui_instances } else { &layer.ui_instances };
                let instance = instances.get(item).ok_or_else(|| "prepared UI scalar cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_ui_scalar") });
                self.pipelines
                    .encode_prepared_ui_scalar(&self.device, &self.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, instance, None, layer.scissor, width, height, packet.time_seconds)
                    .map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::LayerVector { layer, item, overlay } if item % 3 == 2 => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared vector layer cursor was stale".to_string())?;
                let vertices = if overlay { &layer.overlay_vector_vertices } else { &layer.vector_vertices };
                let start = item.checked_sub(2).ok_or_else(|| "prepared vector triangle cursor underflowed".to_string())?;
                let triangle = vertices.get(start..=item).ok_or_else(|| "prepared vector triangle cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_vector_triangle") });
                self.pipelines.encode_prepared_vector_triangle(&self.device, &self.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, triangle, layer.scissor, width, height, packet.time_seconds).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::LayerRaster { layer, raster, overlay } => {
                let layer = draw.layers.get(layer).ok_or_else(|| "prepared raster layer cursor was stale".to_string())?;
                let instances = if overlay { &layer.overlay_raster_instances } else { &layer.raster_instances };
                let (key, instance) = instances.get(raster).ok_or_else(|| "prepared raster scalar cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_raster_scalar") });
                self.pipelines
                    .encode_prepared_ui_scalar(&self.device, &self.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, instance, Some(key), layer.scissor, width, height, packet.time_seconds)
                    .map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassShadowBegin(pass) => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared shadow pass cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_shadow_clear") });
                self.pipelines.encode_prepared_world_shadow_begin(&self.device, &mut encoder, pass_owner);
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassShadowInstance { pass, draw: draw_index, instance } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared shadow pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.shadow_draws.get(draw_index).ok_or_else(|| "prepared shadow draw cursor was stale".to_string())?;
                if !draw_owner.shadow_role.casts {
                    return Err("prepared shadow draw did not own the caster role".to_string());
                }
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared shadow instance cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_shadow_instance") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_shadow_instance(&self.device, &self.queue, &mut encoder, &mut self.frame_buffers, &self.mesh_store, pass_owner, &draw_owner.mesh_key, draw_owner.mesh_version, instance_owner)
                    .map_err(str::to_owned)?;
                if drawn {
                    self.queue.submit(Some(encoder.finish()));
                }
            }
            DrawMeasureCursor::PassInstance { pass, draw: draw_index, instance, translucent } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared world pass cursor was stale".to_string())?;
                let draws = if translucent { &pass_owner.translucent_draws } else { &pass_owner.draws };
                let draw_owner = draws.get(draw_index).ok_or_else(|| "prepared world draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared world instance cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_instance") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_instance(
                        &self.device,
                        &self.queue,
                        &mut encoder,
                        color_view,
                        depth,
                        &mut self.frame_buffers,
                        &self.mesh_store,
                        pass_owner,
                        &draw_owner.mesh_key,
                        draw_owner.mesh_version,
                        instance_owner,
                        draw_owner.shadow_role.receives,
                        translucent,
                        width,
                        height,
                    )
                    .map_err(str::to_owned)?;
                if !drawn {
                    self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));
                    return Ok(());
                }
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassMaterialInstance { pass, draw: draw_index, instance, .. } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared material pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.material_draws.get(draw_index).ok_or_else(|| "prepared material draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared material instance cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_material") });
                let drawn = self
                    .pipelines
                    .encode_prepared_world_material(
                        &self.device,
                        &self.queue,
                        &mut encoder,
                        color_view,
                        depth,
                        &mut self.frame_buffers,
                        &self.mesh_store,
                        &self.raster_store,
                        pass_owner,
                        draw_owner,
                        instance_owner,
                        width,
                        height,
                    )
                    .map_err(str::to_owned)?;
                if !drawn {
                    self.missing_world_mesh = Some((draw_owner.mesh_key.clone(), draw_owner.mesh_version));
                    return Ok(());
                }
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassTexturedInstance { pass, draw: draw_index, instance } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared textured pass cursor was stale".to_string())?;
                let draw_owner = pass_owner.textured_draws.get(draw_index).ok_or_else(|| "prepared textured draw cursor was stale".to_string())?;
                let instance_owner = draw_owner.instances.get(instance).ok_or_else(|| "prepared textured instance cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_textured") });
                self.pipelines.encode_prepared_world_textured(&self.device, &self.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, &self.raster_store, pass_owner, instance_owner).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassGrid { pass } => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared grid pass cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_grid") });
                self.pipelines.encode_prepared_world_grid(&self.device, &self.queue, &mut encoder, color_view, depth, pass_owner).map_err(str::to_owned)?;
                self.queue.submit(Some(encoder.finish()));
            }
            DrawMeasureCursor::PassLineVertex { pass, draw: draw_index, vertex } if vertex % 2 == 1 => {
                let pass_owner = draw.scene_passes.get(pass).ok_or_else(|| "prepared line pass cursor was stale".to_string())?;
                let line_owner = pass_owner.line_draws.get(draw_index).ok_or_else(|| "prepared line draw cursor was stale".to_string())?;
                let start = vertex.checked_sub(1).ok_or_else(|| "prepared line segment cursor underflowed".to_string())?;
                let segment = line_owner.vertices.get(start..=vertex).ok_or_else(|| "prepared line segment cursor was stale".to_string())?;
                let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("prepared_world_line") });
                self.pipelines.encode_prepared_world_line(&self.device, &self.queue, &mut encoder, color_view, depth, &mut self.frame_buffers, pass_owner, segment).map_err(str::to_owned)?;
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
        self.raster_store.commit_presented_step(witness).map_err(|fault| {
            let (candidate, presenting, staged) = self.raster_store.presentation_witnesses();
            format!("{fault} (commit {witness:?}, candidate {candidate:?}, presenting {presenting:?}, staged {staged})")
        })
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

    pub fn reserve_engine_texture(&mut self, key: &str, width: u32, height: u32, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<RasterTextureAdmission, String> {
        self.raster_store.reserve_engine_texture(key, width, height, identity, candidate, expected).map_err(str::to_owned)
    }

    pub fn begin_raster_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.begin_candidate_ownership(witness).map_err(str::to_owned)
    }

    pub fn publish_raster_ownership(&mut self, witness: RasterTextureWitness, key: &str) -> Result<(), String> {
        self.raster_store.publish_candidate_ownership(witness, key).map_err(str::to_owned)
    }

    pub fn seal_raster_ownership(&mut self, witness: RasterTextureWitness) -> Result<(), String> {
        self.raster_store.seal_candidate_ownership(witness).map_err(str::to_owned)
    }

    pub fn prepare_raster_admission_step(&mut self, key: &str, width: u32, height: u32, identity: RasterContentIdentity, witness: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.prepare_admission_step(key, width, height, identity, witness).map_err(str::to_owned)
    }

    pub fn raster_content_is_reusable(&self, key: &str, identity: RasterContentIdentity, candidate: RasterTextureWitness, expected: RasterTextureWitness) -> Result<bool, String> {
        self.raster_store.content_is_reusable(key, identity, candidate, expected).map_err(str::to_owned)
    }

    pub fn release_previous_raster_ownership(&mut self) {
        self.raster_store.release_previous_ownership();
    }

    pub fn retire_unowned_raster_step(&mut self) -> Result<bool, String> {
        self.raster_store.retire_unowned_step().map_err(str::to_owned)
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

    pub fn retain_engine_allocation_fault(&mut self, admission: RasterTextureAdmission, texture: Option<wgpu::Texture>, view: Option<wgpu::TextureView>) {
        self.raster_store.retain_engine_allocation_fault(admission, texture, view);
    }

    #[expect(clippy::result_large_err, reason = "GPU admission returns the exact resource and reservation owner without allocating during refusal.")]
    pub fn stage_engine_texture(&mut self, admission: RasterTextureAdmission, texture: wgpu::Texture, view: wgpu::TextureView, expected: RasterTextureWitness) -> Result<(), RasterTextureStageFault> {
        self.raster_store.stage_gpu_bind_group(&self.device, self.pipelines.globals_buffer(), &self.pipelines.glyph_view(), self.pipelines.glyph_sampler(), admission, view, texture, expected)
    }

    /// 📐️ PHYSICAL surface width in device pixels.
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 📐️ PHYSICAL surface height in device pixels.
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 📐️ LOGICAL surface width — the unit every draw list, hit test and chrome constant speaks.
    pub fn logical_width(&self) -> f32 {
        self.logical_width
    }

    /// 📐️ LOGICAL surface height — the vertical twin of [`Self::logical_width`].
    pub fn logical_height(&self) -> f32 {
        self.logical_height
    }

    /// 📐️ Device pixels per logical pixel for this surface.
    pub fn scale_factor(&self) -> f32 {
        self.dpr
    }
}

#[cfg(test)]
#[path = "../../../🧪️tests/🔬️targets-wgpu-gpu-prepared-present/🦀️.rs"]
mod prepared_present_tests;

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
