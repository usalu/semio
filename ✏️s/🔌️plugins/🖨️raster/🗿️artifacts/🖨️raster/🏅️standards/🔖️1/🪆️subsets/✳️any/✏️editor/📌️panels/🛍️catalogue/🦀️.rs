//! 🛍️ Raster play app panel — the layer-kind catalogue.

use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::ui_label;
use semio_framework_plugin::{
    tree_item_desc, BuiltNode, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, TreeWindows, UiAssemblyResult, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};

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
/// 🛍️ The fixed layer-kind roster, windowed like any other section so the host owns its open state and
/// the panel never truncates. Deliberately bound to NO interaction domain: a catalogue row is an
/// offer, not a document target.
pub fn render(labels: &RasterPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let rows: [(&str, LabelText); 3] = [("raster-catalogue.pixel", labels.catalogue_pixel), ("raster-catalogue.group", labels.catalogue_group), ("raster-catalogue.adjustment", labels.catalogue_adjustment)];
    PanelTreeBuilder::new("raster-catalogue")?.window_section(windows, "raster-catalogue.layer-kinds", Some(ui_label(labels.layer_kinds.as_str())?), true, &rows, |row| tree_item_desc(row.0, ui_label(row.1.as_str())?, None))?.build()
}
//#endregion 🔖️Render
