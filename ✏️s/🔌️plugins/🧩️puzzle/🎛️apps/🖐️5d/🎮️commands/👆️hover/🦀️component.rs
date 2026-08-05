//! 👆️ Puzzle 5d play app commands — the pointer-proximity channel: part hover from either surface and
//! the grip (vortex) hover/select the brush engine keys its candidate cache on.

use crate::apps::puzzle5d::config::puzzle5d_clear_non_grip_selection;
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;

pub fn world_hover(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_part_id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).map(str::to_string);
}

pub fn set_hover(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.hovered_part_id = args.and_then(|value| value.get("objectId")).and_then(|value| value.as_str()).map(str::to_string);
}

/// 🌀️ Hovering a grip re-drives the precompute session only while the brush utility is active.
pub fn world_vortex_hover(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.selection.grip_ids = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()).map(|full_id| SelectionSet::from_ids(vec![full_id.to_string()])).unwrap_or_default();
    if ctx.scene.active_utility == "brush" && !ctx.scene.runtime.selection.grip_ids.is_empty() {
        ctx.app.drive_precompute(ctx.scene);
    }
}

pub fn world_vortex_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) else {
        return;
    };
    puzzle5d_clear_non_grip_selection(&mut ctx.scene.runtime.selection);
    ctx.scene.runtime.selection.grip_ids = SelectionSet::from_ids(vec![full_id.to_string()]);
    ctx.app.drive_precompute(ctx.scene);
}
