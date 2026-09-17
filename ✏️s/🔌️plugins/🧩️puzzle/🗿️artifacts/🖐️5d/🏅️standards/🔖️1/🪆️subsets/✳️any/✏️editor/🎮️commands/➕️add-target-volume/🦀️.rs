//! ➕️ `add-target-volume` command.

use crate::editor::puzzle5d::{puzzle5d_value_as_f64_3, Puzzle5dActionCtx, Puzzle5dFreshIds, Puzzle5dTargetVolume};
use dsl::os_pack::json::Value;

/// 🧊️ Places one grid-snapped target volume at `args.origin`, sized by the world window's own W/D/H
/// voxel dimensions. A dispatch that carries no usable `origin` answers with a localized notice
/// instead of returning silently — the Volume Brush's Alt+click would be indistinguishable from a
/// dead gesture otherwise (the same defect puzzle 3d fixed in `📓️2026-09-11-wave-B1` §5 defect 12).
pub fn add_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(origin) = args.and_then(|value| value.get("origin")).and_then(puzzle5d_value_as_f64_3) else {
        ctx.notice(|labels| labels.target_volume_origin_required.as_str());
        ctx.abort = true;
        return;
    };
    let grid_spacing = ctx.scene.runtime.grid_spacing.max(0.1);
    let snapped = [0, 1, 2].map(|axis| (origin[axis] / grid_spacing).round() * grid_spacing);
    let [w, d, h] = ctx.scene.runtime.voxel_dims;
    let scale = serde_json::json!([f64::from(w) * grid_spacing, f64::from(d) * grid_spacing, f64::from(h) * grid_spacing]);
    let id = Puzzle5dFreshIds::from_document(&ctx.scene.document).next_target_volume();
    ctx.scene.document.target_volumes.push(Puzzle5dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false });
}
