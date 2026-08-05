//! 🔭️ Puzzle 3d play app commands — level of detail: the automatic-zoom and depth-variable toggles
//! plus the manual LOD slider, clamped to the slider's own declared range.

use crate::apps::puzzle3d::modes::edit::options::lod::{PUZZLE3D_LOD_SLIDER_MAX, PUZZLE3D_LOD_SLIDER_MIN};
use crate::apps::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub fn set_automatic(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_automatic = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.lod_automatic);
}

pub fn set_depth_variable(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.lod_depth_variable = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.lod_depth_variable);
}

pub fn set_manual(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
    }
}
