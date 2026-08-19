//! 🕸️ `force-layout` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;

/// 🌀️ Re-runs the force-graph layout over the whole fixture — shared by `forceLayout` and `reorganize`.
pub async fn force_layout(ctx: &mut Puzzle2dActionCtx<'_>) {
    let Ok(layout_json) = crate::editor::puzzle2d::engine::apply_force_graph_layout_to_fixture_v1_json(&ctx.scene.fixture.to_string(), r#"{"mode":"force-graph"}"#) else {
        return;
    };
    if let Ok(parsed) = serde_json::from_str(&layout_json) {
        ctx.scene.fixture = parsed;
    }
}
