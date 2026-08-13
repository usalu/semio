//! ⚙️ `set-vortex-direction` command.

use crate::apps::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use serde_json::Value;

pub fn set_vortex_direction(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        if mode == PUZZLE3D_VORTEX_DIRECTION_OUTWARDS || mode == PUZZLE3D_VORTEX_DIRECTION_INWARDS {
            ctx.scene.runtime.vortex_direction = mode.into();
        }
    }
}
