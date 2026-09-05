//! ⚙️ `set-chunk-size` command.

use crate::editor::puzzle3d::{puzzle3d_absolute_or_delta, Puzzle3dActionCtx};
use dsl::os_pack::json::Value;

pub fn set_chunk_size(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle3d_absolute_or_delta(args, ctx.scene.runtime.chunk_size) {
        ctx.scene.runtime.chunk_size = value.max(1.0);
    }
}
