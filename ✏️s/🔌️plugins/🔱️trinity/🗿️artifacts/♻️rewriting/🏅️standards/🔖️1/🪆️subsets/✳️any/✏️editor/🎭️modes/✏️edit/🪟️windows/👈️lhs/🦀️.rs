//! ⬅️ Trinity Rewriting app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::RewritingSnapshot;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let snapshot_json = crate::editor::rewriting::lhs_graph_snapshot(&state.lhs, &state.rule_layout);
    crate::editor::rewriting::render_graph_snapshot(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS, &snapshot_json, cfg, true)
}
