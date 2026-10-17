//! 🔭️ `lod-scale-json` command.

use crate::editor::puzzle2d::interaction::Puzzle2dActionCtx;
use semio_framework::kernel::UiDirtyScope;

/// 📶️ Warms the board's LOD scale table (the chrome reads it through
/// `🎭️modes/✏️edit/☑️options/🔭️lod`); nothing is dirtied.
pub fn lod_scale_json(ctx: &mut Puzzle2dActionCtx<'_>) {
    let _ = crate::editor::puzzle2d::engine::board_lod_scale_json();
    *ctx.ui_scope = UiDirtyScope::None;
}
