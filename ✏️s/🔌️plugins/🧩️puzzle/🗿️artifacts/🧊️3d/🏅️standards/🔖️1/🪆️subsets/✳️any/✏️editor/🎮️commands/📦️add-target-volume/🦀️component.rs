//! 📦️ `add-target-volume` command.

use crate::editor::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, Puzzle3dTargetVolume, PUZZLE3D_ID_COUNTER};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

pub async fn add_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let Some(origin) = args.and_then(|value| value.get("origin")).and_then(value_as_vec3) else {
        return;
    };
    let grid_spacing = ctx.scene.runtime.grid_spacing.max(0.1);
    let snapped = [(origin[0] / grid_spacing).round() * grid_spacing, (origin[1] / grid_spacing).round() * grid_spacing, (origin[2] / grid_spacing).round() * grid_spacing];
    let [w, d, h] = ctx.scene.runtime.voxel_dims;
    let scale = json!([w as f64 * grid_spacing, d as f64 * grid_spacing, h as f64 * grid_spacing]);
    let id = format!("target-volume-{}", PUZZLE3D_ID_COUNTER.fetch_add(1, Ordering::Relaxed));
    ctx.scene.fixture.target_volumes.push(Puzzle3dTargetVolume { id, origin: snapped, orientation: None, scale: Some(scale), hidden: false, locked: false });
}
