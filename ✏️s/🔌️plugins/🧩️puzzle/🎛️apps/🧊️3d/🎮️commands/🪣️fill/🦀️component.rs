//! 🪣️ Puzzle 3d play app commands — the Fill tool: committing a count from the slider (always
//! clamped to what planning has actually produced), the background planning tick, and the nested
//! object/vortex distribution weights that steer which kinds the planner reaches for.

use crate::apps::puzzle3d::{
    apply_puzzle3d_fill_count, puzzle3d_ensure_catalog_kind_weights, puzzle3d_fill_build_scope, puzzle3d_fill_options_scope, puzzle3d_fill_tool_active, puzzle3d_kind_ids, puzzle3d_normalize_kind_weight_group, sync_precompute_weights,
    Puzzle3dActionCtx, PUZZLE3D_FILL_COUNT_MAX,
};
use crate::artifacts::puzzle3d::engine::PrecomputeLane;
use semio_framework_core::kernel::UiDirtyScope;
use serde_json::Value;

pub fn set_fill_count(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let count = args.and_then(|value| value.get("count").or_else(|| value.get("value"))).and_then(|value| value.as_f64()).map_or(0, |value| value.round().max(0.0) as u32).min(PUZZLE3D_FILL_COUNT_MAX);
    apply_puzzle3d_fill_count(&mut ctx.app.precompute.borrow_mut(), ctx.scene, count);
    *ctx.ui_scope = puzzle3d_fill_build_scope();
}

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

/// 🎲️ `setObjectKindWeight`/`setVortexKindWeight` share one arm. Object weights live on their own
/// simplex; a vortex slider nested under an object row carries the JOINT `P(object)×P(vortex)` value
/// and is converted back to the relative `P(vortex)` on the shared vortex simplex before normalizing.
pub fn set_kind_weight(ctx: &mut Puzzle3dActionCtx<'_>, action: &str, args: Option<&Value>) {
    let kind_id = args.and_then(|v| v.get("kindId")).and_then(|v| v.as_str()).unwrap_or("");
    let value = args.and_then(|v| v.get("value")).and_then(|v| v.as_f64()).unwrap_or(1.0).clamp(0.0, 1.0);
    let object_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "objects");
    let vortex_ids = puzzle3d_kind_ids(&ctx.scene.fixture, "vortices");
    puzzle3d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.object_kind_weights, &object_ids);
    puzzle3d_ensure_catalog_kind_weights(&mut ctx.scene.runtime.vortex_kind_weights, &vortex_ids);
    if action == "setObjectKindWeight" {
        ctx.scene.runtime.object_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.object_kind_weights, &object_ids, kind_id, value);
    } else if let Some(object_kind_id) = args.and_then(|v| v.get("objectKindId")).and_then(|v| v.as_str()) {
        let object_weight = ctx.scene.runtime.object_kind_weights.get(object_kind_id).copied().unwrap_or(0.0);
        if object_weight > f64::EPSILON {
            let relative = (value / object_weight).clamp(0.0, 1.0);
            ctx.scene.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &vortex_ids, kind_id, relative);
        }
        // 🚫️ Parent object weight is 0 — joint contribution is always 0; ignore vortex edits.
    } else {
        ctx.scene.runtime.vortex_kind_weights = puzzle3d_normalize_kind_weight_group(&ctx.scene.runtime.vortex_kind_weights, &vortex_ids, kind_id, value);
    }
    sync_precompute_weights(&mut ctx.app.precompute.borrow_mut(), ctx.scene);
    *ctx.ui_scope = puzzle3d_fill_options_scope();
}
