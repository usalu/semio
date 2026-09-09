//! 🤝️ `engagement-input` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub fn engagement_input(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    ctx.scene.runtime.engagement_input_by_pane.insert(ctx.window_kind.to_string(), value.to_string());
    *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
}
