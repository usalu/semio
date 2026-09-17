//! 🚚️ `relocate-target-region` command.

use crate::editor::puzzle2d::{puzzle2d_relocate_target_region, Puzzle2dActionCtx};
use serde_json::Value;

/// 🚚️ Absolute pose push from the board gumball for one unlocked target region — `after.position`
/// carries the new minimum corner and `after.size` the new extent, the flat stand-in for puzzle3d's
/// `relocateTargetVolume` quaternion-and-scale payload.
pub fn relocate_target_region(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let region_id = args.and_then(|value| value.get("regionId")).and_then(Value::as_str).unwrap_or("");
    let Some(after) = args.and_then(|value| value.get("after")) else {
        return;
    };
    if region_id.is_empty() {
        return;
    }
    puzzle2d_relocate_target_region(&mut ctx.scene.fixture, region_id, after);
}
