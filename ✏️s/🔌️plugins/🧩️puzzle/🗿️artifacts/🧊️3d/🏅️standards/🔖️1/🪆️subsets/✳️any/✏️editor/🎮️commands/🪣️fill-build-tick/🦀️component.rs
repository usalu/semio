//! 🪣️ `fill-build-tick` command.

use crate::artifacts::puzzle3d::schema::PrecomputeLane;
use semio_framework::kernel::UiDirtyScope;
use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use crate::editor::puzzle3d::puzzle3d_fill_build_scope;
use crate::editor::puzzle3d::puzzle3d_fill_tool_active;

/// 🪣️ No catch-up `setFillCount` dispatch here: `apply_puzzle3d_fill_count` always clamps the
/// committed count to what's available at commit time, so `fill_count` can never run ahead of
/// `applied_count` — a slider can only request what `render`'s reveal-tagged instances already show.
/// Ticks purely advance background planning, and only claim a UI refresh when they actually produced
/// something new.
pub fn fill_build_tick(ctx: &mut Puzzle3dActionCtx<'_>) {
    if !puzzle3d_fill_tool_active(ctx.config) {
        *ctx.ui_scope = UiDirtyScope::None;
        return;
    }
    let available_before = ctx.app.precompute.borrow().fill_available_count();
    let done_before = ctx.app.precompute.borrow().fill_is_done();
    ctx.app.precompute.borrow_mut().precompute_step_lane(PrecomputeLane::Fill, 8);
    let available_after = ctx.app.precompute.borrow().fill_available_count();
    let done_after = ctx.app.precompute.borrow().fill_is_done();
    *ctx.ui_scope = if available_after != available_before || done_after != done_before { puzzle3d_fill_build_scope() } else { UiDirtyScope::None };
}
