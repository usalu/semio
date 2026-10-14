//! ⚙️ `set-proximity-radius` command.

use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_absolute_or_delta, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx};
use semio_framework_pack_json::Value;

pub fn set_proximity_radius(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.proximity_radius) {
        ctx.scene.runtime.proximity_radius = value.max(0.0);
    }
}
