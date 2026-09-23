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
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, fixture_target_regions, puzzle2d_node_display_label, ui_label, Puzzle2dScene, PUZZLE2D_GRANULARITY_EDGE, PUZZLE2D_GRANULARITY_NODE, PUZZLE2D_GRANULARITY_TARGET_REGION, PUZZLE2D_INTERACTION_DOMAIN, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::plugin_app_close_prelude::{ActionBinding, Buildable, HasBase, RowAction, RowActionPlacement, Trigger};
use semio_framework_plugin::{
    ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiListBuilder, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL,
};
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
fn node_label(node: &Value, fixture: &Value) -> String {
    puzzle2d_node_display_label(node, fixture)
}

fn edge_label(edge: &Value, fixture: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let endpoint = |id: &str| fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(Value::as_str) == Some(id)).map_or_else(|| id.to_string(), |node| node_label(node, fixture));
    format!("{} → {}", endpoint(source), endpoint(target))
}

fn flag(entity: &Value, key: &str) -> bool {
    entity.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn ui_value_text(value: &str) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action text admission failed"))
}

/// 🔁️ One inline row toggle's `setSelectionFlag` args. `value` is the state the click ASKS FOR —
/// always the inverse of the row's current one, the same negation this app's own context menu
/// (`value: any_visible`) and inspection panel (`UiValue::Bool(!pressed)`) already carry. A hardcoded
/// `true` (puzzle3d's outliner bug, since fixed there) would make "Show"/"Unlock" re-apply the state
/// the row was already in, so a row-hidden node could never be un-hidden from the row that hid it.
///
/// 🔑️ `UiMapBuilder::push` admits keys in STRICTLY ASCENDING order only — `flag`, `ids`, `value`.
fn flag_args(id: &str, flag_name: &str, value: bool) -> UiAssemblyResult<UiValue> {
    let mut ids = UiListBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action list admission failed"))?;
    ids.push(ui_value_text(id)?).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action list item admission failed"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action map admission failed"))?;
    for (key, value) in [("flag", ui_value_text(flag_name)?), ("ids", UiValue::List(ids.finish())), ("value", UiValue::Bool(value))] {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action map entry admission failed"))?;
    }
    Ok(UiValue::Map(args.finish()))
}

fn flag_row_action(icon: &str, label: &str, id: &str, flag_name: &str, next: bool) -> UiAssemblyResult<RowAction> {
    let (action, args) = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action("setSelectionFlag", Some(flag_args(id, flag_name, next)?))?;
    Ok(RowAction {
        icon: UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row action icon admission failed"))?,
        label: Some(ui_label(label)?),
        action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
        placement: RowActionPlacement::Row,
    })
}

/// 🪙️ Attaches a row's INLINE hide/lock toggles, and KEEPS the row when the argument arena cannot
/// afford them. The arena is process-global and holds one live page for every panel of every plugin
/// at once, so on Nakagin (180 nodes + 179 edges) the catalogue's own rows can leave the outliner too
/// little credit for a row's two maps; propagating that refusal would end the section at zero rows —
/// four headers and nothing selectable. A row without its toggles is still a pick target and still
/// names its node; a row that was never materialised is neither. Edge rows carry no toggles at all,
/// mirroring puzzle3d's attraction rows, which halves the per-document arena cost.
fn with_hide_lock_actions(item: ui::TreeItemBuilder, hidden: bool, locked: bool, labels: &Puzzle2dLabels, id: &str) -> ui::TreeItemBuilder {
    let actions = [
        flag_row_action(if hidden { "eye-off" } else { "eye" }, if hidden { labels.show.as_str() } else { labels.hide.as_str() }, id, "hidden", !hidden),
        flag_row_action(if locked { "lock" } else { "lock-open" }, if locked { labels.unlock.as_str() } else { labels.lock.as_str() }, id, "locked", !locked),
    ];
    let mut item = item;
    for action in actions {
        let Ok(action) = action else { return item };
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

fn node_row(node: &Value, fixture: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = node.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d node id is required"))?;
    let (hidden, locked) = (flag(node, "hidden"), flag(node, "locked"));
    let item = pick_item(id, node_label(node, fixture), node.get("nodeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_NODE)?.dimmed(hidden);
    with_hide_lock_actions(item, hidden, locked, labels, id).try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d row admission failed"))
}

fn edge_row(edge: &Value, fixture: &Value) -> UiAssemblyResult<BuiltNode> {
    let id = edge.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d edge id is required"))?;
    pick_row(id, edge_label(edge, fixture), edge.get("edgeKind").and_then(Value::as_str), PUZZLE2D_GRANULARITY_EDGE)
}
//#endregion 🔖️Rows

//#region 🎯️TargetRegionRows
// 🤝️ Slice 2F owns everything in this region. It is kept whole and separate from `🔖️Rows` above so
// slice 2C's outliner work and this one never anchor on the same lines.
/// 🎯️ The inline hide/lock toggles of one region row. A region's flags are written by
/// `setTargetRegionFlag`, not by `setSelectionFlag` — the two verbs address different collections,
/// and a shared row action would silently flag a node that happened to share the id.
fn region_flag_row_action(icon: &str, label: &str, id: &str, flag_name: &str, next: bool) -> UiAssemblyResult<RowAction> {
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d region row action map admission failed"))?;
    for (key, value) in [("id", ui_value_text(id)?), ("flag", ui_value_text(flag_name)?), ("value", UiValue::Bool(next))] {
        args.push(key.to_owned(), value).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d region row action map entry admission failed"))?;
    }
    let (action, args) = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action("setTargetRegionFlag", Some(UiValue::Map(args.finish())))?;
    Ok(RowAction {
        icon: UiText::try_from_str(icon).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d region row action icon admission failed"))?,
        label: Some(ui_label(label)?),
        action: ActionBinding { trigger: Trigger::Activate, action, args, capability: None },
        placement: RowActionPlacement::Row,
    })
}

/// 🎯️ One fill-constraining rectangle as an outliner row: its label or id, its extent as the
/// description, dimmed while hidden, with the same two inline toggles a node row carries. Refused
/// toggles keep the row, exactly as [`with_hide_lock_actions`] argues for nodes.
fn target_region_row(region: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let id = region.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.document", "puzzle2d target region id is required"))?;
    let (hidden, locked) = (flag(region, "hidden"), flag(region, "locked"));
    let label = region.get("label").and_then(Value::as_str).map_or_else(|| id.to_string(), str::to_string);
    let extent = format!("{} × {}", region.get("width").and_then(Value::as_f64).unwrap_or(0.0).abs().round(), region.get("height").and_then(Value::as_f64).unwrap_or(0.0).abs().round());
    let mut item = pick_item(id, label, Some(extent.as_str()), PUZZLE2D_GRANULARITY_TARGET_REGION)?.dimmed(hidden);
    for action in [
        region_flag_row_action(if hidden { "eye-off" } else { "eye" }, if hidden { labels.show.as_str() } else { labels.hide.as_str() }, id, "hidden", !hidden),
        region_flag_row_action(if locked { "lock" } else { "lock-open" }, if locked { labels.unlock.as_str() } else { labels.lock.as_str() }, id, "locked", !locked),
    ] {
        let Ok(action) = action else { break };
        match item.try_row_action(action) {
            Ok(next) => item = next,
            Err((refused, _)) => {
                item = refused;
                break;
            }
        }
    }
    item.try_build().map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle2d region row admission failed"))
}
//#endregion 🎯️TargetRegionRows

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    let fixture = &envelope.fixture;
    PanelTreeBuilder::new(ROOT)?
        .interaction_domain(PUZZLE2D_PLAY_CONTROLLER_ID, PUZZLE2D_INTERACTION_DOMAIN)?
        .window_section_or_placeholder(windows, NODES_SECTION, Some(ui_label(labels.nodes.as_str())?), true, fixture_nodes(fixture), |node| node_row(node, fixture, labels), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, EDGES_SECTION, Some(ui_label(labels.edges.as_str())?), false, fixture_edges(fixture), |edge| edge_row(edge, fixture), ui_label(labels.none.as_str())?)?
        // 🎯️ Slice 2F's own section — collapsed by default, exactly as puzzle3d collapses its target volumes.
        .window_section_or_placeholder(windows, TARGET_REGIONS_SECTION, Some(ui_label(labels.target_regions.as_str())?), false, fixture_target_regions(fixture), |region| target_region_row(region, labels), ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
