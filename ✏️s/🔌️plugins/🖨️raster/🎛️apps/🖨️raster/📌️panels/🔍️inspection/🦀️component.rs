//! 🔍️ Raster play app panel — the selected-layer(s) inspector.

use crate::apps::raster::config::RasterConfig;
use crate::apps::raster::terminology::RasterPlayLabels;
use crate::artifacts::raster::engine::{find_layer, layer_name, layer_opacity};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{
    ui_inspector_groups_to_tree, ui_inspector_mixed_number, ui_inspector_mixed_text, ui_inspector_readonly_field, ui_stack_vertical, ui_text, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiInspectorFieldGroup, UiNode,
    UiPresence, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_PROPERTIES: &str = "raster.play.properties";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"), group: PanelGroup::Details, body_key: Some(RASTER_PLAY_BODY_PROPERTIES.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &RasterDocument, runtime: &RasterConfig, labels: &RasterPlayLabels) -> UiNode {
    let selected = &runtime.selected_ids;
    let layers: Vec<&RasterLayerNode> = selected.iter().filter_map(|id| find_layer(&document.layers, id)).collect();
    if layers.is_empty() {
        return ui_stack_vertical(vec![ui_text(Label::data(format!("{}: {}", labels.schema_prefix.as_str(), document.schema))), ui_text(Label::data(format!("{}: {} @ {}", labels.brush_prefix.as_str(), runtime.brush_size, runtime.brush_opacity)))]);
    }
    let names: Vec<String> = layers.iter().map(|layer| layer_name(layer).into()).collect();
    let opacities: Vec<f64> = layers.iter().map(|layer| layer_opacity(layer) as f64).collect();
    let mixed_name = ui_inspector_mixed_text(&names);
    let mixed_opacity = ui_inspector_mixed_number(&opacities);
    ui_inspector_groups_to_tree(&[UiInspectorFieldGroup {
        presence: UiPresence::default(),
        id: "raster-properties.layer".into(),
        label: labels.layer.into(),
        default_open: Some(true),
        fields: vec![
            ui_inspector_readonly_field("raster-properties.name", labels.name, mixed_name.placeholder.unwrap_or(mixed_name.value)),
            ui_inspector_readonly_field("raster-properties.opacity", labels.opacity, if mixed_opacity.uniform { mixed_opacity.value.to_string() } else { labels.mixed.into() }),
        ],
    }])
}
//#endregion 🔖️Render
