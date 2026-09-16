//! 🔍️ Puzzle 2d play app panel — the field inspector for whatever is selected. The live `vortex`
//! selection reaches the render as [`Puzzle2dInteractionSnapshot`] (resolved once per render by
//! `Puzzle2dPlayApp::render_with_request_context`), so this panel shows the first selected node,
//! edge or handle's real fields; an empty selection falls back to the document summary.
//!
//! 🩹️ The two mutating rows dispatch `setSelectionFlag` with an explicit `{flag, value}` over the live
//! selection — the same arg shape the context menu sends.

use crate::editor::puzzle2d::terminology::Puzzle2dLabels;
use crate::editor::puzzle2d::{fixture_edges, fixture_nodes, puzzle_extension_id, ui_label, Puzzle2dInteractionSnapshot, Puzzle2dScene, PUZZLE2D_FIXTURE_SCHEMA, PUZZLE2D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action, ui_node_list, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiAssemblyResult, UiFixedList, UiMapBuilder, UiText,
    UiValue, FRAMEWORK_PANEL_TAB_INSPECTION_ID, FRAMEWORK_PANEL_TAB_INSPECTION_LABEL,
};
use serde_json::Value;

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_PROPERTIES: &str = "puzzle2d.play.properties";
const ROOT: &str = "puzzle2d-play-inspector";
/// 🧾️ The windowed section listing the live selection's ids, ahead of the entity's own fields — a
/// whole-board selection publishes its full `total` and materialises only the host's slice.
const IDS_SECTION: &str = "puzzle2d-play-inspector.ids";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_INSPECTION_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL, "Inspektion"),
        group: PanelGroup::Details,
        body_key: Some(PUZZLE2D_PLAY_BODY_PROPERTIES.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
fn error(scope: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.inspection.fields", scope)
}

fn push(fields: &mut UiFixedList<BuiltNode>, node: UiAssemblyResult<BuiltNode>) -> UiAssemblyResult<()> {
    fields.try_push(node?).map_err(|_| error("puzzle2d inspector field admission failed"))
}

fn read_only(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, value: impl std::fmt::Display) -> UiAssemblyResult<()> {
    push(fields, tree_item_desc(format!("{ROOT}.{id}"), ui_label(label)?, Some(value.to_string())))
}

fn text(value: &Value, key: &str) -> String {
    value.get(key).map(|field| match field {
        Value::String(text) => text.clone(),
        Value::Null => String::new(),
        other => other.to_string(),
    }).unwrap_or_default()
}

fn number(value: &Value, key: &str) -> String {
    value.get(key).and_then(Value::as_f64).map(|number| format!("{number:.2}")).unwrap_or_default()
}

fn flag(value: &Value, key: &str) -> bool {
    value.get(key).and_then(Value::as_bool).unwrap_or(false)
}

fn text_value(value: &str) -> UiAssemblyResult<UiValue> {
    UiText::try_from_str(value).map(UiValue::Text).ok_or_else(|| error("puzzle2d inspector action text admission failed"))
}

/// 🩹️ One `setSelectionFlag` toggle row over the live selection.
fn flag_row(fields: &mut UiFixedList<BuiltNode>, id: &str, label: &str, flag_name: &str, pressed: bool) -> UiAssemblyResult<()> {
    let mut builder = UiMapBuilder::try_new().ok_or_else(|| error("puzzle2d inspector action map admission failed"))?;
    // 🔑️ `UiMapBuilder::push` admits keys in strictly ascending order only.
    builder.push("flag".to_owned(), text_value(flag_name)?).map_err(|_| error("puzzle2d inspector action map entry admission failed"))?;
    builder.push("value".to_owned(), UiValue::Bool(!pressed)).map_err(|_| error("puzzle2d inspector action map entry admission failed"))?;
    let action = ActionFactory::new(PUZZLE2D_PLAY_CONTROLLER_ID).action("setSelectionFlag", Some(UiValue::Map(builder.finish())))?;
    push(fields, tree_item_with_action(format!("{ROOT}.{id}"), ui_label(label)?, Some(pressed.to_string()), action))
}

/// 🪟️ The selection's id list as its own windowed section — keyed by raw id, never truncated: the
/// section stamps `total` over the whole selection and the host streams the rest as it scrolls.
fn ids_section(builder: PanelTreeBuilder, windows: &TreeWindows<'_>, ids: &[String], labels: &Puzzle2dLabels) -> UiAssemblyResult<PanelTreeBuilder> {
    if ids.len() <= 1 {
        return Ok(builder);
    }
    builder.window_section(windows, IDS_SECTION, Some(ui_label(labels.selected.as_str())?), true, ids, |id| tree_item_desc(id, ui_label(labels.id.as_str())?, Some(id.clone())))
}
//#endregion 🔖️Rows

//#region 🔖️Sections
fn node_fields(node: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "node.id", labels.id.as_str(), text(node, "id"))?;
    read_only(&mut fields, "node.text", labels.text.as_str(), text(node, "text"))?;
    read_only(&mut fields, "node.kind", labels.node_kind.as_str(), text(node, "nodeKind"))?;
    read_only(&mut fields, "node.shape", labels.shape.as_str(), text(node, "shape"))?;
    read_only(&mut fields, "node.x", labels.x.as_str(), number(node, "x"))?;
    read_only(&mut fields, "node.y", labels.y.as_str(), number(node, "y"))?;
    if node.get("shape").and_then(Value::as_str) == Some("rectangle") {
        read_only(&mut fields, "node.width", labels.width.as_str(), number(node, "width"))?;
        read_only(&mut fields, "node.height", labels.height.as_str(), number(node, "height"))?;
    } else {
        read_only(&mut fields, "node.radius", labels.radius.as_str(), number(node, "radius"))?;
    }
    read_only(&mut fields, "node.handles", labels.handles.as_str(), node.get("handles").and_then(Value::as_array).map_or(0, Vec::len))?;
    flag_row(&mut fields, "node.hidden", labels.hidden.as_str(), "hidden", flag(node, "hidden"))?;
    flag_row(&mut fields, "node.locked", labels.locked.as_str(), "locked", flag(node, "locked"))?;
    Ok(fields)
}

fn edge_fields(edge: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "edge.id", labels.id.as_str(), text(edge, "id"))?;
    read_only(&mut fields, "edge.kind", labels.edge_kind.as_str(), text(edge, "edgeKind"))?;
    read_only(&mut fields, "edge.source", labels.source.as_str(), text(edge, "source"))?;
    read_only(&mut fields, "edge.target", labels.target.as_str(), text(edge, "target"))?;
    flag_row(&mut fields, "edge.hidden", labels.hidden.as_str(), "hidden", flag(edge, "hidden"))?;
    flag_row(&mut fields, "edge.locked", labels.locked.as_str(), "locked", flag(edge, "locked"))?;
    Ok(fields)
}

fn handle_fields(node: &Value, handle: &Value, labels: &Puzzle2dLabels) -> UiAssemblyResult<UiFixedList<BuiltNode>> {
    let mut fields = UiFixedList::default();
    read_only(&mut fields, "handle.id", labels.id.as_str(), text(handle, "id"))?;
    read_only(&mut fields, "handle.node", labels.node.as_str(), text(node, "id"))?;
    read_only(&mut fields, "handle.kind", labels.handle_kind.as_str(), text(handle, "handleKind"))?;
    read_only(&mut fields, "handle.angle", labels.angle.as_str(), number(handle, "angle"))?;
    read_only(&mut fields, "handle.radius", labels.radius.as_str(), number(handle, "radius"))?;
    Ok(fields)
}

/// 🈳️ The document summary — what an empty (or unresolvable) selection shows.
fn summary(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiAssemblyResult<BuiltNode> {
    let rows = ui_node_list([
        tree_item_desc(format!("{ROOT}.schema"), ui_label(labels.schema.as_str())?, Some(PUZZLE2D_FIXTURE_SCHEMA.into())),
        tree_item_desc(format!("{ROOT}.extension"), ui_label(labels.extension.as_str())?, Some(puzzle_extension_id().into())),
        tree_item_desc(format!("{ROOT}.nodes"), ui_label(labels.nodes.as_str())?, Some(fixture_nodes(&envelope.fixture).len().to_string())),
        tree_item_desc(format!("{ROOT}.edges"), ui_label(labels.edges.as_str())?, Some(fixture_edges(&envelope.fixture).len().to_string())),
    ])?;
    PanelTreeBuilder::new(ROOT)?.section(format!("{ROOT}.summary"), Some(ui_label(FRAMEWORK_PANEL_TAB_INSPECTION_LABEL)?), true, rows)?.build()
}

/// 🔍️ The first selected entity's field group: a node, an edge, or a handle nested under a node.
fn selected_section(fixture: &Value, interaction: &Puzzle2dInteractionSnapshot, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> Option<UiAssemblyResult<BuiltNode>> {
    let ids = interaction.selected_ids();
    let section = |label: &str, id: &str, fields: UiAssemblyResult<UiFixedList<BuiltNode>>| -> UiAssemblyResult<BuiltNode> {
        ids_section(PanelTreeBuilder::new(ROOT)?, windows, ids, labels)?.section(format!("{ROOT}.{id}"), Some(ui_label(label)?), true, fields?)?.build()
    };
    let first = ids.first()?;
    let nodes = fixture_nodes(fixture);
    if let Some(node) = nodes.iter().find(|node| node.get("id").and_then(Value::as_str) == Some(first)) {
        return Some(section(labels.node.as_str(), "node", node_fields(node, labels)));
    }
    if let Some(edge) = fixture_edges(fixture).iter().find(|edge| edge.get("id").and_then(Value::as_str) == Some(first)) {
        return Some(section(labels.edge.as_str(), "edge", edge_fields(edge, labels)));
    }
    nodes
        .iter()
        .find_map(|node| node.get("handles").and_then(Value::as_array).and_then(|handles| handles.iter().find(|handle| handle.get("id").and_then(Value::as_str) == Some(first))).map(|handle| (node, handle)))
        .map(|(node, handle)| section(labels.handle.as_str(), "handle", handle_fields(node, handle, labels)))
}
//#endregion 🔖️Sections

//#region 🔖️Render
pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels, windows: &TreeWindows<'_>) -> UiAssemblyResult<BuiltNode> {
    match selected_section(&envelope.fixture, &envelope.interaction, labels, windows) {
        Some(section) => section,
        None => summary(envelope, labels),
    }
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
