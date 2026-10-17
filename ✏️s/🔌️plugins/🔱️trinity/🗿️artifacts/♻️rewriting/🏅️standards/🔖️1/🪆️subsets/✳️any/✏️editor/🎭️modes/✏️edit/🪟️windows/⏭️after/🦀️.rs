//! ➡️ Trinity Rewriting app — After window (read-only node-graph over the rule-applied result graph).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
pub fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let snapshot_json = crate::editor::rewriting::document::rewritten_graph(state).map_err(|error|semio_framework_plugin::PluginAssemblyError::new("trinity.graph.invalid",error))?;
    crate::editor::rewriting::ui::render_graph_snapshot(crate::editor::rewriting::ids::TRINITY_REWRITING_PLAY_SURFACE_AFTER, &snapshot_json, cfg, false)
}
