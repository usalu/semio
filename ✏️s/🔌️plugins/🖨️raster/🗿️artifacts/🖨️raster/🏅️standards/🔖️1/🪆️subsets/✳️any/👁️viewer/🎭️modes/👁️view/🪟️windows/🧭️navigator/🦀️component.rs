//! 🧭️ Raster viewer — the Navigator window: a read-only minimap of the same composited document the
//! Composite window shows. The editor's own navigator window (`✏️editor/…/🪟️windows/🧭️navigator`) has
//! no `🎚️options`/utilities/interactions scoped to it — a purely read-only minimap already, ported here
//! rather than dropped (this file imports nothing from the sibling editor surface,
//! `policyViewerPurityBreaches` forbids it outright). Reuses `ImageWindowKit::render` (contract §2.6)
//! for the actual pixel payload, under its own distinct window kind id (`ImageWindowKit::window_kind()`
//! is reserved for the Composite window — reusing it verbatim here would collide on id/body_key).

use crate::artifacts::raster::RasterSnapshot;
use semio_framework_plugin::app::{ImageWindowKit, WindowKit};
use semio_framework_plugin::{LocalizedLabel, SurfaceKind, UiNode, WindowKindDefinition, WindowOptions};

//#region 🔖️Constants
pub const RASTER_VIEW_WINDOW_NAVIGATOR: &str = "raster-view-navigator";
pub const RASTER_VIEW_BODY_NAVIGATOR: &str = "raster.view.navigator";
//#endregion 🔖️Constants

//#region 🔖️Definition
/// 🧱️ Stitched into the viewer manifest by `crate::viewer::raster::create_raster_viewer`. Own distinct
/// id/body_key (`ImageWindowKit::window_kind()` is already claimed by the Composite window in this
/// manifest) but the same `SurfaceKind::Canvas2d`/icon shape the kit itself declares.
pub fn definition() -> WindowKindDefinition {
    WindowKindDefinition {
        id: RASTER_VIEW_WINDOW_NAVIGATOR.into(),
        label: LocalizedLabel::native("Navigator", "Navigator"),
        body_key: RASTER_VIEW_BODY_NAVIGATOR.into(),
        surface_kind: SurfaceKind::Canvas2d,
        icon_id: "focus".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 👁️ Same real composited pixels the Composite window shows — a navigator/minimap is a scaled-down
/// view of the same content, not different content; the host renderer handles the scale-down
/// presentation, not this pure snapshot read.
pub fn render(document: &RasterSnapshot) -> UiNode {
    ImageWindowKit::render(&super::composite::composited_image_view(document))
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn definition_declares_a_canvas2d_navigator_window() {
        let def = definition();
        assert_eq!(def.id, RASTER_VIEW_WINDOW_NAVIGATOR);
        assert_eq!(def.body_key, RASTER_VIEW_BODY_NAVIGATOR);
        assert_eq!(def.surface_kind, SurfaceKind::Canvas2d);
    }

    #[test]
    fn render_produces_a_scene_node_for_the_default_document() {
        let document = crate::artifacts::raster::schema::empty_raster_document();
        let _node = render(&document);
    }
}
//#endregion 🧪️Tests
