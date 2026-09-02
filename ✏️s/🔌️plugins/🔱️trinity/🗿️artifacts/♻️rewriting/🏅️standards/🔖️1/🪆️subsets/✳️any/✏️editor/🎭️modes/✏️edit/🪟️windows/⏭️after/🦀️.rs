//! ➡️ Trinity Rewriting app — After window (read-only node-graph over the rule-applied result graph).

use crate::artifacts::rewriting::RewritingSnapshot;
use crate::editor::rewriting::config::RewritingConfig;
pub(crate) fn render(state: &RewritingSnapshot, cfg: &RewritingConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    let fixture_json = crate::editor::rewriting::after_fixture_json(state);
    crate::editor::rewriting::render_fixture_graph(crate::editor::rewriting::TRINITY_REWRITING_PLAY_SURFACE_AFTER, crate::editor::rewriting::TRINITY_REWRITING_PLAY_WINDOW_AFTER, &fixture_json, cfg, false, None)
}
