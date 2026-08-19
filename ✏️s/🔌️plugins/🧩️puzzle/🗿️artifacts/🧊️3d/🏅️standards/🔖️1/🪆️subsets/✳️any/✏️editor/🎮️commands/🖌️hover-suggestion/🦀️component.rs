//! 🖌️ `hover-suggestion` command.

use serde_json::Value;
use crate::editor::puzzle3d::Puzzle3dActionCtx;

pub async fn hover_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
        ctx.scene.runtime.brush_candidate_index = index as usize;
    }
}
