//! 🌐️ `set-grid-factor` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub async fn set_grid_factor(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) else {
        return;
    };
    ctx.scene.runtime.grid_factor = value;
    let _ = ctx.host.borrow_mut().set_grid_factor(value);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
