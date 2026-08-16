//! 🗂️ `set-selectable-kind` command.

use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

pub fn set_selectable_kind(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let kind = args.and_then(|value| value.get("kind")).and_then(|value| value.as_str()).unwrap_or("");
    let pressed = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool());
    let kinds = &mut ctx.scene.runtime.selectable_kinds;
    match kind {
        "objects" => kinds.objects = pressed.unwrap_or(!kinds.objects),
        "vortices" => kinds.vortices = pressed.unwrap_or(!kinds.vortices),
        "attractions" => kinds.attractions = pressed.unwrap_or(!kinds.attractions),
        _ => {}
    }
}
