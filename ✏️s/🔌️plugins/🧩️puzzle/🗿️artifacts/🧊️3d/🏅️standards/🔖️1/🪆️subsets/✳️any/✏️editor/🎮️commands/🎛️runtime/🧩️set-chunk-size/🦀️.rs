//! ⚙️ `set-chunk-size` command.

use {semio_s_artifact_puzzle_3d_editor_snapshot::puzzle3d_absolute_or_delta, semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx};
use semio_framework_pack_json::Value;

pub fn set_chunk_size(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.chunk_size) {
        ctx.scene.runtime.chunk_size = value.max(1.0);
    }
}
