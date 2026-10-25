//! 🔭️ `set-lod-mode-for-pane` command.

use {crate::editor::puzzle2d::snapshot::puzzle2d_window_and_measures_scope, crate::editor::puzzle2d::interaction::Puzzle2dActionCtx, crate::editor::puzzle2d::config::PUZZLE2D_LOD_MODE_AUTOMATIC, crate::editor::puzzle2d::snapshot::PUZZLE2D_PANES};
use semio_framework_pack_json::Value;

pub fn set_lod_mode_for_pane(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or("");
    let mode = args.and_then(|value| value.get("value")).and_then(|value| value.as_str());
    let (true, Some(mode)) = (PUZZLE2D_PANES.contains(&pane), mode) else {
        return;
    };
    ctx.scene.runtime.lod_mode_by_pane.insert(pane.to_string(), mode.to_string());
    if pane == crate::editor::puzzle2d::config::PUZZLE2D_OVERVIEW_WINDOW_KIND_ID {
        if mode == PUZZLE2D_LOD_MODE_AUTOMATIC {
            ctx.host.borrow_mut().style.set_automatic_lod(true);
        } else {
            ctx.host.borrow_mut().style.set_automatic_lod(false);
            ctx.host.borrow_mut().style.set_forced_draw_lod_label(mode);
        }
    }
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
