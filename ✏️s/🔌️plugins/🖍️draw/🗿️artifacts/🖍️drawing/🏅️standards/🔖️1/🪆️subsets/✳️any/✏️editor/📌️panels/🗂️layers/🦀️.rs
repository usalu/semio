//! 🗂️ Drawing play app panel — the layer tree (constitutional: was `ui`'s `Panels` region, layers half).

use crate::artifacts::drawing::schema::{drawing_play_boolean_child_row_id, drawing_play_layers_tree_row_id, find_drawing_layer, layer_base};
use crate::artifacts::drawing::{DrawingLayerNode, DrawingSnapshot};
use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::editor::drawing::{drawing_play_action, DRAWING_INTERACTION_DOMAIN};
use semio_framework_plugin::{tree_item, tree_item_with_action, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

pub const DRAWING_PLAY_BODY_LAYERS: &str = "drawing.play.layers";
pub const DRAWING_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-drawing-layer-kind";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAWING_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn layer_icon(layer: &DrawingLayerNode) -> &str {
    match layer {
        DrawingLayerNode::Group(_) => "folder",
        DrawingLayerNode::Boolean(_) => "combine",
        DrawingLayerNode::Trace(_) => "scan-line",
        DrawingLayerNode::Path(_) => "pen-tool",
        DrawingLayerNode::Shape(_) => "square",
        DrawingLayerNode::Text(_) => "type",
        DrawingLayerNode::Image(_) => "image",
    }
}

/// 🕹️ No per-row selection `action`: the tree is bound to the `strokes` interaction domain via
/// `.interaction_domain(...)?` below, so the framework auto-injects `interactionSelect` for row
/// clicks — never declare that yourself (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
fn layer_tree_item(doc: &DrawingSnapshot, layer: &DrawingLayerNode) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = drawing_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let nested_items = match layer {
        DrawingLayerNode::Group(group) => Some(crate::editor::drawing::ui_node_list(group.children.iter().map(|child| layer_tree_item(doc, child)))?),
        DrawingLayerNode::Boolean(boolean) => Some(crate::editor::drawing::ui_node_list(boolean.children.iter().map(|child_id| boolean_child_item(doc, &boolean.base.id, child_id)))?),
        _ => None,
    };
    let description = match layer {
        DrawingLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    };
    let mut drag_data = semio_framework_plugin::UiFixedMap::default();
    let drag_key = semio_framework_plugin::UiText::try_from_str("application/x-semio-drawing-layer-id")
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-key", "fixed drag key admission failed"))?;
    let drag_value = semio_framework_plugin::UiText::try_from_str(&base.id)
        .ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-value", "fixed drag value admission failed"))?;
    drag_data.try_push(drag_key, drag_value).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.drag-data", "fixed drag-data admission failed"))?;
    let label = semio_framework_ui_contract::Label::try_from(base.name.clone())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.label", "fixed layer label admission failed"))?;
    let mut builder = semio_framework_ui_contract::tree_item(label)
        .try_id(row_id)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.id", "fixed layer id admission failed"))?
        .description(semio_framework_plugin::UiText::try_from_str(&description).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.description", "fixed layer description admission failed"))?)
        .icon(semio_framework_plugin::UiText::try_from_str(layer_icon(layer)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?)
        .default_open(matches!(layer, DrawingLayerNode::Group(_)))
        .draggable(true)
        .drag_data(drag_data)
        .dimmed(!base.visible);
    if let Some(children) = nested_items {
        builder = builder.try_children(children).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.children", "fixed layer child admission failed"))?;
    }
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.build", "fixed layer admission failed"))
}

fn boolean_child_item(doc: &DrawingSnapshot, boolean_id: &str, child_id: &str) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let row_id = drawing_play_boolean_child_row_id(boolean_id, child_id);
    let mut item = match find_drawing_layer(doc, child_id) {
        Some(child) => tree_item(row_id, layer_base(child).name.clone())?,
        None => tree_item(row_id, format!("{child_id} (missing)"))?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.draggable = Some(false);
        if let Some(child) = find_drawing_layer(doc, child_id) {
            props.description = Some(semio_framework_plugin::UiText::try_from_str(&crate::artifacts::drawing::schema::layer_kind_label(child)).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.description", "fixed layer description admission failed"))?);
        } else {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str("alert-circle").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
        }
    }
    Ok(item)
}

fn tree_button(id: &str, label: &str, icon: &str, action: &str, args: semio_framework_plugin::UiValue) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let mut item = tree_item_with_action(id, label, None, drawing_play_action(action, Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(semio_framework_plugin::UiText::try_from_str(icon).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
    }
    Ok(item)
}

pub fn render(document: &DrawingSnapshot, labels: &DrawingPlayLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let action_items = crate::editor::drawing::ui_node_list([
        tree_button("drawing-play-layers.add.path", labels.add_path.as_str(), "pen-tool", "addLayer", crate::editor::drawing::ui_value_map([("kind", crate::editor::drawing::ui_value_text("path")?)])?),
        tree_button("drawing-play-layers.add.rect", labels.add_rectangle.as_str(), "square", "addLayer", crate::editor::drawing::ui_value_map([("kind", crate::editor::drawing::ui_value_text("shape:rect")?)])?),
        tree_button("drawing-play-layers.add.text", labels.add_text.as_str(), "type", "addLayer", crate::editor::drawing::ui_value_map([("kind", crate::editor::drawing::ui_value_text("text")?)])?),
        tree_button("drawing-play-layers.add.group", labels.add_group.as_str(), "folder-plus", "addLayer", crate::editor::drawing::ui_value_map([("kind", crate::editor::drawing::ui_value_text("group")?)])?),
        tree_button("drawing-play-layers.add.boolean", labels.add_boolean.as_str(), "combine", "addLayer", crate::editor::drawing::ui_value_map([("kind", crate::editor::drawing::ui_value_text("boolean")?)])?),
    ])?;
    let layer_items = if document.layers.is_empty() {
        let mut empty = tree_item("drawing-play-layers.empty", labels.empty_state.as_str())?;
        if let semio_framework_plugin::Component::TreeItem(props) = &mut empty.component {
            props.icon = Some(semio_framework_plugin::UiText::try_from_str("pen-tool").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("ui.layer.icon", "fixed layer icon admission failed"))?);
        }
        crate::editor::drawing::ui_node_list([Ok(empty)])?
    } else {
        crate::editor::drawing::ui_node_list(document.layers.iter().map(|layer| layer_tree_item(document, layer)))?
    };
    // 🕹️ `.interaction_domain(...)?` replaces the deleted `.selected()?`/`.highlighted()?`/
    // `.selection_change(...)` calls — the framework stamps presence from `InteractionState` and
    // would overwrite them anyway (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
    let mut items = semio_framework_plugin::UiFixedList::default();
    for item in action_items.into_iter().chain(layer_items) {
        items.try_push(item).map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.items", "fixed layer-list admission failed"))?;
    }
    let section = semio_framework_ui_contract::Label::try_from(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL)
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("ui.layer.section-label", "fixed layer section label admission failed"))?;
    PanelTreeBuilder::new("drawing-play-layers")?.section("drawing-play-layers", Some(section), true, items)?.interaction_domain(DRAWING_INTERACTION_DOMAIN)?.build()
}
//#endregion 🔖️Render
