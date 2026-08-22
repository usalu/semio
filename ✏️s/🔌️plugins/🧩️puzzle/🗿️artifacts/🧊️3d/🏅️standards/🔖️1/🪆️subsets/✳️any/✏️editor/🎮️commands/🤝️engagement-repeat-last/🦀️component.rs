//! 🤝️ `engagement-repeat-last` command.

use crate::editor::puzzle3d::commands::set_fill_count;
use crate::editor::puzzle3d::{Puzzle3dActionCtx, PUZZLE3D_FILL_COUNT_MAX};

pub fn engagement_repeat_last(ctx: &mut Puzzle3dActionCtx<'_>) {
    if ctx.scene.active_utility == "fill" {
        let count = (ctx.scene.runtime.fill_count + 1).min(PUZZLE3D_FILL_COUNT_MAX);
        ctx.effects.push(set_fill_count::request(count));
    }
}
