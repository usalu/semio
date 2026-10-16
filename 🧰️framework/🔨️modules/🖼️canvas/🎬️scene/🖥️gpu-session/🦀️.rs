//! 🖥️ The browser WebGPU canvas session: surface bring-up, resize, detach and the frame render that blits a `Scene` onto the attached canvas.

// #region 🔖️GpuSession
use super::renderer::vello_backend::{util, vello, wgpu};
use super::{Color, Scene};
pub use wasm_bindgen::prelude::JsValue;
pub use web_sys::HtmlCanvasElement;

#[derive(Default)]
pub struct CanvasGpuSession {
    #[allow(dead_code, reason = "Retains canvas for the WebGPU surface lifetime.")]
    canvas: Option<HtmlCanvasElement>,
    render_ctx: Option<util::RenderContext>,
    renderer: Option<vello::Renderer>,
    surface: Option<util::RenderSurface<'static>>,
}

/// 🔌️ Owned admission result for attaching the Canvas GPU backend.
pub struct CanvasAttachSession(CanvasGpuSession);

impl CanvasGpuSession {
    pub fn gpu_ready(&self) -> bool {
        self.surface.is_some()
    }

    /// 🖥️ WebGPU surface bring-up; returns `String` (not `CanvasError`) because every call site is a wasm-bindgen boundary fn that immediately erases the error into a `JsValue` for JS — see `render_frame` below for the same convention.
    ///
    /// The adapter is acquired BEFORE the canvas is touched, and that order is the contract, not
    /// a preference: `Instance::create_surface` binds a `webgpu` context to the element, a canvas
    /// admits exactly one context kind for its whole life, and a bring-up that binds the context
    /// and only then discovers there is no adapter leaves an element on which the deviceless 2D
    /// fallback can never paint either. Measured on a headless browser, which exposes
    /// `navigator.gpu` and hands out no adapter: `No available adapters` followed by a node-graph
    /// canvas whose `getContext("2d")` was null for the rest of the session.
    pub async fn create_canvas_surface(canvas: HtmlCanvasElement, pw: u32, ph: u32) -> Result<CanvasAttachSession, String> {
        let mut render_ctx = util::RenderContext::new();
        if render_ctx.device(None).await.is_none() {
            return Err("no WebGPU adapter".to_string());
        }
        let surface = render_ctx.create_surface(wgpu::SurfaceTarget::Canvas(canvas.clone()), pw, ph, wgpu::PresentMode::AutoVsync).await.map_err(|err| format!("{err:?}"))?;
        let dev = &render_ctx.devices[surface.dev_id].device;
        let renderer =
            vello::Renderer::new(dev, vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None }).map_err(|err| format!("{err:?}"))?;
        Ok(CanvasAttachSession(Self { canvas: Some(canvas), render_ctx: Some(render_ctx), renderer: Some(renderer), surface: Some(surface) }))
    }

    pub fn finish_attach(&mut self, admission: CanvasAttachSession) {
        *self = admission.0;
    }

    pub fn resize_surface(&mut self, pw: u32, ph: u32) {
        if let (Some(surface), Some(render_ctx)) = (self.surface.as_mut(), self.render_ctx.as_mut()) {
            let cur_w = surface.config.width;
            let cur_h = surface.config.height;
            if cur_w != pw || cur_h != ph {
                render_ctx.resize_surface(surface, pw, ph);
            }
        }
    }

    pub fn detach(&mut self) {
        self.canvas = None;
        self.render_ctx = None;
        self.renderer = None;
        self.surface = None;
    }

    pub fn render_frame(&mut self, scene: &Scene, clear_color: Color) -> Result<(), JsValue> {
        for _attempt in 0..3u8 {
            let (surface, renderer, render_ctx) = match (self.surface.as_mut(), self.renderer.as_mut(), self.render_ctx.as_mut()) {
                (Some(s), Some(r), Some(rc)) => (s, r, rc),
                _ => return Ok(()),
            };
            let dh = &render_ctx.devices[surface.dev_id];
            let pw = surface.config.width.max(1);
            let ph = surface.config.height.max(1);
            let params = vello::RenderParams { base_color: clear_color.to_peniko(), width: pw, height: ph, antialiasing_method: vello::AaConfig::Area };
            let vello_scene = scene.vello_scene();
            renderer.render_to_texture(&dh.device, &dh.queue, &vello_scene, &surface.target_view, &params).map_err(|err| JsValue::from_str(&format!("{err:?}")))?;

            let surface_tex = match surface.surface.get_current_texture() {
                wgpu::CurrentSurfaceTexture::Success(frame) | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
                wgpu::CurrentSurfaceTexture::Outdated => {
                    surface.surface.configure(&dh.device, &surface.config);
                    continue;
                }
                wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => return Ok(()),
                wgpu::CurrentSurfaceTexture::Lost | wgpu::CurrentSurfaceTexture::Validation => {
                    return Err(JsValue::from_str("surface lost or validation error"));
                }
            };
            let view = surface_tex.texture.create_view(&wgpu::TextureViewDescriptor::default());
            let mut encoder = dh.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("canvas_surface_blit") });
            surface.blitter.copy(&dh.device, &mut encoder, &surface.target_view, &view);
            dh.queue.submit(std::iter::once(encoder.finish()));
            surface_tex.present();
            let _ = dh.device.poll(wgpu::PollType::Poll).ok();
            return Ok(());
        }
        Ok(())
    }
}
// #endregion 🔖️GpuSession
