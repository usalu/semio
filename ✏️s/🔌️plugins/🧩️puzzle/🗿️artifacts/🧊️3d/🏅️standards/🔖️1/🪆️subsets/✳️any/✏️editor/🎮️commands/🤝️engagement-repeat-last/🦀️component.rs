//! 🤝️ `engagement-repeat-last` command.

use crate::editor::puzzle3d::{apply_puzzle3d_fill_count, Puzzle3dActionCtx, PUZZLE3D_FILL_COUNT_MAX};

pub async fn engagement_repeat_last(ctx: &mut Puzzle3dActionCtx<'_>) {
    if ctx.scene.active_utility == "fill" {
        let count = (ctx.scene.runtime.fill_count + 1).min(PUZZLE3D_FILL_COUNT_MAX);
        apply_puzzle3d_fill_count(&mut ctx.app.precompute.borrow_mut(), ctx.scene, count);
    }
}
