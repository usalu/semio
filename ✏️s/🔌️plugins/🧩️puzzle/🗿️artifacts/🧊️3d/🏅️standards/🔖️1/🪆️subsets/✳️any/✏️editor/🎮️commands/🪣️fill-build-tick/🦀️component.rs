//! 🪣️ `fill-build-tick` command.

use crate::artifacts::puzzle3d::Puzzle3dMutation;
use crate::editor::puzzle3d::config::{Puzzle3dConfig, Puzzle3dConfigMutation};
use crate::editor::puzzle3d::precompute::FILL_JOB_KIND;
use crate::editor::puzzle3d::puzzle3d_fill_build_scope;
use crate::editor::puzzle3d::puzzle3d_fill_tool_active;
use crate::editor::puzzle3d::{Puzzle3dActionCtx, Puzzle3dPlayApp};
use semio_framework::kernel::UiDirtyScope;
use semio_framework_plugin::kernel::{Effect, JobPlacement};
use semio_framework_plugin::Emit;

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

/// ♻️ Polls a restored immutable fill plan without rebuilding the document-shaped scene bridge.
pub fn fill_build_tick_cached(app: &Puzzle3dPlayApp, config: &Puzzle3dConfig) -> Option<Emit<Puzzle3dMutation, Puzzle3dConfigMutation>> {
    if !puzzle3d_fill_tool_active(config) {
        return Some(Emit { ui_scope: UiDirtyScope::None, ..Default::default() });
    }
    let mut precompute = app.precompute.borrow_mut();
    if !precompute.restore_persisted_fill(&config.fill_checkpoint) {
        return None;
    }
    precompute.set_fill_applied_count(config.fill_applied_count);
    let changed = precompute.poll_fill_job();
    let spawn = precompute.enqueue_fill_job();
    let checkpoint = precompute.fill_checkpoint_bytes();
    drop(precompute);
    let spawned = spawn.is_some();
    let effects = spawn.into_iter().map(|(job, input)| Effect::SpawnJob { job, kind: FILL_JOB_KIND.into(), input, placement: JobPlacement::Isolated }).collect();
    let config_mutations = if checkpoint == config.fill_checkpoint {
        Vec::new()
    } else {
        let mut next = config.clone();
        next.fill_checkpoint = checkpoint;
        vec![Puzzle3dConfigMutation::Snapshot { config: next }]
    };
    Some(Emit { config_mutations, effects, ui_scope: if changed || spawned { puzzle3d_fill_build_scope() } else { UiDirtyScope::None }, ..Default::default() })
}
