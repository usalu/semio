//! 🌐️ `set-grid-snap-enabled` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub async fn set_grid_snap_enabled(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
    ctx.scene.runtime.grid_snap_enabled = enabled;
    ctx.host.borrow_mut().set_grid_snap_enabled(enabled);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
