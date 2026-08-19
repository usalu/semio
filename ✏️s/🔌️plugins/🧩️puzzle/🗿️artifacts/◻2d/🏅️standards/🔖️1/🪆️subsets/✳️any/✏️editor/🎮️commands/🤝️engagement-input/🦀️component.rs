//! 🤝️ `engagement-input` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx, PUZZLE2D_PANES};
use serde_json::Value;

pub async fn engagement_input(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let pane = args.and_then(|value| value.get("pane")).and_then(|value| value.as_str()).unwrap_or(overview::WINDOW_KIND_ID);
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()).unwrap_or("");
    if PUZZLE2D_PANES.contains(&pane) {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), value.to_string());
        *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
    }
}
