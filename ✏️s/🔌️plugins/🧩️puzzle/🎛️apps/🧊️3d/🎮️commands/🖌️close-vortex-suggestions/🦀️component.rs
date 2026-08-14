//! 🖌️ `close-vortex-suggestions` command.

use crate::apps::puzzle3d::Puzzle3dActionCtx;

pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
}
