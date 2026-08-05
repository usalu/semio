//! 📄️ Puzzle 2d play app panel — the document tree: one row per node and per edge, each selecting
//! its entity, with the current selection mirrored back as the tree's selected ids.

use crate::apps::puzzle2d::terminology::Puzzle2dLabels;
use crate::apps::puzzle2d::{fixture_edges, fixture_nodes, puzzle2d_action, Puzzle2dScene};
use semio_framework_plugin::{
    tree_item_with_action, Label, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, PanelTreeBuilder, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::{json, Value};

//#region 🔖️Constants
pub const PUZZLE2D_PLAY_BODY_LAYERS: &str = "puzzle2d.play.layers";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition {
        kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()),
        label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"),
        group: PanelGroup::Workbench,
        body_key: Some(PUZZLE2D_PLAY_BODY_LAYERS.into()),
        children: Vec::new(),
    }
}
//#endregion 🔖️Definition

//#region 🔖️Render
fn node_label(node: &Value) -> String {
    node.get("text").and_then(|value| value.as_str()).filter(|value| !value.is_empty()).or_else(|| node.get("id").and_then(|value| value.as_str())).unwrap_or("node").into()
}

fn edge_label(edge: &Value, fixture: &Value) -> String {
    let source = edge.get("source").and_then(|value| value.as_str()).unwrap_or("?");
    let target = edge.get("target").and_then(|value| value.as_str()).unwrap_or("?");
    let source_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(source)).map(node_label).unwrap_or_else(|| source.into());
    let target_label = fixture_nodes(fixture).iter().find(|node| node.get("id").and_then(|value| value.as_str()) == Some(target)).map(node_label).unwrap_or_else(|| target.into());
    format!("{source_label} → {target_label}")
}

fn document_tree_selected_ids(fixture: &Value, selected: &[String]) -> Vec<String> {
    selected
        .iter()
        .filter_map(|id| {
            if fixture_nodes(fixture).iter().any(|node| node.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(format!("puzzle2d-play-document.node.{id}"));
            }
            if fixture_edges(fixture).iter().any(|edge| edge.get("id").and_then(|value| value.as_str()) == Some(id.as_str())) {
                return Some(format!("puzzle2d-play-document.edge.{id}"));
            }
            None
        })
        .collect()
}

pub fn render(envelope: &Puzzle2dScene, labels: &Puzzle2dLabels) -> UiNode {
    let fixture = &envelope.fixture;
    let node_items: Vec<UiTreeItemNode> = fixture_nodes(fixture)
        .iter()
        .filter_map(|node| {
            let id = node.get("id")?.as_str()?;
            Some(tree_item_with_action(
                format!("puzzle2d-play-document.node.{id}"),
                Label::data(node_label(node)),
                node.get("nodeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_action("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture_edges(fixture)
        .iter()
        .filter_map(|edge| {
            let id = edge.get("id")?.as_str()?;
            Some(tree_item_with_action(
                format!("puzzle2d-play-document.edge.{id}"),
                Label::data(edge_label(edge, fixture)),
                edge.get("edgeKind").and_then(|value| value.as_str()).map(str::to_string),
                puzzle2d_action("setSelection", Some(json!({ "ids": [id] }))),
            ))
        })
        .collect();
    PanelTreeBuilder::new("puzzle2d-play-document")
        .section_or_placeholder("puzzle2d-play-document.nodes", Some(labels.nodes.into()), true, node_items, labels.none)
        .section_or_placeholder("puzzle2d-play-document.edges", Some(labels.edges.into()), false, edge_items, labels.none)
        .selected(document_tree_selected_ids(fixture, &envelope.runtime.selected_ids))
        .selection_change(puzzle2d_action("setSelection", None))
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::puzzle2d::testkit::*;

    #[test]
    fn document_panel_lists_nodes_section() {
        let mut app = concrete_forest_app();
        let json = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
        assert!(json.contains("puzzle2d-play-document.nodes"));
        assert!(json.contains("seed-left-001"));
    }

    /// 🗣️ B1: locale/terminology are now real VCS'd `Puzzle2dConfig` state (was a per-call `ViewState`
    /// override) — dispatch `setLocale`/`setTerminology` to change them, then render.
    #[test]
    fn labels_resolve_native_english_and_german_and_reuse() {
        let mut app = concrete_forest_app();
        let english = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
        assert!(english.contains("\"Nodes\"") && english.contains("\"Edges\""));
        dispatch(&mut app, "setLocale", Some(&json!({ "value": "de" })), None).expect("setLocale");
        let german = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
        assert!(german.contains("\"Knoten\"") && german.contains("\"Kanten\""));
        dispatch(&mut app, "setLocale", Some(&json!({ "value": "en" })), None).expect("setLocale");
        dispatch(&mut app, "setTerminology", Some(&json!({ "value": "reuse" })), None).expect("setTerminology");
        let reuse = render_body(&mut app, PUZZLE2D_PLAY_BODY_LAYERS);
        assert!(reuse.contains("Building components"));
    }
}
//#endregion 🧪️Tests
