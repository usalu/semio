//! 🕸️ `redraw-handles` command.

use crate::apps::puzzle2d::{add_node_to_fixture, patch_inspector_nodes, Puzzle2dActionCtx};
use serde_json::Value;

/// 🧲️ Re-snaps every edge's handle angle onto its node-center line.
pub fn redraw_handles(ctx: &mut Puzzle2dActionCtx<'_>) {
    if let Ok(next) = crate::apps::puzzle2d::engine::apply_edge_handle_snap_to_fixture_v1_json(&ctx.scene.fixture.to_string()) {
        if let Ok(parsed) = serde_json::from_str(&next) {
            ctx.scene.fixture = parsed;
        }
    }
}
