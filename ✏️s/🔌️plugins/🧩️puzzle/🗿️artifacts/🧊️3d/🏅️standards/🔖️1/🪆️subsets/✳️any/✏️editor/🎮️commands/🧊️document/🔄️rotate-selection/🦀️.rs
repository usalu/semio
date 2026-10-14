//! 🔄️ `rotate-selection` command.

use semio_s_artifact_puzzle_3d_editor_session::Puzzle3dActionCtx;
use semio_framework_pack_json::Value;

/// 🔄️ One gumball rotate `{ids?, ax, ay, az, angle}` as ONE transform-tool transaction yielding `rotate-selection`.
pub fn rotate_selection(ctx: &mut Puzzle3dActionCtx<'_>, args: Option<&Value>) {
    ctx.commit_gumball("rotateSelection", args);
}
