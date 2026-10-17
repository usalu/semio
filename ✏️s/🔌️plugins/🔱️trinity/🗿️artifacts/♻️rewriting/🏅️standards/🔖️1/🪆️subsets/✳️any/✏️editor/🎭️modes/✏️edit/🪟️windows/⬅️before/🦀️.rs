//! ⬅️ Trinity Rewriting app — Before window (editable node-graph over `working_graph`).

use crate::editor::rewriting::window_config::RewritingWindowConfig;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
pub fn render(state: &RewritingSnapshot, cfg: &RewritingWindowConfig) -> semio_framework_plugin::UiAssemblyResult<semio_framework_plugin::BuiltNode> {
    crate::editor::rewriting::ui::render_graph_snapshot(crate::editor::rewriting::ids::TRINITY_REWRITING_PLAY_SURFACE_BEFORE, &state.working_graph, cfg, true)
}
