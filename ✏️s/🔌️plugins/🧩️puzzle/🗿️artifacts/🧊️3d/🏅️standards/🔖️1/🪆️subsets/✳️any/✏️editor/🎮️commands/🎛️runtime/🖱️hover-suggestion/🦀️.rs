//! 🖌️ `hover-suggestion` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

pub fn hover_suggestion(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    if let Some(index) = args.and_then(|value| value.get("index")).and_then(|value| value.as_u64()) {
        ctx.scene.runtime.brush_candidate_index = index as usize;
    }
}
