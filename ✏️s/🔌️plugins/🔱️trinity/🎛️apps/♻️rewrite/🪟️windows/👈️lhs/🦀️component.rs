//! ⬅️ Trinity Rewrite app — LHS window (editable semantic node-graph over the rule's left-hand side).

use crate::apps::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::UiNode;

pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> UiNode {
    let fixture_json = crate::apps::rewrite::lhs_graph_fixture_json(&state.lhs_json, &state.rule_layout);
    crate::apps::rewrite::render_fixture_graph(crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_LHS, crate::apps::rewrite::TRINITY_REWRITE_PLAY_WINDOW_LHS, &fixture_json, cfg, true, None)
}
