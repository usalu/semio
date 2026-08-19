//! ⚙️ `set-brush-placement-overlap-budget` command.

use crate::editor::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx};
use serde_json::Value;

/// 🖌️ The collision budget every brush/fill placement is tested against — re-syncs the precompute
/// session immediately so already-cached candidates are recomputed under the new budget.
pub async fn set_brush_placement_overlap_budget(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.overlap_budget) {
        ctx.scene.runtime.overlap_budget = value.clamp(0.0, 1.0);
        sync_precompute_session(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    }
}
