//! 📄️ DAG play app panel — the node/edge outline tree.

use crate::apps::dag::terminology::DagPlayLabels;
use crate::apps::dag::DAG_PLAY_INTERACTION_DOMAIN;
use crate::artifacts::dag::DagSnapshot;
use infinite_board_port_directed_dag::dag_node_kind_tag;
use semio_framework_plugin::{tree_item_desc, PanelTreeBuilder, LocalizedLabel, PanelGroup, PanelTabDefinition, PanelTabKind, UiNode, UiTreeItemNode, FRAMEWORK_PANEL_TAB_ARTIFACT_ID, FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL};

//#region 🔖️Constants
pub const DAG_PLAY_BODY_DOCUMENT: &str = "dag.play.document";
//#endregion 🔖️Constants

//#region 🔖️Definition
pub fn definition() -> PanelTabDefinition {
    PanelTabDefinition { kind: PanelTabKind::App(FRAMEWORK_PANEL_TAB_ARTIFACT_ID.into()), label: LocalizedLabel::native(FRAMEWORK_PANEL_TAB_ARTIFACT_LABEL, "Dokument"), group: PanelGroup::Workbench, body_key: Some(DAG_PLAY_BODY_DOCUMENT.into()), children: Vec::new() }
}
//#endregion 🔖️Definition

//#region 🔖️Render
/// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: item ids are the SAME raw node/edge
/// ids `DagPlayApp::interaction_topology` registers for the `graph` domain — the framework stamps this
/// tree's selection/hover presence from that domain (`.interaction_domain`) and prunes stale ids
/// through that same topology, so no per-item click action is declared here anymore (clicks are
/// translated into `interactionSelect` generically).
pub fn render(document: &DagSnapshot, labels: &DagPlayLabels) -> UiNode {
    let scene = crate::artifacts::dag::dag_working_scene(document);
    let node_items: Vec<UiTreeItemNode> = scene
        .nodes
        .iter()
        .map(|node| tree_item_desc(node.id.clone(), semio_framework_plugin::Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(dag_node_kind_tag(&node.kind).into())))
        .collect();
    let edge_items: Vec<UiTreeItemNode> = scene.edges.iter().map(|edge| tree_item_desc(edge.id.clone(), semio_framework_plugin::Label::data(format!("{} → {}", edge.source, edge.target)), Some(edge.id.clone()))).collect();
    PanelTreeBuilder::new("dag-play-document")
        .section_or_placeholder("dag-play-document.nodes", Some(labels.nodes.into()), true, node_items, labels.empty)
        .section_or_placeholder("dag-play-document.edges", Some(labels.edges.into()), false, edge_items, labels.empty)
        .interaction_domain(DAG_PLAY_INTERACTION_DOMAIN)
        .build()
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
        assert_eq!(definition.id(), FRAMEWORK_PANEL_TAB_ARTIFACT_ID);
        assert_eq!(definition.body_key.as_deref(), Some(DAG_PLAY_BODY_DOCUMENT));
    }
}
//#endregion 🧪️Tests
