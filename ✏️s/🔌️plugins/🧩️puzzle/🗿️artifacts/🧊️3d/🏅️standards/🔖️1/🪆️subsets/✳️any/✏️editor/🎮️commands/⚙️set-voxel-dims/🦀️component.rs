//! ⚙️ `set-voxel-dims` command.

use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_voxel_dims(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
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
