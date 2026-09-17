//! 🛍️ Puzzle 5d play app panel — the kind catalogue: the part/grip/fastener/rope kind rows, with the
//! part rows draggable onto the board (and clickable to place through `addPartKind`) — a catalogue
//! row owns its own binding, so it stays an ordinary interactive row.
//!
//! 🪟️ All four sections are windowed: each publishes its full `total` and materialises only the
//! host's slice (see `📌️panels/🗿️artifact` for the windowing law).

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_inferred_kind_rows, ui_label, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action_draggable, ActionFactory, BuiltNode, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, TreeWindows, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.kinds";
pub const ROOT: &str = "puzzle5d-play-kinds";
pub const PARTS_SECTION: &str = "puzzle5d-play-kinds.parts";
pub const GRIPS_SECTION: &str = "puzzle5d-play-kinds.grips";
pub const FASTENERS_SECTION: &str = "puzzle5d-play-kinds.fasteners";
pub const ROPES_SECTION: &str = "puzzle5d-play-kinds.ropes";
/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx)? reads to auto-wire catalogue drag sources.
const PUZZLE5D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_CATALOGUE_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL, "Katalog"),
        group: PanelGroup::Workbench,
        body_key: Some(BODY_KEY.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Rows
/// 🏷️ One catalogue entry's display name: its authored `label`, else its `name`, else its id.
pub fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("label")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

/// 🖱️ The catalogue drop payload. `kindId` is what the board pane's drop reads; `objectKind`/`meshUrl`
/// are what the world pane's engine-side ghost reader resolves a live mesh preview from, so one row
/// drags into either pane of the same app.
fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> Value {
    let mut payload = json!({ "kindId": kind_id, "objectKind": kind_id, "partKind": kind_id, "catalogSlice": "nodes" });
    if let Some(object) = payload.as_object_mut() {
        for key in ["shape", "radius", "width", "height", "iconKind", "meshUrl"] {
            if let Some(value) = entry.get(key) {
                object.insert(key.into(), value.clone());
            }
        }
    }
    json!({ (PUZZLE5D_CATALOGUE_DRAG_MIME): payload.to_string() })
}

fn add_part_args(kind_id: &str) -> semio_framework_plugin::UiAssemblyResult<UiValue> {
    let kind = UiText::try_from_str(kind_id).map(UiValue::Text).ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d catalogue kind admission failed"))?;
    let mut args = UiMapBuilder::try_new().ok_or_else(|| PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d catalogue action map admission failed"))?;
    args.push("partKind".into(), kind).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d catalogue action entry admission failed"))?;
    Ok(UiValue::Map(args.finish()))
}

/// 🛍️ One catalogue row. `index` keeps the key unique even when a catalog repeats a kind id.
fn kind_catalog_item(section_id: &str, entry: &(usize, &Value), add_action: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let (index, entry) = (entry.0, entry.1);
    let actions = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID);
    let kind_id = entry.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "puzzle5d catalogue kind id is required"))?;
    match add_action {
        Some(action) => {
            let drag_data = puzzle5d_catalog_item_drag_data(kind_id, entry);
            // 🌉️ `tree_item_with_action_draggable` (framework-owned, out of this ticket's
            // scope) is typed against `dsl::os_pack::json::Value`; bridges this panel's own
            // `serde_json::Value` drag payload through `DslValue` at this one call.
            let drag_data = dsl::os_pack::json::from_dsl_value(&dsl::DslValue::from(&drag_data));
            tree_item_with_action_draggable(format!("{section_id}.{index}.{kind_id}"), ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), actions.action(action, Some(add_part_args(kind_id)?))?, &drag_data)
        }
        None => tree_item_desc(format!("{section_id}.{index}.{kind_id}"), ui_label(catalog_kind_label(entry))?, Some(kind_id.into())),
    }
}

/// 🛍️ The windowed entries of one catalogue section, each carrying its ordinal.
fn indexed(entries: &[Value]) -> Vec<(usize, &Value)> {
    entries.iter().enumerate().collect()
}

//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<BuiltNode> {
    let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
    // 🧬️ None of the three 5d examples authors `kindCatalogs`, so an authored-only read renders four
    // empty sections and makes every catalogue drop impossible; a slice the document does not author
    // falls back to the kinds the document itself already names.
    let slice = |key: &str| match catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default() {
        entries if entries.is_empty() => puzzle5d_inferred_kind_rows(&envelope.document, key),
        entries => entries,
    };
    let part_entries = slice("parts");
    let grip_entries = slice("grips");
    let fastener_entries = slice("fasteners");
    let rope_entries = slice("ropes");
    let part_rows = indexed(&part_entries);
    let grip_rows = indexed(&grip_entries);
    let fastener_rows = indexed(&fastener_entries);
    let rope_rows = indexed(&rope_entries);
    PanelTreeBuilder::new(ROOT)?
        .window_section_or_placeholder(windows, PARTS_SECTION, Some(ui_label(labels.parts.as_str())?), !part_rows.is_empty(), &part_rows, |entry| kind_catalog_item(PARTS_SECTION, entry, Some("addPartKind")), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, GRIPS_SECTION, Some(ui_label(labels.grips.as_str())?), !grip_rows.is_empty(), &grip_rows, |entry| kind_catalog_item(GRIPS_SECTION, entry, None), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, FASTENERS_SECTION, Some(ui_label(labels.fasteners.as_str())?), !fastener_rows.is_empty(), &fastener_rows, |entry| kind_catalog_item(FASTENERS_SECTION, entry, None), ui_label(labels.none.as_str())?)?
        .window_section_or_placeholder(windows, ROPES_SECTION, Some(ui_label(labels.ropes.as_str())?), !rope_rows.is_empty(), &rope_rows, |entry| kind_catalog_item(ROPES_SECTION, entry, None), ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
