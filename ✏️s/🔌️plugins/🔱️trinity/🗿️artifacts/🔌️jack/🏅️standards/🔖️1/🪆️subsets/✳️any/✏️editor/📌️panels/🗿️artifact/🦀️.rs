//! 📄️ Trinity Jack app — Document panel (node/edge tree).

use crate::editor::jack::terminology::TrinityJackLabels;
use crate::JackSnapshot;
use semio_framework_plugin::{tree_item, tree_item_desc, PanelTreeBuilder};

pub(crate) fn render(fixture: &JackSnapshot, _cfg: &semio_framework_plugin::NoConfig, labels: &TrinityJackLabels) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let builder = PanelTreeBuilder::new("trinity-document")?;
    let scene = crate::jack_working_scene(fixture);
    // 🕹️ Domain "ast" targets nodes by their RAW document id (matching `interaction_topology` and the
    // node-graph surface's own pick targets) — NOT the namespaced `builder.item_id(...)?` convention,
    // so a click here and a click on the graph canvas land in the same selection.
    let node_items = crate::editor::jack::ui_node_list(scene.nodes.iter().map(|node| {
        let label = crate::editor::jack::ui_label(if node.name.is_empty() { node.id.as_str() } else { node.name.as_str() })?;
        tree_item_desc(node.id.clone(), label, Some(node.kind.clone()))
    }))?;
    let edge_items = crate::editor::jack::ui_node_list(scene.edges.iter().map(|edge| {
        let label = crate::editor::jack::ui_label(format!("{} → {}", edge.source, edge.target))?;
        tree_item(builder.item_id("edge", &edge.id)?, label)
    }))?;
    builder
        .section("trinity-document.nodes", Some(crate::editor::jack::ui_label(labels.pieces.as_str())?), true, node_items)?
        .section("trinity-document.edges", Some(crate::editor::jack::ui_label(labels.connections.as_str())?), false, edge_items)?
        .interaction_domain("ast")?
        .build()
}
