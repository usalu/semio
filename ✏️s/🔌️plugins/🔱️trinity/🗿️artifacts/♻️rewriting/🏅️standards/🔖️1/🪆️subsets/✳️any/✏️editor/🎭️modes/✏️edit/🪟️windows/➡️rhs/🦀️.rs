//! ➡️ Trinity Rewriting app — RHS window (editable semantic node-graph over the rule's right-hand side).

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewriting::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
    crate::editor::rewriting::render_fixture_graph(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_RHS, crate::editor::rewriting::TRINITY_REWRITING_PLAY_WINDOW_RHS, &fixture_json, cfg, true, None)
}
