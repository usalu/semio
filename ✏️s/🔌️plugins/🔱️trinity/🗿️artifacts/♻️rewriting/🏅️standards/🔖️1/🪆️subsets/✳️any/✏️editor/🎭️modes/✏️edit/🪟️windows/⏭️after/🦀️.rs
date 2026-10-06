//! ➡️ Trinity Rewriting app — After window (read-only node-graph over the rule-applied result graph).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::RewritingSnapshot;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let snapshot_json = crate::editor::rewriting::rewritten_graph(state).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("trinity.graph.invalid",error))?;
    crate::editor::rewriting::render_graph_snapshot(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_AFTER, &snapshot_json, cfg, false)
}
