//! 🌐️ `set-grid-visible` command — the grid group's show/hide toggle, per window instance.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 👁️ Reads the toggle's own `pressed`; a bare dispatch flips the current state, exactly like
/// puzzle3d's `setGridVisible`.
pub fn set_grid_visible(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let next = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool).unwrap_or(!ctx.scene.runtime.grid_visible);
    ctx.scene.runtime.grid_visible = next;
    ctx.host.borrow_mut().set_grid_visible(next);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
