//! ⬅️ Trinity Rewrite app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewrite::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_LHS, &fixture_json, cfg, true, None)
}
