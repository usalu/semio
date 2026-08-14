//! 🛍️ Puzzle 3d play app panel — the kind catalogue: the object kinds available to place (draggable
//! into the viewport, with their rim-vortex templates nested) plus the vortex/cable/attraction kind
//! rows the fixture's `meta.kindCatalogs` declares.

use crate::apps::puzzle3d::terminology::Puzzle3dLabels;
use crate::apps::puzzle3d::{puzzle3d_action, Puzzle3dScene};
use semio_framework_plugin::{
    Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_CATALOGUE_ID, FRAMEWORK_PANEL_TAB_CATALOGUE_LABEL,
};
use serde_json::{json, Value};
use std::collections::HashMap;

//#region 🔖️Constants
pub const BODY_KEY: &str = "puzzle.3d.play.kinds";
/// 🖱️ MIME key `DeclarativeTreePanel` (framework/renderer/react/ui-interpreter.tsx) reads to auto-wire catalogue drag sources.
pub const PUZZLE3D_CATALOGUE_DRAG_MIME: &str = "application/x-semio-catalogue-item";
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
fn catalog_entry_label(entry: &Value) -> String {
    entry.get("label").and_then(|value| value.as_str()).or_else(|| entry.get("name").and_then(|value| value.as_str())).or_else(|| entry.get("id").and_then(|value| value.as_str())).unwrap_or("kind").into()
}

fn object_kind_vortex_items(entry: &Value) -> Vec<UiTreeItemNode> {
    entry
        .get("vortices")
        .and_then(|value| value.as_array())
        .map(|templates| {
            templates
                .iter()
                .enumerate()
                .map(|(index, template)| {
                    let vortex_kind = template.get("vortexKind").and_then(|value| value.as_str()).unwrap_or("vortex");
                    let position = template.get("position").cloned().unwrap_or(json!([0.0, 0.0, 0.0]));
                    UiTreeItemNode {
                        presence: UiPresence::default(),
                        id: format!("puzzle3d-kind-vortex.{index}.{vortex_kind}"),
                        label: Label::data(vortex_kind),
                        description: Some(position.to_string()),
                        icon_id: Some("circle-dot".into()),
                        default_open: None,
                        action: None,
                        actions: None,
                        draggable: None,
                        drag_data: None,
                        items: None,
                        control: None,
                        dimmed: None,
                        menu: None,
                    }
                })
                .collect()
        })
        .unwrap_or_default()
}

fn object_kind_item(entry: &Value) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    let mesh_url = entry
        .get("meshUrl")
        .and_then(|value| value.as_str())
        .filter(|url| !url.is_empty())
        .map(str::to_string)
        .or_else(|| {
            entry
                .get("representations")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|rep| rep.get("url").and_then(Value::as_str))
                .find(|url| !url.is_empty())
                .map(str::to_string)
        });
    let draggable = mesh_url.is_some();
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: kind_id.clone(),
        label: Label::data(catalog_entry_label(entry)),
        description: Some(kind_id.clone()),
        icon_id: Some("box".into()),
        default_open: Some(false),
        action: Some(puzzle3d_action("addObjectKind", Some(json!({ "objectKind": kind_id })))),
        actions: None,
        draggable: draggable.then_some(true),
        drag_data: draggable.then(|| {
            let mut payload = json!({ "objectKind": kind_id });
            if let Some(url) = mesh_url {
                payload["meshUrl"] = json!(url);
            }
            HashMap::from([(PUZZLE3D_CATALOGUE_DRAG_MIME.to_string(), payload.to_string())])
        }),
        items: Some(object_kind_vortex_items(entry)),
        control: None,
        dimmed: None,
        menu: None,
    }
}

fn catalog_kind_item(entry: &Value, icon_id: &str) -> UiTreeItemNode {
    let kind_id = entry.get("id").and_then(|value| value.as_str()).unwrap_or("kind").to_string();
    UiTreeItemNode {
        presence: UiPresence::default(),
        id: format!("puzzle3d-kind-entry:{kind_id}"),
        label: Label::data(catalog_entry_label(entry)),
        description: Some(kind_id),
        icon_id: Some(icon_id.into()),
        default_open: None,
        action: None,
        actions: None,
        draggable: None,
        drag_data: None,
        items: None,
        control: None,
        dimmed: None,
        menu: None,
    }
}
//#endregion 🔖️Rows

//#region 🔖️Render
pub fn render(envelope: &Puzzle3dScene, labels: &Puzzle3dLabels) -> UiNode {
    let entries = |section: &str| crate::apps::puzzle3d::puzzle3d_catalog_entries(&envelope.fixture, section);
    let object_entries = entries("objects");
    let vortex_entries = entries("vortices");
    let cable_entries = entries("cables");
    let attraction_entries = entries("attractions");
    UiNode::Tree(UiTreeNode {
        presence: UiPresence::default(),
        interaction_domain: Some(crate::apps::puzzle3d::PUZZLE3D_INTERACTION_DOMAIN.into()),
        sections: vec![
            UiTreeSectionNode { id: "puzzle3d-play-kinds.objects".into(), label: Some(labels.objects.into()), default_open: Some(false), presence: UiPresence::default(), items: object_entries.iter().map(object_kind_item).collect() },
            UiTreeSectionNode {
                id: "puzzle3d-play-kinds.vortices".into(),
                label: Some(labels.vortices.into()),
                default_open: Some(false),
                presence: UiPresence::default(),
                items: vortex_entries.iter().map(|entry| catalog_kind_item(entry, "circle-dot")).collect(),
            },
            UiTreeSectionNode {
                id: "puzzle3d-play-kinds.cables".into(),
                label: Some(labels.cables.into()),
                default_open: Some(false),
                presence: UiPresence::default(),
                items: cable_entries.iter().map(|entry| catalog_kind_item(entry, "plug")).collect(),
            },
            UiTreeSectionNode {
                id: "puzzle3d-play-kinds.attractions".into(),
                label: Some(labels.attractions.into()),
                default_open: Some(false),
                presence: UiPresence::default(),
                items: attraction_entries.iter().map(|entry| catalog_kind_item(entry, "link")).collect(),
            },
        ],
        drop_action: None,
        menu: None,
    })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle3d::config::{Puzzle3dConfig, Puzzle3dRuntime};
    use crate::apps::puzzle3d::terminology::puzzle3d_labels;
    use crate::apps::puzzle3d::{nakagin_fixture, Puzzle3dScene, PUZZLE3D_DEFAULT_UTILITY};

    #[test]
    fn kinds_tree_object_drag_data_carries_object_kind_and_mesh_url() {
        let envelope = Puzzle3dScene { fixture: nakagin_fixture(), runtime: Puzzle3dRuntime::default(), active_utility: PUZZLE3D_DEFAULT_UTILITY.into() };
        let labels = puzzle3d_labels(&Puzzle3dConfig::default());
        let node = render(&envelope, labels);
        let tree = match node {
            UiNode::Tree(tree) => tree,
            _ => panic!("expected kinds tree"),
        };
        let objects = tree.sections.iter().find(|section| section.id == "puzzle3d-play-kinds.objects").expect("objects section");
        let draggable = objects.items.iter().find(|item| item.draggable == Some(true)).expect("draggable object kind");
        let drag_data = draggable.drag_data.as_ref().expect("drag data");
        let encoded = drag_data.get(PUZZLE3D_CATALOGUE_DRAG_MIME).expect("catalogue mime");
        let payload: Value = serde_json::from_str(encoded).expect("drag payload json");
        assert!(payload.get("objectKind").and_then(Value::as_str).is_some(), "drag payload must carry objectKind");
        assert!(payload.get("meshUrl").and_then(Value::as_str).filter(|url| !url.is_empty()).is_some(), "drag payload must carry meshUrl for preview");
    }
}
//#endregion 🧪️Tests
