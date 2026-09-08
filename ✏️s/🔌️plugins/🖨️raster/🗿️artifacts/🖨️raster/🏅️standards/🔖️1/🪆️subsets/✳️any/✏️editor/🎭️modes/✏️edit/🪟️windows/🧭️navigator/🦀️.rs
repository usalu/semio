//! 🧭️ Raster play app — the navigator window: the small overview/minimap surface.

use crate::RasterSnapshot as RasterDocument;
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::raster_scene;
use semio_framework_plugin::plugin_app_close_prelude::SurfaceKind as ContractSurfaceKind;
use semio_framework_plugin::{scene_surface, BuiltNode, LocalizedLabel, SurfaceKind, UiAssemblyResult, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const RASTER_PLAY_WINDOW_NAVIGATOR: &str = "raster-navigator";
pub const RASTER_PLAY_BODY_NAVIGATOR: &str = "raster.play.navigator";
const RASTER_PLAY_SURFACE_NAVIGATOR: &str = "raster.play.navigator";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the app manifest by `crate::editor::raster::create_raster_app`. No `☑️options` node:
/// the navigator has no live chrome measures of its own.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: RASTER_PLAY_WINDOW_NAVIGATOR.into(),
        label: LocalizedLabel::native("Navigator", "Navigator"),
        body_key: RASTER_PLAY_BODY_NAVIGATOR.into(),
        surface_kind: SurfaceKind::Paint2d,
        icon_id: "focus".into(),
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
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🎬️ Same `Paint2dScene` payload as the composite window under this window's own surface id — encoded
/// behind the semantic surface contract (see the composite window's note on the dropped controller id).
pub fn render(document: &RasterDocument, config: &RasterConfig) -> UiAssemblyResult<BuiltNode> {
    scene_surface(RASTER_PLAY_SURFACE_NAVIGATOR, ContractSurfaceKind::Paint2d, &raster_scene(document, config, config.active_utility_id.as_str(), "navigator"))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn definition_declares_the_paint2d_surface_and_body_key() {
        let definition = definition();
        assert_eq!(definition.body_key, RASTER_PLAY_BODY_NAVIGATOR);
        assert!(matches!(definition.surface_kind, SurfaceKind::Paint2d));
    }
}
//#endregion 🧪️Tests
