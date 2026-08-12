//! 📄️ Raster play app panel — the layer tree.

use crate::apps::raster::config::RasterConfig;
use crate::apps::raster::terminology::RasterPlayLabels;
use crate::apps::raster::{layer_row_id, raster_action, RASTER_TREE_PREFIX};
use crate::artifacts::raster::schema::{find_layer, layer_name, layer_visible};
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(RASTER_PLAY_BODY_LAYERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn layer_tree_item(layer: &RasterLayerNode) -> UiTreeItemNode {
    let nested = match layer {
        RasterLayerNode::Group { children, .. } => {
            if children.is_empty() {
                None
            } else {
                Some(children.iter().map(layer_tree_item).collect())
            }
        }
        _ => None,
    };
    let description = match layer {
        RasterLayerNode::Pixel { .. } => "pixel",
        RasterLayerNode::Group { .. } => "group",
        RasterLayerNode::Adjustment { .. } => "adjustment",
    };
    let icon_id = match layer {
        RasterLayerNode::Pixel { .. } => "image",
        RasterLayerNode::Group { .. } => "folder",
        RasterLayerNode::Adjustment { .. } => "sliders-horizontal",
    };
    UiTreeItemNode {
        icon_id: Some(icon_id.into()),
        default_open: Some(matches!(layer, RasterLayerNode::Group { .. })),
        draggable: Some(true),
        items: nested,
        dimmed: if layer_visible(layer) { None } else { Some(true) },
        ..tree_item_with_action(layer_row_id(layer), Label::data(layer_name(layer)), Some(description.into()), raster_action("setSelection", Some(json!({ "ids": [crate::artifacts::raster::schema::layer_node_id(layer)] }))))
    }
}

pub fn render(document: &RasterDocument, runtime: &RasterConfig, labels: &RasterPlayLabels) -> UiNode {
    let action_rows = vec![
        UiTreeItemNode { icon_id: Some("image".into()), ..tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.pixel"), labels.add_pixel, None, raster_action("addLayer", Some(json!({ "kind": "pixel" })))) },
        UiTreeItemNode { icon_id: Some("folder-plus".into()), ..tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.group"), labels.add_group, None, raster_action("addLayer", Some(json!({ "kind": "group" })))) },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    let selected_ids: Vec<String> = runtime.selected_ids.iter().filter_map(|id| find_layer(&document.layers, id).map(layer_row_id)).collect();
    let highlighted_ids: Vec<String> = runtime.hovered_id.as_deref().and_then(|id| find_layer(&document.layers, id)).map(|layer| vec![layer_row_id(layer)]).unwrap_or_default();
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section(RASTER_TREE_PREFIX, Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, [action_rows, layer_items].concat())
        .selected(selected_ids)
        .highlighted(highlighted_ids)
        .selection_change(raster_action("setSelection", None))
        .build()
}
//#endregion 🔖️Render
