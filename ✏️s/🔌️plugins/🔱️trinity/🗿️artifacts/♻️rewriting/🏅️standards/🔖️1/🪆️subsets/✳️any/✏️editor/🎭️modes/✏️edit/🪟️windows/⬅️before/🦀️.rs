//! ⬅️ Trinity Rewriting app — Before window (editable node-graph over `before_fixture_json`).

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::editor::rewriting::render_fixture_graph(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_BEFORE, crate::editor::rewriting::TRINITY_REWRITING_PLAY_WINDOW_BEFORE, &state.before_fixture_json, cfg, true, Some(&cfg.before_pane_camera))
}
