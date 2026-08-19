//! 🕸️ `redraw-handles` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;

/// 🧲️ Re-snaps every edge's handle angle onto its node-center line.
pub async fn redraw_handles(ctx: &mut Puzzle2dActionCtx<'_>) {
    if let Ok(next) = crate::editor::puzzle2d::engine::apply_edge_handle_snap_to_fixture_v1_json(&ctx.scene.fixture.to_string()) {
        if let Ok(parsed) = serde_json::from_str(&next) {
            ctx.scene.fixture = parsed;
        }
    }
}
