//! 📄️ Trinity Rewrite app — Document panel (before-fixture node tree).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
use crate::editor::rewrite::terminology::TrinityRewriteLabels;
use semio_framework_plugin::{tree_item_desc, PanelTreeBuilder};

pub(crate) fn render(state: &RewriteSnapshot, _cfg: &RewriteConfig, labels: &TrinityRewriteLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(fixture) = crate::editor::rewrite::parse_fixture_json(&state.before_fixture_json) else {
        return Err(semio_framework_plugin::PluginAssemblyError::new("trinity.fixture.invalid", "invalid Trinity fixture"));
    };
    let builder = PanelTreeBuilder::new("trinity-document")?;
    // 🕹️ Domain "graph" targets nodes by their RAW document id (matching `interaction_topology` and
    // the node-graph surface's own pick targets) — NOT the namespaced `builder.item_id(...)?`
    // convention, so a click here and a click on the graph canvas land in the same selection.
    let node_items = crate::editor::rewrite::ui_node_list(fixture.nodes().iter().map(|node| {
        let label = crate::editor::rewrite::ui_label(if node.name.is_empty() { node.id.as_str() } else { node.name.as_str() })?;
        tree_item_desc(node.id.clone(), label, Some(node.kind.clone()))
    }))?;
    builder.section("trinity-document.nodes", Some(labels.pieces.as_str().into()), true, node_items)?.interaction_domain("graph")?.build()
}
