//! ⬅️ Trinity Rewriting app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewriting::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
    crate::editor::rewriting::render_fixture_graph(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS, crate::editor::rewriting::TRINITY_REWRITING_PLAY_WINDOW_LHS, &fixture_json, cfg, true, None)
}
