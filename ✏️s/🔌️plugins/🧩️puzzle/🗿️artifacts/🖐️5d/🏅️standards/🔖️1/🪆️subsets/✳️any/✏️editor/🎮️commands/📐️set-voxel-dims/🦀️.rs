//! 📐️ `set-voxel-dims` command.

use crate::editor::puzzle5d::{Puzzle5dActionCtx, PUZZLE5D_VOXEL_DIM_MAX, PUZZLE5D_VOXEL_DIM_MIN};
use dsl::os_pack::json::Value;

/// 🧊️ One axis of the Volume Brush's voxel extent, in grid-spacing units, clamped into the declared
/// `[1, 64]` band the world-window config schema states. An unknown axis writes nothing.
pub fn set_voxel_dims(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    let axis = args.and_then(|value| value.get("axis")).and_then(Value::as_str).unwrap_or("");
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(Value::as_f64) {
        let dimension = value.clamp(PUZZLE5D_VOXEL_DIM_MIN, PUZZLE5D_VOXEL_DIM_MAX).round() as u32;
        match axis {
            "w" => ctx.scene.runtime.voxel_dims[0] = dimension,
            "d" => ctx.scene.runtime.voxel_dims[1] = dimension,
            "h" => ctx.scene.runtime.voxel_dims[2] = dimension,
            _ => {}
        }
    }
}
