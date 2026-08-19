//! 🌐️ `set-snap-enabled` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_snap_enabled(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_snap_enabled);
}
