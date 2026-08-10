//! 📄️ Trinity Jack app — Document panel (node/edge tree).

use crate::apps::jack::config::JackConfig;
use crate::apps::jack::terminology::TrinityJackLabels;
use crate::artifacts::jack::JackSnapshot;
use semio_framework_plugin::{tree_item, tree_item_with_action, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};
use serde_json::json;

pub(crate) fn render(fixture: &JackSnapshot, cfg: &JackConfig, labels: &TrinityJackLabels) -> UiNode {
    let jack_action = crate::apps::jack::jack_action;
    let builder = PanelTreeBuilder::new("trinity-document");
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(builder.item_id("node", &node.id), Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(node.kind.clone()), jack_action("setSelection", Some(json!({ "ids": [node.id] }))))
        })
        .collect();
    let edge_items: Vec<UiTreeItemNode> = fixture.edges.iter().map(|edge| tree_item(builder.item_id("edge", &edge.id), Label::data(format!("{} → {}", edge.source, edge.target)))).collect();
    let selected = cfg.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
    builder
        .section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)
        .section("trinity-document.edges", Some(labels.connections.into()), false, edge_items)
        .selected(selected)
        .selection_change(jack_action("setSelection", Some(json!({ "ids": [] }))))
        .build()
}
