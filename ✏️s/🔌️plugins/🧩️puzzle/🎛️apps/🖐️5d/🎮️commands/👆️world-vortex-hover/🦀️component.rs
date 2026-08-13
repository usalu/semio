//! 👆️ `world-vortex-hover` command.

use crate::apps::puzzle5d::config::puzzle5d_clear_non_grip_selection;
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;

/// 🌀️ Hovering a grip re-drives the precompute session only while the brush utility is active.
pub fn world_vortex_hover(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| SelectionSet::from_ids(vec![full_id.to_string()])).unwrap_or_default();
    if ctx.scene.active_utility == "brush" && !ctx.scene.runtime.selection.grip_ids.is_empty() {
        ctx.app.drive_precompute(ctx.scene);
    }
}
