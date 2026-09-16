//! 📄️ Trinity Rewriting app — Document panel (before-fixture node tree).

use crate::editor::rewriting::terminology::TrinityRewritingLabels;
use crate::RewritingSnapshot;
use semio_framework_plugin::NoConfig;
use semio_framework_plugin::plugin_app_close_prelude::{Buildable, HasBase};
use semio_framework_plugin::{PanelTreeBuilder, TreeWindows};
use semio_framework_ui_contract as ui;

//#region 🔖️Render
/// 🕹️ One `"graph"`-domain pick row: keyed by the node's RAW document id (matching
/// `interaction_topology` and the node-graph surface's own pick targets, so a click here and a click on
/// the graph canvas land in the same selection) and marked with the domain's `"node"` granularity. The
/// tree-level `.interaction_domain(...)` binding carries the dispatch, so the row binds no action and
/// spends no `UiValue` arena.
fn node_row(node: &semio_s_artifact_trinity_jack::Node) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let label = crate::editor::rewriting::ui_label(if node.name.is_empty() { node.id.as_str() } else { node.name.as_str() })?;
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

pub(crate) fn render(state: &RewritingSnapshot, _cfg: &NoConfig, labels: &TrinityRewritingLabels, windows: &TreeWindows<'_>) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let Some(fixture) = crate::editor::rewriting::parse_fixture_json(&state.before_fixture_json) else {
        return Err(semio_framework_plugin::PluginAssemblyError::new("trinity.fixture.invalid", "invalid Trinity fixture"));
    };
    let nodes = fixture.nodes();
    PanelTreeBuilder::new("trinity-document")?
        .window_section(windows, "trinity-document.nodes", Some(crate::editor::rewriting::ui_label(labels.pieces.as_str())?), true, &nodes, node_row)?
        .interaction_domain(crate::editor::rewriting::TRINITY_REWRITING_PLAY_CONTROLLER_ID, "graph")?
        .build()
}
//#endregion 🔖️Render

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
