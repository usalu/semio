//! 🔭️ `set-manual` command.

use crate::editor::puzzle3d::modes::edit::options::lod::{PUZZLE3D_LOD_SLIDER_MAX, PUZZLE3D_LOD_SLIDER_MIN};
use crate::editor::puzzle3d::Puzzle3dActionCtx;
use serde_json::Value;

pub async fn set_manual(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
    }
}
