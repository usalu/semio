//! 🧰️ `set-active-utility` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{drain_board_events_json, Puzzle2dActionCtx, PUZZLE2D_PANES};
use serde_json::Value;

/// 🧰️ Switching utility abandons any fill in flight. The fill session lives inside its own retained
/// job, so "abandon" is exactly a runtime discard: the retained work reads its continuation state
/// from `Puzzle2dFillRuntime`, and a discarded runtime is one no `brushFillSessionStep` will resume.
pub fn set_active_utility(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
        let wid = ctx.window_id.unwrap_or(overview::WINDOW_KIND_ID).to_string();
        ctx.scene.runtime.active_utility_by_window_id.insert(wid, utility_id.to_string());
    }
    let mut fill_runtime = crate::editor::puzzle2d::config::Puzzle2dFillRuntime::from_config(&ctx.scene.runtime);
    crate::editor::puzzle2d::commands::set_fill_count::discard_fill_session(&mut fill_runtime);
    fill_runtime.fill_count = 0;
    fill_runtime.apply_to(&mut ctx.scene.runtime);
    ctx.scene.runtime.fill_job_stage.clear();
    ctx.host.borrow_mut().brush_cancel_slot();
    let _ = drain_board_events_json(&mut ctx.host.borrow_mut());
    ctx.scene.runtime.brush_candidates.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.runtime.brush_candidate_source_handle_id = String::new();
    for pane in PUZZLE2D_PANES {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
    }
}
