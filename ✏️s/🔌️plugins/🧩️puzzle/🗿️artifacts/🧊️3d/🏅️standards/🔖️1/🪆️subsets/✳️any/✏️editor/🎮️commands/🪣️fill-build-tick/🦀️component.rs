//! 🪣️ `fill-build-tick` command.

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
    drop(precompute);
    let spawned = spawn.is_some();
    if let Some((job, input)) = spawn {
        ctx.effects.push(Effect::SpawnJob { job, kind: FILL_JOB_KIND.into(), input, placement: JobPlacement::Isolated });
    }
    *ctx.ui_scope = if changed || spawned { puzzle3d_fill_build_scope() } else { UiDirtyScope::None };
}
