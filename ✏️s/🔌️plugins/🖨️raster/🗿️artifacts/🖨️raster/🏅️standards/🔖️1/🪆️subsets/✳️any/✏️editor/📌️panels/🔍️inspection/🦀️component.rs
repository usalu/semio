//! 🔍️ Raster play app panel — the selected-layer(s) inspector.

use crate::artifacts::raster::RasterSnapshot as RasterDocument;
use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(RASTER_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ The selected-layer(s) name/opacity summary used to read `RasterConfig.selected_ids` (deleted,
/// ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM); the `"layers"` domain's selection is
/// framework-owned `InteractionState` now, and `ArtifactEditor::render` is not threaded an
/// `InteractionView` this wave — dropped rather than shown stale (matches the acceptance-bar
/// precedent in lowpoly's inspection panel), always falling back to the schema+brush summary.
pub fn render(document: &RasterDocument, runtime: &RasterConfig, labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = crate::editor::raster::ui_node_list([
        tree_item_desc("raster-play-inspector.schema", labels.schema_prefix, Some(document.schema.clone()))?,
        tree_item_desc("raster-play-inspector.brush", labels.brush_prefix, Some(format!("{} @ {}", runtime.brush_size, runtime.brush_opacity)))?,
    ])?;
    PanelTreeBuilder::new("raster-play-inspector")?.section("raster-play-inspector.summary", Some(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL.into()), true, rows)?.build()
}
//#endregion 🔖️Render
