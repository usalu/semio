//! 📦️ `add-target-volume` command.

use crate::editor::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, PUZZLE3D_ID_COUNTER};
use crate::standards::v1::subsets::any::schema::mutations::{create_target_volume, Puzzle3dMutation};
use semio_framework_pack_json::Value;
use std::sync::atomic::Ordering;

/// 📦️ The one `create-target-volume` that places a grid-snapped volume at `origin`, sized by the utility's own W/D/H
/// voxel dimensions.
pub fn add_target_volume_mutation(origin: [f64; 3], grid_spacing: f64, voxel_dims: [u32; 3]) -> Puzzle3dMutation {
    let grid_spacing = grid_spacing.max(0.1);
    let snapped = [(origin[0] / grid_spacing).round() * grid_spacing, (origin[1] / grid_spacing).round() * grid_spacing, (origin[2] / grid_spacing).round() * grid_spacing];
    let [w, d, h] = voxel_dims;
    let scale = crate::Puzzle3dScale::Vec3([f64::from(w) * grid_spacing, f64::from(d) * grid_spacing, f64::from(h) * grid_spacing]);
    let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
    create_target_volume(crate::Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false }, None)
}

/// 📦️ Places one grid-snapped target volume at `args.origin`. A dispatch that carries no usable `origin` answers with a
/// localized notice instead of returning silently — the Volume Brush's Alt+click was indistinguishable from a dead
/// gesture otherwise (`📓️2026-09-11-wave-B1-battery-extension.md` §5 defect 12).
pub fn add_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(origin) = args.and_then(|value| value.get("origin")).and_then(value_as_vec3) else {
        ctx.notice(|labels| labels.target_volume_origin_required.as_str());
        ctx.abort = true;
        return;
    };
    let mutation = add_target_volume_mutation(origin, ctx.scene.runtime.grid_spacing, ctx.scene.runtime.voxel_dims);
    ctx.artifact_mutations.push(mutation);
}
