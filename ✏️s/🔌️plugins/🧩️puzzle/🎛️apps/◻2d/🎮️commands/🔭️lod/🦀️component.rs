//! 🔭️ Puzzle 2d play app commands — the per-pane level-of-detail mode. Only the interactive overview
//! pane's mode reaches the shared board host; the other two panes are framed per render.

use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx, PUZZLE2D_LOD_MODE_AUTOMATIC, PUZZLE2D_PANES};
use semio_framework_core::kernel::UiDirtyScope;
use serde_json::Value;

pub fn set_lod_mode_for_pane(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or("");
    let mode = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
    let (true, Some(mode)) = (PUZZLE2D_PANES.contains(&pane), mode) else {
        return;
    };
    ctx.scene.runtime.lod_mode_by_pane.insert(pane.to_string(), mode.to_string());
    if pane == overview::WINDOW_KIND_ID {
        if mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
            ctx.host.borrow_mut().set_automatic_lod(true);
        } else {
            ctx.host.borrow_mut().set_automatic_lod(false);
            ctx.host.borrow_mut().set_forced_draw_lod_label(mode);
        }
    }
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}

/// 📶️ Warms the board's LOD scale table (the chrome reads it through
/// `🎭️modes/✏️edit/🎚️options/🔭️lod`); nothing is dirtied.
pub fn lod_scale_json(ctx: &mut Puzzle2dActionCtx<'_>) {
    let _ = crate::artifacts::puzzle2d::engine::puzzle_2d_lod_scale_json();
    *ctx.ui_scope = UiDirtyScope::None;
}
