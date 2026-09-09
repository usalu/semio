//! ⬅️ Trinity Rewriting app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::RewritingSnapshot;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewriting::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
    crate::editor::rewriting::render_fixture_graph(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_LHS, &fixture_json, cfg, true)
}
