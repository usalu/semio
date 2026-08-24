//! 🛍️ Puzzle 2d play app panel — the kind catalogue: node/handle/edge kind rows read from the
//! fixture's `meta.kindCatalogs` (falling back to the kinds actually present in the document). Node
//! rows are drag sources for the canvas; every row also adds a node on click.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{inferred_kind_entries, kind_catalog_entries, ui_label, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, ActionFactory, BuiltNode, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_CATALOGUE: &str = "puzzle2d.play.catalogue";

/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx)? reads to auto-wire catalogue drag sources.
const PUZZLE2D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(PUZZLE2D_PLAY_BODY_CATALOGUE.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn catalog_kind_label(entry: &Value) -> String {
    entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn puzzle2d_catalog_item_drag_data(slice: &str, kind_id: &str, entry: &Value) -> Value {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": slice });
    if let Some(obj) = payload.as_object_mut() {
        if let Some(shape) = entry.get("shape") {
            obj.insert("shape".into(), shape.clone());
        }
        if let Some(radius) = entry.get("radius") {
            obj.insert("radius".into(), radius.clone());
        }
        if let Some(width) = entry.get("width") {
            obj.insert("width".into(), width.clone());
        }
        if let Some(height) = entry.get("height") {
            obj.insert("height".into(), height.clone());
        }
        if let Some(icon_kind) = entry.get("iconKind") {
            obj.insert("iconKind".into(), icon_kind.clone());
        }
    }
    json!({ (PUZZLE2D_CATALOGUE_DRAG_MIME): payload.to_string() })
}

fn add_node_args(kind_id: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let kind = UiText::try_from_str(kind_id)
        .map(UiValue::Text)
        .ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue kind admission failed"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue action map admission failed"))?;
    args.push("kind".into(), kind)
        .map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

fn kind_catalog_items(section_id: &str, slice: &str, entries: &[Value]) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let actions = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID);
    let mut items = UiFixedList::<BuiltNode>::default();
    for (index, entry) in entries.iter().enumerate() {
        let kind_id = entry.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "puzzle2d catalogue kind id is required"))?;
        let draggable = slice == "nodes";
        let id = format!("{section_id}.{index}.{kind_id}");
        let action = actions.action("addNode", Some(add_node_args(kind_id)?))?;
        let item = if draggable {
            let drag_data = puzzle2d_catalog_item_drag_data(slice, kind_id, entry);
            tree_item_with_action_draggable(id, ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), action, &drag_data)?
        } else {
            tree_item_with_action(id, ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), action)?
        };
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue item admission failed"))?;
    }
    Ok(items)
}

pub fn render(fixture: &Value, labels: &Puzzle2dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let inferred_nodes = inferred_kind_entries(fixture, "nodes");
    let inferred_handles = inferred_kind_entries(fixture, "handles");
    let inferred_edges = inferred_kind_entries(fixture, "edges");
    let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
    let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
    let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
    PanelTreeBuilder::new("puzzle2d-play-kinds")?
        .section_or_placeholder("puzzle2d-play-kinds.nodes", Some(ui_label(labels.nodes.as_str())?), true, kind_catalog_items("puzzle2d-play-kinds.nodes", "nodes", node_entries)?, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle2d-play-kinds.handles", Some(ui_label(labels.handles.as_str())?), true, kind_catalog_items("puzzle2d-play-kinds.handles", "handles", handle_entries)?, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle2d-play-kinds.edges", Some(ui_label(labels.edges.as_str())?), true, kind_catalog_items("puzzle2d-play-kinds.edges", "edges", edge_entries)?, ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render
