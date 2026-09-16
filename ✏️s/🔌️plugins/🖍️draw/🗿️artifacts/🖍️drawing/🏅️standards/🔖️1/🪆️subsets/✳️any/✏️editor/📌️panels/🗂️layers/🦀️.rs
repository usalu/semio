//! 🗂️ Drawing play app panel — the layer tree (constitutional: was `ui`'s `Panels` region, layers half).
//!
//! 🪟️ The tree is VIRTUALISED, never paged: the section and every nesting level (a `Group`'s children,
//! a `Boolean`'s operands) is a window container that stamps the FULL `total` of its logical child list
//! and materialises only the slice the host asked for (`TreeWindows::for_body`, threaded in from the
//! editor's `render_drawing_body`). There is no `+N` row and no truncation — a closed group costs one
//! stamped `total` and nothing else.
//!
//! 🕹️ The whole tree is bound to the `"strokes"` interaction domain, so a layer row declares only its
//! `granularity` and dispatches through the ONE tree-level `interactionSelect` binding
//! `PanelTreeBuilder::interaction_domain` stamps — never a per-row action or argument map
//! (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).

use crate::editor::drawing::terminology::DrawingPlayLabels;
use crate::editor::drawing::{drawing_play_action, ui_value_map, ui_value_text, DRAWING_INTERACTION_DOMAIN, DRAWING_INTERACTION_GRANULARITY, DRAWING_PLAY_CONTROLLER_ID};
use crate::schema::{drawing_play_boolean_child_row_id, drawing_play_layers_tree_row_id, find_drawing_layer, layer_base};
use crate::{DrawingLayerNode, DrawingSnapshot};
use semio_framework_plugin::{
    tree_item, tree_item_with_action, tree_window_item, Buildable, BuiltNode, HasBase, LabelText, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};

pub const DRAWING_PLAY_BODY_LAYERS: &str = "drawing.play.layers";
pub const DRAWING_LAYER_KIND_DRAG_MIME: &str = "application/x-semio-drawing-layer-kind";

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: semio_framework_plugin::LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(DRAWING_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🏷️ Admits one short drawing tree key — an icon key or an interaction granularity id.
fn ui_key(value: &str, stage: &'static str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", stage))
}

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

/// 🌳️ One row of the layer tree's single logical roster: the fixed "add" buttons first, then either
/// the document's top-level layers or the empty-state hint. ONE windowed section over the whole
/// roster, so the host scrolls a thousand-layer document without the panel ever truncating it.
enum LayersRow<'a> {
    Add(&'static str, LabelText, &'static str, &'static str),
    Empty,
    Layer(&'a DrawingLayerNode),
}

fn layers_rows<'a>(document: &'a DrawingSnapshot, labels: &DrawingPlayLabels) -> Vec<LayersRow<'a>> {
    let mut rows = vec![
        LayersRow::Add("drawing-play-layers.add.path", labels.add_path, "pen-tool", "path"),
        LayersRow::Add("drawing-play-layers.add.rect", labels.add_rectangle, "square", "shape:rect"),
        LayersRow::Add("drawing-play-layers.add.text", labels.add_text, "type", "text"),
        LayersRow::Add("drawing-play-layers.add.group", labels.add_group, "folder-plus", "group"),
        LayersRow::Add("drawing-play-layers.add.boolean", labels.add_boolean, "combine", "boolean"),
    ];
    if document.layers.is_empty() {
        rows.push(LayersRow::Empty);
    } else {
        rows.extend(document.layers.iter().map(LayersRow::Layer));
    }
    rows
}

/// 🕹️ No per-row selection `action` and no argument map: the tree carries the ONE `interactionSelect`
/// binding `.interaction_domain(...)?` stamps and this row declares only its `granularity`, keyed by
/// its own row id — never declare a select action here
/// (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM).
///
/// 🪟️ Recursive: a `Group` nests layer rows and a `Boolean` nests operand rows, and BOTH are windowed
/// containers of their own at every depth, so an expanded group costs the first paint one slice.
fn layer_tree_item(windows: &TreeWindows<'_>, doc: &DrawingSnapshot, layer: &DrawingLayerNode) -> UiAssemblyResult<BuiltNode> {
    let row_id = drawing_play_layers_tree_row_id(layer);
    let base = layer_base(layer);
    let description = match layer {
        DrawingLayerNode::Boolean(boolean) => boolean.operation.clone(),
        _ => base.blend_mode.clone(),
    };
    let mut drag_data = semio_framework_plugin::UiFixedMap::default();
    let drag_key = ui_key("application/x-semio-drawing-layer-id", "fixed drag key admission failed")?;
    let drag_value = ui_key(&base.id, "fixed drag value admission failed")?;
    drag_data.try_push(drag_key, drag_value).map_err(|_| PluginAssemblyError::new("ui.layer.drag-data", "fixed drag-data admission failed"))?;
    let label = semio_framework_ui_contract::Label::try_from(base.name.clone()).map_err(|_| PluginAssemblyError::new("ui.layer.label", "fixed layer label admission failed"))?;
    let item = semio_framework_ui_contract::tree_item(label)
        .try_id(&row_id)
        .map_err(|_| PluginAssemblyError::new("ui.layer.id", "fixed layer id admission failed"))?
        .description(ui_key(&description, "fixed layer description admission failed")?)
        .icon(ui_key(layer_icon(layer), "fixed layer icon admission failed")?)
        .granularity(ui_key(DRAWING_INTERACTION_GRANULARITY, "fixed layer granularity admission failed")?)
        .draggable(true)
        .drag_data(drag_data)
        .dimmed(!base.visible);
    match layer {
        DrawingLayerNode::Group(group) => tree_window_item(windows, item, &row_id, true, &group.children, |child| layer_tree_item(windows, doc, child)),
        DrawingLayerNode::Boolean(boolean) => tree_window_item(windows, item, &row_id, false, &boolean.children, |child_id| boolean_child_item(doc, &boolean.base.id, child_id)),
        _ => item.default_open(false).try_build().map_err(|_| PluginAssemblyError::new("ui.layer.build", "fixed layer admission failed")),
    }
}

fn boolean_child_item(doc: &DrawingSnapshot, boolean_id: &str, child_id: &str) -> UiAssemblyResult<BuiltNode> {
    let row_id = drawing_play_boolean_child_row_id(boolean_id, child_id);
    let mut item = match find_drawing_layer(doc, child_id) {
        Some(child) => tree_item(row_id, layer_base(child).name.clone())?,
        None => tree_item(row_id, format!("{child_id} (missing)"))?,
    };
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.draggable = Some(false);
        if let Some(child) = find_drawing_layer(doc, child_id) {
            props.description = Some(ui_key(&crate::schema::layer_kind_label(child), "fixed layer description admission failed")?);
        } else {
            props.icon = Some(ui_key("alert-circle", "fixed layer icon admission failed")?);
        }
    }
    Ok(item)
}

fn tree_button(id: &str, label: &str, icon: &str, action: &str, args: UiValue) -> UiAssemblyResult<BuiltNode> {
    let mut item = tree_item_with_action(id, label, None, drawing_play_action(action, Some(args))?)?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut item.component {
        props.icon = Some(ui_key(icon, "fixed layer icon admission failed")?);
    }
    Ok(item)
}

fn empty_row(labels: &DrawingPlayLabels) -> UiAssemblyResult<BuiltNode> {
    let mut empty = tree_item("drawing-play-layers.empty", labels.empty_state.as_str())?;
    if let semio_framework_plugin::Component::TreeItem(props) = &mut empty.component {
        props.icon = Some(ui_key("pen-tool", "fixed layer icon admission failed")?);
    }
    Ok(empty)
}

pub fn render(document: &DrawingSnapshot, labels: &DrawingPlayLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let rows = layers_rows(document, labels);
    let section = semio_framework_ui_contract::Label::try_from(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL).map_err(|_| PluginAssemblyError::new("ui.layer.section-label", "fixed layer section label admission failed"))?;
    PanelTreeBuilder::new("drawing-play-layers")?
        .window_section(windows, "drawing-play-layers", Some(section), true, &rows, |row| match row {
            LayersRow::Add(id, label, icon, kind) => tree_button(id, label.as_str(), icon, "addLayer", ui_value_map([("kind", ui_value_text(kind)?)])?),
            LayersRow::Empty => empty_row(labels),
            LayersRow::Layer(layer) => layer_tree_item(windows, document, layer),
        })?
        .interaction_domain(DRAWING_PLAY_CONTROLLER_ID, DRAWING_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
