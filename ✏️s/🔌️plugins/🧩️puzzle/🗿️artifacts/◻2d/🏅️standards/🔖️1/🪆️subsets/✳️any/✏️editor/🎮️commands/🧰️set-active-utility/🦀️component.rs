//! 🧰️ `set-active-utility` command.

use crate::editor::puzzle2d::modes::edit::windows::overview;
use crate::editor::puzzle2d::{drain_board_events_json, Puzzle2dActionCtx, PUZZLE2D_PANES};
use serde_json::Value;

pub fn set_active_utility(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(utility_id) = args.and_then(|value| value.get("utilityId")).and_then(|value| value.as_str()) {
        let wid = ctx.window_id.unwrap_or(overview::WINDOW_KIND_ID).to_string();
        ctx.scene.runtime.active_utility_by_window_id.insert(wid, utility_id.to_string());
    }
    let fill_generation = ctx.scene.runtime.fill_job_generation;
    let operation = ctx.operation.clone();
    let mut fill_runtime = crate::editor::puzzle2d::config::Puzzle2dFillRuntime::from_config(&ctx.scene.runtime);
    let mut boundary_fault = None;
    {
        let fill_ctx =
            &mut crate::editor::puzzle2d::commands::set_fill_count::Puzzle2dFillActionCtx { runtime: &mut fill_runtime, effects: &mut *ctx.effects, artifact_mutations: &mut *ctx.artifact_mutations, operation, boundary_fault: &mut boundary_fault };
        crate::editor::puzzle2d::commands::set_fill_count::discard_fill_job(fill_ctx, Some(fill_generation));
    }
    fill_runtime.apply_to(&mut ctx.scene.runtime);
    if let Some(code) = boundary_fault {
        let Some(code) = crate::editor::puzzle2d::config::Puzzle2dFillText::try_from_str(code) else { return };
        ctx.scene.runtime.fill_job_fault_code = Some(code);
        ctx.scene.runtime.fill_job_lifecycle = crate::editor::puzzle2d::config::Puzzle2dFillLifecycle::Faulted;
    }
    ctx.scene.runtime.fill_job_accepted_count = 0;
    ctx.scene.runtime.fill_job_search_count = 0;
    ctx.scene.runtime.fill_job_stage.clear();
    ctx.scene.runtime.fill_job_fault_code = None;
    ctx.host.borrow_mut().brush_cancel_slot();
    let _ = drain_board_events_json(&mut ctx.host.borrow_mut());
    ctx.scene.runtime.fill_count = 0;
    ctx.scene.runtime.brush_candidates.clear();
    ctx.scene.runtime.brush_candidate_index = 0;
    ctx.scene.runtime.brush_candidate_source_handle_id = String::new();
    for pane in PUZZLE2D_PANES {
        ctx.scene.runtime.engagement_input_by_pane.insert(pane.to_string(), String::new());
    }
}
