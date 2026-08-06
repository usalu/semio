//! 📄️ Trinity Rewrite app — Document panel (before-fixture node tree).

use crate::apps::rewrite::config::RewriteConfig;
use crate::apps::rewrite::terminology::TrinityRewriteLabels;
use crate::artifacts::rewrite::RewriteRuleModel;
use semio_framework_plugin::{tree_item_with_action, Label, PanelTreeBuilder, UiNode, UiTreeItemNode, ui_text};
use serde_json::json;

pub(crate) fn render(state: &RewriteRuleModel, cfg: &RewriteConfig, labels: &TrinityRewriteLabels) -> UiNode {
    let Some(fixture) = crate::apps::rewrite::parse_fixture_json(&state.before_fixture_json) else {
        return ui_text(Label::data("Invalid trinity fixture"));
    };
    let jack_action = crate::apps::rewrite::rewrite_action;
    let builder = PanelTreeBuilder::new("trinity-document");
    let node_items: Vec<UiTreeItemNode> = fixture
        .nodes
        .iter()
        .map(|node| {
            tree_item_with_action(builder.item_id("node", &node.id), Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(node.kind.clone()), jack_action("setSelection", Some(json!({ "ids": [node.id] }))))
        })
        .collect();
    let selected = cfg.selected_node_ids.iter().map(|id| builder.item_id("node", id)).collect();
    builder.section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items).selected(selected).selection_change(jack_action("setSelection", Some(json!({ "ids": [] })))).build()
}
