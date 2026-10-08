//! ➕️ `add-target-volume` command.

use crate::editor::puzzle5d::{puzzle5d_value_as_f64_3, Puzzle5dActionCtx, Puzzle5dFreshIds};
use crate::standards::v1::subsets::any::schema::mutations::{create_target_volume, Puzzle5dMutation};
use semio_framework_pack_json::Value;

/// 🧊️ The one `create-target-volume` that places a grid-snapped volume `id` at `origin`, sized by the world window's own
/// W/D/H voxel dimensions.
pub fn add_target_volume_mutation(id: String, origin: [f64; 3], grid_spacing: f64, voxel_dims: [u32; 3]) -> Puzzle5dMutation {
    let grid_spacing = grid_spacing.max(0.1);
    let snapped = [0, 1, 2].map(|axis| (origin[axis] / grid_spacing).round() * grid_spacing);
    let [w, d, h] = voxel_dims;
    let scale = crate::Puzzle5dScale::Vec3([f64::from(w) * grid_spacing, f64::from(d) * grid_spacing, f64::from(h) * grid_spacing]);
    create_target_volume(crate::Puzzle5dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false }, None)
}

/// 🧊️ Places one grid-snapped target volume at `args.origin`, sized by the world window's own W/D/H
/// voxel dimensions, as one `create-target-volume`. A dispatch that carries no usable `origin` answers with a localized
/// notice instead of returning silently — the Volume Brush's Alt+click would be indistinguishable from a dead gesture
/// otherwise (the same defect puzzle 3d fixed in `📓️2026-09-11-wave-B1` §5 defect 12).
pub fn add_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let Some(origin) = args.and_then(|value| value.get("origin")).and_then(puzzle5d_value_as_f64_3) else {
        ctx.notice(|labels| labels.target_volume_origin_required.as_str());
        ctx.abort = true;
        return;
    };
    let id = Puzzle5dFreshIds::from_document(&ctx.scene.document).next_target_volume();
    ctx.artifact_mutations.push(add_target_volume_mutation(id, origin, ctx.scene.runtime.grid_spacing, ctx.scene.runtime.voxel_dims));
}
