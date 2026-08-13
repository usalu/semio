//! 👆️ `world-vortex-select` command.

use crate::apps::puzzle5d::config::puzzle5d_clear_non_grip_selection;
use crate::apps::puzzle5d::Puzzle5dActionCtx;
use semio_framework_plugin::SelectionSet;
use serde_json::Value;

pub fn world_vortex_select(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(full_id) = args.and_then(|value| value.get("fullId")).and_then(|value| value.as_str()) else {
        return;
    };
    puzzle5d_clear_non_grip_selection(&mut ctx.scene.runtime.selection);
    ctx.scene.runtime.selection.grip_ids = SelectionSet::from_ids(vec![full_id.to_string()]);
    ctx.app.drive_precompute(ctx.scene);
}
