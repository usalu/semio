//! 🌐️ Puzzle 5d play app commands — the board grid: snap toggle and spacing factor.

use crate::apps::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

pub fn set_grid_snap_enabled(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
}

pub fn set_grid_factor(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.grid_factor = value;
    }
}
