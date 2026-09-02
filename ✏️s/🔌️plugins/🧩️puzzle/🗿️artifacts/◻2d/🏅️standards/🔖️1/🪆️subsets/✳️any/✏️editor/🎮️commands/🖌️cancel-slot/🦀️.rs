//! 🖌️ `cancel-slot` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;

pub fn cancel_slot(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_cancel_slot();
}
