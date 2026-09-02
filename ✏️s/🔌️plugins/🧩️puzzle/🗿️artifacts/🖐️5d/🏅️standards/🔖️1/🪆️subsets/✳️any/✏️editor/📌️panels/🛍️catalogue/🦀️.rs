//! 🛍️ Puzzle 5d play app panel — the kind catalogue: the part/grip/fastener/rope kind rows, with the
//! part rows draggable onto the board (and clickable to place through `addPartKind`).

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{ui_label, Puzzle5dScene, PUZZLE5D_PLAY_CONTROLLER_ID};
use semio_framework_plugin::{
    tree_item_desc, tree_item_with_action_draggable, ActionFactory, BuiltNode, LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, PluginAssemblyError, UiFixedList, UiMapBuilder, UiText, UiValue,
    FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.kinds";
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
fn catalog_kind_label(entry: &Value) -> String {
    entry
        .get("label")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .or_else(|| entry.get("name").and_then(|value| value.as_str()).filter(|value| !value.is_empty()))
        .or_else(|| entry.get("id").and_then(|value| value.as_str()))
        .unwrap_or("kind")
        .into()
}

fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> Value {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": "nodes" });
    if let Some(object) = payload.as_object_mut() {
        for key in ["shape", "radius", "width", "height", "iconKind"] {
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

fn kind_catalog_items(section_id: &str, entries: &[Value], add_action: Option<&str>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::UiFixedList<semio_framework_plugin::BuiltNode>> {
    let actions = ActionFactory::new(PUZZLE5D_PLAY_CONTROLLER_ID);
    let mut items = UiFixedList::<BuiltNode>::default();
    for (index, entry) in entries.iter().enumerate() {
        let kind_id = entry.get("id").and_then(Value::as_str).ok_or_else(|| PluginAssemblyError::new("ui.catalogue", "puzzle5d catalogue kind id is required"))?;
        let item = match add_action {
            Some(action) => {
                let drag_data = puzzle5d_catalog_item_drag_data(kind_id, entry);
                tree_item_with_action_draggable(format!("{section_id}.{index}.{kind_id}"), ui_label(catalog_kind_label(entry))?, Some(kind_id.into()), actions.action(action, Some(add_part_args(kind_id)?))?, &drag_data)?
            }
            None => tree_item_desc(format!("{section_id}.{index}.{kind_id}"), ui_label(catalog_kind_label(entry))?, Some(kind_id.into()))?,
        };
        items.try_push(item).map_err(|_| PluginAssemblyError::new("ui.fixed-capacity", "puzzle5d catalogue item admission failed"))?;
    }
    Ok(items)
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
    let slice = |key: &str| catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default();
    let mut part_entries = slice("parts");
    if part_entries.is_empty() {
        let mut ids: Vec<String> = envelope.document.parts.iter().map(|part| part.part_kind.clone()).collect();
        ids.sort();
        ids.dedup();
        part_entries = ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect();
    }
    let grips = slice("grips");
    let fasteners = slice("fasteners");
    let ropes = slice("ropes");
    PanelTreeBuilder::new("puzzle5d-play-kinds")?
        .section_or_placeholder("puzzle5d-play-kinds.parts", Some(ui_label(labels.parts.as_str())?), !part_entries.is_empty(), kind_catalog_items("puzzle5d-play-kinds.parts", &part_entries, Some("addPartKind"))?, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle5d-play-kinds.grips", Some(ui_label(labels.grips.as_str())?), !grips.is_empty(), kind_catalog_items("puzzle5d-play-kinds.grips", &grips, None)?, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle5d-play-kinds.fasteners", Some(ui_label(labels.fasteners.as_str())?), !fasteners.is_empty(), kind_catalog_items("puzzle5d-play-kinds.fasteners", &fasteners, None)?, ui_label(labels.none.as_str())?)?
        .section_or_placeholder("puzzle5d-play-kinds.ropes", Some(ui_label(labels.ropes.as_str())?), !ropes.is_empty(), kind_catalog_items("puzzle5d-play-kinds.ropes", &ropes, None)?, ui_label(labels.none.as_str())?)?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::editor::puzzle5d::testkit::*;

    #[test]
    fn catalogue_tree_lists_all_four_kind_sections() {
        let mut app = app();
        let rendered = render_body(&mut app, BODY_KEY);
        for section in ["puzzle5d-play-kinds.parts", "puzzle5d-play-kinds.grips", "puzzle5d-play-kinds.fasteners", "puzzle5d-play-kinds.ropes"] {
            assert!(rendered.contains(section), "catalogue must carry {section}");
        }
    }
}
//#endregion 🧪️Tests
