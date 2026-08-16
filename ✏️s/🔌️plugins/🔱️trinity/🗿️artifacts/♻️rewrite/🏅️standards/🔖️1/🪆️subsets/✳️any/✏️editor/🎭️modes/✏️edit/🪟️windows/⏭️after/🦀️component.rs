//! ➡️ Trinity Rewrite app — After window (read-only node-graph over the rule-applied result graph).

use crate::editor::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::UiNode;

pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> UiNode {
    let fixture_json = crate::editor::rewrite::after_fixture_json(state);
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_AFTER, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_AFTER, &fixture_json, cfg, false, None)
}
