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
use crate::editor::puzzle2d::{board_snapshot_edges, board_snapshot_nodes, snapshot_target_regions, puzzle2d_node_display_label, ui_label, Puzzle2dScene, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_GRANULARITY_TARGET_REGION, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase, RowActionPlacement};
use semio_framework_plugin::row_action;
use semio_framework_plugin::row_target;
use semio_framework_plugin::BuiltNode;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::PanelGroup;
use semio_framework_plugin::PanelTabDefinition;
use semio_framework_plugin::PanelTabKind;
use semio_framework_plugin::PanelTreeBuilder;
use semio_framework_plugin::PluginAssemblyError;
use semio_framework_plugin::TreeWindows;
use semio_framework_plugin::UiAssemblyResult;
use semio_framework_plugin::UiListBuilder;
use semio_framework_plugin::UiMapBuilder;
use semio_framework_plugin::UiText;
use semio_framework_plugin::UiValue;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_ID;
use semio_framework_plugin::FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL;
use semio_framework_ui_contract as ui;
use serde_json::Value;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
pub const ROOT: &str = "puzzle2d-play-document";
pub const NODES_SECTION: &str = "puzzle2d-play-document.nodes";
pub const EDGES_SECTION: &str = "puzzle2d-play-document.edges";
pub const TARGET_REGIONS_SECTION: &str = "puzzle2d-play-document.target-regions";
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
/// 🏷️ What one row reads — the shared authored-label / catalogue-name / id precedence, so an outliner
/// row, a board glyph and the inspector all name the same node the same way.
fn node_label(node: &Value, snapshot: &Value) -> String {
    puzzle2d_node_display_label(node, snapshot)
}

fn edge_label(edge: &Value, snapshot: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let endpoint = |id: &str| board_snapshot_nodes(snapshot).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).map_or_else(|| id.to_string(), |node| node_label(node, snapshot));
    format!("{} → {}", endpoint(source), endpoint(target))
}

fn flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn ui_value_text(value: &str) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action text admission failed"))
}

/// 🎯️ A flag row's ONE target: the id key (`ids` for a node, `id` for a region) and the flag state each inline toggle
/// ASKS FOR — always the inverse of the row's current one, stated as a value (`setSelectionHidden{hidden}` …), so a stale
/// view sets a value and never flips one. A hardcoded `true` (puzzle3d's outliner bug, since fixed there) would make
/// "Show"/"Unlock" re-apply the state the row was already in, so a row-hidden node could never be un-hidden from the row.
///
/// 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only — `hidden`, `id`/`ids`, `locked`.
fn flag_target(identity: (&str, UiValue), hidden: bool, locked: bool) -> UiAssemblyResult<ui::RowTarget> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row target map admission failed"))?;
    for (key, value) in [("hidden", UiValue::Bool(!hidden)), identity, ("locked", UiValue::Bool(!locked))] {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row target map entry admission failed"))?;
    }
    row_target(PUZZLE2D_PLAY_CONTROLLER_ID, Some(UiValue::Map(args.finish())), None)
}

/// 🎯️ A node row's target: `ids` names exactly that node.
fn node_flag_target(id: &str, hidden: bool, locked: bool) -> UiAssemblyResult<ui::RowTarget> {
    let mut ids = UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row target list admission failed"))?;
    ids.push(ui_value_text(id)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row target list item admission failed"))?;
    flag_target(("ids", UiValue::List(ids.finish())), hidden, locked)
}

/// 🎬️ A flag row's two inline toggles — `verbs` names the collection's own set-verbs (a region's flags are written by
/// `setTargetRegion*`, not by `setSelection*`: the two address different collections, and a shared verb would silently
/// flag a node that happened to share the id).
fn hide_lock_actions(hidden: bool, locked: bool, labels: &Puzzle2dLabels, verbs: [&str; 2]) -> UiAssemblyResult<[ui::RowAction; 2]> {
    Ok([
        row_action(if hidden { "eye-off" } else { "eye" }, if hidden { labels.show.as_str() } else { labels.hide.as_str() }, verbs[0], RowActionPlacement::Row)?,
        row_action(if locked { "lock" } else { "lock-open" }, if locked { labels.unlock.as_str() } else { labels.lock.as_str() }, verbs[1], RowActionPlacement::Row)?,
    ])
}

/// 🪙️ Attaches a row's INLINE hide/lock toggles, and KEEPS the row when the argument arena cannot
/// afford them. The arena is process-global and holds one live page for every panel of every plugin
/// at once, so on Nakagin (180 nodes + 179 edges) the catalogue's own rows can leave the outliner too
/// little credit for a row's two maps; propagating that refusal would end the section at zero rows —
/// four headers and nothing selectable. A row without its toggles is still a pick target and still
/// names its node; a row that was never materialised is neither. Edge rows carry no toggles at all,
/// mirroring puzzle3d's attraction rows, which halves the per-document arena cost.
fn with_hide_lock_actions(item: ui::TreeItemBuilder, target: UiAssemblyResult<ui::RowTarget>, actions: UiAssemblyResult<[ui::RowAction; 2]>) -> ui::TreeItemBuilder {
    let (Ok(target), Ok(actions)) = (target, actions) else { return item };
    let mut item = item.target(target);
    for action in actions {
        match item.try_row_action(action) {
            Ok(next) => item = next,
            Err((refused, _)) => return refused,
        }
    }
    item
}

fn ui_text(value: &str) -> UiAssemblyResult<UiText> {
    UiText::try_from_str(value).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d document text admission failed"))
}

/// 🎯️ One pick row of the tree's interaction domain — keyed by the raw entity id, carrying its
/// granularity instead of a per-row `interactionSelect` argument map.
fn pick_item(id: &str, label: String, description: Option<&str>, granularity: &str) -> UiAssemblyResult<ui::TreeItemBuilder> {
    let mut builder = ui::tree_item(ui_label(label)?).try_id(id).map_err(|_| PluginAssemblyError::new("ui.document", "puzzle2d row id admission failed"))?;
    if let Some(description) = description {
        builder = builder.description(ui_text(description)?);
    }
    Ok(builder.granularity(ui_text(granularity)?))
}

fn pick_row(id: &str, label: String, description: Option<&str>, granularity: &str) -> UiAssemblyResult<BuiltNode> {
    pick_item(id, label, description, granularity)?.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row admission failed"))
}

fn node_row(node: &Value, snapshot: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = node.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d node id is required"))?;
    let (hidden, locked) = (crate::editor::puzzle2d::puzzle2d_entity_hidden(node), flag(node, "locked"));
    let item = pick_item(id, node_label(node, snapshot), node.get("nodeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_NODE)?.dimmed(hidden);
    with_hide_lock_actions(item, node_flag_target(id, hidden, locked), hide_lock_actions(hidden, locked, labels, ["setSelectionHidden", "setSelectionLocked"])).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row admission failed"))
}

fn edge_row(edge: &Value, snapshot: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = edge.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d edge id is required"))?;
    pick_row(id, edge_label(edge, snapshot), edge.get("edgeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_EDGE)
}
//#endregion 🔖️Rows

//#region 🎯️TargetRegionRows
// 🤝️ Slice 2F owns everything in this region. It is kept whole and separate from `🔖️Rows` above so
// slice 2C's outliner work and this one never anchor on the same lines.
/// 🎯️ One fill-constraining rectangle as an outliner row: its label or id, its extent as the
/// description, dimmed while hidden, with the same two inline toggles a node row carries. Refused
/// toggles keep the row, exactly as [`with_hide_lock_actions`] argues for nodes.
fn target_region_row(region: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = region.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d target region id is required"))?;
    let (hidden, locked) = (flag(region, "hidden"), flag(region, "locked"));
    let label = region.get("label").and_then(Value::as_str).map_or_else(|| id.to_string(), str::to_string);
    let extent = format!("{} × {}", region.get("width").and_then(Value::as_f64).unwrap_or(0.0).abs().round(), region.get("height").and_then(Value::as_f64).unwrap_or(0.0).abs().round());
    let item = pick_item(id, label, Some(extent.as_str()), PUZZLE2D_GRANULARITY_TARGET_REGION)?.dimmed(hidden);
    let target = ui_value_text(id).and_then(|id| flag_target(("id", id), hidden, locked));
    with_hide_lock_actions(item, target, hide_lock_actions(hidden, locked, labels, ["setTargetRegionHidden", "setTargetRegionLocked"])).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d region row admission failed"))
}
//#endregion 🎯️TargetRegionRows

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let snapshot = &envelope.board_snapshot;
    PanelTreeBuilder::new(ROOT)?
        .interaction_domain(PUZZLE2D_PLAY_CONTROLLER_ID, PUZZLE2D_INTERACTION_DOMAIN)?
        .window_section_or_placeholder(windows, NODES_SECTION, Some(ui_label(labels.nodes.as_str())?), true, board_snapshot_nodes(snapshot), |node| node_row(node, snapshot, labels), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, EDGES_SECTION, Some(ui_label(labels.edges.as_str())?), false, board_snapshot_edges(snapshot), |edge| edge_row(edge, snapshot), ui_label(labels.none.as_str())?)?
        // 🎯️ Slice 2F's own section — collapsed by default, exactly as puzzle3d collapses its target volumes.
        .window_section_or_placeholder(windows, TARGET_REGIONS_SECTION, Some(ui_label(labels.target_regions.as_str())?), false, snapshot_target_regions(snapshot), |region| target_region_row(region, labels), ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
