//! 🔭️ `lod-scale-json` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework::kernel::UiDirtyScope;

/// 📶️ Warms the board's LOD scale table (the chrome reads it through
/// `🎭️modes/✏️edit/🎚️options/🔭️lod`); nothing is dirtied.
pub async fn lod_scale_json(ctx: &mut Puzzle2dActionCtx<'_>) {
    let _ = crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json();
    *ctx.ui_scope = UiDirtyScope::None;
}
