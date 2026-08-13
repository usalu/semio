//! 🖌️ `cycle-brush-candidate` command.

use serde_json::{json, Value};
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use crate::apps::puzzle5d::parse_brush_candidates_free;
use crate::apps::puzzle5d::puzzle5d_brush_target_grip;

/// 🔁️ Advances the candidate index, wrapping around the engine's collision-free list for the current
/// target grip (or just incrementing when there is no target yet).
pub fn cycle_brush_candidate(ctx: &mut Puzzle5dActionCtx<'_>) {
    ctx.app.drive_precompute(ctx.scene);
    if let Some(grip_full_id) = puzzle5d_brush_target_grip(ctx.scene) {
        let free = parse_brush_candidates_free(&ctx.app.precompute.borrow().brush_candidates(&grip_full_id)).len();
        if free > 0 {
            ctx.scene.runtime.brush_candidate_index = (ctx.scene.runtime.brush_candidate_index + 1) % free;
        }
    } else {
        ctx.scene.runtime.brush_candidate_index = ctx.scene.runtime.brush_candidate_index.saturating_add(1);
    }
}
