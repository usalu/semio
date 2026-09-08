//! 🖌️ Headless offscreen rasterization of first-party vector scenes to RGBA8 pixel buffers.
//!
//! Every public type is first-party (`semio_framework_geometry::{BezPath, Affine}` plus plain Rust
//! types), so a caller never needs to import `vello`/`wgpu` itself (CLAUDE.md: "use external
//! libraries behind an interface" / "MUST NOT export api that ... requires an interface/class/type
//! outside of this codebase"). Relocated from `🎞️animate`'s `⚙️engine/🎥️video` `VelloRenderer`
//! (ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
//!
//! Tier split (same ticket, `🔍️research/📓️raster-tier-split.md`): `FillOp`/`StrokeOp`/`DrawOp`/
//! `VectorScene`/`RasterError` are scene-description value types with zero `wgpu::`/`vello::`
//! reference — they stay unconditional, including on `wasm32-wasip2`. `SceneRasterizer` (and every
//! private fn it alone uses) genuinely opens a GPU device, which a `wasm32-wasip2` guest component
//! cannot do (WASI Preview 2 defines no graphics API), so it is gated
//! `#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]`. `target_arch = "wasm32"` alone is
//! TRUE for `wasm32-wasip2` (confirm with `rustc --print cfg --target wasm32-wasip2`), so a bare
//! `cfg(not(target_arch = "wasm32"))` would NOT exclude this target and the shipped component would
//! still link `wgpu`/`vello` and their `wasm-bindgen`/`js-sys`/`web-sys` transitive edge — this is
//! the exact bug class `🔍️research/📓️verified-outcomes.md` already found once in `🧩️puzzle`.

use semio_framework_geometry::{Affine, BezPath};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
use semio_framework_geometry::{PathEl, Point};

//#region 🔖️VectorScene

/// 🖊️ A filled path, in the coordinate space `transform` maps into device pixels.
pub struct FillOp {
    pub path: BezPath,
    pub transform: Affine,
    pub color: [f32; 4],
}

/// 🖊️ A stroked path, in the coordinate space `transform` maps into device pixels.
pub struct StrokeOp {
    pub path: BezPath,
    pub transform: Affine,
    pub color: [f32; 4],
    pub width: f64,
}

/// 🎨️ One paint operation, in the order it must be composited.
pub enum DrawOp {
    Fill(FillOp),
    Stroke(StrokeOp),
}

/// 🖼️ An ordered, first-party description of a 2D vector scene — the only input `SceneRasterizer`
/// accepts. Draw order is composite order (painter's algorithm); callers sort their own domain
/// objects (z-order, id, ...) before pushing ops.
#[derive(Default)]
pub struct VectorScene {
    pub ops: Vec<DrawOp>,
}

impl VectorScene {
    pub fn new() -> Self {
        Self { ops: Vec::new() }
    }

    pub fn push(&mut self, op: DrawOp) {
        self.ops.push(op);
    }

    pub fn fill(&mut self, path: BezPath, transform: Affine, color: [f32; 4]) {
        self.push(DrawOp::Fill(FillOp { path, transform, color }));
    }

    pub fn stroke(&mut self, path: BezPath, transform: Affine, color: [f32; 4], width: f64) {
        self.push(DrawOp::Stroke(StrokeOp { path, transform, color, width }));
    }
}

//#endregion 🔖️VectorScene

//#region 🔖️RasterError

/// 🚨️ Everything that can fail while standing up or driving a headless GPU rasterizer.
#[derive(Debug)]
pub enum RasterError {
    Adapter(String),
    Device(String),
    Render(String),
    ReadbackChannelClosed,
    ReadbackMap(String),
}

impl std::fmt::Display for RasterError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Adapter(message) => write!(formatter, "no wgpu adapter available: {message}"),
            Self::Device(message) => write!(formatter, "wgpu device request failed: {message}"),
            Self::Render(message) => write!(formatter, "scene render failed: {message}"),
            Self::ReadbackChannelClosed => formatter.write_str("gpu readback channel closed before a result arrived"),
            Self::ReadbackMap(message) => write!(formatter, "gpu readback map failed: {message}"),
        }
    }
}

impl std::error::Error for RasterError {}

//#endregion 🔖️RasterError

//#region 🔖️SceneRasterizer

/// 🖥️ A headless wgpu + Vello device, sized once at construction, that rasterizes any number of
/// [`VectorScene`]s to RGBA8 pixel buffers of that fixed size. Native/host-only — see module
/// docstring; a `wasm32-wasip2` guest has no GPU device access.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub struct SceneRasterizer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    renderer: vello::Renderer,
    width: u32,
    height: u32,
    target_texture: wgpu::Texture,
    target_view: wgpu::TextureView,
    readback_buffer: wgpu::Buffer,
    readback_bytes_per_row: u32,
}

/// 📏️ Rounds `unpadded` up to `wgpu::COPY_BYTES_PER_ROW_ALIGNMENT` — wgpu rejects a
/// `copy_texture_to_buffer` whose row stride isn't aligned, and `4 * width` only happens to be
/// aligned when `width` is itself a multiple of 64.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
const fn align_bytes_per_row(unpadded: u32) -> u32 {
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    unpadded.div_ceil(align) * align
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl SceneRasterizer {
    /// 🏗️ Creates a headless wgpu + Vello rasterizer at `width` × `height`.
    pub async fn new(width: u32, height: u32) -> Result<Self, RasterError> {
        let width = width.max(1);
        let height = height.max(1);
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor { backends: wgpu::Backends::PRIMARY, ..wgpu::InstanceDescriptor::new_without_display_handle() });
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions { power_preference: wgpu::PowerPreference::HighPerformance, compatible_surface: None, force_fallback_adapter: false })
            .await
            .map_err(|err| RasterError::Adapter(format!("{err:?}")))?;
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: Some("semio_framework_raster"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
                experimental_features: Default::default(),
            })
            .await
            .map_err(|err| RasterError::Device(format!("{err:?}")))?;
        let renderer = vello::Renderer::new(
            &device,
            vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None },
        )
        .map_err(|err| RasterError::Render(format!("{err:?}")))?;
        let (target_texture, target_view) = create_target_texture(&device, width, height);
        let readback_bytes_per_row = align_bytes_per_row(width * 4);
        let readback_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("semio_framework_raster_readback"),
            size: u64::from(readback_bytes_per_row) * u64::from(height),
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        Ok(Self { device, queue, renderer, width, height, target_texture, target_view, readback_buffer, readback_bytes_per_row })
    }

    /// 🖼️ Renders `scene` over `background` and reads the result back as tightly packed RGBA8 rows.
    pub fn render(&mut self, scene: &VectorScene, background: [f32; 4]) -> Result<Vec<u8>, RasterError> {
        let vello_scene = build_vello_scene(scene);
        let params = vello::RenderParams { base_color: vello::peniko::Color::new(background), width: self.width, height: self.height, antialiasing_method: vello::AaConfig::Area };
        self.renderer.render_to_texture(&self.device, &self.queue, &vello_scene, &self.target_view, &params).map_err(|err| RasterError::Render(format!("{err:?}")))?;
        read_pixels(&self.device, &self.queue, &self.target_texture, &self.readback_buffer, self.width, self.height, self.readback_bytes_per_row)
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn create_target_texture(device: &wgpu::Device, width: u32, height: u32) -> (wgpu::Texture, wgpu::TextureView) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("semio_framework_raster_target"),
        size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor::default());
    (texture, view)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn build_vello_scene(scene: &VectorScene) -> vello::Scene {
    let mut vello_scene = vello::Scene::new();
    for op in &scene.ops {
        match op {
            DrawOp::Fill(fill) => {
                vello_scene.fill(vello::peniko::Fill::NonZero, affine_to_vello(fill.transform), vello::peniko::Color::new(fill.color), None, &path_to_vello(&fill.path));
            }
            DrawOp::Stroke(stroke) => {
                let stroke_style = vello::kurbo::Stroke::new(stroke.width);
                vello_scene.stroke(&stroke_style, affine_to_vello(stroke.transform), vello::peniko::Color::new(stroke.color), None, &path_to_vello(&stroke.path));
            }
        }
    }
    vello_scene
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn affine_to_vello(value: Affine) -> vello::kurbo::Affine {
    vello::kurbo::Affine::new(value.as_coeffs())
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn point_to_vello(value: Point) -> vello::kurbo::Point {
    vello::kurbo::Point::new(value.x, value.y)
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn path_to_vello(value: &BezPath) -> vello::kurbo::BezPath {
    let mut path = vello::kurbo::BezPath::new();
    for element in value.elements() {
        path.push(match element {
            PathEl::MoveTo(point) => vello::kurbo::PathEl::MoveTo(point_to_vello(point)),
            PathEl::LineTo(point) => vello::kurbo::PathEl::LineTo(point_to_vello(point)),
            PathEl::QuadTo(control, point) => vello::kurbo::PathEl::QuadTo(point_to_vello(control), point_to_vello(point)),
            PathEl::CurveTo(control1, control2, point) => vello::kurbo::PathEl::CurveTo(point_to_vello(control1), point_to_vello(control2), point_to_vello(point)),
            PathEl::ClosePath => vello::kurbo::PathEl::ClosePath,
        });
    }
    path
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn read_pixels(device: &wgpu::Device, queue: &wgpu::Queue, texture: &wgpu::Texture, readback_buffer: &wgpu::Buffer, width: u32, height: u32, bytes_per_row: u32) -> Result<Vec<u8>, RasterError> {
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: Some("semio_framework_raster_readback") });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo { texture, mip_level: 0, origin: wgpu::Origin3d::ZERO, aspect: wgpu::TextureAspect::All },
        wgpu::TexelCopyBufferInfo { buffer: readback_buffer, layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(bytes_per_row), rows_per_image: Some(height) } },
        wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
    );
    queue.submit(Some(encoder.finish()));
    let slice = readback_buffer.slice(..);
    let (sender, receiver) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = sender.send(result);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    receiver.recv().map_err(|_| RasterError::ReadbackChannelClosed)?.map_err(|err| RasterError::ReadbackMap(format!("{err:?}")))?;
    let data = slice.get_mapped_range();
    let unpadded_bytes_per_row = (width * 4) as usize;
    let padded_bytes_per_row = bytes_per_row as usize;
    let pixels = if padded_bytes_per_row == unpadded_bytes_per_row {
        data.to_vec()
    } else {
        let mut out = Vec::with_capacity(unpadded_bytes_per_row * height as usize);
        for row in 0..height as usize {
            let start = row * padded_bytes_per_row;
            out.extend_from_slice(&data[start..start + unpadded_bytes_per_row]);
        }
        out
    };
    drop(data);
    readback_buffer.unmap();
    Ok(pixels)
}

//#endregion 🔖️SceneRasterizer

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
