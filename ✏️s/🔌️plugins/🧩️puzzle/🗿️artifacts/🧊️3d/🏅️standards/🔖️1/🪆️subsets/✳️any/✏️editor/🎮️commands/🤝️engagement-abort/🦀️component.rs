//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle3d::{Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY};

pub fn engagement_abort(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
}
