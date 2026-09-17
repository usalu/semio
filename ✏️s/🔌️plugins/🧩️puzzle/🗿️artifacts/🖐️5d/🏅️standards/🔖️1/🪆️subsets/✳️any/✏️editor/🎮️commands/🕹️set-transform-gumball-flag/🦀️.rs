//! 🎛️ `set-transform-gumball-flag` command — which handle families the transform gumball exposes.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_transform_gumball_flag(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    match flag {
        "move" => ctx.scene.runtime.transform_move = pressed.unwrap_or(!ctx.scene.runtime.transform_move),
        "rotate" => ctx.scene.runtime.transform_rotate = pressed.unwrap_or(!ctx.scene.runtime.transform_rotate),
        _ => {}
    }
}
