//! ⚙️ `set-transform-gumball-flag` command.

use crate::editor::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use serde_json::Value;

pub fn set_transform_gumball_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    match flag {
        "move" => ctx.scene.runtime.transform_move = pressed.unwrap_or(!ctx.scene.runtime.transform_move),
        "rotate" => ctx.scene.runtime.transform_rotate = pressed.unwrap_or(!ctx.scene.runtime.transform_rotate),
        _ => {}
    }
}
