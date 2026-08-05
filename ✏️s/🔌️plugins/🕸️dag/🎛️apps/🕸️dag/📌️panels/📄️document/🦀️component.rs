//! 📄️ DAG play app panel — the node/edge outline tree.

use crate::apps::dag::terminology::DagPlayLabels;
use crate::apps::dag::dag_action;
use crate::artifacts::dag::DagDocument;
use infinite_board_port_directed_dag::dag_node_kind_tag;
use semio_framework_plugin::{
    tree_item, tree_item_desc, tree_item_with_action, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiPresence, UiTreeItemNode, UiTreeNode, UiTreeSectionNode, FRAMEWORK_PANEL_TAB_DOCUMENT_ID, FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL,
};
use serde_json::json;

//#region 🔖️Constants
pub const DAG_PLAY_BODY_DOCUMENT: &str = "dag.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_DOCUMENT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_DOCUMENT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(DAG_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
pub fn render(document: &DagDocument, selected: &[String], labels: &DagPlayLabels) -> UiNode {
    let node_items: Vec<UiTreeItemNode> = document
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(
                format!("dag-play-document.node.{}", node.id),
                semio_framework_plugin::Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }),
                Some(dag_node_kind_tag(&node.kind).into()),
                dag_action("setSelection", Some(json!({ "ids": [node.id.clone()] }))),
            )
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = document.edges.iter().map(|edge| tree_item_desc(format!("dag-play-document.edge.{}", edge.id), semio_framework_plugin::Label::data(format!("{} → {}", edge.source, edge.target)), Some(edge.id.clone()))).collect();
    let mut sections = vec![
        UiTreeSectionNode {
            id: "dag-play-document.nodes".into(),
            label: Some(labels.nodes.into()),
            default_open: Some(true),
            presence: UiPresence::default(),
            items: if node_items.is_empty() { vec![tree_item("dag-play-document.nodes.empty", labels.empty)] } else { node_items },
        },
        UiTreeSectionNode {
            id: "dag-play-document.edges".into(),
            label: Some(labels.edges.into()),
            default_open: Some(false),
            presence: UiPresence::default(),
            items: if edge_items.is_empty() { vec![tree_item("dag-play-document.edges.empty", labels.empty)] } else { edge_items },
        },
    ];
    let selected_ids: std::collections::HashSet<String> = selected.iter().map(|id| format!("dag-play-document.node.{id}")).collect();
    semio_framework_plugin::ui_tree_stamp_presence(&mut sections, &selected_ids, &std::collections::HashSet::new());
    UiNode::Tree(UiTreeNode { sections, presence: UiPresence::default(), selected_ids: None, highlighted_ids: None, selection_change: None, drop_action: None, menu: None })
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::dag::testkit::{new_app, render as render_body};

    #[test]
    fn dag_play_labels_resolve_native_by_default() {
        let mut app = new_app();
        let json = render_body(&mut app, DAG_PLAY_BODY_DOCUMENT);
        assert!(json.contains("Nodes"));
        assert!(json.contains("Edges"));
    }

    #[test]
    fn definition_binds_the_framework_document_tab_to_this_body_key() {
        let definition = definition();
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_DOCUMENT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
