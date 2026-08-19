//! ⚙️ `set-transform-gumball-flag` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_transform_gumball_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    match flag {
        "move" => ctx.scene.runtime.transform_move = pressed.unwrap_or(!ctx.scene.runtime.transform_move),
        "rotate" => ctx.scene.runtime.transform_rotate = pressed.unwrap_or(!ctx.scene.runtime.transform_rotate),
        _ => {}
    }
}
