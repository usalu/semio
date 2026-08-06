//! ➡️ Trinity Rewrite app — RHS window (editable semantic node-graph over the rule's right-hand side).

use crate::apps::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteRuleModel;
use semio_framework_plugin::UiNode;

pub(crate) fn render(state: &RewriteRuleModel, cfg: &RewriteConfig) -> UiNode {
    let fixture_json = crate::apps::rewrite::rhs_graph_fixture_json(&state.rhs_json, &state.rule_layout);
    crate::apps::rewrite::render_fixture_graph(crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_RHS, crate::apps::rewrite::TRINITY_REWRITE_PLAY_WINDOW_RHS, &fixture_json, cfg, true, None)
}
