//! ⌨️ `engagement-input` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

pub fn engagement_input(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    ctx.scene.runtime.engagement_input_by_window.insert(ctx.window_id.to_string(), value.to_string());
}
