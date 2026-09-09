//! 🪣️ `fill-build-tick` command.

use dsl::os_pack::json::Value;
use crate::editor::puzzle3d::precompute::FILL_JOB_KIND;
use crate::editor::puzzle3d::puzzle3d_fill_build_scope;
use crate::editor::puzzle3d::puzzle3d_fill_tool_active;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{Effect, JobPlacement};

/// 🪣️ No catch-up `setFillCount` dispatch here: `apply_puzzle3d_fill_count` always clamps the
/// committed count to what's available at commit time, so `fill_count` can never run ahead of
/// `applied_count` — a slider can only request what `render`'s reveal-tagged instances already show.
/// Each tick only observes the latest worker publication and, when no fill job is live, requests one
/// isolated shared-pool job. Solver work is exclusively driven by `fill_job`.
pub fn fill_build_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    if !puzzle3d_fill_tool_active(ctx.config) {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    }
    let mut precompute = ctx.app.precompute.borrow_mut();
    let changed = precompute.poll_fill_job();
    let spawn = precompute.enqueue_fill_job();
    let faulted = precompute.take_fill_fault_notice();
    drop(precompute);
    let spawned = spawn.is_some();
    if let Some((job, input)) = spawn {
        ctx.effects.push(Effect::SpawnJob { job, kind: FILL_JOB_KIND.into(), input, placement: JobPlacement::Isolated });
    }
    if faulted {
        ctx.notice(|labels| labels.fill_failed.as_str());
    }
    *ctx.ui_scope = if changed || spawned || faulted { puzzle3d_fill_build_scope() } else { UiDirtyScope::None };
}

/// 🛑 `cancelFillBuild` — stops the live background fill job, and only that one: the action carries the
/// job's own `(job, operation, generation)` triple, so a cancel dispatched against a superseded run is a
/// no-op instead of killing the plan the user is looking at. Same convention as `🔋️energy`'s
/// `cancel-energy-simulation` identity args feeding `Effect::CancelJob`.
pub fn cancel_fill_build(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let identity = |key: &str| args.and_then(|args| args.get(key)).and_then(Value::as_u64);
    let (Some(job), Some(operation), Some(generation)) = (identity("job"), identity("operation"), identity("generation")) else {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    };
    let cancelled = ctx.app.precompute.borrow_mut().cancel_fill_job_for(job, operation, generation);
    if !cancelled {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    }
    ctx.effects.push(Effect::CancelJob { job });
    *ctx.ui_scope = puzzle3d_fill_build_scope();
}
