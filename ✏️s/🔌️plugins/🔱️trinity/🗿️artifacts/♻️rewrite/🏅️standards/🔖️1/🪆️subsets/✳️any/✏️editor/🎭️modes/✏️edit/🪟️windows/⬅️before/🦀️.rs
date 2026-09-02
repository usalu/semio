//! ⬅️ Trinity Rewrite app — Before window (editable node-graph over `before_fixture_json`).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_BEFORE, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_BEFORE, &state.before_fixture_json, cfg, true, Some(&cfg.before_pane_camera))
}
