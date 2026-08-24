//! 🗂️ Draw play app panel — the layer tree (constitutional: was `ui`'s `Panels` region, layers half).

use crate::artifacts::draw::schema::{draw_play_boolean_child_row_id, draw_play_layers_tree_row_id, find_draw_layer, layer_base};
use crate::artifacts::draw::{DrawLayerNode, DrawSnapshot};
use crate::editor::draw::terminology::DrawPlayLabels;
use crate::editor::draw::{draw_play_action, DRAW_INTERACTION_DOMAIN};
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

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

/// 🕹️ No per-row selection `action`: the tree is bound to the `strokes` interaction domain via
/// `.interaction_domain(...)?` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn layer_tree_item(doc: &DrawSnapshot, layer: &DrawLayerNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = draw_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let nested_items = match layer {
        DrawLayerNode::Group(group) => Some(crate::editor::draw::ui_node_list(group.children.iter().map(|child| layer_tree_item(doc, child)))?),
        DrawLayerNode::Boolean(boolean) => Some(crate::editor::draw::ui_node_list(boolean.children.iter().map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id)))?),
        _ => None,
    };
    let description = match layer {
        DrawLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    };
    let mut drag_data = semio_framework_plugin::UiFixedMap::default();
    let drag_key = semio_framework_plugin::UiText::try_from_str("application/x-semio-draw-layer-id")
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-key", "fixed drag key admission failed"))?;
    let drag_value = semio_framework_plugin::UiText::try_from_str(&base.id)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-value", "fixed drag value admission failed"))?;
    drag_data.try_push(drag_key, drag_value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-data", "fixed drag-data admission failed"))?;
    let mut builder = semio_framework_ui_contract::tree_item(Label::data(base.name.clone()))?
        .try_id(row_id)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.id", "fixed layer id admission failed"))?
        .description(semio_framework_plugin::UiText::try_from_str(&description).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.description", "fixed layer description admission failed"))?)
        .icon(semio_framework_plugin::UiText::try_from_str(layer_icon(layer)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?)
        .default_open(matches!(layer, DrawLayerNode::Group(_)))
        .draggable(true)
        .drag_data(drag_data)
        .dimmed(!base.visible);
    if let Some(children) = nested_items {
        builder = builder.try_children(children).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.children", "fixed layer child admission failed"))?;
    }
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.build", "fixed layer admission failed"))
}

fn boolean_child_item(doc: &DrawSnapshot, boolean_id: &str, child_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = draw_play_boolean_child_row_id(boolean_id, child_id);
    let mut item = match find_draw_layer(doc, child_id) {
        Some(child) => tree_item(row_id, Label::data(layer_base(child).name.clone()))?,
        None => tree_item(row_id, Label::data(format!("{child_id} (missing)")))?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.draggable = Some(false);
        if let Some(child) = find_draw_layer(doc, child_id) {
            props.description = Some(semio_framework_plugin::UiText::try_from_str(&crate::artifacts::draw::schema::layer_kind_label(child)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.description", "fixed layer description admission failed"))?);
        } else {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str("alert-circle").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
        }
    }
    Ok(item)
}

fn tree_button(id: &str, label: impl TryInto<Label>, icon: &str, action: &str, args: semio_framework_plugin::UiValue) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_with_action(id, label, None, draw_play_action(action, Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
    }
    Ok(item)
}

pub async fn render(document: &DrawSnapshot, labels: &DrawPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action_items = crate::editor::draw::ui_node_list([
        tree_button("draw-play-layers.add.path", labels.add_path, "pen-tool", "addLayer", crate::editor::draw::ui_value_map([("kind", crate::editor::draw::ui_value_text("path")?)])?),
        tree_button("draw-play-layers.add.rect", labels.add_rectangle, "square", "addLayer", crate::editor::draw::ui_value_map([("kind", crate::editor::draw::ui_value_text("shape:rect")?)])?),
        tree_button("draw-play-layers.add.text", labels.add_text, "type", "addLayer", crate::editor::draw::ui_value_map([("kind", crate::editor::draw::ui_value_text("text")?)])?),
        tree_button("draw-play-layers.add.group", labels.add_group, "folder-plus", "addLayer", crate::editor::draw::ui_value_map([("kind", crate::editor::draw::ui_value_text("group")?)])?),
        tree_button("draw-play-layers.add.boolean", labels.add_boolean, "combine", "addLayer", crate::editor::draw::ui_value_map([("kind", crate::editor::draw::ui_value_text("boolean")?)])?),
    ])?;
    let layer_items = if document.layers.is_empty() {
        let mut empty = tree_item("draw-play-layers.empty", labels.empty_state)?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut empty.component {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str("pen-tool").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
        }
        crate::editor::draw::ui_node_list([Ok(empty)])?
    } else {
        crate::editor::draw::ui_node_list(document.layers.iter().map(|layer| layer_tree_item(document, layer)))?
    };
    // 🕹️ `.interaction_domain(...)?` replaces the deleted `.selected()?`/`.highlighted()?`/
    // `.selection_change(...)` calls — the framework stamps presence from `InteractionState` and
    // would overwrite them anyway (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    let mut items = semio_framework_plugin::UiFixedList::default();
    for item in action_items.into_iter().chain(layer_items) {
        items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.items", "fixed layer-list admission failed"))?;
    }
    PanelTreeBuilder::new("draw-play-layers")?.section("draw-play-layers", Some(Label::data(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)), true, items)?.interaction_domain(DRAW_INTERACTION_DOMAIN)?.build()
}
//#endregion 🔖️Render
