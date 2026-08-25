//! 🖌️ `fill-session-clear` command.

use crate::editor::puzzle2d::commands::set_fill_count::Puzzle2dFillActionCtx;

pub fn fill_session_clear(ctx: &mut Puzzle2dFillActionCtx<'_>) {
    let generation = ctx.runtime.fill_job_generation;
    crate::editor::puzzle2d::commands::set_fill_count::discard_fill_job(ctx, Some(generation));
    ctx.runtime.fill_count = 0;
}
