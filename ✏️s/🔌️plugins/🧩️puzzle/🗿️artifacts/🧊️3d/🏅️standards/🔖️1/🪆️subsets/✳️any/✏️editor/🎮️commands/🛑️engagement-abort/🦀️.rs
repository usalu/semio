//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::{puzzle3d_fill_tool_active, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY};
use semio_framework_plugin::kernel::Effect;
use semio_framework_tool_run::{TOOL_RUN_ABORT_ACTION_ID, TOOL_RUN_ARG_GENERATION, TOOL_RUN_ARG_RUN_ID};

/// 🛑 Escape is the user saying "stop what is armed": the typed engagement line and the brush candidate
/// cursor are dropped, and — when the Fill TOOL is what is armed — the fill run of this document instance is
/// aborted through the framework-reserved `toolRunAbort` and the tool is disarmed, the way the shell's own
/// Escape leaves a tool (`ShellHost`'s `SET_ACTIVE_TOOL ""`). The run identity is the instance's live run as the
/// framework handed it to this command at admission (`ArtifactOwnedToolJobContext::tool_run`); a generation the
/// ledger moved past since answers `toolRun.stale` as a silent no-op. Nothing here writes the document: the
/// run's provisional placements live in the ledger, so aborting it leaves no trace to roll back.
pub fn engagement_abort(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    if puzzle3d_fill_tool_active(ctx.config) || ctx.scene.active_utility == fill_tool::TOOL_ID {
        if let Some(run) = fill_tool::live_fill_run(ctx.tool_run) {
            let args = serde_json::json!({ TOOL_RUN_ARG_RUN_ID: run.identity.id.run.to_string(), TOOL_RUN_ARG_GENERATION: run.identity.generation });
            ctx.effects.push(Effect::DispatchAction { req: semio_framework_plugin::RequestId(semio_framework_job::allocate_operation_id().0), action: TOOL_RUN_ABORT_ACTION_ID.into(), args: semio_framework::optional_json_to_dsl(Some(args)), delay_ms: 0 });
        }
        ctx.effects.push(Effect::SetActiveTool { tool_id: String::new() });
    }
    ctx.scene.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
}
