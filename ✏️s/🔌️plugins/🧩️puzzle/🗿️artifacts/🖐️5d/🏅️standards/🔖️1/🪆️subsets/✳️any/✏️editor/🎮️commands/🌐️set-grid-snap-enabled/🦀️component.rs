//! 🌐️ `set-grid-snap-enabled` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use serde_json::Value;

pub async fn set_grid_snap_enabled(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("enabled")).and_then(|value| value.as_bool()).unwrap_or(false);
}
