//! 🌐️ `set-snap-enabled` command.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, Puzzle3dActionCtx};
use serde_json::Value;

pub fn set_snap_enabled(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_snap_enabled);
}
