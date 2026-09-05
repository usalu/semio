//! 📄️ Trinity Rewriting app — Document panel (before-fixture node tree).

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
use crate::editor::rewriting::terminology::TrinityRewritingLabels;
use semio_framework_plugin::{tree_item_desc, PanelTreeBuilder};

pub(crate) fn render(state: &RewritingSnapshot, _cfg: &RewritingConfig, labels: &TrinityRewritingLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(fixture) = crate::editor::rewriting::parse_fixture_json(&state.before_fixture_json) else {
        return Err(semio_framework_plugin::PluginAssemblyError::new("trinity.fixture.invalid", "invalid Trinity fixture"));
    };
    let builder = PanelTreeBuilder::new("trinity-document")?;
    // 🕹️ Domain "graph" targets nodes by their RAW document id (matching `interaction_topology` and
    // the node-graph surface's own pick targets) — NOT the namespaced `builder.item_id(...)?`
    // convention, so a click here and a click on the graph canvas land in the same selection.
    let node_items = crate::editor::rewriting::ui_node_list(fixture.nodes().iter().map(|node| {
        let label = crate::editor::rewriting::ui_label(if node.name.is_empty() { node.id.as_str() } else { node.name.as_str() })?;
        tree_item_desc(node.id.clone(), label, Some(node.kind.clone()))
    }))?;
    builder.section("trinity-document.nodes", Some(labels.pieces.as_str().into()), true, node_items)?.interaction_domain("graph")?.build()
}
