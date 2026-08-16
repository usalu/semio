//! ⬅️ Trinity Rewrite app — Before window (editable node-graph over `before_fixture_json`).

use crate::editor::rewrite::config::RewriteConfig;
use crate::artifacts::rewrite::RewriteSnapshot;
use semio_framework_plugin::UiNode;

pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> UiNode {
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_BEFORE, &state.before_fixture_json, cfg, true, Some(&cfg.before_pane_camera))
}
