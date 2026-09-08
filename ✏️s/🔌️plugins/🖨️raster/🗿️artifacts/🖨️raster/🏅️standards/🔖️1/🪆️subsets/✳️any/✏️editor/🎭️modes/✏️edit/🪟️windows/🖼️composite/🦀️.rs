//! 🖼️ Raster play app — the composite window: the main paintable 2D surface.

use crate::RasterSnapshot as RasterDocument;
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::modes::edit::windows::composite::options;
use crate::editor::raster::raster_scene;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
pub const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::raster::create_raster_app`. `options.measures`
/// stays empty here on purpose: raster's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: RASTER_PLAY_WINDOW_COMPOSITE.into(),
        label: LocalizedLabel::native("Composite", "Komposit"),
        body_key: RASTER_PLAY_BODY_COMPOSITE.into(),
        surface_kind: SurfaceKind::Paint2d,
        icon_id: "image".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
        interactions: Vec::new(),
    }
}

/// 🎚️ The live chrome measures for this window, collected from its `☑️options/*` components.
pub fn window_measures(config: &RasterConfig) -> Vec<WindowMeasure> {
    vec![options::brush::measure(config), options::eraser::measure(config)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Encodes the shared `Paint2dScene` behind the semantic surface contract — the app's controller is
/// resolved by the host from the owning app instance now, so the surface node carries only the scene.
pub fn render(document: &RasterDocument, config: &RasterConfig) -> UiAssemblyResult<BuiltNode> {
    scene_surface(RASTER_PLAY_SURFACE_COMPOSITE, ContractSurfaceKind::Paint2d, &raster_scene(document, config, config.active_utility_id.as_str(), "composite"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
