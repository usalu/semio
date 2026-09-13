//! 🛑️ `cancel-fill-build` command.

use crate::editor::puzzle5d::Puzzle5dActionCtx;
use dsl::os_pack::json::Value;

/// 🛑️ Stops the live fill run where it stands. The run's own `(job, operation, generation)` identity
/// travels in the arguments exactly as the 3d tool's cancel does, so a click that arrives after the
/// run it was rendered for was superseded cancels nothing. Whether or not a background job was
/// reached, the requested count is pinned to what the document already holds — that pin IS the stop
/// for the synchronous planner, and it leaves the operator with exactly the parts they can see.
pub fn cancel_fill_build(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let identity = ["job", "operation", "generation"].map(|key| args.and_then(|value| value.get(key)).and_then(|value| value.as_u64()));
    if let [Some(job), Some(operation), Some(generation)] = identity {
        ctx.app.precompute.borrow_mut().cancel_fill_job_for(job, operation, generation);
    }
    let locked = ctx.app.precompute.borrow().fill_progress().applied_count;
    ctx.scene.runtime.fill_count = u32::try_from(locked).unwrap_or(u32::MAX);
}
