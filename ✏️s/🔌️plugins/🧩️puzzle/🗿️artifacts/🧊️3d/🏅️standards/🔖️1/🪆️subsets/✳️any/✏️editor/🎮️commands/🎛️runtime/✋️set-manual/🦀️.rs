//! 🔭️ `set-manual` command.

use semio_s_artifact_puzzle_3d_editor_modes::options::lod::PUZZLE3D_LOD_SLIDER_MAX;

use semio_s_artifact_puzzle_3d_editor_modes::options::lod::PUZZLE3D_LOD_SLIDER_MIN;
use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

pub fn set_manual(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = args.and_then(|value| value.get("value")).and_then(|value| value.as_f64()) {
        ctx.scene.runtime.lod_manual = value.clamp(PUZZLE3D_LOD_SLIDER_MIN, PUZZLE3D_LOD_SLIDER_MAX);
    }
}
