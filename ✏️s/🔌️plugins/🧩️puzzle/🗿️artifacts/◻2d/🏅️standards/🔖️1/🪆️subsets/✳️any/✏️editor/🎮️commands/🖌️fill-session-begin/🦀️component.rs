//! 🖌️ `fill-session-begin` command.

use crate::editor::puzzle2d::Puzzle2dActionCtx;
use serde_json::Value;

pub async fn fill_session_begin(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    let max_count = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()).unwrap_or(0) as u32;
    let seed = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()).unwrap_or(1);
    crate::editor::puzzle2d::commands::set_fill_count::begin_fill_job(ctx, max_count, seed);
}
