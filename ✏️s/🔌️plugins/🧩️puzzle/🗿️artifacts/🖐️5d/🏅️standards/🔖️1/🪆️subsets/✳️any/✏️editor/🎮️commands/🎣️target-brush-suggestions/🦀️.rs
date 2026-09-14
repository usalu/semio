//! 🎣️ `target-brush-suggestions` command.

use crate::editor::puzzle5d::modes::edit::windows::board2d::utilities::brush;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🎣️ Points the brush suggestions run at the grip the armed brush is over (`fullId`, else the first selected
/// grip), or at nothing once the brush is disarmed or the pointer left every grip. It only writes the instance's
/// brush suggestions link; the refresh after it starts, wakes or aborts the read-only puzzle 3d brush run, so a
/// hover never waits on a candidate search.
pub fn target_brush_suggestions(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let armed = ctx.scene.active_utility == brush::UTILITY_ID;
    let requested = args.and_then(|value| value.get("fullId")).and_then(Value::as_str).filter(|id| !id.is_empty()).map(str::to_string);
    let target = armed.then(|| requested.or_else(|| ctx.selected_grip_ids().first().cloned())).flatten();
    ctx.brush_suggestions(|link| link.hover(target));
}
