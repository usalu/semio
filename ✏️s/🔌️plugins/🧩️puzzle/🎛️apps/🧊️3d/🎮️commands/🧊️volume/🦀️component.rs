//! 🧊️ Puzzle 3d play app commands — target volumes: the oriented boxes that constrain where the Fill
//! tool may place. The Volume Brush paints grid-snapped voxel-sized ones; the transform gumball (via
//! `relocateTargetVolume`) edits arbitrary oriented ones.

use crate::apps::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, Puzzle3dTargetVolume, PUZZLE3D_ID_COUNTER};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

pub fn add_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
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

pub fn delete_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(id) = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()) {
        ctx.scene.fixture.target_volumes.retain(|volume| volume.id != id);
    }
}

pub fn set_target_volume_flag(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let id = args.and_then(|value| value.get("id")).and_then(|value| value.as_str()).unwrap_or("");
    let flag = args.and_then(|value| value.get("flag")).and_then(|value| value.as_str()).unwrap_or("");
    let value = args.and_then(|value| value.get("value")).and_then(|value| value.as_bool()).unwrap_or(false);
    if let Some(volume) = ctx.scene.fixture.target_volumes.iter_mut().find(|volume| volume.id == id) {
        match flag {
            "hidden" => volume.hidden = value,
            "locked" => volume.locked = value,
            _ => {}
        }
    }
}

/// 🚚️ Absolute pose push from the gumball for one unlocked target volume.
pub fn relocate_target_volume(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let volume_id = args.and_then(|value| value.get("volumeId")).and_then(|value| value.as_str()).unwrap_or("");
    let after = args.and_then(|value| value.get("after"));
    let (Some(volume), Some(after)) = (ctx.scene.fixture.target_volumes.iter_mut().find(|volume| volume.id == volume_id && !volume.locked), after) else {
        return;
    };
    if let Some(origin) = after.get("position").and_then(value_as_vec3) {
        volume.origin = origin;
    }
    if let Some(values) = after.get("quaternion").and_then(|value| value.as_array()).filter(|values| values.len() >= 4) {
        volume.orientation = Some([values[0].as_f64().unwrap_or(0.0), values[1].as_f64().unwrap_or(0.0), values[2].as_f64().unwrap_or(0.0), values[3].as_f64().unwrap_or(1.0)]);
    }
    if let Some(scale) = after.get("scale").and_then(|value| value.as_array()).filter(|values| values.len() >= 3) {
        volume.scale = Some(json!([scale[0].as_f64().unwrap_or(1.0), scale[1].as_f64().unwrap_or(1.0), scale[2].as_f64().unwrap_or(1.0),]));
    }
}
