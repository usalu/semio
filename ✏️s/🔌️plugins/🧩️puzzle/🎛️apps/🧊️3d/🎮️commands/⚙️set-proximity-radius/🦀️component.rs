//! ⚙️ `set-proximity-radius` command.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use serde_json::Value;

pub fn set_proximity_radius(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.proximity_radius) {
        ctx.scene.runtime.proximity_radius = value.max(0.0);
    }
}
