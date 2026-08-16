//! 🔭️ `lod-scale-json` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx, PUZZLE2D_LOD_MODE_AUTOMATIC, PUZZLE2D_PANES};
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;

/// 📶️ Warms the board's LOD scale table (the chrome reads it through
/// `🎭️modes/✏️edit/🎚️options/🔭️lod`); nothing is dirtied.
pub fn lod_scale_json(ctx: &mut Puzzle2dActionCtx<'_>) {
    let _ = crate::editor::puzzle2d::engine::puzzle_2d_lod_scale_json();
    *ctx.ui_scope = UiDirtyScope::None;
}
