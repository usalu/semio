//! 📄️ Puzzle 2d play app panel — the document tree: one row per node and per edge, each selecting
//! its entity — bound to the `vortex` interaction domain (ticket
//! 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM), so the framework paints selected/hovered
//! presence after render.
//!
//! 📄 Both sections are virtualised: one bounded page per section starting at that section's
//! `setPanelPage` cursor (window transient), closed by a `+N` continuation row that advances the
//! cursor. Nakagin carries 180 nodes and 179 edges — three pages of the 32-slot `UiFixedList` a
//! section may hold, and one page of the process-wide argument arena every row's action costs.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, ui_label, Puzzle2dScene, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_with_action, ActionFactory, BuiltNode, InteractionTarget, LocalizedLabel, PanelGroup, PanelRowBudget, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiAssemblyResult, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
use serde_json::Value;
use std::collections::BTreeMap;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
pub const ROOT: &str = "puzzle2d-play-document";
pub const NODES_SECTION: &str = "puzzle2d-play-document.nodes";
pub const EDGES_SECTION: &str = "puzzle2d-play-document.edges";
/// 📄 Rows one section shows per page — half the `UiFixedList` slot budget, so two sections and their
/// continuation rows always fit one page of the argument arena.
pub const SECTION_ROWS: usize = 16;
const SECTIONS: usize = 2;
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

fn ui_text(value: &str) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d document action text admission failed"))
}

fn selection_args(granularity: &str, id: &str) -> UiAssemblyResult<UiValue> {
    let targets = serde_json::to_string(&[InteractionTarget { granularity: granularity.into(), id: id.into() }]).map_err(|error| PluginAssemblyError::new("ui.action-argument", error.to_string()))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d document action map admission failed"))?;
    for (key, value) in [("domainId", ui_text(PUZZLE2D_INTERACTION_DOMAIN)?), ("merge", ui_text("replace")?), ("method", ui_text("pick")?), ("targets", ui_text(&targets)?)] {
        args.push(key.into(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d document action entry admission failed"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn node_row(node: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = node.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d node id is required"))?;
    let action = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID, Some(selection_args(PUZZLE2D_GRANULARITY_NODE, id)?))?;
    tree_item_with_action(id, ui_label(node_label(node))?, node.get("nodeKind").and_then(Value::as_str).map(str::to_string), action)
}

fn edge_row(edge: &Value, fixture: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = edge.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d edge id is required"))?;
    let action = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action(semio_framework_plugin::INTERACTION_SELECT_ACTION_ID, Some(selection_args(PUZZLE2D_GRANULARITY_EDGE, id)?))?;
    tree_item_with_action(id, ui_label(edge_label(edge, fixture))?, edge.get("edgeKind").and_then(Value::as_str).map(str::to_string), action)
}
//#endregion 🔖️Rows

//#region 🔖️Paging
fn section_page(pages: &BTreeMap<String, u32>, section_id: &str, len: usize) -> usize {
    if len == 0 {
        return 0;
    }
    (pages.get(section_id).copied().unwrap_or(0) as usize).min((len - 1) / SECTION_ROWS)
}

/// 📄 The continuation row: `+N` omitted, advancing `setPanelPage` to the next page (wrapping to the
/// first page after the last, so the row never dead-ends).
fn continuation_row(section_id: &str, omitted: usize, next_page: u32) -> UiAssemblyResult<BuiltNode> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d continuation map admission failed"))?;
    args.push("page".into(), UiValue::Number(f64::from(next_page))).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d continuation entry admission failed"))?;
    args.push("section".into(), ui_text(section_id)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d continuation entry admission failed"))?;
    match ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action("setPanelPage", Some(UiValue::Map(args.finish()))) {
        Ok(action) => tree_item_with_action(format!("{section_id}.more"), ui_label(format!("+{omitted}"))?, None, action),
        Err(_) => semio_framework_plugin::panel_continuation_row(section_id, omitted),
    }
}

/// 📄 One page of `entries` for `section_id`, starting at that section's cursor in `pages`.
pub fn paged_section<T>(section_id: &str, entries: &[T], pages: &BTreeMap<String, u32>, budget: &mut PanelRowBudget, mut row: impl FnMut(&T) -> UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let page = section_page(pages, section_id, entries.len());
    let offset = page.saturating_mul(SECTION_ROWS).min(entries.len());
    let slice = &entries[offset..];
    let mut items = UiFixedList::<BuiltNode>::default();
    let truncated = slice.len() > SECTION_ROWS;
    let mut placed = 0;
    for entry in slice {
        if placed == SECTION_ROWS || (truncated && budget.remaining() <= 1) || !budget.spend() {
            break;
        }
        let node = match row(entry) {
            Ok(node) => node,
            Err(error) if error.code == "ui.fixed-capacity" => break,
            Err(error) => return Err(error),
        };
        if items.try_push(node).is_err() {
            break;
        }
        placed += 1;
    }
    if placed < slice.len() && budget.spend() {
        let last_page = (entries.len() - 1) / SECTION_ROWS;
        let next_page = if page >= last_page { 0 } else { page as u32 + 1 };
        if let Ok(more) = continuation_row(section_id, slice.len() - placed, next_page) {
            let _ = items.try_push(more);
        }
    }
    Ok(items)
}
//#endregion 🔖️Paging

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let fixture = &envelope.fixture;
    let pages = &envelope.runtime.panel_pages;
    let budget = &mut PanelRowBudget::new(semio_framework_plugin::panel_page_rows());
    let node_items = budget.nested(SECTIONS - 1, |share| paged_section(NODES_SECTION, fixture_nodes(fixture), pages, share, node_row))?;
    let edge_items = budget.nested(SECTIONS - 2, |share| paged_section(EDGES_SECTION, fixture_edges(fixture), pages, share, |edge| edge_row(edge, fixture)))?;
    PanelTreeBuilder::new(ROOT)?
        .section_or_placeholder(NODES_SECTION, Some(ui_label(labels.nodes.as_str())?), true, node_items, ui_label(labels.none.as_str())?)?
        .section_or_placeholder(EDGES_SECTION, Some(ui_label(labels.edges.as_str())?), false, edge_items, ui_label(labels.none.as_str())?)?
        .interaction_domain(PUZZLE2D_INTERACTION_DOMAIN)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
