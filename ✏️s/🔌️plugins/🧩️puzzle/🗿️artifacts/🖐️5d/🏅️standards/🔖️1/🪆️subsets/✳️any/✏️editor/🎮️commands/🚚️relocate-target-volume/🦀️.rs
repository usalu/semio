//! 🚚️ `relocate-target-volume` command.

use crate::editor::puzzle5d::{puzzle5d_value_as_f64_3, Puzzle5dActionCtx};
use dsl::os_pack::json::Value;

/// 🚚️ Absolute pose push from the transform gumball for one UNLOCKED target volume — origin,
/// orientation and extent in one gesture, each arm written only when the host sent it.
pub fn relocate_target_volume(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let volume_id = args.and_then(|value| value.get("volumeId")).and_then(Value::as_str).unwrap_or("");
    let after = args.and_then(|value| value.get("after"));
    let (Some(volume), Some(after)) = (ctx.scene.document.target_volumes.iter_mut().find(|volume| volume.id == volume_id && !volume.locked), after) else {
        return;
    };
    if let Some(origin) = after.get("position").and_then(puzzle5d_value_as_f64_3) {
        volume.origin = origin;
    }
    if let Some(values) = after.get("quaternion").and_then(Value::as_array).filter(|values| values.len() >= 4) {
        volume.orientation = Some([values[0].as_f64().unwrap_or(0.0), values[1].as_f64().unwrap_or(0.0), values[2].as_f64().unwrap_or(0.0), values[3].as_f64().unwrap_or(1.0)]);
    }
    if let Some(values) = after.get("scale").and_then(Value::as_array).filter(|values| values.len() >= 3) {
        volume.scale = Some(serde_json::json!([values[0].as_f64().unwrap_or(1.0), values[1].as_f64().unwrap_or(1.0), values[2].as_f64().unwrap_or(1.0)]));
    }
}
