//! 🛑️ `engagement-abort` command.

use crate::editor::puzzle5d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle5d::modes::edit::windows::world3d;
use crate::editor::puzzle5d::{puzzle5d_fill_tool_active, Puzzle5dActionCtx, PUZZLE5D_DEFAULT_UTILITY};
use dsl::os_pack::json::Value;
use semio_framework_plugin::kernel::Effect;
use semio_framework_tool_run::{TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID};

/// 🛑 Escape is the operator saying "stop what is armed": the typed engagement line and the brush candidate
/// cursor are dropped, and — when the Fill TOOL is what is armed — the fill run of this document instance is
/// ABORTED through the framework-reserved `toolRunAbort` and the tool disarmed the way the shell's own Escape
/// leaves a tool (`SET_ACTIVE_TOOL ""`), rather than merely disarming the input. The run identity is the
/// instance's live run as the framework handed it to this command at admission; a generation the ledger moved
/// past answers `toolRun.stale` as a silent no-op. Nothing here writes the document: the run's provisional
/// placements live in the ledger, so aborting leaves no trace to roll back.
pub fn engagement_abort(ctx: &mut Puzzle5dActionCtx<'_>, _args: Option<&Value>) {
    ctx.scene.runtime.engagement_input_by_window.insert(ctx.window_id.to_string(), String::new());
    ctx.scene.runtime.brush_candidate_index = 0;
    if puzzle5d_fill_tool_active(ctx.view_state) || ctx.scene.active_utility == fill_tool::TOOL_ID {
        if let Some(run) = fill_tool::live_fill_run(ctx.tool_run) {
            ctx.effects.push(Effect::DispatchAction {
                req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0),
                action: TOOL_RUN_ABORT_ACTION_ID.into(),
                args: semio_framework::optional_json_to_dsl(Some(serde_json::json!({ TOOL_RUN_ARG_RUN_ID: run.identity.id.run.to_string(), TOOL_RUN_ARG_GENERATION: run.identity.generation }))),
                delay_ms: 0,
            });
        }
        ctx.effects.push(Effect::SetActiveTool { tool_id: String::new() });
        ctx.scene.active_utility = PUZZLE5D_DEFAULT_UTILITY.into();
        return;
    }
    ctx.scene.active_utility = if ctx.window_kind == world3d::WINDOW_KIND_ID { world3d::utilities::transform::UTILITY_ID.into() } else { PUZZLE5D_DEFAULT_UTILITY.into() };
}
