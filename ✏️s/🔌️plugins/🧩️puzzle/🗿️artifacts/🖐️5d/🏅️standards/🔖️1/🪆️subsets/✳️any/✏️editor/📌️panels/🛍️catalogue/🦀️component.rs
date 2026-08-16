//! 🛍️ Puzzle 5d play app panel — the kind catalogue: the part/grip/fastener/rope kind rows, with the
//! part rows draggable onto the board (and clickable to place through `addPartKind`).

use crate::editor::puzzle5d::terminology::Puzzle5dLabels;
use crate::editor::puzzle5d::{puzzle5d_action, tree_info_item, tree_item_with_action, Puzzle5dScene};
use semio_framework_plugin::{LabelText, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.5d.play.kinds";
/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
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

fn puzzle5d_catalog_item_drag_data(kind_id: &str, entry: &Value) -> HashMap<String, String> {
    let mut payload = json!({ "kindId": kind_id, "catalogSlice": "nodes" });
    if let Some(object) = payload.as_object_mut() {
        for key in ["shape", "radius", "width", "height", "iconKind"] {
            if let Some(value) = entry.get(key) {
                object.insert(key.into(), value.clone());
            }
        }
    }
    HashMap::from([(PUZZLE5D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
}

fn kind_catalog_section(section_id: &str, label: LabelText, entries: &[Value], add_action: Option<&str>, none_label: LabelText) -> UiTreeSectionNode {
    let items: Vec<UiTreeItemNode> = entries
        .iter()
        .enumerate()
        .map(|(index, entry)| {
            let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind");
            match add_action {
                Some(action) => {
                    let mut item = tree_item_with_action(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some("box"), puzzle5d_action(action, Some(json!({ "partKind": kind_id }))));
                    item.description = Some(kind_id.into());
                    item.draggable = Some(true);
                    item.drag_data = Some(puzzle5d_catalog_item_drag_data(kind_id, entry));
                    item
                }
                None => tree_info_item(format!("{section_id}.{index}.{kind_id}"), catalog_kind_label(entry), Some(kind_id.into())),
            }
        })
        .collect();
    UiTreeSectionNode {
        presence: UiPresence::default(),
        id: section_id.into(),
        label: Some(label.into()),
        default_open: Some(!items.is_empty()),
        items: if items.is_empty() { vec![tree_info_item(format!("{section_id}.empty"), none_label, None)] } else { items },
    }
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle5dScene, labels: &Puzzle5dLabels) -> UiNode {
    let catalogs = envelope.document.kind_catalogs.clone().unwrap_or(json!({}));
    let slice = |key: &str| catalogs.get(key).and_then(|value| value.as_array()).cloned().unwrap_or_default();
    let mut part_entries = slice("parts");
    if part_entries.is_empty() {
        let mut ids: Vec<String> = envelope.document.parts.iter().map(|part| part.part_kind.clone()).collect();
        ids.sort();
        ids.dedup();
        part_entries = ids.into_iter().map(|id| json!({ "id": id, "name": id })).collect();
    }
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        sections: vec![
            kind_catalog_section("puzzle5d-play-kinds.parts", labels.parts, &part_entries, Some("addPartKind"), labels.none),
            kind_catalog_section("puzzle5d-play-kinds.grips", labels.grips, &slice("grips"), None, labels.none),
            kind_catalog_section("puzzle5d-play-kinds.fasteners", labels.fasteners, &slice("fasteners"), None, labels.none),
            kind_catalog_section("puzzle5d-play-kinds.ropes", labels.ropes, &slice("ropes"), None, labels.none),
        ],
        drop_action: None,
        menu: None,
        interaction_domain: None,
    })
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
