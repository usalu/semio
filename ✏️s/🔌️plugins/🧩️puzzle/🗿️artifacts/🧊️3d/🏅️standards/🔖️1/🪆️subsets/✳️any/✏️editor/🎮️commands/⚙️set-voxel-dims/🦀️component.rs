//! ⚙️ `set-voxel-dims` command.

use crate::editor::puzzle3d::{puzzle3d_absolute_or_delta, sync_precompute_session, Puzzle3dActionCtx, PUZZLE3D_VORTEX_DIRECTION_INWARDS, PUZZLE3D_VORTEX_DIRECTION_OUTWARDS, PUZZLE3D_VORTEX_SHOW_ALWAYS, PUZZLE3D_VORTEX_SHOW_SELECTED};
use serde_json::Value;

pub fn set_voxel_dims(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    let axis = args.and_then(|value| value.get("axis")).and_then(|value| value.as_str()).unwrap_or("");
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        let dimension = value.max(1.0).round() as u32;
        match axis {
            "w" => ctx.scene.runtime.voxel_dims[0] = dimension,
            "d" => ctx.scene.runtime.voxel_dims[1] = dimension,
            "h" => ctx.scene.runtime.voxel_dims[2] = dimension,
            _ => {}
        }
    }
}
