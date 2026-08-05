//! 🌐️ Puzzle 3d play app commands — the world grid: visibility, snap and spacing. Spacing doubles as
//! the quantum `addTargetVolume` snaps new voxel volumes onto, so it is floored rather than clamped.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, Puzzle3dActionCtx};
use serde_json::Value;

pub fn set_visible(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_visible = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_visible);
}

pub fn set_snap_enabled(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_snap_enabled = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_snap_enabled);
}

pub fn set_spacing(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.grid_spacing) {
        ctx.scene.runtime.grid_spacing = value.max(0.1);
    }
}
