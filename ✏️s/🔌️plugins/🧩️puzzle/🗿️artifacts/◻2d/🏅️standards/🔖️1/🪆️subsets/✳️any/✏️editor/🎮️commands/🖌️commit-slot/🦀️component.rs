//! 🖌️ `commit-slot` command.

use crate::editor::puzzle2d::{apply_host_events, Puzzle2dActionCtx};

pub async fn commit_slot(ctx: &mut Puzzle2dActionCtx<'_>) {
    ctx.host.borrow_mut().brush_commit_slot();
    apply_host_events(&mut ctx.host.borrow_mut(), ctx.scene);
}
