//! 🛑️ `engagement-abort` command.

use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn engagement_abort(ctx: &mut Puzzle5dActionCtx<'_>, _args: Option<&Value>) {
    ctx.scene.runtime.engagement_input_by_window.insert(ctx.window_id.to_string(), String::new());
    ctx.scene.active_utility = if ctx.window_kind == world3d::WINDOW_KIND_ID { "move".into() } else { "select".into() };
}
