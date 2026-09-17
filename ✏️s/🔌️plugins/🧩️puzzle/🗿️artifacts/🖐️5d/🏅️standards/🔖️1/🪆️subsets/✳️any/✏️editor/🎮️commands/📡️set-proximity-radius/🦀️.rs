//! 📡️ `set-proximity-radius` command.

use crate::editor::puzzle5d::{puzzle5d_absolute_or_delta, Puzzle5dActionCtx, PUZZLE5D_PROXIMITY_RADIUS_MAX};
use dsl::os_pack::json::Value;

/// 📡️ How near (m) two open grips must come before a drop auto-connects them — an absolute `value` or
/// a `delta` nudge, clamped into the band the ⚙️settings stepper declares. Session-wide: both panes
/// mean the same distance, so this writes the shared configuration, not one pane's.
pub fn set_proximity_radius(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle5d_absolute_or_delta(args, ctx.scene.runtime.proximity_radius) {
        ctx.scene.runtime.proximity_radius = value.clamp(0.0, PUZZLE5D_PROXIMITY_RADIUS_MAX);
    }
}
