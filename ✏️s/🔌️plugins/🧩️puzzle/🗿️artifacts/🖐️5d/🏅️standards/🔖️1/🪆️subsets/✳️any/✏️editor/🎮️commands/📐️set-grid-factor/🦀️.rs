//! 📐️ `set-grid-factor` command.

use semio_s_artifact_puzzle_5d_mut_any_8::editor::puzzle5d::entities::{puzzle5d_absolute_or_delta, PUZZLE5D_GRID_FACTOR_MAX, PUZZLE5D_GRID_FACTOR_MIN};
use semio_s_artifact_puzzle_5d_interaction::editor::puzzle5d::interaction::Puzzle5dActionCtx;
use semio_framework_pack_json::Value;

/// 📐️ The board pane's snap factor: an absolute `value` (typed entry) or a `delta` (stepper nudge),
/// clamped into the declared band — the window-config schema states `gridFactor` as
/// `exclusiveMinimum: 0`, so an unclamped write is a state the persisted partition refuses.
pub fn set_grid_factor(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle5d_absolute_or_delta(args, ctx.scene.runtime.grid_factor) {
        ctx.scene.runtime.grid_factor = value.clamp(PUZZLE5D_GRID_FACTOR_MIN, PUZZLE5D_GRID_FACTOR_MAX);
    }
}
