//! 🖌️ `close-vortex-suggestions` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;

/// 🔒️ Closing the popup releases its vortex from the brush suggestions run: with no armed brush target left,
/// the refresh after it aborts the run.
pub fn close_vortex_suggestions(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.suggestion_menu = None;
    ctx.brush_suggestions(|link| link.close_menu());
}
