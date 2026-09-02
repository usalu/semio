//! 🛍️ Raster play app panel — the layer-kind catalogue.

use crate::editor::raster::terminology::RasterPlayLabels;
use semio_framework_plugin::{tree_item_desc, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_CATALOGUE: &str = "raster.play.catalogue";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(RASTER_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(labels: &RasterPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let rows = crate::editor::raster::ui_node_list([
        tree_item_desc("raster-catalogue.pixel", labels.catalogue_pixel, None)?,
        tree_item_desc("raster-catalogue.group", labels.catalogue_group, None)?,
        tree_item_desc("raster-catalogue.adjustment", labels.catalogue_adjustment, None)?,
    ])?;
    PanelTreeBuilder::new("raster-catalogue")?.section("raster-catalogue.layer-kinds", Some(labels.layer_kinds.into()), true, rows)?.build()
}
//#endregion 🔖️Render
