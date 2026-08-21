//! 🖼️ Raster play app — the composite window: the main paintable 2D surface.

use crate::artifacts::raster::RasterSnapshot as RasterDocument;
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::modes::edit::windows::composite::options;
use crate::editor::raster::raster_scene;
use semio_framework_plugin::{build_paint_2d_scene, LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowMeasure, WindowOptions};

//#region 🔖️Constants
pub const RASTER_PLAY_WINDOW_COMPOSITE: &str = "raster-composite";
pub const RASTER_PLAY_BODY_COMPOSITE: &str = "raster.play.composite";
const RASTER_PLAY_SURFACE_COMPOSITE: &str = "raster.play.composite";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::raster::create_raster_app`. `options.measures`
/// stays empty here on purpose: raster's measures are config-derived and rebuilt per frame by
/// [`window_measures`], not frozen into the manifest.
pub async fn definition() -> WindowKindDefinition {
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

/// 🎚️ The live chrome measures for this window, collected from its `🎚️options/*` components.
pub async fn window_measures(config: &RasterConfig) -> Vec<WindowMeasure> {
    vec![options::brush::measure(config), options::eraser::measure(config)]
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub async fn render(document: &RasterDocument, config: &RasterConfig) -> UiNode {
    build_paint_2d_scene(RASTER_PLAY_SURFACE_COMPOSITE, crate::editor::raster::RASTER_PLAY_CONTROLLER_ID, raster_scene(document, config, config.active_utility_id.as_str(), "composite"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_paint2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, RASTER_PLAY_BODY_COMPOSITE);
        assert!(matches!(definition.surface_kind, SurfaceKind::Paint2d));
        assert!(definition.options.measures.is_empty(), "measures are config-derived per frame, never frozen into the manifest");
    }

    #[semio_framework_async_macros::async_test]
    async fn window_measures_surface_brush_and_eraser_groups() {
        let config = RasterConfig::default();
        let measures = window_measures(&config);
        assert_eq!(measures.len(), 2);
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintBrush")));
        assert!(measures.iter().any(|m| matches!(m, WindowMeasure::Group { id, .. } if id == "raster-utility-options-paintEraser")));
    }
}
//#endregion 🧪️Tests
