//! 🎥️ `set-camera` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

pub fn set_camera(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(camera) = args.and_then(|value| value.get("camera")) {
        if let Ok(parsed) = semio_framework_value::FromValue::from_value(semio_framework_pack_json::to_dsl_value(camera)) {
            ctx.scene.runtime.camera = parsed;
        }
    }
}
