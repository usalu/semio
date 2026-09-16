//! 📄️ Trinity Jack app — Document panel (node/edge tree).

use crate::editor::jack::terminology::TrinityJackLabels;
use crate::JackSnapshot;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{PanelTreeBuilder, TreeWindows};
use semio_framework_ui_contract as ui;

//#region 🔖️Render
/// 🕹️ One `"ast"`-domain pick row: keyed by the node's RAW document id (matching `interaction_topology`
/// and the node-graph surface's own pick targets, so a click here and a click on the canvas land in the
/// same selection) and marked with the domain's `"node"` granularity. The tree-level
/// `.interaction_domain(...)` binding carries the dispatch, so the row itself binds no action and spends
/// no `UiValue` arena — which is what lets a full window of rows exist at all.
fn node_row(node: &crate::Node) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label = crate::editor::jack::ui_label(if node.name.is_empty() { node.id.as_str() } else { node.name.as_str() })?;
    let granularity = semio_framework_plugin::UiText::try_from_str("node").ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("trinity.document.granularity", "the fixed pick granularity exceeds its UI bound"))?;
    let mut builder = ui::tree_item(label)
        .try_id(node.id.clone())
        .map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.document.node-id", "the node row id exceeds its UI bound"))?
        .granularity(granularity);
    if !node.kind.is_empty() {
        builder = builder.description(semio_framework_plugin::UiText::try_from_str(node.kind.as_str()).ok_or_else(|| semio_framework_plugin::PluginAssemblyError::new("trinity.document.node-kind", "the node kind exceeds its UI bound"))?);
    }
    builder.try_build().map_err(|_| semio_framework_plugin::PluginAssemblyError::new("trinity.document.node", "the node row exceeds its UI bound"))
}

fn edge_row(edge: &crate::Edge) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label = crate::editor::jack::ui_label(format!("{} → {}", edge.source, edge.target))?;
    semio_framework_plugin::tree_item(format!("trinity-document.edge.{}", edge.id), label)
}

pub(crate) fn render(snapshot: &JackSnapshot, _cfg: &semio_framework_plugin::NoConfig, labels: &TrinityJackLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let scene = crate::jack_working_scene(snapshot);
    PanelTreeBuilder::new("trinity-document")?
        .window_section(windows, "trinity-document.nodes", Some(crate::editor::jack::ui_label(labels.pieces.as_str())?), true, &scene.nodes, node_row)?
        .window_section(windows, "trinity-document.edges", Some(crate::editor::jack::ui_label(labels.connections.as_str())?), false, &scene.edges, edge_row)?
        .interaction_domain(crate::editor::jack::TRINITY_JACK_PLAY_CONTROLLER_ID, "ast")?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
