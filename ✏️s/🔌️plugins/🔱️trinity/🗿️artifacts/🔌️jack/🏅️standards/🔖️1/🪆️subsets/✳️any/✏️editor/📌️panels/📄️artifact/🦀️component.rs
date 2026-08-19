//! 📄️ Trinity Jack app — Document panel (node/edge tree).

use crate::editor::jack::config::JackConfig;
use crate::editor::jack::terminology::TrinityJackLabels;
use crate::artifacts::jack::JackSnapshot;
use semio_framework_plugin::{tree_item, tree_item_desc, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};

pub(crate) async fn render(fixture: &JackSnapshot, _cfg: &JackConfig, labels: &TrinityJackLabels) -> UiNode {
    let builder = PanelTreeBuilder::new("trinity-document");
    let scene = crate::artifacts::jack::jack_working_scene(fixture);
    // 🕹️ Domain "ast" targets nodes by their RAW document id (matching `interaction_topology` and the
    // node-graph surface's own pick targets) — NOT the namespaced `builder.item_id(...)` convention,
    // so a click here and a click on the graph canvas land in the same selection.
    let node_items: Vec<UiTreeItemNode> =
        scene.nodes.iter().map(|node| tree_item_desc(node.id.clone(), Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(node.kind.clone()))).collect();
    let edge_items: Vec<UiTreeItemNode> = scene.edges.iter().map(|edge| tree_item(builder.item_id("edge", &edge.id), Label::data(format!("{} → {}", edge.source, edge.target)))).collect();
    builder
        .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
        .section("trinity-document.edges", Some(labels.connections.into()), false, edge_items)
        .interaction_domain("ast")
        .build()
}
