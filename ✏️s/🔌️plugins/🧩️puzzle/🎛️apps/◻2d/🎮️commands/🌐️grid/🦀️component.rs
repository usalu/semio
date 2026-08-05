//! 🌐️ Puzzle 2d play app commands — the grid snap toggle and its spacing factor. Pure config state,
//! mirrored into the board host so the live marquee snaps immediately.

use crate::apps::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub fn set_grid_snap_enabled(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
    ctx.scene.runtime.grid_snap_enabled = enabled;
    ctx.host.borrow_mut().set_grid_snap_enabled(enabled);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}

pub fn set_grid_factor(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) else {
        return;
    };
    ctx.scene.runtime.grid_factor = value;
    let _ = ctx.host.borrow_mut().set_grid_factor(value);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
