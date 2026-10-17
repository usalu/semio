//! ⬅️ Trinity Rewriting app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
pub fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let snapshot_json = crate::editor::rewriting::document::lhs_graph_snapshot(&state.lhs, &state.rule_layout);
    crate::editor::rewriting::ui::render_graph_snapshot(crate::editor::rewriting::ids::TRINITY_REWRITING_PLAY_SURFACE_LHS, &snapshot_json, cfg, true)
}
