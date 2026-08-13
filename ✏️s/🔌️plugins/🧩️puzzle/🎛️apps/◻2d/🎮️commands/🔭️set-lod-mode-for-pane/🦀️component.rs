//! 🔭️ `set-lod-mode-for-pane` command.

use crate::apps::puzzle2d::modes::edit::windows::overview;
use crate::apps::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx, PUZZLE2D_LOD_MODE_AUTOMATIC, PUZZLE2D_PANES};
use semio_framework::kernel::UiDirtyScope;
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
