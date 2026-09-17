//! 🎯️ `set-selectable-kind` command — which entity kinds a pick in this pane may even reach.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn set_selectable_kind(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(Value::as_str).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(Value::as_bool);
    let kinds = &mut ctx.scene.runtime.selectable_kinds;
    match kind {
        "parts" => kinds.parts = pressed.unwrap_or(!kinds.parts),
        "grips" => kinds.grips = pressed.unwrap_or(!kinds.grips),
        "fasteners" => kinds.fasteners = pressed.unwrap_or(!kinds.fasteners),
        _ => {}
    }
}
