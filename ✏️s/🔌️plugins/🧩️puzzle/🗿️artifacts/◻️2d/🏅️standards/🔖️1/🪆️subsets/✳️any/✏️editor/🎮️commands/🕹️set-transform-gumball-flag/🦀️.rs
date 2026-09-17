//! 🕹️ `set-transform-gumball-flag` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_measures_scope, Puzzle2dActionCtx};
use serde_json::Value;

/// 🕹️ Composes which selection-gumball handles the board's select utility offers. `move` is the
/// native node drag, `rotate` the ring around the selection centroid; a scale handle is deliberately
/// absent — a node's size comes from its kind catalog, never from a free drag (same law as puzzle3d).
/// Omitting `pressed` toggles, matching the framework toggle measure's own echo.
pub fn set_transform_gumball_flag(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let flag = args.and_then(|value| value.get("flag")).and_then(Value::as_str).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    match flag {
        "move" => ctx.scene.runtime.transform_move = pressed.unwrap_or(!ctx.scene.runtime.transform_move),
        "rotate" => ctx.scene.runtime.transform_rotate = pressed.unwrap_or(!ctx.scene.runtime.transform_rotate),
        _ => return,
    }
    ctx.host.borrow_mut().set_transform_flags(ctx.scene.runtime.transform_move, ctx.scene.runtime.transform_rotate);
    *ctx.ui_scope = puzzle2d_window_and_measures_scope();
}
