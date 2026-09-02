//! ➡️ Trinity Rewrite app — RHS window (editable semantic node-graph over the rule's right-hand side).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewrite::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_RHS, &fixture_json, cfg, true, None)
}
