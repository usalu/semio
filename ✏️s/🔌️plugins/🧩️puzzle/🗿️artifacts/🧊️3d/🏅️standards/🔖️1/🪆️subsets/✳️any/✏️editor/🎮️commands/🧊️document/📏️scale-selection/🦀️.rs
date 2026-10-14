//! 📏️ `scale-selection` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

/// 📏️ One gumball scale `{ids?, sx, sy, sz}` as ONE transform-tool transaction yielding `scale-selection`.
pub fn scale_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("scaleSelection", args);
}
