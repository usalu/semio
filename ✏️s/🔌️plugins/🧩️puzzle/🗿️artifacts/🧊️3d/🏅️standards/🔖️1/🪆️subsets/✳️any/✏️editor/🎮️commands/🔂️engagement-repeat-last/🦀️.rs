//! 🤝️ `engagement-repeat-last` command.

use crate::editor::puzzle3d::commands::set_fill_count;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

pub fn engagement_repeat_last(ctx: &mut Puzzle3dActionCtx<'_>) {
    if ctx.scene.active_utility == "fill" {
        let count = ctx.scene.runtime.fill_count.saturating_add(1);
        ctx.effects.push(set_fill_count::request(count));
    }
}
