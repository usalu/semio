//! 🌐️ `set-grid-factor` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

pub async fn set_grid_factor(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.grid_factor = value;
    }
}
