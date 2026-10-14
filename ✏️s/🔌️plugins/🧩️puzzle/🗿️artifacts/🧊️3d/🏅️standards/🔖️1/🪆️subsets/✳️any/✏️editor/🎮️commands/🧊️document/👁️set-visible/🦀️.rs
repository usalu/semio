//! 🌐️ `set-visible` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

pub fn set_visible(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.scene.runtime.grid_visible = args.and_then(|value| value.get("pressed")).and_then(|value| value.as_bool()).unwrap_or(!ctx.scene.runtime.grid_visible);
}
