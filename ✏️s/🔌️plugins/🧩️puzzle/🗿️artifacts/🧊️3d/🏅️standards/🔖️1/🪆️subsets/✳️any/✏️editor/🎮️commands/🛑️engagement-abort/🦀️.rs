//! 🤝️ `engagement-abort` command.

use crate::editor::puzzle3d::modes::edit::tools::fill as fill_tool;
use crate::editor::puzzle3d::{puzzle3d_fill_tool_active, Puzzle3dActionCtx, PUZZLE3D_DEFAULT_UTILITY};
use semio_framework_plugin::kernel::Effect;

/// 🛑 Escape is the user saying "stop what is armed": the typed engagement line and the brush
/// candidate cursor are dropped, and — when the Fill TOOL is what is armed — the in-flight fill plan
/// is cancelled and the tool is disarmed, so the app leaves Fill exactly the way the shell's own
/// Escape leaves a tool (`ShellHost`'s `SET_ACTIVE_TOOL ""`), rather than to some remembered previous
/// tool this app keeps no history of.
///
/// 🧊️ Ticket 26/09/02/PUZZLE-3D-END-TO-END wave B31: this arm used to `return` untouched while
/// `puzzle3d_fill_tool_active`, on the rule that "leaving fill is exclusively a host
/// `setActiveTool \"\"`". That rule is right for the epilogue's GENERIC utility-switch effect — an
/// empty tool effect there bounce-disarms a just-armed fill — and wrong here, because this arm IS the
/// gesture that means "leave it": with `fill 5` typed and Escape pressed, `activeUtility` stayed
/// `fill` forever (`engagement-abort rearmed=true activeUtility=brush waitedMs=15511`, wave B29
/// §2.5). The disarm is therefore stated by the arm itself, one explicit effect, and the epilogue's
/// generic rule is untouched. The utility reset is a plain `active_utility` write, so nothing about
/// this arm's `WindowTransient`-only publication contract changes.
pub fn engagement_abort(ctx: &mut Puzzle3dActionCtx<'_>) {
    ctx.scene.runtime.engagement_input = String::new();
    ctx.scene.runtime.brush_candidate_index = 0;
    if puzzle3d_fill_tool_active(ctx.config) || ctx.scene.active_utility == fill_tool::TOOL_ID {
        let mut precompute = ctx.app.precompute.borrow_mut();
        let cancelled = match precompute.fill_job_identity() {
            Some((job, operation, generation)) => precompute.cancel_fill_job_for(job, operation, generation).then_some(job),
            None => None,
        };
        drop(precompute);
        if let Some(job) = cancelled {
            ctx.effects.push(Effect::CancelJob { job });
        }
        ctx.effects.push(Effect::SetActiveTool { tool_id: String::new() });
    }
    ctx.scene.active_utility = PUZZLE3D_DEFAULT_UTILITY.into();
}
