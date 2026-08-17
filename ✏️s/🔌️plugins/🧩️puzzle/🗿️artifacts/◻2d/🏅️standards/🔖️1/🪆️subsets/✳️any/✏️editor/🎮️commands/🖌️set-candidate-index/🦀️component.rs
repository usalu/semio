//! 🖌️ `set-candidate-index` command.

use crate::editor::puzzle2d::{puzzle2d_window_and_engagements_scope, Puzzle2dActionCtx};
use serde_json::Value;

pub fn set_candidate_index(ctx: &mut Puzzle2dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
        ctx.host.borrow_mut().brush_set_candidate_index(index as usize);
        ctx.scene.runtime.brush_candidate_index = index as usize;
        *ctx.ui_scope = puzzle2d_window_and_engagements_scope();
    }
}
