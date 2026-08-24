//! 📄️ Trinity Rewrite app — Document panel (before-fixture node tree).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
use crate::editor::rewrite::terminology::TrinityRewriteLabels;
use semio_framework_plugin::{tree_item_desc, ui_text, Label, PanelTreeBuilder, UiNode, UiTreeItemNode};

pub(crate) async fn render(state: &RewriteSnapshot, _cfg: &RewriteConfig, labels: &TrinityRewriteLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(fixture) = crate::editor::rewrite::parse_fixture_json(&state.before_fixture_json) else {
        return ui_text(Label::data("Invalid trinity fixture"));
    };
    let builder = PanelTreeBuilder::new("trinity-document")?;
    // 🕹️ Domain "graph" targets nodes by their RAW document id (matching `interaction_topology` and
    // the node-graph surface's own pick targets) — NOT the namespaced `builder.item_id(...)?`
    // convention, so a click here and a click on the graph canvas land in the same selection.
    let node_items: Vec<UiTreeItemNode> = fixture.nodes().iter().map(|node| tree_item_desc(node.id.clone(), Label::data(if node.name.is_empty() { node.id.clone() } else { node.name.clone() }), Some(node.kind.clone()))?).collect();
    builder.section("trinity-document.nodes", Some(labels.pieces.into()), true, node_items)?.interaction_domain("graph")?.build()
}
