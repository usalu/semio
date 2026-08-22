//! 🖌️ `fill-session-clear` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;

pub fn fill_session_clear(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.scene.runtime.fill_job_generation = ctx.scene.runtime.fill_job_generation.saturating_add(1);
    ctx.scene.runtime.fill_job_checkpoint = None;
    ctx.scene.runtime.fill_job_applied_count = 0;
    ctx.scene.runtime.fill_job_preview = None;
    ctx.scene.runtime.fill_count = 0;
}
