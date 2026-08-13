//! 🔭️ `set-depth-variable` command.

use crate::apps::puzzle3d::modes::edit::options::lod::{PUZZLE3D_LOD_SLIDER_MAX, PUZZLE3D_LOD_SLIDER_MIN};
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub fn set_depth_variable(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.lod_depth_variable);
}
