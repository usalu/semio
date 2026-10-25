//! 🧠️ Gpu context and meshes: the session roots, the `GpuContext` state, the constructors, the readback, the target and depth management and the mesh upload steps. See [`super`].

use super::*;

// #region gpu
pub(super) struct GpuSessionRoots { pub(super) device:gpu_device::Device,pub(super) queue:gpu_device::Queue,pub(super) commands:gpu_device::Buffer }

pub struct GpuContext {
    pub(super) roots: Arc<GpuSessionRoots>,
    pub(super) surface: Option<Surface>,
    config: gpu_device::SurfaceConfiguration,
    pub(super) color_target_format: gpu_device::TextureFormat,
    pub(super) pipelines: UiPipelines,
    pub(super) frame_buffers: FrameBuffers,
    depth_texture: Option<gpu_device::Texture>,
    pub(super) depth_view: Option<gpu_device::TextureView>,
    pub(super) mesh_store: MeshGpuTable,
    pub(super) raster_store: RasterTextureTable,
    pub(super) scene_color: Option<SceneColorTarget>,
    pub(super) composite_color: Option<PreparedCompositeTarget>,
    pub(super) accepted_offscreen: Option<(u64, u64)>,
    pub(super) atlas_upload: Option<PreparedAtlasUploadCursor>,
    /// 🧷️ The last prepared world draw whose mesh was not resident at submit, kept for ONE readable
    /// host report instead of the fatal `present_step` fault that used to quarantine the surface.
    pub(super) missing_world_mesh: Option<(String, u64)>,
    /// 📐️ PHYSICAL (device) surface extent — the only place physical pixels are legal. Everything
    /// a draw list, a hit test or a chrome constant speaks is logical; see `logical_width`.
    pub(super) width: u32,
    pub(super) height: u32,
    /// 📏️ LOGICAL (CSS) surface extent — what the projection divides by, so a draw list authored in
    /// logical pixels lands on the full physical surface at any scale factor.
    pub(super) logical_width: f32,
    pub(super) logical_height: f32,
    pub(super) dpr: f32,
}

impl GpuContext {
    pub fn from_prepared_device(prepared:gpu_device::platform::PreparedDevice)->Self{Self::from_device(prepared.device,prepared.queue,prepared.surface,prepared.config,prepared.logical_width,prepared.logical_height,prepared.dpr)}

    fn from_device(device: gpu_device::Device, queue: gpu_device::Queue, surface: Option<Surface>, config: gpu_device::SurfaceConfiguration, css_width: f32, css_height: f32, dpr: f32) -> Self {
        let color_target_format = config.format.add_srgb_suffix();
        let (width, height) = (config.width, config.height);
        let mut pipelines = UiPipelines::new(&device, &queue, color_target_format);
        pipelines.set_surface_scale(dpr);
        let raster_store = RasterTextureTable::new(&device, pipelines.bind_group_layout());
        let prepared_command_buffer = device.create_buffer(&gpu_device::BufferDescriptor {
            label: Some("prepared_render_command_pages"),
            size: (PREPARED_RENDER_COMMAND_PAGES as u64) * (PREPARED_RENDER_COMMAND_PAGE_ITEMS as u64) * 16,
            usage: gpu_device::BufferUsages::COPY_DST | gpu_device::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let mut gpu = Self {
            roots: Arc::new(GpuSessionRoots{device,queue,commands:prepared_command_buffer}),
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
            accepted_offscreen: None,
            atlas_upload: None,
            missing_world_mesh: None,
            width,
            height,
            logical_width: css_width.max(1.0),
            logical_height: css_height.max(1.0),
            dpr,
        };
        gpu.ensure_depth();
        gpu
    }

    /// 🏞️ Creates an export device without acquiring a window or browser canvas.
    pub async fn headless(width:u32,height:u32)->Result<Self,String>{let layout=PreparedReadbackLayout::new(width,height,false)?;Ok(Self::from_prepared_device(gpu_device::platform::prepare_headless(width,height,layout.byte_length).await?))}

    /// 🖼️ Creates an independent request-sized target on the existing device and queue.
    pub fn offscreen(&self, width: u32, height: u32) -> Result<Self, String> {
        let layout = PreparedReadbackLayout::new(width, height, false)?;
        let limits = self.roots.device.limits();
        if width > limits.max_texture_dimension_2d || height > limits.max_texture_dimension_2d || layout.byte_length > limits.max_buffer_size {
            return Err("prepared offscreen dimensions exceed the device budget".into());
        }
        let mut config = self.config.clone();
        config.width = width;
        config.height = height;
        config.format = gpu_device::TextureFormat::Rgba8Unorm;
        config.view_formats = vec![gpu_device::TextureFormat::Rgba8UnormSrgb];
        Ok(Self::from_device(self.roots.device.clone(), self.roots.queue.clone(), None, config, width as f32, height as f32, 1.0))
    }

    /// 📥️ Reads only a completed offscreen packet; live surface ownership is untouched.
    pub fn begin_prepared_readback(&mut self, cursor: &PreparedGpuPresentCursor) -> Result<PreparedGpuReadback, String> {
        if self.surface.is_some() || cursor.phase != PreparedGpuPresentPhase::Complete || self.accepted_offscreen != Some((cursor.scene_revision, cursor.preview_generation)) {
            return Err("prepared readback requires a completed offscreen packet".into());
        }
        let composite = self.composite_color.as_ref().ok_or_else(|| "prepared readback target was missing".to_string())?;
        let layout = PreparedReadbackLayout::new(self.width, self.height, false)?;
        let readback = PreparedGpuReadback::begin(&self.roots.device, &self.roots.queue, composite, layout)?;
        self.accepted_offscreen = None;
        Ok(readback)
    }

    pub fn prepared_readback_step(&self, readback: &mut PreparedGpuReadback) -> Result<bool, String> {
        readback.advance(&self.roots.device)
    }

    fn ensure_depth(&mut self) {
        let depth_texture = self.roots.device.create_texture(&gpu_device::TextureDescriptor {
            label: Some("ui_depth"),
            size: gpu_device::Extent3d { width: self.width.max(1), height: self.height.max(1), depth_or_array_layers: 1 },
            mip_level_count: 1,
            sample_count: 1,
            dimension: gpu_device::TextureDimension::D2,
            format: self.pipelines.depth_format(),
            usage: gpu_device::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&gpu_device::TextureViewDescriptor::default());
        self.depth_texture = Some(depth_texture);
        self.depth_view = Some(depth_view);
    }

    /// 🔄️ `css_width`/`css_height` are LOGICAL pixels; `dpr` is the surface scale factor. A scale
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
        if let Some(surface) = self.surface.as_ref() {
            surface.configure(&self.roots.device, &self.config);
        }
        self.scene_color = None;
        self.composite_color = None;
        self.accepted_offscreen = None;
        self.ensure_depth();
    }

    /// 🎯️ Both surface-sized colour targets a prepared frame needs: the scene it draws into and the
    /// composite the swapchain is finally handed.
    pub(super) fn ensure_prepared_targets(&mut self) {
        SceneColorTarget::ensure(&self.roots.device, &mut self.scene_color, self.width, self.height, self.color_target_format);
        PreparedCompositeTarget::ensure(&self.roots.device, &mut self.composite_color, self.width, self.height, self.color_target_format);
    }

    pub fn mesh_index_count(&self,key:&str,version:u64)->Option<u32>{self.mesh_store.get_versioned(key,version).map(|mesh|mesh.index_count)}
    pub fn mesh_observation(&self,key:&str,version:u64)->Option<crate::MeshGpuObservation>{self.mesh_store.get_versioned(key,version).map(|mesh|crate::MeshGpuObservation{source:mesh.original_source_lease(),index_count:mesh.index_count,index_bytes:mesh.index_buffer.as_ref().map_or(0,gpu_device::Buffer::size),vertex_bytes:mesh.vertex_buffer.as_ref().map_or(0,gpu_device::Buffer::size)})}

    pub fn ensure_mesh_step(&mut self, key: &str, version: u64, lease: crate::kernel_3d_scene::Mesh3dLease) -> Result<bool, &'static str> {
        self.mesh_store.ensure_mesh_step(&self.roots.device, &self.roots.queue, key, version, lease)
    }

    pub fn close_mesh_upload_step(&mut self) -> bool {
        self.mesh_store.close_upload_step()
    }

    pub fn mesh_upload_terminal_is_empty(&self) -> bool {
        self.mesh_store.upload_terminal_is_empty()
    }

    /// 🕳️ The last prepared world draw that named a non-resident mesh, TAKEN so the host reports it
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

    pub fn retire_original_mesh_source_step(&mut self,key:&str,source:&crate::kernel_3d_scene::Mesh3dResourceLoan)->Result<Option<crate::MeshGpuSourceReturn>,&'static str>{self.mesh_store.retire_original_source_step(key,source)}

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

}
// #endregion gpu
