//! 🧱️ `set-chunk-size` command.

use crate::editor::puzzle5d::{puzzle5d_absolute_or_delta, Puzzle5dActionCtx, PUZZLE5D_CHUNK_SIZE_MAX, PUZZLE5D_CHUNK_SIZE_MIN};
use dsl::os_pack::json::Value;

/// 🧱️ The broad-phase chunk edge (m) the placement search buckets parts into — an absolute `value` or
/// a `delta` nudge, clamped into the band the ⚙️settings stepper declares. A zero chunk would bucket
/// every part into one cell, so the floor is a real minimum rather than `0`.
pub fn set_chunk_size(ctx: &mut Puzzle5dActionCtx<'_>, args: Option<&Value>) {
    if let Some(value) = puzzle5d_absolute_or_delta(args, ctx.scene.runtime.chunk_size) {
        ctx.scene.runtime.chunk_size = value.clamp(PUZZLE5D_CHUNK_SIZE_MIN, PUZZLE5D_CHUNK_SIZE_MAX);
    }
}
