//! ⌨️ `engagement-input` command.

use crate::editor::puzzle5d::modes::edit::windows::board2d;
use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_PLAY_WINDOWS};
use dsl::os_pack::json::Value;

pub fn engagement_input(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(board2d::WINDOW_KIND_ID);
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
        ctx.scene.runtime.engagement_input_by_window.insert(window.to_string(), value.to_string());
    }
}
