//! 📄️ Puzzle 2d play app panel — the document tree: one row per node and per edge, each a pick
//! target of the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.
//!
//! 🪟️ Both sections are virtualised the framework way: each publishes its FULL extent through
//! `TreeWindow { total, offset }` and materialises only the slice the host asked for
//! (`ViewModel::tree_windows`, read once per render as [`TreeWindows`]). Nakagin's 180 nodes and 179
//! edges therefore scroll as one document — there is no page cursor and no `+N` continuation row.
//!
//! 🎯️ Rows carry a `granularity` and no binding of their own: the tree root owns the single
//! `interactionSelect` binding [`PanelTreeBuilder::interaction_domain`] stamps, and every row is
//! keyed by its raw entity id, so a pick costs nothing of the argument arena.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, ui_label, Puzzle2dScene, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{
    BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiText, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use semio_framework_ui_contract as ui;
use serde_json::Value;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
pub const ROOT: &str = "puzzle2d-play-document";
pub const NODES_SECTION: &str = "puzzle2d-play-document.nodes";
pub const EDGES_SECTION: &str = "puzzle2d-play-document.edges";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Artefakt"),
        group: PanelGroup::Workbench,
        body_key: Some(PUZZLE2D_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn node_label(node: &Value) -> String {
    node.get("text").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| node.get("id").and_then(|value| value.as_str())).unwrap_or("node").into()
}

fn edge_label(edge: &Value, fixture: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let source_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(source)).map_or_else(|| source.into(), node_label);
    let target_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(target)).map_or_else(|| target.into(), node_label);
    format!("{source_label} → {target_label}")
}

fn ui_text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d document text admission failed"))
}

/// 🎯️ One pick row of the tree's interaction domain — keyed by the raw entity id, carrying its
/// granularity instead of a per-row `interactionSelect` argument map.
fn pick_row(id: &str, label: String, description: Option<&str>, granularity: &str) -> UiAssemblyResult<BuiltNode> {
    let mut builder = ui::tree_item(ui_label(label)?).try_id(id).map_err(|_| PluginAssemblyError::new("ui.document", "puzzle2d row id admission failed"))?;
    if let Some(description) = description {
        builder = builder.description(ui_text(description)?);
    }
    builder.granularity(ui_text(granularity)?).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row admission failed"))
}

fn node_row(node: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = node.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d node id is required"))?;
    pick_row(id, node_label(node), node.get("nodeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_NODE)
}

fn edge_row(edge: &Value, fixture: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = edge.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d edge id is required"))?;
    pick_row(id, edge_label(edge, fixture), edge.get("edgeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_EDGE)
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let fixture = &envelope.fixture;
    PanelTreeBuilder::new(ROOT)?
        .window_section_or_placeholder(windows, NODES_SECTION, Some(ui_label(labels.nodes.as_str())?), true, fixture_nodes(fixture), node_row, ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, EDGES_SECTION, Some(ui_label(labels.edges.as_str())?), false, fixture_edges(fixture), |edge| edge_row(edge, fixture), ui_label(labels.none.as_str())?)?
        .interaction_domain(PUZZLE2D_PLAY_CONTROLLER_ID, PUZZLE2D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
