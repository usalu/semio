//! 🎬️ Canvas scene: the first-party command list `Scene` and its paint types, the portable draw list, SVG icon painting, shaped labels, theme paints and the GPU sessions that rasterize a scene. Application-neutral; extend through `CanvasExtension` in the facade.
#![allow(clippy::missing_errors_doc, reason = "Canvas bundle is internal infrastructure.")]

#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
extern crate semio_framework_async as wasm_bindgen_futures;

// #region 🔖️Renderer
#[path = "🖌️renderer/🦀️.rs"]
mod renderer;

pub use geometry::{append_shape_to_path, geom_sel, Affine, Arc, BezPath, Circle, CubicBez, Line, PathEl, Point, Rect, RoundedRect, RoundedRectRadii, ShapeRef, Vec2};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub(crate) use renderer::vello_backend::usvg;
pub use renderer::draw_list;
#[path = "../♻️retirement/➰️path/🦀️.rs"]
mod path_retirement;
pub use path_retirement::CanvasPathRetirement;
#[path = "../♻️retirement/🖊️stroke/🦀️.rs"]
mod stroke_retirement;
pub use stroke_retirement::CanvasStrokeRetirement;
pub use renderer::{
    CanvasSceneRetirement, advance_opaque_scene_retirement, opaque_scene_retirement_status, publish_opaque_scene_retirement, reserve_opaque_scene_retirement, BlendMode, Cap, Color, FillRule, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Paint, RasterImage, Rgba8,
    Scene, Stroke,
};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use renderer::{append_svg_document, SvgDocument};
// #endregion 🔖️Renderer

/// 📐️ First-party intrinsic-dimension reader (`🧰️framework/🔨️modules/📏️intrinsic-size`) — the
/// `wasm32-wasip2` arm of `svg_icon::svg_icon_content_bounds_from_str` uses it instead of `usvg` so that
/// dimension-only call site stops pulling the SVG pipeline onto the guest component. Native/browser keep the
/// real `usvg` parse unchanged. Ticket `26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`,
/// `🔍️research/📓️intrinsic-size-wiring.md`.
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_intrinsic_size as intrinsic_size;

/// 🖥️ Framework GPU device boundary for the owned Canvas render session.
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub mod native_render {
    pub use vello::wgpu::{Device, Queue, TextureView};
    use super::{Color, Scene};
    /// 🎨️ Owns the private backend interpreter and rasterizer.
    pub struct CanvasRasterSession(vello::Renderer);
    impl CanvasRasterSession {
        pub fn new(device: &Device) -> Result<Self, String> {
            vello::Renderer::new(device, vello::RendererOptions { use_cpu: false, antialiasing_support: vello::AaSupport::area_only(), num_init_threads: std::num::NonZeroUsize::new(1), pipeline_cache: None }).map(Self).map_err(|error| format!("{error:?}"))
        }
        pub fn render(&mut self, device: &Device, queue: &Queue, scene: &Scene, target: &TextureView, clear: Color, width: u32, height: u32) -> Result<(), String> {
            let params = vello::RenderParams { base_color: clear.to_peniko(), width, height, antialiasing_method: vello::AaConfig::Area };
            let backend = scene.vello_scene();
            self.0.render_to_texture(device, queue, &backend, target, &params).map_err(|error| format!("{error:?}"))
        }
    }
}

// #region ⚠️ Errors
/// 🚨️ SVG-parse failures raised by canvas icon/label rendering.
#[derive(Clone, Debug, PartialEq)]
pub enum CanvasError {
    /// 🏷️ SVG source failed to parse into a `usvg` tree.
    SvgParse(String),
}

impl std::fmt::Display for CanvasError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SvgParse(message) => write!(formatter, "SVG parse failed: {message}"),
        }
    }
}

impl std::error::Error for CanvasError {}
// #endregion ⚠️ Errors

pub mod theme {
    // #region theme
    //! 🎨️ Default canvas paint helpers from centralized styling tokens.

    use super::{Color, Rgba8};
    use ui_styling::{appearance::AppearanceName, CANVAS_LIGHT};

    /// 🌈️ Maps a linear-sRGB token color to `Color`.
    pub fn linear_color(rgba: [f32; 4]) -> Color {
        Color::new(rgba)
    }

    /// 🎨️ Shared default clear color for graph board canvases.
    pub fn default_raster_clear() -> Color {
        linear_color(CANVAS_LIGHT.raster_clear)
    }

    /// 🎨️ Default themed icon foreground paint.
    pub fn default_icon_fg() -> Color {
        linear_color(CANVAS_LIGHT.icon_fg)
    }

    /// 🎨️ Default themed icon background paint.
    pub fn default_icon_bg() -> Color {
        linear_color(CANVAS_LIGHT.icon_bg)
    }

    /// 🎨️ Resolves canvas paints for a theme name.
    pub fn canvas_clear_for(theme: AppearanceName) -> Color {
        linear_color(theme.canvas().raster_clear)
    }

    /// 🗂️ Supplies optional sRGB8888 theme fields by their literal names.
    pub trait ThemeColorFields {
        fn rgba8_field(&self, key: &str) -> Option<Rgba8>;
    }

    mod json_backend {
        use super::{Rgba8, ThemeColorFields};

        impl ThemeColorFields for serde_json::Value {
            fn rgba8_field(&self, key: &str) -> Option<Rgba8> {
                let arr = self.get(key)?.as_array()?;
                Some(Rgba8 {
                    r: arr.first()?.as_u64().unwrap_or(0).min(255) as u8,
                    g: arr.get(1)?.as_u64().unwrap_or(0).min(255) as u8,
                    b: arr.get(2)?.as_u64().unwrap_or(0).min(255) as u8,
                    a: arr.get(3).and_then(|x| x.as_u64()).unwrap_or(255).min(255) as u8,
                })
            }
        }
    }

    /// 🎨️ Replaces a paint when the selected theme field supplies color bytes.
    pub fn merge_color_field(next: &mut Color, fields: &impl ThemeColorFields, key: &str) {
        if let Some(rgba) = fields.rgba8_field(key) {
            *next = Color::from_rgba8(rgba.r, rgba.g, rgba.b, rgba.a);
        }
    }

    /// 🌓️ Returns whether a canvas clear color reads as a light background.
    pub fn clear_is_light(clear: Color) -> bool {
        let [r, g, b, _] = clear.components();
        0.2126 * f64::from(r) + 0.7152 * f64::from(g) + 0.0722 * f64::from(b) > 0.5
    }

    /// 🎨️ Checkerboard cell shades for transparent raster layers.
    pub fn checkerboard_shades_for_clear(clear: Color) -> (u8, u8) {
        if clear_is_light(clear) {
            (220, 180)
        } else {
            (64, 48)
        }
    }
    #[cfg(all(test, not(all(target_arch = "wasm32", target_env = "p2"))))]
    include!("../🧪️tests/🎨️theme-color-fields/🦀️.rs");
    // #endregion theme
}

#[path = "🖼️svg-icon/🦀️.rs"]
pub mod svg_icon;

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl SvgDocument {
    /// 🏷️ Parses icon SVG with bundled emoji font options.
    pub fn parse_icons(svg: &str) -> Result<Self, CanvasError> {
        let tree = usvg::Tree::from_str(svg, svg_icon::usvg_options_icons()).map_err(|e| CanvasError::SvgParse(e.to_string()))?;
        Ok(Self::from_tree(tree))
    }

    /// 📐️ Visible content bounds in absolute SVG space.
    pub fn content_bounds(&self) -> (f64, f64, f64, f64) {
        svg_icon::svg_icon_content_bounds(&self.0)
    }

    /// 🏷️ Renders themed icon paints into a scene.
    pub fn render_themed(&self, scene: &mut Scene, fg: Color, bg: Color) {
        svg_icon::render_svg_tree_themed(scene, &self.0, fg, bg);
    }

    /// 🏷️ Renders literal SVG paints into a scene.
    pub fn render_literal(&self, scene: &mut Scene) {
        svg_icon::render_svg_tree_literal(scene, &self.0);
    }
}

#[path = "🏷️text/🦀️.rs"]
pub mod text;

// #region 🔖️Raster
pub mod raster {
    use super::{Affine, RasterImage, Scene};
    use std::collections::HashMap;
    use std::sync::Arc;

    pub fn draw_image(scene: &mut Scene, image: &RasterImage, transform: Affine) {
        scene.draw_image(image, transform);
    }

    pub fn draw_image_arc(scene: &mut Scene, image: &Arc<RasterImage>, transform: Affine) {
        scene.draw_image(image, transform);
    }

    #[derive(Clone, Default)]
    pub struct RasterImageCache {
        entries: HashMap<String, Arc<RasterImage>>,
    }

    impl RasterImageCache {
        pub fn get(&self, key: &str) -> Option<Arc<RasterImage>> {
            self.entries.get(key).cloned()
        }

        /// 🧹️ Releases ONE cached decode per call — the bounded drain a host retirement walks so a
        /// surface with a thousand uploaded layers never frees them all inside one frame turn.
        pub fn close_step(&mut self) -> bool {
            let Some(key) = self.entries.keys().next().cloned() else { return true };
            self.entries.remove(&key);
            false
        }

        pub fn is_empty(&self) -> bool {
            self.entries.is_empty()
        }

        pub fn insert(&mut self, key: String, image: RasterImage) -> Arc<RasterImage> {
            let arc = Arc::new(image);
            self.entries.insert(key, arc.clone());
            arc
        }
    }
}
// #endregion 🔖️Raster

// #region 🔖️Render
pub mod render {
    use super::{Affine, Scene};

    /// 📐️ Scales a logical-viewport scene to the physical GPU surface (device pixel ratio).
    pub fn scale_scene_for_device_pixel_ratio(scene: Scene, dpr: f64) -> Scene {
        let scale = dpr.max(1.0);
        if (scale - 1.0).abs() < f64::EPSILON {
            return scene;
        }
        let mut scaled = Scene::new();
        scaled.append(&scene, Some(Affine::IDENTITY.scale(scale)));
        scaled
    }
}
// #endregion 🔖️Render

// #region 🔖️CanvasContent
pub mod canvas_content {
    use super::{Color, Scene};

    pub trait CanvasContent {
        fn build_scene(&self) -> Scene;
        fn clear_color(&self) -> Color;
    }
}
// #endregion 🔖️CanvasContent

// 🌉️ `target_arch = "wasm32"` is TRUE for `wasm32-wasip2` too; this is the browser WebGPU canvas
// session shared by every surface/editor session bridge, so it is narrowed to exclude the WASI
// component target (which attaches no `HtmlCanvasElement`).
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
#[path = "🖥️gpu-session/🦀️.rs"]
pub mod gpu_session;

// #region 🧪️Tests
#[cfg(test)]
#[path = "../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[cfg(test)]
#[path = "../🧪️tests/🏷️label-fit/🦀️.rs"]
mod label_fit_laws;


#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
// #endregion 🧪️Tests
