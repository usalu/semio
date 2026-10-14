//! 🌐️ `set-spacing` command.

use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_absolute_or_delta, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx};
use semio_framework_pack_json::Value;

pub fn set_spacing(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.grid_spacing) {
        ctx.scene.runtime.grid_spacing = value.max(0.1);
    }
}
