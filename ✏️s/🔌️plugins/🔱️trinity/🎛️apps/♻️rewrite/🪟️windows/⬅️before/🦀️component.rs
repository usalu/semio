//! ⬅️ Trinity Rewrite app — Before window (editable node-graph over `before_fixture_json`).

use crate::apps::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteRuleModel;
use semio_framework_plugin::UiNode;

pub(crate) fn render(state: &RewriteRuleModel, cfg: &RewriteConfig) -> UiNode {
    crate::apps::rewrite::render_fixture_graph(crate::apps::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE, crate::apps::rewrite::TRINITY_REWRITE_PLAY_WINDOW_BEFORE, &state.before_fixture_json, cfg, true, Some(&cfg.before_pane_camera))
}
