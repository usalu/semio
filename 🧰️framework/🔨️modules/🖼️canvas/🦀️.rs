//! 🖼️ Application-neutral tile-based infinite canvas; extend via `CanvasExtension`. A thin facade over the producer crates: the scene,
//! the camera, the icon codec and the runtime asset port.

// #region 🔖️Reexports
pub use semio_framework_canvas_assets::{icon_pack, install_runtime_assets, FontRole, FontSet, IconPack, RuntimeAsset, RuntimeAssets, RuntimeAssetsInstallError};
pub use semio_framework_canvas_camera::{camera, lod};
pub use semio_framework_canvas_icons::{decode_icon, encode_icon, icon_codec, resolve_icon_kind, Icon, IconName, MetabolismIconName, ResolvedIcon, ThemedSvgLookup};
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub use semio_framework_canvas_scene::gpu_session;
pub use semio_framework_canvas_scene::{
    advance_opaque_scene_retirement, append_shape_to_path, canvas_content, draw_list, geom_sel, opaque_scene_retirement_status, publish_opaque_scene_retirement, raster, render, reserve_opaque_scene_retirement, svg_icon, text, theme, Affine, Arc,
    BezPath, BlendMode, CanvasError, CanvasPathRetirement, CanvasSceneRetirement, CanvasStrokeRetirement, Cap, Circle, Color, CubicBez, FillRule, Line, OpaqueSceneRetirementStep, OpaqueSceneRetirementToken, Paint, PathEl, Point, RasterImage, Rect,
    Rgba8, RoundedRect, RoundedRectRadii, Scene, ShapeRef, Stroke, Vec2,
};
#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
pub use semio_framework_canvas_scene::{append_svg_document, native_render, SvgDocument};
#[cfg(feature = "testkit")]
pub use semio_framework_canvas_assets::testkit;
// #endregion 🔖️Reexports

// #region 🔖️CanvasExtension
/// 🧩️ Extension hook for domain-specific canvas behavior (hit-test, paint, kinds).
pub trait CanvasExtension: Send + Sync {
    fn extension_id(&self) -> &str;
}

/// ⚙️ Generic infinite-canvas engine shell; domain logic lives in `E`.
pub struct CanvasEngine<E: CanvasExtension> {
    pub extension: E,
}

impl<E: CanvasExtension> CanvasEngine<E> {
    pub fn new(extension: E) -> Self {
        Self { extension }
    }
}
// #endregion 🔖️CanvasExtension

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🎬️draw-list/🦀️.rs"]
mod draw_list_tests;
// #endregion 🔖️Tests
