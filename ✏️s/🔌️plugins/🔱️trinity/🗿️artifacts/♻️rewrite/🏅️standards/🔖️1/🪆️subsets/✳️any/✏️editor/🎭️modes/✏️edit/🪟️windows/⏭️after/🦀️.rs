//! ➡️ Trinity Rewrite app — After window (read-only node-graph over the rule-applied result graph).

use crate::artifacts::rewrite::RewriteSnapshot;
use crate::editor::rewrite::config::RewriteConfig;
pub(crate) fn render(state: &RewriteSnapshot, cfg: &RewriteConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewrite::after_fixture_json(state);
    crate::editor::rewrite::render_fixture_graph(crate::editor::rewrite::TRINITY_REWRITE_PLAY_SURFACE_AFTER, crate::editor::rewrite::TRINITY_REWRITE_PLAY_WINDOW_AFTER, &fixture_json, cfg, false, None)
}
