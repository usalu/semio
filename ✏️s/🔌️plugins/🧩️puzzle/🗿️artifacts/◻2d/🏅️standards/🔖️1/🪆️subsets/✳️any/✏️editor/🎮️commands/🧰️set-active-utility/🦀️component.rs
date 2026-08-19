//! 🧰️ `set-active-utility` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{Puzzle2dActionCtx, PUZZLE2D_PANES};
use serde_json::Value;

pub async fn set_active_utility(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
        let wid = ctx.window_id.unwrap_or(overview::WINDOW_KIND_ID).to_string();
        ctx.scene.runtime.active_utility_by_window_id.insert(wid, utility_id.to_string());
    }
    ctx.host.borrow_mut().brush_fill_session_clear();
    ctx.host.borrow_mut().brush_cancel_slot();
    let _ = ctx.host.borrow_mut().drain_events_json();
    ctx.scene.runtime.fill_count = 0;
    ctx.scene.runtime.brush_candidates.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.runtime.brush_candidate_source_handle_id = String::new();
    for pane in PUZZLE2D_PANES {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
    }
}
