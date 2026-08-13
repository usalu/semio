//! 🤝️ `engagement-abort` command.

use crate::apps::puzzle5d::config::puzzle5d_clear_selection;
use crate::apps::puzzle5d::modes::edit::windows::{board2d, world3d};
use crate::apps::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_PLAY_WINDOWS};
use serde_json::Value;

pub fn engagement_abort(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(window) = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()) {
        if PUZZLE5D_PLAY_WINDOWS.contains(&window) {
            ctx.scene.runtime.engagement_input_by_window.insert(window.to_string(), String::new());
        }
    }
    let window = args.and_then(|value| value.get("window")).and_then(|value| value.as_str()).unwrap_or(board2d::WINDOW_KIND_ID);
    ctx.scene.active_utility = if window == world3d::WINDOW_KIND_ID { "move".into() } else { "select".into() };
}
