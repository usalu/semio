//! 🖌️ `fill-session-begin` command.

use crate::editor::puzzle2d::commands::set_fill_count::Puzzle2dFillActionCtx;
use serde_json::Value;

pub fn fill_session_begin(ctx: &mut Puzzle2dFillActionCtx<'_>, args: Option<&Value>) {
    let Some(max_count) = args.and_then(|value| value.get("maxCount")).and_then(|value| value.as_u64()) else {
        crate::editor::puzzle2d::commands::set_fill_count::reject_fill_request(ctx, "puzzle2d-fill-start-count");
        return;
    };
    let Ok(max_count) = u32::try_from(max_count) else {
        crate::editor::puzzle2d::commands::set_fill_count::reject_fill_request(ctx, "puzzle2d-fill-start-count");
        return;
    };
    let Some(seed) = args.and_then(|value| value.get("seed")).and_then(|value| value.as_u64()) else {
        crate::editor::puzzle2d::commands::set_fill_count::reject_fill_request(ctx, "puzzle2d-fill-start-seed");
        return;
    };
    crate::editor::puzzle2d::commands::set_fill_count::begin_fill_job(ctx, max_count, seed);
}
