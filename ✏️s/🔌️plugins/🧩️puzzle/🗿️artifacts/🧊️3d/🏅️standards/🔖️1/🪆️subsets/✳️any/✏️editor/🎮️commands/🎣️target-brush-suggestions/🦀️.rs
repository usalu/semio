//! 🖌️ `target-brush-suggestions` command.

use crate::editor::puzzle3d::modes::edit::windows::main::utilities::brush;
use crate::editor::puzzle3d::puzzle3d_brush_target_vortex;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use dsl::os_pack::json::Value;

/// 🎣️ Points the brush suggestions run at the vortex the armed brush is over (`fullId`, else the brush's own
/// selection/hover target), or at nothing once the brush is disarmed or the pointer left every vortex. It only
/// writes the instance's brush suggestions link; the refresh after it starts, wakes or aborts the read-only run
/// (`utilities::brush::run_effects`), so a hover never waits on a candidate search.
pub fn target_brush_suggestions(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let armed = ctx.scene.active_utility == brush::UTILITY_ID;
    let requested = args.and_then(|value| value.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
    let target = armed.then(|| requested.or_else(|| puzzle3d_brush_target_vortex(ctx.scene, ctx.interaction))).flatten();
    ctx.brush_suggestions(|link| link.hover(target));
}
