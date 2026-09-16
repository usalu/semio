//! 🛍️ Puzzle 2d play app panel — the kind catalogue: node/handle/edge kind rows read from the
//! fixture's `meta.kindCatalogs` (falling back to the kinds actually present in the document). Node
//! rows are drag sources for the canvas; every row also adds a node on click — a catalogue row owns
//! its own `addNode` binding, so it stays an ordinary interactive row.
//!
//! 🪟️ All three sections are windowed: each publishes its full `total` and materialises only the
//! host's slice (see `📌️panels/🗿️artifact` for the windowing law).

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{inferred_kind_entries, kind_catalog_entries, ui_label, Puzzle2dScene, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_with_action, tree_item_with_action_draggable, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_CATALOGUE: &str = "puzzle2d.play.catalogue";
pub const ROOT: &str = "puzzle2d-play-kinds";
pub const NODES_SECTION: &str = "puzzle2d-play-kinds.nodes";
pub const HANDLES_SECTION: &str = "puzzle2d-play-kinds.handles";
pub const EDGES_SECTION: &str = "puzzle2d-play-kinds.edges";

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
    let kind = UiText::try_from_str(kind_id).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue kind admission failed"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue action map admission failed"))?;
    args.push("kind".into(), kind).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d catalogue action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

fn kind_catalog_item(section_id: &str, slice: &str, entry: &Value) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let actions = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID);
    let kind_id = entry.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "puzzle2d catalogue kind id is required"))?;
    let id = format!("{section_id}.{kind_id}");
    let action = actions.action("addNode", Some(add_node_args(kind_id)?))?;
    if slice == "nodes" {
        let drag_data = puzzle2d_catalog_item_drag_data(slice, kind_id, entry);
        // 🌉️ `tree_item_with_action_draggable` (framework-owned) is typed against `dsl::os_pack::json::Value`;
        // bridges this panel's own `serde_json::Value` drag payload through `DslValue` at this one call.
        let drag_data = dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&drag_data));
        tree_item_with_action_draggable(id, ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), action, &drag_data)
    } else {
        tree_item_with_action(id, ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), action)
    }
}

pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let fixture = &envelope.fixture;
    let inferred_nodes = inferred_kind_entries(fixture, "nodes");
    let inferred_handles = inferred_kind_entries(fixture, "handles");
    let inferred_edges = inferred_kind_entries(fixture, "edges");
    let node_entries = kind_catalog_entries(fixture, "nodes").unwrap_or(inferred_nodes.as_slice());
    let handle_entries = kind_catalog_entries(fixture, "handles").unwrap_or(inferred_handles.as_slice());
    let edge_entries = kind_catalog_entries(fixture, "edges").unwrap_or(inferred_edges.as_slice());
    PanelTreeBuilder::new(ROOT)?
        .window_section_or_placeholder(windows, NODES_SECTION, Some(ui_label(labels.nodes.as_str())?), true, node_entries, |entry| kind_catalog_item(NODES_SECTION, "nodes", entry), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, HANDLES_SECTION, Some(ui_label(labels.handles.as_str())?), false, handle_entries, |entry| kind_catalog_item(HANDLES_SECTION, "handles", entry), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, EDGES_SECTION, Some(ui_label(labels.edges.as_str())?), false, edge_entries, |entry| kind_catalog_item(EDGES_SECTION, "edges", entry), ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
