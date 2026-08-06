// #region gpu
//! 🖥️ WebGPU device, surface, and frame loop.

use crate::wgpu::draw::{DrawList, FrameBuffers, MeshGpuStore, RasterTextureStore, SceneColorTarget, UiPipelines};
use crate::wgpu::text::FontAtlas;
use wgpu::Surface;

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
    mesh_store: MeshGpuStore,
    raster_store: RasterTextureStore,
    scene_color: Option<SceneColorTarget>,
    width: u32,
    height: u32,
    dpr: f32,
}

impl GpuContext {
    pub async fn from_window(window: std::sync::Arc<winit::window::Window>) -> Result<Self, String> {
        let dpr = window.scale_factor() as f32;
        let size = window.inner_size();
        let css_width = size.width as f32 / dpr;
        let css_height = size.height as f32 / dpr;
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor { backends: if cfg!(target_arch = "wasm32") { wgpu::Backends::BROWSER_WEBGPU } else { wgpu::Backends::PRIMARY }, ..Default::default() });
        let surface = instance.create_surface(wgpu::SurfaceTarget::Window(Box::new(window))).map_err(|err| format!("surface: {err:?}"))?;
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: Some(&surface), force_fallback_adapter: false })
            .await
            .map_err(|err| format!("adapter: {err:?}"))?;
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
        let raster_store = RasterTextureStore::new(&device, pipelines.bind_group_layout());
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
            mesh_store: MeshGpuStore::default(),
            raster_store,
            scene_color: None,
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

    pub fn mesh_store_mut(&mut self) -> &mut MeshGpuStore {
        &mut self.mesh_store
    }

    pub fn ensure_mesh(&mut self, key: &str, version: u64, positions: &[f32], normals: &[f32], indices: &[u32]) {
        self.mesh_store.ensure_mesh(&self.device, key, version, positions, normals, indices);
    }

    pub fn evict_mesh(&mut self, key: &str) {
        self.mesh_store.evict_mesh(key);
    }

    pub fn render_frame(&mut self, draw: &DrawList, overlay: Option<&DrawList>, time_seconds: f32) -> Result<(), String> {
        self.ensure_scene_color();
        let scene = self.scene_color.as_ref().expect("scene_color");
        let frame = self.surface.get_current_texture().map_err(|err| format!("frame: {err:?}"))?;
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor { format: Some(self.color_target_format), ..Default::default() });
        let depth_view = self.depth_view.as_ref();
        let mut scene_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_scene") });
        self.pipelines.render_scene_content(&self.device, &self.queue, &mut scene_encoder, scene, depth_view, draw, &self.mesh_store, &self.raster_store, &mut self.frame_buffers, self.width as f32, self.height as f32, time_seconds);
        self.queue.submit(Some(scene_encoder.finish()));
        let mut composite_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("ui_wgpu_composite") });
        self.pipelines.composite_to_swapchain(&self.device, &self.queue, &mut composite_encoder, &view, scene, depth_view, draw, overlay, &self.mesh_store, &self.raster_store, &mut self.frame_buffers, self.width as f32, self.height as f32);
        self.queue.submit(Some(composite_encoder.finish()));
        frame.present();
        Ok(())
    }

    pub fn upload_font_atlas(&self, atlas: &FontAtlas) {
        self.pipelines.upload_glyph_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub fn upload_icon_atlas(&self, atlas: &crate::wgpu::draw::IconAtlas) {
        self.pipelines.upload_icon_atlas(&self.queue, &atlas.pixels, atlas.width, atlas.height);
    }

    pub fn ensure_raster_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
        self.raster_store.ensure_raster(
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
        );
    }

    pub fn ensure_world_plane_texture(&mut self, key: &str, pixels: &[u8], width: u32, height: u32) {
        self.ensure_raster_texture(key, pixels, width, height);
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

    pub fn register_engine_texture(&mut self, key: &str, texture: wgpu::Texture, view: &wgpu::TextureView, width: u32, height: u32) {
        self.raster_store.replace_gpu_bind_group(&self.device, self.pipelines.globals_buffer(), &self.pipelines.glyph_view(), self.pipelines.glyph_sampler(), key, view, texture, width, height);
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

#[cfg(target_arch = "wasm32")]
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

#[cfg(not(target_arch = "wasm32"))]
pub fn schedule_frame(window: &winit::window::Window, _callback: impl FnMut() + 'static) {
    window.request_redraw();
}
// #endregion gpu
