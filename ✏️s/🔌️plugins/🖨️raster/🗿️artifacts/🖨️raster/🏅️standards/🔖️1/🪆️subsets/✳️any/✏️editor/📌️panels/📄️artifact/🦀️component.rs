//! 📄️ Raster play app panel — the layer tree.

use crate::editor::raster::config::RasterConfig;
use crate::editor::raster::terminology::RasterPlayLabels;
use crate::editor::raster::{layer_row_id, raster_action, RASTER_TREE_PREFIX};
use crate::artifacts::raster::schema::layer_name;
use crate::artifacts::raster::schema::layer_visible;
use crate::artifacts::raster::{RasterLayerNode, RasterSnapshot as RasterDocument};
use semio_framework_plugin::{tree_item_desc, tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

//#region 🔖️Constants
pub const RASTER_PLAY_BODY_LAYERS: &str = "raster.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(RASTER_PLAY_BODY_LAYERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn layer_tree_item(layer: &RasterLayerNode) -> UiTreeItemNode {
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
        ..tree_item_desc(layer_row_id(layer), Label::data(layer_name(layer)), Some(description.into()))
    }
}

/// 🕹️ `runtime` is unused now — layer selection/hover moved into the framework-owned `"layers"`
/// interaction domain (granularity `"layer"`, ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-
/// MECHANISM); `.interaction_domain("layers")` below has the framework's renderer translate row
/// clicks into injected `interactionSelect` and stamp presence from `InteractionState`, replacing the
/// deleted `.selected()`/`.highlighted()`/`.selection_change()` calls (row ids ARE the domain's ids —
/// this tree is the sole consumer of the `"layers"` domain today).
pub async fn render(document: &RasterDocument, _runtime: &RasterConfig, labels: &RasterPlayLabels) -> UiNode {
    let action_rows = vec![
        UiTreeItemNode { icon_id: Some("image".into()), ..tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.pixel"), labels.add_pixel, None, raster_action("addLayer", Some(json!({ "kind": "pixel" })))) },
        UiTreeItemNode { icon_id: Some("folder-plus".into()), ..tree_item_with_action(format!("{RASTER_TREE_PREFIX}.add.group"), labels.add_group, None, raster_action("addLayer", Some(json!({ "kind": "group" })))) },
    ];
    let layer_items: Vec<UiTreeItemNode> = document.layers.iter().map(layer_tree_item).collect();
    PanelTreeBuilder::new(RASTER_TREE_PREFIX)
        .section(RASTER_TREE_PREFIX, Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, [action_rows, layer_items].concat())
        .interaction_domain("layers")
        .build()
}
//#endregion 🔖️Render
