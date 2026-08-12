//! 🗂️ Draw play app panel — the layer tree (constitutional: was `ui`'s `Panels` region, layers half).

use crate::apps::draw::config::DrawConfig;
use crate::apps::draw::terminology::DrawPlayLabels;
use crate::apps::draw::draw_play_action;
use crate::artifacts::draw::engine::{draw_play_boolean_child_row_id, draw_play_layers_tree_row_id, find_draw_layer, layer_base};
use crate::artifacts::draw::{DrawSnapshot, DrawLayerNode};
use semio_framework_plugin::{tree_item, tree_item_with_action, tree_item_with_action_draggable, Label, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};
use serde_json::json;

pub const DRAW_PLAY_BODY_LAYERS: &str = "draw.play.layers";
pub const DRAW_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-draw-layer-kind";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(DRAW_PLAY_BODY_LAYERS.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn layer_icon(layer: &DrawLayerNode) -> &str {
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

fn layer_tree_item(doc: &DrawSnapshot, layer: &DrawLayerNode) -> UiTreeItemNode {
    let row_id = draw_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let nested_items = match layer {
        DrawLayerNode::Group(group) => Some(group.children.iter().map(|child| layer_tree_item(doc, child)).collect()),
        DrawLayerNode::Boolean(boolean) => Some(boolean.children.iter().map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id)).collect()),
        _ => None,
    };
    let description = Some(match layer {
        DrawLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    });
    let action = draw_play_action("setSelection", Some(json!({ "ids": [base.id] })));
    let drag_data = json!({ "application/x-semio-draw-layer-id": base.id });
    UiTreeItemNode {
        icon_id: Some(layer_icon(layer).into()),
        default_open: Some(matches!(layer, DrawLayerNode::Group(_))),
        items: nested_items,
        dimmed: if base.visible { None } else { Some(true) },
        ..tree_item_with_action_draggable(row_id, Label::data(base.name.clone()), description, action, &drag_data)
    }
}

fn boolean_child_item(doc: &DrawSnapshot, boolean_id: &str, child_id: &str) -> UiTreeItemNode {
    let row_id = draw_play_boolean_child_row_id(boolean_id, child_id);
    match find_draw_layer(doc, child_id) {
        Some(child) => UiTreeItemNode { draggable: Some(false), ..tree_item_with_action(row_id, Label::data(layer_base(child).name.clone()), Some(crate::artifacts::draw::engine::layer_kind_label(child)), draw_play_action("setSelection", Some(json!({ "ids": [child_id] })))) },
        None => UiTreeItemNode { icon_id: Some("alert-circle".into()), draggable: Some(false), ..tree_item(row_id, Label::data(format!("{child_id} (missing)"))) },
    }
}

fn tree_button(id: &str, label: impl Into<Label>, icon: &str, action: &str, args: serde_json::Value) -> UiTreeItemNode {
    UiTreeItemNode { icon_id: Some(icon.into()), menu: None, ..tree_item_with_action(id, label, None, draw_play_action(action, Some(args))) }
}

pub fn render(document: &DrawSnapshot, interaction: &DrawConfig, labels: &DrawPlayLabels) -> UiNode {
    let action_items = vec![
        tree_button("draw-play-layers.add.path", labels.add_path, "pen-tool", "addLayer", json!({ "kind": "path" })),
        tree_button("draw-play-layers.add.rect", labels.add_rectangle, "square", "addLayer", json!({ "kind": "shape:rect" })),
        tree_button("draw-play-layers.add.text", labels.add_text, "type", "addLayer", json!({ "kind": "text" })),
        tree_button("draw-play-layers.add.group", labels.add_group, "folder-plus", "addLayer", json!({ "kind": "group" })),
        tree_button("draw-play-layers.add.boolean", labels.add_boolean, "combine", "addLayer", json!({ "kind": "boolean" })),
    ];
    let layer_items = if document.layers.is_empty() {
        vec![UiTreeItemNode { icon_id: Some("pen-tool".into()), menu: None, ..tree_item("draw-play-layers.empty", labels.empty_state) }]
    } else {
        document.layers.iter().map(|layer| layer_tree_item(document, layer)).collect()
    };
    let selected_tree_ids: Vec<String> = interaction.selected_ids.iter().filter_map(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id)).collect();
    let highlighted_ids: Vec<String> = interaction.hovered_id.as_ref().and_then(|id| find_draw_layer(document, id).map(draw_play_layers_tree_row_id)).into_iter().collect();
    let builder = PanelTreeBuilder::new("draw-play-layers")
        .section("draw-play-layers", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, action_items.into_iter().chain(layer_items).collect())
        .selected(selected_tree_ids)
        .selection_change(draw_play_action("setSelection", None));
    if highlighted_ids.is_empty() {
        builder.build()
    } else {
        builder.highlighted(highlighted_ids).build()
    }
}
//#endregion 🔖️Render
