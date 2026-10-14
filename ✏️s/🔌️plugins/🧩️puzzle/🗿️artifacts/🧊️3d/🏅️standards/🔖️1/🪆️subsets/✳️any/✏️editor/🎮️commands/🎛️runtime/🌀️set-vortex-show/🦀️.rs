//! ⚙️ `set-vortex-show` command.

use {semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_VORTEX_SHOW_ALWAYS, semio_s_artifact_puzzle_3d_editor_config::PUZZLE3D_VORTEX_SHOW_SELECTED};
use semio_framework_pack_json::Value;

pub fn set_vortex_show(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(mode) = args.and_then(|value| value.get("value")).and_then(|value| value.as_str()) {
        if mode == PUZZLE3D_VORTEX_SHOW_ALWAYS || mode == PUZZLE3D_VORTEX_SHOW_SELECTED {
            ctx.scene.runtime.vortex_show = mode.into();
        }
    }
}
