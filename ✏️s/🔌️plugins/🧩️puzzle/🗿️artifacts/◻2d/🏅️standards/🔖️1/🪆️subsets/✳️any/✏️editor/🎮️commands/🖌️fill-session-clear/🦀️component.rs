//! 🖌️ `fill-session-clear` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;

pub fn fill_session_clear(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_fill_session_clear();
    ctx.scene.runtime.fill_count = 0;
}
