//! @emoji 🔌️ The `GraphicsBackend` contract every backend replays a [`crate::scene::RenderPacket`]
//! through, plus [`NullBackend`] — the concrete no-op implementation the alias below resolves to
//! until a real backend crate lands.
//!
//! ## U3 — why this trait carries zero `dyn`
//!
//! `dyn GraphicsBackend` is banned (ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`
//! `📌️important.md` U3) — but there is nothing here for a vtable to erase in the first place.
//! Exactly one of the four hand-written backends (browser webgpu, macOS Metal, Windows D3D12, Linux
//! Vulkan) compiles per build target; they are `cfg`-exclusive, so a host never actually chooses
//! among live alternatives at runtime the way a plugin registry or a widget tree does. The seam is
//! resolved at *compile* time, not call time, so the right tool is a `cfg`-selected type alias to a
//! **concrete** type, never an enum, a `Box<dyn _>` or a vtable:
//!
//! ```text
//! pub type ActiveBackend = <the one backend compiled for this target>;
//! ```
//!
//! A host or a generic frame driver never names a concrete backend type itself — it takes
//! `<B: GraphicsBackend>` as a type parameter (or is generic over `ActiveBackend`), so swapping the
//! alias target is the entire cost of retargeting a platform. Nothing anywhere stores
//! `Box<dyn GraphicsBackend>`.
//!
//! ### How a backend crate plugs itself in
//!
//! The four backend crates (`🎯️targets/{🧊️webgpu,🍎️metal,🪟️d3d12,🌋️vulkan}/`) each **depend on this
//! crate** to implement `GraphicsBackend` for their own concrete context type — they are never a
//! dependency *of* this crate, so this file cannot itself alias to them without inverting that
//! graph. [`ActiveBackend`] here therefore resolves to [`NullBackend`] unconditionally: it is this
//! crate's only backend impl and the correct default for headless tests, the conformance harness,
//! and any target with no compiled-in real backend.
//!
//! The crate that *does* depend on all four backend crates — `semio-framework-ui-host` (packet
//! `ui-host`) — is where the real per-target alias belongs, following exactly this shape:
//!
//! ```text
//! #[cfg(target_arch = "wasm32")]
//! pub type ActiveBackend = semio_framework_ui_backend_webgpu::WebGpuBackend;
//! #[cfg(target_os = "macos")]
//! pub type ActiveBackend = semio_framework_ui_backend_metal::MetalBackend;
//! #[cfg(target_os = "windows")]
//! pub type ActiveBackend = semio_framework_ui_backend_d3d12::D3d12Backend;
//! #[cfg(all(unix, not(target_os = "macos")))]
//! pub type ActiveBackend = semio_framework_ui_backend_vulkan::VulkanBackend;
//! ```
//! Each backend crate's own context type is `pub` and implements [`GraphicsBackend`] directly —
//! nothing about the trait or this file needs to change when that lands.

use crate::resource::{AtlasId, MeshId, ResourceOp, TextureId};
use crate::scene::RenderPacket;
use std::collections::HashSet;

//#region 🔖️Backend

//#region 📐️PhysicalSize

/// 📐️ A physical-pixel surface size — what a backend's swapchain/surface is actually configured to,
/// distinct from [`crate::scene::LayoutRect`]'s logical pixels and from
/// [`crate::scene::RenderPacket::viewport`]'s logical-pixel `f32` size. Not defined anywhere else in
/// this crate (checked `🎬️scene.rs` and every other region file before adding it).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

impl PhysicalSize {
    pub const ZERO: Self = Self { width: 0, height: 0 };

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    /// 🕳️ True when either dimension is zero — the "parked" condition every backend must survive
    /// without erroring (see [`GraphicsBackend::resize`]).
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub const fn is_zero(&self) -> bool {
        self.width == 0 || self.height == 0
    }
}

//#endregion 📐️PhysicalSize

//#region 🧭️DeviceCapabilities

/// 🖼️ A backend-neutral description of the device/surface a backend obtained, so product surfaces
/// can degrade deliberately (drop MSAA, shrink an atlas, skip a storage-buffer path) instead of each
/// one independently probing a device through this trait's other methods.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeviceCapabilities {
    pub max_texture_dimension: u32,
    pub max_bind_groups: u32,
    pub supports_msaa: bool,
    pub supports_timestamp_queries: bool,
    pub supports_storage_buffers: bool,
    pub preferred_surface_format: SurfaceFormat,
    pub memory_class: MemoryClass,
    pub gpu_tier: GpuTier,
}

/// 🎨️ The handful of color formats that occur across all four backends' preferred surface config —
/// a marker a backend maps onto its own device-level format, never a device-level type itself
/// (same shape as [`crate::scene::StencilPolicy`]'s relationship to `wgpu::StencilState`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurfaceFormat {
    Bgra8UnormSrgb,
    Rgba8UnormSrgb,
    Rgba16Float,
}

/// 💾️ A coarse bucket for how much GPU-adjacent memory headroom a device offers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MemoryClass {
    Constrained,
    Standard,
    Abundant,
}

/// 🏎️ A coarse bucket for expected GPU throughput, driving degrade decisions above the shader level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpuTier {
    Software,
    Integrated,
    Discrete,
}

//#endregion 🧭️DeviceCapabilities

//#region ⚠️Errors

/// 🧨️ Why a device/surface transitioned to [`DeviceStatus::Lost`]. A subset of [`BackendError`]'s
/// causes — the ones that leave the device or surface itself unusable until [`GraphicsBackend::recover`]
/// runs, as opposed to a one-shot failure like a single unsupported format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LossReason {
    Surface,
    Device,
    Timeout,
}

/// 🏷️ Which resource table an unrecognized [`crate::resource`] id belonged to, for
/// [`BackendError::UnknownResource`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceKind {
    Texture,
    Mesh,
    Atlas,
}

/// ⚠️ The real failure set a `GraphicsBackend` method can hit. Every variant here is something a
/// compliant backend can legitimately return; it is not a hint that every method returns every
/// variant. Notably `ZeroSizeSurface` is never returned by [`GraphicsBackend::resize`] or
/// [`GraphicsBackend::render`] — those park a zero-size surface instead of erroring (see the trait's
/// docstring) — but a `backend-testing` [`GraphicsBackend::read_back`] on a parked surface has
/// nothing to read back and uses it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BackendError {
    SurfaceOutOfDate,
    SurfaceLost,
    DeviceLost(LossReason),
    Timeout,
    OutOfMemory,
    UnsupportedFormat(&'static str),
    ZeroSizeSurface,
    CanvasReplaced,
    ShaderCompilationFailed(String),
    /// 🕳️ `apply_resources` was never called (or never called with this id) before a `render` whose
    /// packet references it — a clean, matchable error, never a panic.
    UnknownResource(ResourceKind),
}

//#endregion ⚠️Errors

//#region 📊️FrameStats

/// ⏱️ Per-frame timings and counts a backend reports on a presented frame, for the observability the
/// program needs (deadline scheduling, regression dashboards, the conformance harness).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct FrameStats {
    pub encode_duration_seconds: f32,
    pub submit_duration_seconds: f32,
    pub present_duration_seconds: f32,
    pub draw_call_count: u32,
    pub instance_count: u32,
}

//#endregion 📊️FrameStats

//#region 🎬️RenderReport

/// 🎬️ The outcome of one [`GraphicsBackend::render`] call.
#[derive(Clone, Copy, Debug)]
pub enum RenderReport {
    Presented { stats: FrameStats },
    SkippedZeroSize,
    SkippedOutOfDate,
}

//#endregion 🎬️RenderReport

//#region 🚦️DeviceStatus

/// 🚦️ Where a backend's device/surface stands right now. `Suboptimal` mirrors the webgpu/Vulkan
/// notion of a surface that still presents but should be reconfigured soon (e.g. an OS resize the
/// backend has not yet reacted to) — distinct from `Lost`, which requires [`GraphicsBackend::recover`]
/// before rendering can resume.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DeviceStatus {
    Healthy,
    Suboptimal,
    Lost(LossReason),
}

//#endregion 🚦️DeviceStatus

//#region ♻️RecoveredResources

/// ♻️ Which resource ids died in a device loss, in exactly the shape
/// [`crate::resource::ResourceRegistry::report_device_loss`] already takes (`lost_textures`,
/// `lost_meshes`, `lost_atlases`) — found there rather than redefined here, so a caller passes this
/// struct's fields straight through: `registry.report_device_loss(&r.lost_textures, &r.lost_meshes,
/// &r.lost_atlases)`. The registry re-marks each surviving id `Requested` without a generation bump,
/// so the next frame's upload request repopulates the same identity.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct RecoveredResources {
    pub lost_textures: Vec<TextureId>,
    pub lost_meshes: Vec<MeshId>,
    pub lost_atlases: Vec<AtlasId>,
}

//#endregion ♻️RecoveredResources

//#region 🧪️ReadbackImage

/// 🧪️ An offscreen readback of the last presented frame, for the cross-backend pixel conformance
/// suite (tolerance |Δ| ≤ 3/255 for ≥ 99.9% of pixels, none > 12 — enforced by that suite, not here).
/// Gated behind `backend-testing` alongside the trait methods that produce/consume it — never
/// compiled into a shipping build.
#[cfg(feature = "backend-testing")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReadbackImage {
    pub width: u32,
    pub height: u32,
    /// 🎞️ Tightly packed RGBA8, row-major, `width * height * 4` bytes.
    pub pixels: Vec<u8>,
}

//#endregion 🧪️ReadbackImage

//#region 🔌️GraphicsBackendTrait

/// 🔌️ The seam every hand-written GPU backend implements so four independently written
/// implementations (browser webgpu, macOS Metal, Windows D3D12, Linux Vulkan) replay the exact same
/// [`crate::scene::RenderPacket`] into pixel-identical (within conformance tolerance) frames.
///
/// **Invariants every implementation must honour:**
///
/// - **Synchronous, and must not call application code.** `render` encodes and submits GPU work and
///   returns; it never invokes a callback or re-enters UI/application state during encoding or
///   submission (ruling U1). Every method here is a plain `fn` — only a backend's own construction
///   (device/adapter setup) is `async`, and that constructor is intentionally not part of this trait
///   since its signature is backend-specific.
/// - **`apply_resources` happens first.** A caller always applies the [`ResourceOp`] stream that
///   accompanies a [`RenderPacket`] before calling `render` with that packet — a backend never sees a
///   packet reference an id it has not yet been told to upload, except as the deliberate error case
///   `render` must handle cleanly (see [`BackendError::UnknownResource`]).
/// - **A zero-size surface parks, it does not error.** `resize` to `(0, 0)` succeeds; a subsequent
///   `render` returns `RenderReport::SkippedZeroSize` rather than touching the OS surface. Resizing
///   back to a nonzero size afterward must restore a fully working surface — no sticky failure state.
/// - **Batches replay verbatim.** A backend walks [`crate::scene::RenderPacket::batches`] in order and
///   issues each [`crate::scene::DrawBatch`]'s draw call over its `instance_range`
///   (and `mask_range`, when present, painted first under
///   [`crate::scene::StencilPolicy::WriteMask`]) exactly as given. It makes no ordering, batching or
///   clipping decisions of its own — `Scene::finish` already made every one of those decisions, which
///   is precisely what lets four independent backends agree pixel-for-pixel.
pub trait GraphicsBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError>;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ResourceOp]) -> Result<(), BackendError>;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, time_seconds: f32) -> Result<RenderReport, BackendError>;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus;

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError>;

    /// 🧪️ Forces the next [`Self::device_status`] to report `Lost`, for the cross-backend
    /// conformance suite to exercise recovery without waiting on a real GPU fault.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self);

    /// 🧪️ Reads back the last presented frame for pixel comparison across backends.
    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ReadbackImage, BackendError>;
}

//#endregion 🔌️GraphicsBackendTrait

//#region 🕳️NullBackend

/// 🕳️ A real, working no-op [`GraphicsBackend`]: it tracks resource residency and surface size
/// exactly like a device-backed backend would, validates every id a packet references, and reports
/// stats — it simply never touches a GPU. This is [`ActiveBackend`]'s target until a real backend
/// crate lands, and stays permanently useful afterward for headless tests and the conformance
/// harness's control run.
pub struct NullBackend {
    size: PhysicalSize,
    dpr: f32,
    status: DeviceStatus,
    known_textures: HashSet<TextureId>,
    known_meshes: HashSet<MeshId>,
    known_atlases: HashSet<AtlasId>,
    #[cfg(feature = "backend-testing")]
    pending_recovery: Option<RecoveredResources>,
}

impl Default for NullBackend {
    fn default() -> Self {
        Self {
            size: PhysicalSize::ZERO,
            dpr: 1.0,
            status: DeviceStatus::Healthy,
            known_textures: HashSet::new(),
            known_meshes: HashSet::new(),
            known_atlases: HashSet::new(),
            #[cfg(feature = "backend-testing")]
            pending_recovery: None,
        }
    }
}

impl NullBackend {
    /// 🕳️ Plain sync constructor — unlike a real backend's `async fn new(window) -> Result<Self,
    /// BackendError>` (device/adapter construction is the one async exception this trait's
    /// invariants call out), there is no device here to await.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    pub fn new() -> Self {
        Self::default()
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn validate_known_resources(&self, packet: &RenderPacket) -> Result<(), BackendError> {
        for batch in &packet.batches {
            if let Some(texture) = batch.texture {
                if !self.known_textures.contains(&texture) {
                    return Err(BackendError::UnknownResource(ResourceKind::Texture));
                }
            }
        }
        for pass in &packet.surface_passes {
            for draw in pass.draws.iter().chain(pass.translucent_draws.iter()) {
                if !self.known_meshes.contains(&draw.mesh) {
                    return Err(BackendError::UnknownResource(ResourceKind::Mesh));
                }
            }
            for textured in &pass.textured_draws {
                for instance in &textured.instances {
                    if !self.known_textures.contains(&instance.texture) {
                        return Err(BackendError::UnknownResource(ResourceKind::Texture));
                    }
                }
            }
        }
        Ok(())
    }
}

impl GraphicsBackend for NullBackend {
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn name(&self) -> &'static str {
        "null"
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn capabilities(&self) -> DeviceCapabilities {
        DeviceCapabilities {
            max_texture_dimension: 8192,
            max_bind_groups: 4,
            supports_msaa: false,
            supports_timestamp_queries: false,
            supports_storage_buffers: false,
            preferred_surface_format: SurfaceFormat::Rgba8UnormSrgb,
            memory_class: MemoryClass::Constrained,
            gpu_tier: GpuTier::Software,
        }
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn resize(&mut self, size: PhysicalSize, dpr: f32) -> Result<(), BackendError> {
        self.size = size;
        self.dpr = dpr;
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn apply_resources(&mut self, ops: &[ResourceOp]) -> Result<(), BackendError> {
        for op in ops {
            match op {
                ResourceOp::UploadTexture { id, .. } => {
                    self.known_textures.insert(*id);
                }
                ResourceOp::UploadAtlas { id, .. } => {
                    self.known_atlases.insert(*id);
                }
                ResourceOp::CreateOrUpdateMesh { id, .. } => {
                    self.known_meshes.insert(*id);
                }
                ResourceOp::EvictTexture(id) => {
                    self.known_textures.remove(id);
                }
                ResourceOp::EvictMesh(id) => {
                    self.known_meshes.remove(id);
                }
            }
        }
        Ok(())
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn render(&mut self, packet: &RenderPacket, _time_seconds: f32) -> Result<RenderReport, BackendError> {
        if let DeviceStatus::Lost(reason) = self.status {
            return Err(BackendError::DeviceLost(reason));
        }
        if self.size.is_zero() {
            return Ok(RenderReport::SkippedZeroSize);
        }
        self.validate_known_resources(packet)?;
        let stats =
            FrameStats { encode_duration_seconds: 0.0, submit_duration_seconds: 0.0, present_duration_seconds: 0.0, draw_call_count: packet.batches.len() as u32, instance_count: (packet.quad_instances.len() + packet.vector_vertices.len()) as u32 };
        Ok(RenderReport::Presented { stats })
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn device_status(&self) -> DeviceStatus {
        self.status
    }

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn recover(&mut self) -> Result<RecoveredResources, BackendError> {
        #[cfg(feature = "backend-testing")]
        let recovered = self.pending_recovery.take().unwrap_or_default();
        #[cfg(not(feature = "backend-testing"))]
        let recovered = RecoveredResources::default();
        self.status = DeviceStatus::Healthy;
        Ok(recovered)
    }

    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn debug_force_device_loss(&mut self) {
        let lost_textures: Vec<TextureId> = self.known_textures.drain().collect();
        let lost_meshes: Vec<MeshId> = self.known_meshes.drain().collect();
        let lost_atlases: Vec<AtlasId> = self.known_atlases.drain().collect();
        self.pending_recovery = Some(RecoveredResources { lost_textures, lost_meshes, lost_atlases });
        self.status = DeviceStatus::Lost(LossReason::Device);
    }

    #[cfg(feature = "backend-testing")]
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn read_back(&mut self) -> Result<ReadbackImage, BackendError> {
        if self.size.is_zero() {
            return Err(BackendError::ZeroSizeSurface);
        }
        let pixels = vec![0u8; self.size.width as usize * self.size.height as usize * 4];
        Ok(ReadbackImage { width: self.size.width, height: self.size.height, pixels })
    }
}

//#endregion 🕳️NullBackend

//#region 🪪️ActiveBackend

/// 🪪️ The concrete backend this build resolves to — see this file's top docstring for the full U3
/// rationale and the exact pattern `semio-framework-ui-host` follows once the four backend crates
/// exist. Always [`NullBackend`] in this crate, by construction: this crate depends on nothing that
/// implements [`GraphicsBackend`] besides its own no-op.
pub type ActiveBackend = NullBackend;

//#endregion 🪪️ActiveBackend

//#region Tests

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resource::ResourceRegistry;
    use crate::scene::{FinishParams, Scene, SceneBuilder};

    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn finish_params(viewport: [f32; 2]) -> FinishParams {
        FinishParams { viewport, dpr: 1.0, time_seconds_origin: 0.0, resource_ops: Vec::new() }
    }

    /// 🧬️ Exercised only through `<B: GraphicsBackend>` — the same shape every generic frame driver
    /// uses — never through `dyn GraphicsBackend`, proving the trait needs no vtable.
    // 🚫️async: U1 run-to-completion frame transaction — see ticket 26/08/20 📌️important.md
    fn drive_backend<B: GraphicsBackend>(backend: &mut B) -> DeviceCapabilities {
        backend.capabilities()
    }

    #[test]
    fn null_backend_satisfies_the_graphics_backend_trait() {
        let mut backend = NullBackend::new();
        let caps = drive_backend(&mut backend);
        assert!(caps.max_texture_dimension > 0);
        assert_eq!(backend.name(), "null");
    }

    #[test]
    fn zero_size_surface_is_skipped_not_errored() {
        let mut backend = NullBackend::new();
        backend.resize(PhysicalSize::ZERO, 1.0).expect("resize to zero");
        let packet = Scene::finish(SceneBuilder::default(), finish_params([0.0, 0.0])).expect("finish");
        assert!(matches!(backend.render(&packet, 0.0), Ok(RenderReport::SkippedZeroSize)));
    }

    #[test]
    fn resize_to_zero_and_back_restores_a_working_surface() {
        let mut backend = NullBackend::new();
        backend.resize(PhysicalSize::new(200, 100), 1.0).expect("initial resize");
        backend.resize(PhysicalSize::ZERO, 1.0).expect("resize to zero");
        backend.resize(PhysicalSize::new(200, 100), 1.0).expect("resize back");
        assert_eq!(backend.device_status(), DeviceStatus::Healthy);

        let mut builder = SceneBuilder::default();
        builder.push_solid([0.0, 0.0, 10.0, 10.0], [1.0, 0.0, 0.0, 1.0]);
        let packet = Scene::finish(builder, finish_params([200.0, 100.0])).expect("finish");
        assert!(matches!(backend.render(&packet, 0.0), Ok(RenderReport::Presented { .. })));
    }

    #[test]
    fn apply_resources_before_render_succeeds_and_an_unapplied_id_errors_cleanly() {
        let mut backend = NullBackend::new();
        backend.resize(PhysicalSize::new(100, 100), 1.0).expect("resize");
        let mut registry = ResourceRegistry::default();

        // 🖼️ Drawing only *interns* a texture — `push_raster_quad` references it by id. Getting the
        // pixels to the device is a separate request, which is what makes a raster quad cheap to emit
        // every frame; so the upload has to be asked for explicitly before the backend can honour it.
        registry.request_texture_upload("known", 4, 4, vec![0; 64]);
        let mut applied_builder = SceneBuilder::default();
        applied_builder.push_raster_quad(&mut registry, "known", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        backend.apply_resources(&registry.drain_ops()).expect("apply_resources");
        let applied_packet = Scene::finish(applied_builder, finish_params([100.0, 100.0])).expect("finish");
        assert!(matches!(backend.render(&applied_packet, 0.0), Ok(RenderReport::Presented { .. })));

        let mut unapplied_builder = SceneBuilder::default();
        unapplied_builder.push_raster_quad(&mut registry, "unknown", [0.0, 0.0, 10.0, 10.0], [0.0, 0.0, 1.0, 1.0], 1.0);
        let unapplied_packet = Scene::finish(unapplied_builder, finish_params([100.0, 100.0])).expect("finish");
        let result = backend.render(&unapplied_packet, 0.0);
        assert!(matches!(result, Err(BackendError::UnknownResource(ResourceKind::Texture))));
    }

    #[cfg(feature = "backend-testing")]
    #[test]
    fn forced_device_loss_reports_lost_status_and_recover_returns_dead_generations() {
        let mut backend = NullBackend::new();
        backend.resize(PhysicalSize::new(100, 100), 1.0).expect("resize");
        let mut registry = ResourceRegistry::default();
        let texture = registry.request_texture_upload("icon", 4, 4, vec![0; 64]);
        backend.apply_resources(&registry.drain_ops()).expect("apply_resources");

        backend.debug_force_device_loss();
        assert!(matches!(backend.device_status(), DeviceStatus::Lost(_)));

        let recovered = backend.recover().expect("recover");
        assert_eq!(recovered.lost_textures, vec![texture]);
        assert_eq!(backend.device_status(), DeviceStatus::Healthy);
    }
}

//#endregion Tests

//#endregion 🔖️Backend
