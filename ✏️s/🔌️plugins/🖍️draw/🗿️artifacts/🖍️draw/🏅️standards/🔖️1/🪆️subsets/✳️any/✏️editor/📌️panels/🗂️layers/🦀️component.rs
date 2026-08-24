//! 🗂️ Draw play app panel — the layer tree (constitutional: was `ui`'s `Panels` region, layers half).

use crate::artifacts::draw::schema::{draw_play_boolean_child_row_id, draw_play_layers_tree_row_id, find_draw_layer, layer_base};
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};
use crate::editor::draw::terminology::DrawPlayLabels;
use crate::editor::draw::{draw_play_action, DRAW_INTERACTION_DOMAIN};
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;
use std::collections::HashMap;

pub const DRAW_PLAY_BODY_LAYERS: &str = "draw.play.layers";
pub const DRAW_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-draw-layer-kind";

//#region 🔖️Definition
pub async fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAW_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
async fn layer_icon(layer: &DrawLayerNode) -> &str {
    match layer {
        DrawLayerNode::Group(_) => "folder",
        DrawLayerNode::Boolean(_) => "combine",
        DrawLayerNode::Trace(_) => "scan-line",
        DrawLayerNode::Path(_) => "pen-tool",
        DrawLayerNode::Shape(_) => "square",
        DrawLayerNode::Text(_) => "type",
        DrawLayerNode::Image(_) => "image",
    }
}

/// 🕹️ No per-row selection `action`: the tree is bound to the `strokes` interaction domain via
/// `.interaction_domain(...)?` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
async fn layer_tree_item(doc: &DrawSnapshot, layer: &DrawLayerNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = draw_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let nested_items = match layer {
        DrawLayerNode::Group(group) => Some(group.children.iter().map(|child| layer_tree_item(doc, child)?).collect()),
        DrawLayerNode::Boolean(boolean) => Some(boolean.children.iter().map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id)).collect()),
        _ => None,
    };
    let description = Some(match layer {
        DrawLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    });
    let drag_data: HashMap<String, String> = [("application/x-semio-draw-layer-id".to_string(), base.id.clone())].into_iter().collect();
    UiTreeItemNode {
        description,
        icon_id: Some(layer_icon(layer).into()),
        default_open: Some(matches!(layer, DrawLayerNode::Group(_))),
        items: nested_items,
        dimmed: if base.visible { None } else { Some(true) },
        draggable: Some(true),
        drag_data: Some(drag_data),
        menu: None,
        ..tree_item(row_id, Label::data(base.name.clone()))?
    }
}

async fn boolean_child_item(doc: &DrawSnapshot, boolean_id: &str, child_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = draw_play_boolean_child_row_id(boolean_id, child_id);
    match find_draw_layer(doc, child_id) {
        Some(child) => UiTreeItemNode { description: Some(crate::artifacts::draw::schema::layer_kind_label(child)), draggable: Some(false), menu: None, ..tree_item(row_id, Label::data(layer_base(child).name.clone()))? },
        None => UiTreeItemNode { icon_id: Some("alert-circle".into()), draggable: Some(false), ..tree_item(row_id, Label::data(format!("{child_id} (missing)")))? },
    }
}

async fn tree_button(id: &str, label: impl Into<Label>, icon: &str, action: &str, args: serde_json::Value) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    UiTreeItemNode { icon_id: Some(icon.into()), menu: None, ..tree_item_with_action(id, label, None, draw_play_action(action, Some(args)))? }
}

pub async fn render(document: &DrawSnapshot, labels: &DrawPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action_items = vec![
        tree_button("draw-play-layers.add.path", labels.add_path, "pen-tool", "addLayer", json!({ "kind": "path" })),
        tree_button("draw-play-layers.add.rect", labels.add_rectangle, "square", "addLayer", json!({ "kind": "shape:rect" })),
        tree_button("draw-play-layers.add.text", labels.add_text, "type", "addLayer", json!({ "kind": "text" })),
        tree_button("draw-play-layers.add.group", labels.add_group, "folder-plus", "addLayer", json!({ "kind": "group" })),
        tree_button("draw-play-layers.add.boolean", labels.add_boolean, "combine", "addLayer", json!({ "kind": "boolean" })),
    ];
    let layer_items = if document.layers.is_empty() {
        vec![UiTreeItemNode { icon_id: Some("pen-tool".into()), menu: None, ..tree_item("draw-play-layers.empty", labels.empty_state)? }]
    } else {
        document.layers.iter().map(|layer| layer_tree_item(document, layer)?).collect()
    };
    // 🕹️ `.interaction_domain(...)?` replaces the deleted `.selected()?`/`.highlighted()?`/
    // `.selection_change(...)` calls — the framework stamps presence from `InteractionState` and
    // would overwrite them anyway (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    PanelTreeBuilder::new("draw-play-layers")?.section("draw-play-layers", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, action_items.into_iter().chain(layer_items).collect())?.interaction_domain(DRAW_INTERACTION_DOMAIN)?.build()
}
//#endregion 🔖️Render
