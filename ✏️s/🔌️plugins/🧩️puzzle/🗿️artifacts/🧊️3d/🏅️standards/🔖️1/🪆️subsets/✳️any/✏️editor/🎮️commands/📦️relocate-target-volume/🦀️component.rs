//! 📦️ `relocate-target-volume` command.

use crate::editor::puzzle3d::{value_as_vec3, Puzzle3dActionCtx, Puzzle3dTargetVolume, PUZZLE3D_ID_COUNTER};
use serde_json::{json, Value};
use std::sync::atomic::Ordering;

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
